//! Synthetic and realistic deterministic scenario generators.

use crate::*;

/// Generates a deterministic sequence of HFT market ticks.
pub fn generate_market_ticks(count: usize) -> (Vec<MarketTick>, ProstMarketTickBatch) {
    let mut stitch_ticks = Vec::with_capacity(count);
    let mut prost_ticks = Vec::with_capacity(count);

    let base_time: u64 = 1_700_000_000_000_000_000;
    let mut current_price = 150.25f64;

    for i in 0..count {
        let ts = base_time + (i as u64 * 250); // 250ns intervals
        let instrument = (i % 8) as u32; // 8 liquid symbols
        let delta_price = ((i % 5) as f64 - 2.0) * 0.01;
        current_price += delta_price;
        let volume = 100 * ((i % 10) + 1) as u32;
        let side = (i % 2) as u8;

        stitch_ticks.push(MarketTick {
            timestamp_ns: ts,
            instrument_id: instrument,
            price: current_price,
            volume,
            side,
        });

        prost_ticks.push(ProstMarketTick {
            timestamp_ns: ts,
            instrument_id: instrument,
            price: current_price,
            volume,
            side: side as u32,
        });
    }

    (stitch_ticks, ProstMarketTickBatch { ticks: prost_ticks })
}

/// Generates a deterministic batch of IoT sensor readings.
pub fn generate_sensor_telemetry(
    count: usize,
) -> (Vec<SensorTelemetry>, ProstSensorTelemetryBatch) {
    let mut stitch_batch = Vec::with_capacity(count);
    let mut prost_batch = Vec::with_capacity(count);

    let base_time: u64 = 1_700_000_000_000;
    let mut temp = 22.5f32;
    let mut hum = 45.0f32;

    for i in 0..count {
        let ts = base_time + (i as u64 * 100);
        let device_id = (i % 16) as u32;
        temp += ((i % 3) as f32 - 1.0) * 0.05;
        hum += ((i % 4) as f32 - 1.5) * 0.1;
        let battery = (100 - (i % 20)) as u8;
        let status = if i % 50 == 0 { 0x01 } else { 0x00 };

        stitch_batch.push(SensorTelemetry {
            timestamp_ms: ts,
            device_id,
            temperature: temp,
            humidity: hum,
            battery_pct: battery,
            status_code: status,
        });

        prost_batch.push(ProstSensorTelemetry {
            timestamp_ms: ts,
            device_id,
            temperature: temp,
            humidity: hum,
            battery_pct: battery as u32,
            status_code: status as u32,
        });
    }

    (
        stitch_batch,
        ProstSensorTelemetryBatch {
            readings: prost_batch,
        },
    )
}

/// Generates a deterministic batch of spatial 3D physics entities.
pub fn generate_spatial_entities(count: usize) -> (Vec<SpatialEntity>, ProstSpatialEntityBatch) {
    let mut stitch_batch = Vec::with_capacity(count);
    let mut prost_batch = Vec::with_capacity(count);

    for i in 0..count {
        let entity_id = i as u64;
        let pos_x = (i as f32 * 0.1).sin();
        let pos_y = (i as f32 * 0.1).cos();
        let pos_z = i as f32 * 0.05;
        let vel_x = 1.0f32;
        let vel_y = 0.5f32;
        let vel_z = -0.2f32;
        let flags = if i % 10 == 0 { 1 } else { 0 };

        stitch_batch.push(SpatialEntity {
            entity_id,
            pos_x,
            pos_y,
            pos_z,
            vel_x,
            vel_y,
            vel_z,
            flags,
        });

        prost_batch.push(ProstSpatialEntity {
            entity_id,
            pos_x,
            pos_y,
            pos_z,
            vel_x,
            vel_y,
            vel_z,
            flags,
        });
    }

    (
        stitch_batch,
        ProstSpatialEntityBatch {
            entities: prost_batch,
        },
    )
}
