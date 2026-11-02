//! # Stitch Ecosystem Cross-Serializer Benchmark Suite
//!
//! Provides realistic domain schemas and generators for comparative benchmarking:
//! - **HFT L2 OrderBook Market Data**: High frequency tick updates, price levels, bid/ask quantities.
//! - **IoT Robotics / Sensor Telemetry**: Floating point measurements, sensor IDs, status masks.
//! - **Spatial 3D Physics Entity**: Positions, velocities, orientations for real-time game loops.

use serde::{Deserialize, Serialize};
use stitch::prelude::*;

// ============================================================================
// 1. Domain: HFT L2 OrderBook Market Data
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Record, Serialize, Deserialize)]
pub struct MarketTick {
    #[stitch(id = 1)]
    pub timestamp_ns: u64,
    #[stitch(id = 2, column = ColumnCodec::Dict)]
    pub instrument_id: u32,
    #[stitch(id = 3, column = ColumnCodec::Gorilla)]
    pub price: f64,
    #[stitch(id = 4, column = ColumnCodec::Rle)]
    pub volume: u32,
    #[stitch(id = 5, column = ColumnCodec::Rle)]
    pub side: u8, // 0 = Buy, 1 = Sell
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstMarketTick {
    #[prost(uint64, tag = "1")]
    pub timestamp_ns: u64,
    #[prost(uint32, tag = "2")]
    pub instrument_id: u32,
    #[prost(double, tag = "3")]
    pub price: f64,
    #[prost(uint32, tag = "4")]
    pub volume: u32,
    #[prost(uint32, tag = "5")]
    pub side: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstMarketTickBatch {
    #[prost(message, repeated, tag = "1")]
    pub ticks: ::prost::alloc::vec::Vec<ProstMarketTick>,
}

// ============================================================================
// 2. Domain: IoT / Robotics Sensor Telemetry
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Record, Serialize, Deserialize)]
pub struct SensorTelemetry {
    #[stitch(id = 1)]
    pub timestamp_ms: u64,
    #[stitch(id = 2, column = ColumnCodec::Dict)]
    pub device_id: u32,
    #[stitch(id = 3, column = ColumnCodec::Gorilla)]
    pub temperature: f32,
    #[stitch(id = 4, column = ColumnCodec::Gorilla)]
    pub humidity: f32,
    #[stitch(id = 5, column = ColumnCodec::Rle)]
    pub battery_pct: u8,
    #[stitch(id = 6, column = ColumnCodec::Rle)]
    pub status_code: u16,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstSensorTelemetry {
    #[prost(uint64, tag = "1")]
    pub timestamp_ms: u64,
    #[prost(uint32, tag = "2")]
    pub device_id: u32,
    #[prost(float, tag = "3")]
    pub temperature: f32,
    #[prost(float, tag = "4")]
    pub humidity: f32,
    #[prost(uint32, tag = "5")]
    pub battery_pct: u32,
    #[prost(uint32, tag = "6")]
    pub status_code: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstSensorTelemetryBatch {
    #[prost(message, repeated, tag = "1")]
    pub readings: ::prost::alloc::vec::Vec<ProstSensorTelemetry>,
}

// ============================================================================
// 3. Domain: Spatial 3D Physics Entity
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Record, Serialize, Deserialize)]
pub struct SpatialEntity {
    #[stitch(id = 1)]
    pub entity_id: u64,
    #[stitch(id = 2, column = ColumnCodec::Gorilla)]
    pub pos_x: f32,
    #[stitch(id = 3, column = ColumnCodec::Gorilla)]
    pub pos_y: f32,
    #[stitch(id = 4, column = ColumnCodec::Gorilla)]
    pub pos_z: f32,
    #[stitch(id = 5, column = ColumnCodec::Gorilla)]
    pub vel_x: f32,
    #[stitch(id = 6, column = ColumnCodec::Gorilla)]
    pub vel_y: f32,
    #[stitch(id = 7, column = ColumnCodec::Gorilla)]
    pub vel_z: f32,
    #[stitch(id = 8, column = ColumnCodec::Rle)]
    pub flags: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstSpatialEntity {
    #[prost(uint64, tag = "1")]
    pub entity_id: u64,
    #[prost(float, tag = "2")]
    pub pos_x: f32,
    #[prost(float, tag = "3")]
    pub pos_y: f32,
    #[prost(float, tag = "4")]
    pub pos_z: f32,
    #[prost(float, tag = "5")]
    pub vel_x: f32,
    #[prost(float, tag = "6")]
    pub vel_y: f32,
    #[prost(float, tag = "7")]
    pub vel_z: f32,
    #[prost(uint32, tag = "8")]
    pub flags: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstSpatialEntityBatch {
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<ProstSpatialEntity>,
}

pub mod scenarios;
