//! Complete end-to-end telemetry pipeline using the unified `stitch` ecosystem.
//!
//! Demonstrates:
//! 1. Memory scratchpad aligned to 64-byte L1D cache-line boundary ([`Blackboard`]).
//! 2. Zero-cost monomorphic U-cycle execution pipeline ([`Pipeline`]).
//! 3. Zero-allocation data encoding with compile-time schema hashing ([`StitchRecord`]).
//! 4. Physical transport framing with hardware Castagnoli CRC32 ([`WireHeader`], [`pack_record`]).

use stitch::prelude::*;

#[repr(C, align(64))]
#[derive(Default, Debug)]
struct TelemetryContext {
    ingested_events: u64,
    anomaly_count: u64,
}

impl Blackboard for TelemetryContext {}

assert_blackboard_aligned!(TelemetryContext);

#[derive(StitchRecord, Debug, PartialEq, Clone, Default)]
struct TelemetryEvent {
    #[stitch(codec = "varint")]
    pub node_id: u32,
    #[stitch(codec = "zigzag")]
    pub temperature_delta_c: i32,
    #[stitch(codec = "varint")]
    pub rpm: u32,
}

struct AnomalyDetectorLayer;

impl Layer<TelemetryContext, TelemetryEvent, TelemetryEvent, &'static str>
    for AnomalyDetectorLayer
{
    fn on_enter(
        &self,
        ctx: &mut TelemetryContext,
        event: TelemetryEvent,
    ) -> FlowControl<TelemetryEvent, TelemetryEvent, &'static str> {
        if event.temperature_delta_c > 100 {
            ctx.anomaly_count += 1;
            FlowControl::Halt("Temperature threshold exceeded (>100C)")
        } else {
            ctx.ingested_events += 1;
            FlowControl::Proceed(event)
        }
    }

    fn on_exit(
        &self,
        _ctx: &mut TelemetryContext,
        _outcome: &mut Result<TelemetryEvent, &'static str>,
    ) {
        // Post-processing hooks / metrics instrumentation
    }
}

struct WirePublisherTerminal;

impl Terminal<TelemetryContext, TelemetryEvent, TelemetryEvent, &'static str>
    for WirePublisherTerminal
{
    fn execute(
        &mut self,
        _ctx: &mut TelemetryContext,
        event: TelemetryEvent,
    ) -> Result<TelemetryEvent, &'static str> {
        Ok(event)
    }
}

fn main() {
    println!("=== Stitch Systems Architecture: End-to-End Telemetry Pipeline ===");

    let mut ctx = TelemetryContext::default();
    let mut pipeline = Pipeline::on_terminal(WirePublisherTerminal).wrap(AnomalyDetectorLayer);

    let event = TelemetryEvent {
        node_id: 101,
        temperature_delta_c: 12,
        rpm: 3600,
    };

    println!("[1] Processing intent through SMA U-cycle: {:?}", event);
    let dispatched = pipeline
        .dispatch(&mut ctx, event)
        .expect("Pipeline dispatch failed");
    println!(
        "    State updated -> Ingested: {}, Anomalies: {}",
        ctx.ingested_events, ctx.anomaly_count
    );

    // Wire framing with hardware-accelerated CRC32
    let mut scratch = [0u8; 128];
    let mut wire_buffer = [0u8; 256];

    let bytes_written = pack_record(
        &dispatched,
        WireOptimizationProfile::Balanced,
        &mut scratch,
        &mut wire_buffer,
    )
    .expect("Wire packing failed");

    println!(
        "[2] Wire frame generated: {} bytes (Magic: 0x{:02X}, Version: 0x{:02X})",
        bytes_written, wire_buffer[0], wire_buffer[1]
    );

    // Unpack on receiving end
    let received: TelemetryEvent =
        unpack_record(&wire_buffer[..bytes_written]).expect("Wire unpack failed");
    println!(
        "[3] Wire frame unpacked & verified with 100% fidelity: {:?}",
        received
    );
    assert_eq!(dispatched, received);

    println!("=== End-to-End Pipeline Completed Successfully ===");
}
