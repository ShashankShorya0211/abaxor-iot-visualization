#![allow(dead_code)]

use std::time::{Duration, Instant};
use std::thread::sleep;
use std::os::unix::prelude::*;

use std::io::Write;
use std::fs::File;

use super::*;

/// Header of one incoming data packet (as emitted by the FPGA)
#[derive(Clone, Copy, Debug, Default)]
struct Header {
    block_num:     u8,
    first_channel: u8,
    num_channels:  u8,
}

/// Result type returned by `decode_datablock`
#[derive(Debug)]
pub struct DecodedStream {
    /// Per-channel sample vectors; last slot `index == settings::FIFO_NUM_STREAM_CH as usize`
    /// carries the measurement number (Measnr) stream aligned with channel 0 start.
    pub channel_data: [Vec<i16>; settings::FIFO_NUM_STREAM_CH as usize + 1],
    /// Raw 32-bit words that were read (headers + data), useful for diagnostics/hex-view
    pub raw_words: Vec<u32>,
    /// Number of detected “glitches” in ramp test mode
    pub glitches: usize,
}

/* ────────────────────────────────────────────────────────────────────────────
   Public control helpers
   ──────────────────────────────────────────────────────────────────────────── */

/// Check if the kernel FIFO device node exists
pub fn exists_fifo_module() -> bool {
    std::fs::metadata("/dev/hh_amv_psfifo_dev").is_ok()
}

/// Minimal, **non-blocking** warmup. No ramp self-test here.
/// Safe to call once on backend start.
pub fn init_fifo(fifo: RawFd, reg: RawFd) {
    if !exists_fifo_module() {
        eprintln!("FIFO kernel module/device not present at /dev/hh_amv_psfifo_dev");
        return;
    }

    // Ensure oscillator is enabled (non-blocking)
    if (reg_bank::get_command(reg) & flags::OSCILLATOR_EN) == 0 {
        let _ = oscillator::enable(reg);
    }

    // Make sure nothing is pushing data right now
    let mut cmd = reg_bank::get_command(reg);
    cmd &= !(flags::STREAM_TRANSFER_EN | flags::STREAM_RAMP_EN | flags::STREAM_SELECT);
    reg_bank::set_command(reg, cmd);

    // Reset FIFO pointers once
    reg_fifo::set_axi_str_rdfr(fifo, flags::RESET_KEY);

    // Best-effort drain with a hard timeout so we never wedge during init
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut last = u32::MAX;

    loop {
        let occ = fifo_occupancy(fifo);
        if occ == 0 { break; }

        if Instant::now() >= deadline && occ == last {
            eprintln!("init_fifo: FIFO still reports {} words; continuing anyway.", occ);
            break;
        }
        last = occ;

        // Drain up to 256 words but never blind-read
        drain_some_words(fifo, occ.min(256));
        sleep(Duration::from_millis(5));
    }

    // Final reset to start from a clean slate
    reg_fifo::set_axi_str_rdfr(fifo, flags::RESET_KEY);

    // Intentionally DO NOT start ramp/stream here. Leave that to the API.
}

/// Start continuous data transfer (with offset)
pub fn start_streaming(reg: RawFd) {
    let cmd = reg_bank::get_command(reg);
    reg_bank::set_command(reg, cmd | flags::STREAM_TRANSFER_EN);
}

/// Start continuous data transfer without offset (select alternate stream)
pub fn start_streaming_without_offset(reg: RawFd) {
    let cmd = reg_bank::get_command(reg);
    reg_bank::set_command(reg, cmd | flags::STREAM_TRANSFER_EN | flags::STREAM_SELECT);
}

/// Stop any streaming (transfer & select bits cleared)
pub fn stop_streaming(reg: RawFd) {
    let cmd = reg_bank::get_command(reg);
    reg_bank::set_command(reg, cmd & !flags::STREAM_TRANSFER_EN & !flags::STREAM_SELECT);
}

/// Start ramp generator (incremental data) and enable transfer
pub fn start_ramp_gen(reg: RawFd) {
    let cmd = reg_bank::get_command(reg);
    reg_bank::set_command(reg, cmd | flags::STREAM_RAMP_EN | flags::STREAM_TRANSFER_EN);
}

/// Stop ramp generator and disable transfer
pub fn stop_ramp_gen(reg: RawFd) {
    let cmd = reg_bank::get_command(reg);
    reg_bank::set_command(reg, cmd & !(flags::STREAM_RAMP_EN | flags::STREAM_TRANSFER_EN));
}

/* ────────────────────────────────────────────────────────────────────────────
   Core reading/decoding
   ──────────────────────────────────────────────────────────────────────────── */

/// Read and return `num_packets` worth of decoded data according to `option`:
/// - `flags::STREAM_WITH_OFFSET`
/// - `flags::STREAM_WITHOUT_OFFSET` (reads until N measurements observed)
/// - `flags::STREAM_WITH_RAMP`
/// - `flags::STREAM_GLITCHES_TEST`
///
/// API-friendly (no panics, no CSV).
pub fn decode_datablock(
    fifo: RawFd,
    mut num_packets: u32,
    option: u8,
) -> DecodedStream {
    let mut bytes: [u8; 4];
    let mut glitches: usize = 0;

    // advancing checks to ensure we’re aligned
    static mut NEXT_BLOCK_NUM: u8 = 0;
    static mut NEXT_CHANN_NUM: u8 = 0;

    let mut header = Header { block_num: 0, first_channel: 0, num_channels: 0 };

    let mut channel_data: [Vec<i16>; settings::FIFO_NUM_STREAM_CH as usize + 1] = Default::default();
    let mut raw_words: Vec<u32> = Vec::new();

    let channel_num_add = (
        (settings::FIFO_STREAM_DATA_PACKET_SIZE - settings::FIFO_STREAM_DATA_HEADER_SIZE)
            / settings::FIFO_NUM_SAMPLES_PER_WORD
            % (settings::FIFO_NUM_STREAM_CH as usize)
    ) as u8;

    let mut first_run = true;
    let mut required_meas_count = 0;
    let mut first_meas = 0u32;

    if option == flags::STREAM_WITHOUT_OFFSET {
        // waits for `num_packets + 1` measurements, then exits
        required_meas_count = num_packets + 1;
        num_packets = 1_000_000; // effectively unlimited until the condition trips
    }

    // Wait until we catch a valid header boundary (bounded wait)
    {
        let deadline = Instant::now() + Duration::from_millis(250);
        loop {
            // If empty, short wait with deadline
            let occ = fifo_occupancy(fifo);
            if occ == 0 {
                if Instant::now() >= deadline { break; }
                sleep(Duration::from_millis(1));
                continue;
            }

            let word = reg_fifo::get_axi4_str_rdfd(fifo);
            raw_words.push(word);
            bytes = word.to_le_bytes();

            // Header bytes:
            // bytes[0] -> start byte (expected 0xA5)
            // bytes[1] -> block number
            // bytes[2] -> first channel
            // bytes[3] -> number of channels (= MAXIMALE_ANZAHL_AN_MESSSTELLEN)
            if bytes[0] == settings::FIFO_STREAM_DATA_HEADER_MARKING
                && bytes[3] == settings::MAXIMALE_ANZAHL_AN_MESSSTELLEN as u8
            {
                if bytes[2] < 14 {
                    header.first_channel = bytes[2];
                } else {
                    // unexpected channel index, reset alignment search
                    header.first_channel = 0;
                    first_run = true;
                    continue;
                }
                header.block_num = bytes[1];

                if first_run {
                    unsafe {
                        NEXT_BLOCK_NUM = header.block_num;
                        NEXT_CHANN_NUM = header.first_channel;
                    }
                    first_run = false;
                }

                let aligned = unsafe {
                    header.block_num == NEXT_BLOCK_NUM && header.first_channel == NEXT_CHANN_NUM
                };

                if aligned {
                    unsafe {
                        NEXT_BLOCK_NUM = header.block_num.wrapping_add(1);
                        NEXT_CHANN_NUM = (header.first_channel
                            .wrapping_add(channel_num_add)
                            .wrapping_add(settings::ANZAHL_AN_ZUSATZ_KANAELE_FIFO_STREAM as u8))
                            % settings::FIFO_NUM_STREAM_CH;
                    }
                    break; // found a valid aligned header
                }
            }

            if Instant::now() >= deadline {
                // Give up cleanly; caller can decide what to do with partials
                break;
            }
        }
    }

    // Main packet loop (bounded waits)
    for j in 0..num_packets {
        // Wait for at least one word for meas_nr, but do not block indefinitely
        if !wait_until_fifo(fifo, 1, Duration::from_millis(50)) {
            break;
        }

        // Read measurement number (Measnr)
        let meas_nr = reg_fifo::get_axi4_str_rdfd(fifo);
        raw_words.push(meas_nr);

        if first_meas == 0 {
            first_meas = meas_nr;
        }

        // Early exit condition for STREAM_WITHOUT_OFFSET:
        if option == flags::STREAM_WITHOUT_OFFSET && (meas_nr.wrapping_sub(first_meas)) == required_meas_count {
            return DecodedStream { channel_data, raw_words, glitches };
        }

        // Read the packet payload: number of 32-bit words per packet (excl. header)
        let mut words_needed: usize = settings::FIFO_STREAM_DATA_WORDS_PER_PACKET;
        let packet_deadline = Instant::now() + Duration::from_millis(100);

        while words_needed > 0 {
            let occ = fifo_occupancy(fifo);
            if occ == 0 {
                if Instant::now() >= packet_deadline { break; }
                sleep(Duration::from_millis(1));
                continue;
            }

            let to_read: usize = std::cmp::min(occ as usize, words_needed);
            for _ in 0..to_read {
                // If first_channel == 0, store the measurement number (as i16) aligned to special index
                if header.first_channel == 0 {
                    channel_data[settings::FIFO_NUM_STREAM_CH as usize].push(meas_nr as i16);
                }

                let word = reg_fifo::get_axi4_str_rdfd(fifo);
                raw_words.push(word);
                let b = word.to_le_bytes();

                // Two 16-bit samples packed little-endian
                let value0 = i16::from_le_bytes([b[0], b[1]]);
                let value1 = i16::from_le_bytes([b[2], b[3]]);

                // Push value0 into current channel
                if (header.first_channel as usize) < channel_data.len() {
                    if option == flags::STREAM_GLITCHES_TEST {
                        glitches += glitch_if_out_of_sequence(&channel_data[header.first_channel as usize], value0);
                    }
                    channel_data[header.first_channel as usize].push(value0);
                }

                // Advance channel index (wrap on total number of stream channels)
                header.first_channel = header.first_channel.wrapping_add(1);
                if header.first_channel >= settings::FIFO_NUM_STREAM_CH {
                    header.first_channel = 0;
                }

                // Push value1 into (possibly next) channel
                if (header.first_channel as usize) < channel_data.len() {
                    if option == flags::STREAM_GLITCHES_TEST {
                        glitches += glitch_if_out_of_sequence(&channel_data[header.first_channel as usize], value1);
                    }
                    channel_data[header.first_channel as usize].push(value1);
                }

                // Advance again for next word placement
                header.first_channel = header.first_channel.wrapping_add(1);
                if header.first_channel >= settings::FIFO_NUM_STREAM_CH {
                    header.first_channel = 0;
                }

                words_needed -= 1;
                if words_needed == 0 { break; }
            }
        }

        // If more packets requested, try to sync to next header with a bounded wait
        if j < num_packets - 1 {
            let hdr_deadline = Instant::now() + Duration::from_millis(50);
            let mut got_header = false;

            while Instant::now() < hdr_deadline {
                let occ = fifo_occupancy(fifo);
                if occ == 0 {
                    sleep(Duration::from_millis(1));
                    continue;
                }
                let word = reg_fifo::get_axi4_str_rdfd(fifo);
                raw_words.push(word);
                bytes = word.to_le_bytes();

                // Fill missing channels (if header jumps) with zeros to keep columns aligned
                let next_first = bytes[2];
                if next_first < header.first_channel || next_first > settings::FIFO_NUM_STREAM_CH {
                    // Pad from current to end, then from 0 up to next_first
                    for i in header.first_channel..settings::FIFO_NUM_STREAM_CH {
                        channel_data[i as usize].push(0);
                    }
                    for i in 0..next_first {
                        if (i as usize) < channel_data.len() && i < 14 {
                            channel_data[i as usize].push(0);
                        }
                    }
                } else {
                    for i in header.first_channel..next_first {
                        channel_data[i as usize].push(0);
                    }
                }

                // Accept next header’s first_channel (guard)
                header.first_channel = if next_first < 14 { next_first } else { 0 };
                got_header = true;
                break;
            }

            if !got_header {
                // Couldn’t find a next header in time; end gracefully
                break;
            }
        }
    }

    DecodedStream { channel_data, raw_words, glitches }
}

/* ────────────────────────────────────────────────────────────────────────────
   Utilities
   ──────────────────────────────────────────────────────────────────────────── */

/// Quickly drain up to `max_words` words, only if occupancy says data is present.
fn drain_some_words(fifo: RawFd, max_words: u32) {
    let mut remaining = max_words;
    while remaining > 0 {
        let occ = fifo_occupancy(fifo);
        if occ == 0 { break; }
        let to_read = occ.min(remaining);
        for _ in 0..to_read {
            let _ = reg_fifo::get_axi4_str_rdfd(fifo);
        }
        remaining -= to_read;
    }
}

/// Empty FIFO with a **bounded** best-effort drain, then assert reset.
pub fn clear_fifo(fifo: RawFd) {
    let deadline = Instant::now() + Duration::from_secs(1);

    while Instant::now() < deadline {
        let occ = fifo_occupancy(fifo);
        if occ == 0 { break; }
        drain_some_words(fifo, occ.min(256));
        sleep(Duration::from_millis(2));
    }

    // Reset FIFO regardless of where we landed
    reg_fifo::set_axi_str_rdfr(fifo, flags::RESET_KEY);
}

/// Return FIFO occupancy in 32-bit words
pub fn fifo_occupancy(fifo: RawFd) -> u32 {
    (reg_fifo::get_axi_str_rdfo(fifo) & flags::OCCUPANCY_MASK) >> flags::OCCUPANCY_SHIFT
}

/// Bounded wait until FIFO occupancy >= min_words.
/// Returns true if condition met before `timeout`, false otherwise.
fn wait_until_fifo(fifo: RawFd, min_words: u32, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        let fill = fifo_occupancy(fifo);
        if fill >= min_words { return true; }
        if Instant::now() >= deadline { return false; }
        sleep(Duration::from_millis(1));
    }
}

/// Low-level drain: read up to `num_packets` * 256 words into a byte vec,
/// but **never** block indefinitely. Uses occupancy with a short per-packet deadline.
fn read_datablock_raw(fifo: RawFd, num_packets: u32) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(num_packets as usize * 256 * 4);
    for _ in 0..num_packets {
        // Wait for at least one word with a short timeout
        if !wait_until_fifo(fifo, 1, Duration::from_millis(20)) {
            break;
        }

        let mut words_left = 256u32;
        let packet_deadline = Instant::now() + Duration::from_millis(50);

        while words_left > 0 {
            let occ = fifo_occupancy(fifo);
            if occ == 0 {
                if Instant::now() >= packet_deadline { break; }
                sleep(Duration::from_millis(1));
                continue;
            }

            let to_read = occ.min(words_left);
            for _ in 0..to_read {
                let word = reg_fifo::get_axi4_str_rdfd(fifo);
                out.extend_from_slice(&word.to_le_bytes());
            }
            words_left -= to_read;
        }
    }
    out
}

/// Ramp “glitch” detection that’s safe for i16 wrap-around.
/// Increments by 1 modulo 2^16 is expected. Any other jump counts as a glitch.
fn glitch_if_out_of_sequence(history: &Vec<i16>, current: i16) -> usize {
    if let Some(&last) = history.last() {
        let last_u = last as u16;
        let cur_u  = current as u16;
        // expected = last + 1 (mod 65536)
        let expected = last_u.wrapping_add(1);
        if cur_u != expected {
            return 1;
        }
    }
    0
}

