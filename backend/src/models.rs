//! models.rs
//! Data structures for API responses

use serde::Serialize;

/// Single measurement from one channel
#[derive(Debug, Serialize)]
pub struct ChannelData {
    pub channel: usize,
    pub value: f64,
}

/// Full measurement set (one timestamp, multiple channels)
#[derive(Debug, Serialize)]
pub struct Measurement {
    pub timestamp: u64, // nanoseconds or milliseconds since epoch
    pub channels: Vec<ChannelData>,
}

/// API response containing multiple measurements
#[derive(Debug, Serialize)]
pub struct MeasurementResponse {
    pub measurements: Vec<Measurement>,
}

