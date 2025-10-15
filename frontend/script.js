const $ = (sel) => document.querySelector(sel);

function logStatus(msg) {
  const el = $("#statusConsole");
  if (!el) return;
  const t = new Date().toLocaleTimeString();
  el.textContent += `\n[${t}] ${msg}`;
  el.scrollTop = el.scrollHeight;
}

// ------- controls -------
function getMode() {
  const el = $("#modeSelect");
  const val = (el?.value || "with_offset").toString().toLowerCase();
  return (val === "without_offset" || val === "ramp") ? val : "with_offset";
}
function getPackets() {
  const n = Number($("#packets")?.value ?? 1);
  return Number.isFinite(n) && n > 0 ? Math.floor(n) : 1;
}
function getPollMs() {
  return Math.max(50, Number($("#pollMs")?.value || 250));
}

// ------- colors -------
function makeColors(n) {
  const out = [];
  for (let i = 0; i < n; i++) {
    const hue = Math.round((360 * i) / Math.max(1, n));
    out.push(`hsl(${hue} 90% 40%)`);
  }
  return out;
}

// ------- chart -------
let chart = null;
let currentChannelCount = 0;


function buildChart(datasetCount, labelsLen) {
  const canvas = $("#channelChart");
  if (!canvas) { console.warn("channelChart canvas not found."); return; }
  const ctx = canvas.getContext("2d");
  if (!ctx) { console.warn("2D context unavailable."); return; }

  const colors = makeColors(datasetCount);
  const datasets = Array.from({ length: datasetCount }, (_, i) => ({
    label: `Ch ${i}`,
    data: Array(labelsLen).fill(null),
    borderColor: colors[i],
    backgroundColor: "transparent",
    borderWidth: 1,
    pointRadius: 0,
    tension: 0.1,
  }));

  if (chart) chart.destroy();
  chart = new Chart(ctx, {
    type: "line",
    data: { labels: Array.from({ length: labelsLen }, (_, i) => i), datasets },
    options: {
      animation: false,
      responsive: true,
      maintainAspectRatio: false,
      interaction: { mode: "nearest", intersect: false },
      scales: {
        x: { title: { display: true, text: "Sample" } },
        y: { title: { display: true, text: "Value" } },
      },
      plugins: { legend: { display: true }},
    },
  });

  currentChannelCount = datasetCount;
  applyAxisScaling(); // apply any user min/max immediately
}

function ensureChartFor(channelCount, maxLen) {
  if (!chart || currentChannelCount !== channelCount) {
    buildChart(channelCount, maxLen);
  } else if (chart.data.labels.length !== maxLen) {
    chart.data.labels = Array.from({ length: maxLen }, (_, i) => i);
    applyAxisScaling(); // preserve scale after label change
  }
}

// ------- scaling -------
function numOrNull(v) {
  if (v === null || v === undefined) return null;
  if (v === "") return null;
  const n = Number(v);
  return Number.isFinite(n) ? n : null;
}

function setScale(axis, minVal, maxVal) {
  if (!chart) return;
  const s = chart.options.scales[axis];
  if (!s) return;

  // Remove min/max when blank; Chart.js will auto-scale
  if (minVal == null) delete s.min; else s.min = minVal;
  if (maxVal == null) delete s.max; else s.max = maxVal;
}

function applyAxisScaling() {
  if (!chart) return;

  // Read inputs
  const xMin = numOrNull($("#xMin")?.value ?? "");
  const xMax = numOrNull($("#xMax")?.value ?? "");
  const yMin = numOrNull($("#yMin")?.value ?? "");
  const yMax = numOrNull($("#yMax")?.value ?? "");

  // Optional sanity checks
  if (xMin != null && xMax != null && xMin >= xMax) {
    logStatus("X scale ignored: Min must be < Max.");
  } else {
    setScale("x", xMin, xMax);
  }

  if (yMin != null && yMax != null && yMin >= yMax) {
    logStatus("Y scale ignored: Min must be < Max.");
  } else {
    setScale("y", yMin, yMax);
  }

  chart.update();
}

// ------- networking -------
async function fetchJson(url, opts) {
  const res = await fetch(url, opts);
  const ct = res.headers.get("content-type") || "";
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`HTTP ${res.status} ${res.statusText} @ ${url}\n${text.slice(0, 200)}`);
  }
  if (!ct.includes("application/json")) {
    const text = await res.text().catch(() => "");
    throw new Error(`Expected JSON but got "${ct || "unknown"}" @ ${url}\n${text.slice(0, 200)}`);
  }
  return res.json();
}

// ------- polling -------
let pollTimer = null;

function updateAllChannels(allChannels) {
  if (!Array.isArray(allChannels) || allChannels.length === 0) return;
  const measIndex = allChannels.length - 1;
  const channelsOnly = allChannels.slice(0, measIndex);

  const maxLen = channelsOnly.reduce((m, arr) => Math.max(m, Array.isArray(arr) ? arr.length : 0), 0);
  ensureChartFor(channelsOnly.length, maxLen);

  if (!chart) return;
  channelsOnly.forEach((arr, i) => {
    const series = Array.isArray(arr) ? arr.slice(0, maxLen) : [];
    while (series.length < maxLen) series.push(null);
    chart.data.datasets[i].data = series;
  });

  applyAxisScaling(); // keep user scaling after each update
  chart.update();
}

async function fetchOnceAndPlot() {
  try {
    const params = new URLSearchParams({ packets: String(getPackets()), mode: getMode() });
    const data = await fetchJson(`/api/stream/once?${params.toString()}`);
    updateAllChannels(data?.channel_data);
  } catch (err) {
    logStatus(`Fetch once failed: ${err.message}`);
    console.error(err);
  }
}

function startPolling() {
  const ms = getPollMs();
  if (pollTimer) clearInterval(pollTimer);
  pollTimer = setInterval(fetchOnceAndPlot, ms);
  logStatus(`Started polling /api/stream/once every ${ms} ms (mode=${getMode()}, packets=${getPackets()})`);
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer);
    pollTimer = null;
    logStatus("Stopped polling.");
  }
}

// ------- start/stop -------
async function onStartStream() {
  try {
    const mode = getMode();
    await fetchJson(`/api/stream/start?mode=${encodeURIComponent(mode)}`, { method: "POST" });
    logStatus(`Backend stream start acknowledged (mode=${mode}).`);
  } catch (e) {
    logStatus(`Stream start request failed: ${e.message}`);
    console.error(e);
  }
  startPolling();
}

async function onStopStream() {
  stopPolling();
  try {
    await fetchJson("/api/stream/stop", { method: "POST" });
    logStatus("Backend stream stop acknowledged.");
  } catch (e) {
    logStatus(`Stream stop request failed: ${e.message}`);
    console.error(e);
  }
}

// ------- boot -------
window.addEventListener("DOMContentLoaded", () => {
  const requiredIds = [
    "modeSelect","packets","pollMs","startStream","stopStream","channelChart","statusConsole",
    "xMin","xMax","yMin","yMax"
  ];
  const missing = requiredIds.filter((id) => !document.getElementById(id));
  if (missing.length) {
    console.warn("Missing expected elements:", missing);
    logStatus(`UI missing elements: ${missing.join(", ")}`);
  }

  $("#startStream")?.addEventListener("click", onStartStream);
  $("#stopStream")?.addEventListener("click", onStopStream);

  $("#modeSelect")?.addEventListener("change", () => logStatus(`Mode set to ${getMode()}`));
  $("#packets")?.addEventListener("change", () => logStatus(`Packets per poll set to ${getPackets()}`));
  $("#pollMs")?.addEventListener("change", () => logStatus(`Polling interval set to ${getPollMs()} ms`));

  // scaling listeners (live)
  ["xMin","xMax","yMin","yMax"].forEach(id => {
    const el = document.getElementById(id);
    el?.addEventListener("input", applyAxisScaling);
    el?.addEventListener("change", applyAxisScaling);
  });

  // first draw
  fetchOnceAndPlot();
});

// ===== LOUPE (magnifier) for Chart.js canvas =====
(function () {
  const chartCanvas = document.getElementById('channelChart');     // your main chart canvas
  const chartWrap   = document.querySelector('.chart-wrap');        // container (position:relative)
  const btnToggle   = document.getElementById('loupeToggle');
  const btnSize     = document.getElementById('loupeSize');

  if (!chartCanvas || !chartWrap || !btnToggle || !btnSize) return;

  let loupeEnabled = false;
  let loupeCanvas = null;      // the floating magnifier <canvas>
  let loupeCtx = null;
  let loupeSize = 140;         // px (diameter)
  let loupeZoom = 2;           // how much to zoom inside the loupe (2×, 3×, …)

  function createLoupe() {
    if (loupeCanvas) return;
    loupeCanvas = document.createElement('canvas');
    loupeCanvas.className = 'loupe-canvas';
    loupeCanvas.width = loupeSize;
    loupeCanvas.height = loupeSize;
    loupeCtx = loupeCanvas.getContext('2d');
    chartWrap.appendChild(loupeCanvas);
    // apply CSS width/height to match current size
    loupeCanvas.style.width = loupeSize + 'px';
    loupeCanvas.style.height = loupeSize + 'px';
  }

  function destroyLoupe() {
    if (!loupeCanvas) return;
    loupeCanvas.remove();
    loupeCanvas = null;
    loupeCtx = null;
  }

  function setLoupeSizeAndZoom(size, zoom) {
    loupeSize = size;
    loupeZoom = zoom;
    if (loupeCanvas) {
      loupeCanvas.width = loupeSize;
      loupeCanvas.height = loupeSize;
      loupeCanvas.style.width = loupeSize + 'px';
      loupeCanvas.style.height = loupeSize + 'px';
    }
    btnSize.textContent = `Loupe: ${loupeSize}px ×${loupeZoom}`;
  }

  function enableLoupe() {
    loupeEnabled = true;
    chartWrap.classList.add('loupe-mode');
    createLoupe();
    // attach listeners
    chartCanvas.addEventListener('mousemove', onMove);
    chartCanvas.addEventListener('mouseleave', onLeave);
    chartCanvas.addEventListener('touchmove', onTouchMove, { passive: false });
    chartCanvas.addEventListener('touchend', onLeave);
  }

  function disableLoupe() {
    loupeEnabled = false;
    chartWrap.classList.remove('loupe-mode');
    // detach listeners
    chartCanvas.removeEventListener('mousemove', onMove);
    chartCanvas.removeEventListener('mouseleave', onLeave);
    chartCanvas.removeEventListener('touchmove', onTouchMove);
    chartCanvas.removeEventListener('touchend', onLeave);
    destroyLoupe();
  }

  function onLeave() {
    if (loupeCanvas) loupeCanvas.style.display = 'none';
  }

  function onTouchMove(e) {
    e.preventDefault();
    if (!e.touches || !e.touches[0]) return;
    const rect = chartCanvas.getBoundingClientRect();
    const x = e.touches[0].clientX - rect.left;
    const y = e.touches[0].clientY - rect.top;
    drawLoupeAt(x, y);
  }

  function onMove(e) {
    const rect = chartCanvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    drawLoupeAt(x, y);
  }

  function drawLoupeAt(x, y) {
    if (!loupeCanvas || !loupeCtx) return;
    loupeCanvas.style.display = 'block';

    // position the floating loupe so its center follows the cursor
    loupeCanvas.style.left = (x - loupeSize / 2) + 'px';
    loupeCanvas.style.top  = (y - loupeSize / 2) + 'px';

    // compute the source rectangle from the main chart canvas we will magnify
    const srcSize = loupeSize / loupeZoom; // smaller area from the source, scaled up into the loupe
    const sx = Math.max(0, Math.min(chartCanvas.width  - srcSize, x * devicePixelRatio - srcSize / 2));
    const sy = Math.max(0, Math.min(chartCanvas.height - srcSize, y * devicePixelRatio - srcSize / 2));

    // clear and draw the zoomed area into the loupe canvas
    loupeCtx.clearRect(0, 0, loupeSize, loupeSize);
    loupeCtx.save();

    // clip to circle to get round “glass” look
    loupeCtx.beginPath();
    loupeCtx.arc(loupeSize / 2, loupeSize / 2, loupeSize / 2 - 2, 0, Math.PI * 2);
    loupeCtx.clip();

    // draw the zoomed patch from the main chart canvas
    // NOTE: use the underlying bitmap size (canvas.width/height account for DPR)
    loupeCtx.imageSmoothingEnabled = true;
    loupeCtx.drawImage(
      chartCanvas,
      sx, sy, srcSize, srcSize,       // source rect (on main canvas)
      0, 0, loupeSize, loupeSize      // destination rect (fill loupe)
    );

    loupeCtx.restore();

    // Optional: crosshair lines inside the loupe
    // loupeCtx.strokeStyle = 'rgba(0,0,0,0.25)';
    // loupeCtx.beginPath();
    // loupeCtx.moveTo(loupeSize/2, 0); loupeCtx.lineTo(loupeSize/2, loupeSize);
    // loupeCtx.moveTo(0, loupeSize/2); loupeCtx.lineTo(loupeSize, loupeSize/2);
    // loupeCtx.stroke();
  }

  // Button: toggle loupe on/off
  btnToggle.addEventListener('click', () => {
    if (loupeEnabled) disableLoupe(); else enableLoupe();
  });

  // Button: cycle loupe size & zoom levels (pick values you like)
  const presets = [
    { size: 120, zoom: 2 },
    { size: 140, zoom: 2 },
    { size: 160, zoom: 2.5 },
    { size: 180, zoom: 3 }
  ];
  let presetIdx = 1; // starts at 140×2
  setLoupeSizeAndZoom(presets[presetIdx].size, presets[presetIdx].zoom);

  btnSize.addEventListener('click', () => {
    presetIdx = (presetIdx + 1) % presets.length;
    const p = presets[presetIdx];
    setLoupeSizeAndZoom(p.size, p.zoom);
  });

  // If your chart resizes with the window, the mouse coordinates will still work
  // because we base positions on getBoundingClientRect() each move.
})();

