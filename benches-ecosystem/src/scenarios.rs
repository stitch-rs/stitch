//! Realistic deterministic scenario generators utilizing pseudo-random distributions.

use crate::*;

/// Deterministic 64-bit Linear Congruential Generator matching `ipatlas` standard.
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        let frac = (self.next_u64() >> 40) as f32 / ((1u32 << 24) as f32);
        min + frac * (max - min)
    }

    fn next_f64_range(&mut self, min: f64, max: f64) -> f64 {
        let frac = (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64);
        min + frac * (max - min)
    }
}

/// Generates a deterministic sequence of HFT market ticks with realistic price movement.
pub fn generate_market_ticks(count: usize) -> (Vec<MarketTick>, ProstMarketTickBatch) {
    let mut stitch_ticks = Vec::with_capacity(count);
    let mut prost_ticks = Vec::with_capacity(count);

    let base_time: u64 = 1_700_000_000_000_000_000;
    let mut current_price = 150.25f64;
    let mut rng = Lcg::new(0x4D5A_9000_1234_5678);

    for i in 0..count {
        let ts_jitter = rng.next_u64() % 100;
        let ts = base_time + (i as u64 * 250) + ts_jitter;
        let instrument = (rng.next_u64() % 8) as u32;
        let delta_price = rng.next_f64_range(-0.05, 0.05);
        current_price = (current_price + delta_price).max(1.0);
        let volume = 100 * ((rng.next_u64() % 20) + 1) as u32;
        let side = (rng.next_u64() % 2) as u8;

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

/// Generates a deterministic batch of IoT sensor readings with realistic sensor drift.
pub fn generate_sensor_telemetry(
    count: usize,
) -> (Vec<SensorTelemetry>, ProstSensorTelemetryBatch) {
    let mut stitch_batch = Vec::with_capacity(count);
    let mut prost_batch = Vec::with_capacity(count);

    let base_time: u64 = 1_700_000_000_000;
    let mut temp = 22.5f32;
    let mut hum = 45.0f32;
    let mut rng = Lcg::new(0xDEAD_BEEF_CAFE_BABE);

    for i in 0..count {
        let ts = base_time + (i as u64 * 100);
        let device_id = (rng.next_u64() % 16) as u32;
        temp += rng.next_f32_range(-0.15, 0.15);
        hum = (hum + rng.next_f32_range(-0.3, 0.3)).clamp(10.0, 95.0);
        let battery = (80 + (rng.next_u64() % 21)) as u8;
        let status = if rng.next_u64().is_multiple_of(50) {
            0x01
        } else {
            0x00
        };

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

/// Generates a deterministic batch of spatial 3D physics entities with non-degenerate kinematics.
pub fn generate_spatial_entities(count: usize) -> (Vec<SpatialEntity>, ProstSpatialEntityBatch) {
    let mut stitch_batch = Vec::with_capacity(count);
    let mut prost_batch = Vec::with_capacity(count);
    let mut rng = Lcg::new(0x0123_4567_89AB_CDEF);

    for i in 0..count {
        let entity_id = i as u64;
        let angle = i as f32 * 0.1;
        let pos_x = angle.sin() * 100.0;
        let pos_y = angle.cos() * 100.0;
        let pos_z = i as f32 * 0.05;
        let vel_x = rng.next_f32_range(-25.0, 25.0);
        let vel_y = rng.next_f32_range(-25.0, 25.0);
        let vel_z = rng.next_f32_range(-5.0, 5.0);
        let flags = if rng.next_u64().is_multiple_of(10) {
            1
        } else {
            0
        };

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
