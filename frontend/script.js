const $ = (sel) => document.querySelector(sel);

/* =========================
   Console logging
========================= */
function logStatus(msg) {
  const el = $("#statusConsole");
  if (!el) return;
  const t = new Date().toLocaleTimeString("en-GB", { hour12: false });
  el.textContent += `\n[${t}] ${msg}`;
  el.scrollTop = el.scrollHeight;
}

/* =========================
   Controls
========================= */
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

/* =========================
   Status Pill + Buttons
========================= */
function setStatus(state) {
  // state: "ready" | "running" | "paused" | "error"
  const led = document.getElementById("pollStatusLed");
  const txt = document.getElementById("uiStatusText");

  if (txt) {
    txt.textContent =
      state === "running" ? "Running" :
      state === "paused"  ? "Paused"  :
      state === "error"   ? "Error"   : "Ready";
  }

  if (led) {
    if (state === "running") {
      led.style.background = "#16a34a";
      led.style.boxShadow = "0 0 0 4px rgba(22,163,74,.12)";
    } else if (state === "paused") {
      led.style.background = "#f59e0b";
      led.style.boxShadow = "0 0 0 4px rgba(245,158,11,.14)";
    } else if (state === "error") {
      led.style.background = "#dc2626";
      led.style.boxShadow = "0 0 0 4px rgba(220,38,38,.12)";
    } else {
      led.style.background = "#dc2626";
      led.style.boxShadow = "0 0 0 4px rgba(220,38,38,.12)";
    }
  }
}

function setButtons() {
  const startBtn = document.getElementById("startStream");
  const pauseBtn = document.getElementById("pauseStream");
  const stopBtn  = document.getElementById("stopStream");

  if (startBtn) startBtn.disabled = pollActive && !pollPaused;
  if (pauseBtn) pauseBtn.disabled = !pollActive;
  if (stopBtn)  stopBtn.disabled  = !pollActive;

  if (pauseBtn) pauseBtn.textContent = pollPaused ? "▶ Resume" : "‖ Pause";
}

/* =========================
   Colors
========================= */
function makeColors(n) {
  const out = [];
  for (let i = 0; i < n; i++) {
    const hue = Math.round((360 * i) / Math.max(1, n));
    out.push(`hsl(${hue} 90% 40%)`);
  }
  return out;
}

/* =========================
   Chart
========================= */
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
    borderWidth: 2,
    pointRadius: 0,
    tension: 0.15,
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
      plugins: { legend: { display: true, position: "top" } },
    },
  });

  currentChannelCount = datasetCount;
  applyAxisScalingIfChanged(true);
}

function ensureChartFor(channelCount, maxLen) {
  if (!chart || currentChannelCount !== channelCount) {
    buildChart(channelCount, maxLen);
  } else if (chart.data.labels.length !== maxLen) {
    chart.data.labels = Array.from({ length: maxLen }, (_, i) => i);
    applyAxisScalingIfChanged(true);
  }
}

/* =========================
   Scaling (apply only when changed)
========================= */
function numOrNull(v) {
  if (v === null || v === undefined) return null;
  if (String(v).trim() === "") return null;
  const n = Number(v);
  return Number.isFinite(n) ? n : null;
}

let scaleCache = { xMin: "", xMax: "", yMin: "", yMax: "" };

function setScale(axis, minVal, maxVal) {
  if (!chart) return;
  const s = chart.options.scales[axis];
  if (!s) return;

  if (minVal == null) delete s.min; else s.min = minVal;
  if (maxVal == null) delete s.max; else s.max = maxVal;
}

function applyAxisScalingIfChanged(force = false) {
  if (!chart) return;

  const raw = {
    xMin: ($("#xMin")?.value ?? "").trim(),
    xMax: ($("#xMax")?.value ?? "").trim(),
    yMin: ($("#yMin")?.value ?? "").trim(),
    yMax: ($("#yMax")?.value ?? "").trim(),
  };

  const changed =
    force ||
    raw.xMin !== scaleCache.xMin ||
    raw.xMax !== scaleCache.xMax ||
    raw.yMin !== scaleCache.yMin ||
    raw.yMax !== scaleCache.yMax;

  if (!changed) return;

  scaleCache = raw;

  const xMin = numOrNull(raw.xMin);
  const xMax = numOrNull(raw.xMax);
  const yMin = numOrNull(raw.yMin);
  const yMax = numOrNull(raw.yMax);

  if (xMin != null && xMax != null && xMin >= xMax) {
    logStatus("X scale ignored: Min must be < Max.");
    setScale("x", null, null);
  } else {
    setScale("x", xMin, xMax);
  }

  if (yMin != null && yMax != null && yMin >= yMax) {
    logStatus("Y scale ignored: Min must be < Max.");
    setScale("y", null, null);
  } else {
    setScale("y", yMin, yMax);
  }
}

/* =========================
   Networking
========================= */
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

/* =========================
   Register 196 (API unchanged)
========================= */
const REG196_GET_URL  = "/api/registers/dac/ch0/msb";
const REG196_POST_URL = "/api/registers/dac/ch0/msb";

function parseU32Input(s) {
  const raw = (s ?? "").trim();
  if (!raw) throw new Error("Enter a value (e.g. 123 or 0x0000007B).");

  let n;
  if (/^0x[0-9a-f]+$/i.test(raw)) n = Number.parseInt(raw, 16);
  else n = Number.parseInt(raw, 10);

  if (!Number.isFinite(n) || Number.isNaN(n)) throw new Error(`Invalid number: "${raw}"`);
  if (n < 0 || n > 0xFFFFFFFF) throw new Error("Value must fit in u32 (0 .. 4294967295).");

  return n >>> 0;
}

function setReg196Ui(valueU32, hexStr) {
  const inp = document.getElementById("reg196Value");
  const hexEl = document.getElementById("reg196Hex");
  if (inp) inp.value = String(valueU32);
  if (hexEl) hexEl.textContent = hexStr || `0x${(valueU32 >>> 0).toString(16).toUpperCase().padStart(8, "0")}`;
}

async function reg196Read() {
  const data = await fetchJson(REG196_GET_URL);
  setReg196Ui(data.value, data.hex);
  logStatus(`Reg ${data.register_id} (${data.register}) read: ${data.value} (${data.hex})`);
  return data;
}

async function reg196Write(valueU32) {
  const data = await fetchJson(REG196_POST_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ value: valueU32 }),
  });
  setReg196Ui(data.value, data.hex);
  logStatus(`Reg ${data.register_id} (${data.register}) written: ${data.value} (${data.hex})`);
  return data;
}

/* =========================
   Plot update (expects { channel_data })
   NOTE: your backend returns channel_data where
   last entry seems to be "measurement info", so we keep your logic.
========================= */
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

  // apply scaling only if user changed inputs
  applyAxisScalingIfChanged(false);

  chart.update("none");

  updateStats();
}

/* =========================
   Stats (right sidebar)
========================= */
let totalPackets = 0;

function updateStats() {
  // Active channels = visible datasets
  const activeEl = document.getElementById("statActiveChannels");
  const pointsEl = document.getElementById("statPointsPerChannel");
  const hzEl     = document.getElementById("statUpdateHz");
  const totalEl  = document.getElementById("statTotalPackets");

  if (chart) {
    let active = 0;
    chart.data.datasets.forEach((_, idx) => {
      const meta = chart.getDatasetMeta(idx);
      if (!meta.hidden) active++;
    });

    const points = chart.data.labels?.length ?? 0;

    if (activeEl) activeEl.textContent = active ? String(active) : "—";
    if (pointsEl) pointsEl.textContent = points ? String(points) : "—";
  }

  // Update Hz from currentPollMs (frozen for session)
  if (hzEl && pollActive) {
    const hz = currentPollMs > 0 ? (1000 / currentPollMs) : 0;
    hzEl.textContent = hz ? hz.toFixed(1) : "—";
  } else if (hzEl) {
    hzEl.textContent = "—";
  }

  if (totalEl) totalEl.textContent = String(totalPackets);
}

/* =========================
   Polling (non-overlapping) + Pause
========================= */
let pollActive = false;
let pollPaused = false;

// frozen config per run
let currentMode = "with_offset";
let currentPackets = 1;
let currentPollMs = 250;

async function fetchOnceAndPlot() {
  try {
    const params = new URLSearchParams({
      packets: String(currentPackets),
      mode: currentMode,
    });
    const data = await fetchJson(`/api/stream/once?${params.toString()}`);
    updateAllChannels(data?.channel_data);

    // count packets (best effort)
    totalPackets += currentPackets;
    updateStats();
  } catch (err) {
    logStatus(`Fetch once failed: ${err.message}`);
    console.error(err);
    setStatus("error");
  }
}

async function pollLoop() {
  while (pollActive) {
    if (!pollPaused) {
      await fetchOnceAndPlot();
    }
    await new Promise((resolve) => setTimeout(resolve, currentPollMs));
  }
}

function startPolling() {
  // freeze config for this run
  currentMode = getMode();
  currentPackets = getPackets();
  currentPollMs = getPollMs();

  if (pollActive) {
    logStatus("Polling is already running.");
    return;
  }

  pollActive = true;
  pollPaused = false;
  setStatus("running");
  setButtons();

  logStatus(
    `Started polling /api/stream/once every ${currentPollMs} ms ` +
    `(mode=${currentMode}, packets=${currentPackets})`
  );

  updateStats();
  pollLoop(); // fire and forget
}

function stopPolling() {
  pollActive = false;
  pollPaused = false;
  setStatus("ready");
  setButtons();
  logStatus("Stopped polling.");
  updateStats();
}

function togglePause() {
  if (!pollActive) return;
  pollPaused = !pollPaused;

  if (pollPaused) {
    setStatus("paused");
    logStatus("Paused polling.");
  } else {
    setStatus("running");
    logStatus("Resumed polling.");
  }
  setButtons();
  updateStats();
}

/* =========================
   Start/Stop (API unchanged)
========================= */
async function onStartStream() {
  try {
    const mode = getMode();
    await fetchJson(`/api/stream/start?mode=${encodeURIComponent(mode)}`, { method: "POST" });
    logStatus(`Backend stream start acknowledged (mode=${mode}).`);
  } catch (e) {
    logStatus(`Stream start request failed: ${e.message}`);
    console.error(e);
    setStatus("error");
    return; // don't start polling if backend failed to start
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

/* =========================
   Export CSV (current chart data)
========================= */
function exportCsv() {
  if (!chart || !chart.data?.labels?.length) {
    logStatus("Nothing to export (no data yet).");
    return;
  }

  const labels = chart.data.labels;
  const ds = chart.data.datasets;

  const header = ["sample", ...ds.map((d) => d.label)];
  const rows = [header.join(",")];

  for (let i = 0; i < labels.length; i++) {
    const row = [labels[i]];
    for (let c = 0; c < ds.length; c++) {
      const v = ds[c].data[i];
      row.push(v === null || v === undefined ? "" : String(v));
    }
    rows.push(row.join(","));
  }

  const blob = new Blob([rows.join("\n")], { type: "text/csv;charset=utf-8" });
  const a = document.createElement("a");
  const ts = new Date().toISOString().replace(/[:.]/g, "-");
  a.href = URL.createObjectURL(blob);
  a.download = `iot_stream_${ts}.csv`;
  document.body.appendChild(a);
  a.click();
  a.remove();

  logStatus(`Exported CSV: ${labels.length} samples × ${ds.length} channels.`);
}

/* =========================
   Boot wiring
========================= */
window.addEventListener("DOMContentLoaded", () => {
  // Required (new UI includes pause, export, clear; but we tolerate missing)
  const requiredIds = [
    "modeSelect", "packets", "pollMs",
    "startStream", "stopStream", "channelChart", "statusConsole",
    "xMin", "xMax", "yMin", "yMax"
  ];
  const missing = requiredIds.filter((id) => !document.getElementById(id));
  if (missing.length) {
    console.warn("Missing expected elements:", missing);
    logStatus(`UI missing elements: ${missing.join(", ")}`);
  }

  $("#startStream")?.addEventListener("click", onStartStream);
  $("#stopStream")?.addEventListener("click", onStopStream);

  // Pause button (new UI)
  $("#pauseStream")?.addEventListener("click", togglePause);

  // Export + Clear (new UI)
  $("#exportCsv")?.addEventListener("click", exportCsv);
  $("#clearConsole")?.addEventListener("click", () => {
    const c = $("#statusConsole");
    if (c) c.textContent = "";
  });

  // Register 196 wiring
  document.getElementById("reg196Read")?.addEventListener("click", async () => {
    try { await reg196Read(); }
    catch (e) { logStatus(`Reg196 read failed: ${e.message}`); console.error(e); }
  });

  document.getElementById("reg196Write")?.addEventListener("click", async () => {
    try {
      const inp = document.getElementById("reg196Value");
      const value = parseU32Input(inp?.value ?? "");
      await reg196Write(value);
    } catch (e) {
      logStatus(`Reg196 write failed: ${e.message}`);
      console.error(e);
    }
  });

  // Config change logs (take effect next Start because config is frozen per run)
  $("#modeSelect")?.addEventListener("change", () =>
    logStatus(`Mode changed to ${getMode()} (applies on next Start)`));
  $("#packets")?.addEventListener("change", () =>
    logStatus(`Packets per poll set to ${getPackets()} (applies on next Start)`));
  $("#pollMs")?.addEventListener("change", () =>
    logStatus(`Polling interval set to ${getPollMs()} ms (applies on next Start)`));

  // scaling listeners (live)
  ["xMin", "xMax", "yMin", "yMax"].forEach(id => {
    const el = document.getElementById(id);
    el?.addEventListener("input", () => {
      applyAxisScalingIfChanged(true);
      chart?.update("none");
    });
    el?.addEventListener("change", () => {
      applyAxisScalingIfChanged(true);
      chart?.update("none");
    });
  });

  // Init UI state
  setStatus("ready");
  setButtons();
  updateStats();
});

/* =========================
   LOUPE (magnifier) — adapted to new UI
   - Looks for .chart-frame (new layout)
========================= */
(function () {
  const chartCanvas = document.getElementById("channelChart");
  const chartWrap = document.querySelector(".chart-frame") || chartCanvas?.parentElement;
  const btnToggle = document.getElementById("loupeToggle");
  const btnSize = document.getElementById("loupeSize");

  if (!chartCanvas || !chartWrap || !btnToggle || !btnSize) return;

  // Ensure container is positioned
  const cs = window.getComputedStyle(chartWrap);
  if (cs.position === "static") chartWrap.style.position = "relative";

  let loupeEnabled = false;
  let loupeCanvas = null;
  let loupeCtx = null;
  let loupeSize = 140;
  let loupeZoom = 2;

  function createLoupe() {
    if (loupeCanvas) return;
    loupeCanvas = document.createElement("canvas");
    loupeCanvas.width = loupeSize;
    loupeCanvas.height = loupeSize;
    loupeCanvas.style.position = "absolute";
    loupeCanvas.style.left = "0px";
    loupeCanvas.style.top = "0px";
    loupeCanvas.style.width = loupeSize + "px";
    loupeCanvas.style.height = loupeSize + "px";
    loupeCanvas.style.borderRadius = "14px";
    loupeCanvas.style.border = "1px solid rgba(226,232,240,.95)";
    loupeCanvas.style.boxShadow = "0 10px 24px rgba(2,6,23,.18)";
    loupeCanvas.style.background = "#fff";
    loupeCanvas.style.pointerEvents = "none";
    loupeCanvas.style.display = "none";
    loupeCanvas.style.zIndex = "50";

    loupeCtx = loupeCanvas.getContext("2d");
    chartWrap.appendChild(loupeCanvas);
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
      loupeCanvas.style.width = loupeSize + "px";
      loupeCanvas.style.height = loupeSize + "px";
    }
    btnSize.textContent = `Loupe: ${loupeSize}px ×${loupeZoom}`;
  }

  function enableLoupe() {
    loupeEnabled = true;
    createLoupe();
    chartCanvas.addEventListener("mousemove", onMove);
    chartCanvas.addEventListener("mouseleave", onLeave);
    chartCanvas.addEventListener("touchmove", onTouchMove, { passive: false });
    chartCanvas.addEventListener("touchend", onLeave);
    logStatus("Loupe enabled.");
  }

  function disableLoupe() {
    loupeEnabled = false;
    chartCanvas.removeEventListener("mousemove", onMove);
    chartCanvas.removeEventListener("mouseleave", onLeave);
    chartCanvas.removeEventListener("touchmove", onTouchMove);
    chartCanvas.removeEventListener("touchend", onLeave);
    destroyLoupe();
    logStatus("Loupe disabled.");
  }

  function onLeave() {
    if (loupeCanvas) loupeCanvas.style.display = "none";
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
    if (!loupeEnabled || !loupeCanvas || !loupeCtx) return;

    loupeCanvas.style.display = "block";

    // Follow cursor, clamped inside chartWrap
    const wrapRect = chartWrap.getBoundingClientRect();
    const left = Math.max(0, Math.min(wrapRect.width - loupeSize, x - loupeSize / 2));
    const top  = Math.max(0, Math.min(wrapRect.height - loupeSize, y - loupeSize / 2));

    loupeCanvas.style.left = left + "px";
    loupeCanvas.style.top  = top + "px";

    // source sampling from main canvas bitmap
    const dpr = window.devicePixelRatio || 1;
    const srcSize = loupeSize / loupeZoom;

    const sx = Math.max(0, Math.min(chartCanvas.width  - srcSize, x * dpr - srcSize / 2));
    const sy = Math.max(0, Math.min(chartCanvas.height - srcSize, y * dpr - srcSize / 2));

    loupeCtx.clearRect(0, 0, loupeSize, loupeSize);
    loupeCtx.imageSmoothingEnabled = true;

    loupeCtx.drawImage(
      chartCanvas,
      sx, sy, srcSize, srcSize,
      0, 0, loupeSize, loupeSize
    );

    // crosshair
    loupeCtx.globalAlpha = 0.5;
    loupeCtx.beginPath();
    loupeCtx.moveTo(loupeSize / 2, 0);
    loupeCtx.lineTo(loupeSize / 2, loupeSize);
    loupeCtx.moveTo(0, loupeSize / 2);
    loupeCtx.lineTo(loupeSize, loupeSize / 2);
    loupeCtx.strokeStyle = "#94a3b8";
    loupeCtx.lineWidth = 1;
    loupeCtx.stroke();
    loupeCtx.globalAlpha = 1;
  }

  btnToggle.addEventListener("click", () => {
    if (loupeEnabled) disableLoupe(); else enableLoupe();
  });

  const presets = [
    { size: 120, zoom: 2 },
    { size: 140, zoom: 2 },
    { size: 160, zoom: 2.5 },
    { size: 180, zoom: 3 },
  ];
  let presetIdx = 1;
  setLoupeSizeAndZoom(presets[presetIdx].size, presets[presetIdx].zoom);

  btnSize.addEventListener("click", () => {
    presetIdx = (presetIdx + 1) % presets.length;
    const p = presets[presetIdx];
    setLoupeSizeAndZoom(p.size, p.zoom);
  });
})();

