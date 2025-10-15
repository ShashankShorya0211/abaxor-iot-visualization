//! parser.rs
//! Convert raw FIFO u32 words into Measurement structs

use crate::models::{Measurement, ChannelData};
use std::time::{SystemTime, UNIX_EPOCH};

/// Number of channels expected from the FPGA FIFO
const NUM_CHANNELS: usize = 12; // matches supervisor's design

/// Parse raw FIFO u32 words into a Measurement
pub fn parse_fifo_words(raw_words: &[u32]) -> Measurement {
    // Get current timestamp in milliseconds
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut channels = Vec::new();

    // Convert each raw word into a float value (dummy scaling for now)
    for (ch, &word) in raw_words.iter().enumerate().take(NUM_CHANNELS) {
        let value = raw_to_voltage(word);
        channels.push(ChannelData {
            channel: ch,
            value,
        });
    }

    Measurement {
        timestamp: now,
        channels,
    }
}

/// Convert raw FPGA word into voltage (scaling factor may change)
fn raw_to_voltage(word: u32) -> f64 {
    // Example: normalize 16-bit signed ADC value to ±10.0V
    let signed = (word as i32) << 16 >> 16; // sign-extend
    (signed as f64) / 32768.0 * 10.0
}

