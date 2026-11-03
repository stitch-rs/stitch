//! End-to-end integration test for the unified Stitch facade (`stitch::prelude::*`).

use stitch::prelude::*;

#[repr(C, align(64))]
#[derive(Default)]
struct TelemetryBlackboard {
    processed_count: u64,
    last_sensor_id: u32,
}

impl Blackboard for TelemetryBlackboard {}

// Verify Scrooge 64-byte L1D cache-line alignment contract at compile-time
assert_blackboard_aligned!(TelemetryBlackboard);

#[derive(Record, Debug, PartialEq, Clone, Default)]
struct SensorPayload {
    #[stitch(codec = "varint")]
    pub sensor_id: u32,
    #[stitch(codec = "zigzag")]
    pub temperature_milli_c: i32,
    #[stitch(codec = "varint")]
    pub pressure_hpa: u32,
}

struct ValidationLayer;

impl Layer<TelemetryBlackboard, SensorPayload, SensorPayload, &'static str> for ValidationLayer {
    fn on_enter(
        &self,
        ctx: &mut TelemetryBlackboard,
        intent: SensorPayload,
    ) -> FlowControl<SensorPayload, SensorPayload, &'static str> {
        if intent.sensor_id == 0 {
            FlowControl::Halt("Invalid sensor ID: 0")
        } else {
            ctx.last_sensor_id = intent.sensor_id;
            FlowControl::Proceed(intent)
        }
    }

    fn on_exit(
        &self,
        ctx: &mut TelemetryBlackboard,
        outcome: &mut Result<SensorPayload, &'static str>,
    ) {
        if outcome.is_ok() {
            ctx.processed_count += 1;
        }
    }
}

struct EgressTerminal;

impl Terminal<TelemetryBlackboard, SensorPayload, SensorPayload, &'static str> for EgressTerminal {
    fn execute(
        &mut self,
        _ctx: &mut TelemetryBlackboard,
        intent: SensorPayload,
    ) -> Result<SensorPayload, &'static str> {
        Ok(intent)
    }
}

#[test]
fn test_end_to_end_sma_data_and_wire_pipeline() {
    let mut blackboard = TelemetryBlackboard::default();

    // 1. Build Monomorphic SMA Pipeline
    let mut pipeline = Pipeline::on_terminal(EgressTerminal).wrap(ValidationLayer);

    let reading = SensorPayload {
        sensor_id: 42,
        temperature_milli_c: -1250,
        pressure_hpa: 1013,
    };

    // 2. Execute through SMA U-cycle
    let processed = pipeline
        .dispatch(&mut blackboard, reading)
        .expect("Pipeline execution failed");
    assert_eq!(processed.sensor_id, 42);
    assert_eq!(blackboard.processed_count, 1);
    assert_eq!(blackboard.last_sensor_id, 42);

    // 3. Low-Level Verification: Serialize record via Stitch Data Engine
    let mut data_scratch = [0u8; 128];
    let encoded_len = processed
        .encode_record_profile(OptimizationProfile::Balanced, &mut data_scratch)
        .expect("Record encoding failed");

    let payload = &data_scratch[..encoded_len];

    // 4. Low-Level Verification: Wrap in Wire Transport Frame with Hardware Castagnoli CRC32
    let wire_header = WireHeader {
        profile: WireOptimizationProfile::Balanced,
        flags: WireFlags::new(true, false, true),
        payload_len: encoded_len,
        schema_hash: Some(SensorPayload::SCHEMA_HASH),
    };

    let mut frame_buf = [0u8; 256];
    let total_frame_len =
        pack_frame(&wire_header, payload, &mut frame_buf).expect("Framing failed");

    // 5. Verify Frame Characteristics
    assert!(total_frame_len > encoded_len);
    assert_eq!(frame_buf[0], WIRE_MAGIC);
    assert_eq!(frame_buf[1], WIRE_VERSION_1);

    // 6. Unpack Wire Frame
    let (unpacked_header, unpacked_payload) =
        unpack_frame(&frame_buf[..total_frame_len]).expect("Unpack failed");
    assert_eq!(unpacked_header.payload_len, encoded_len);
    assert_eq!(
        unpacked_header.schema_hash,
        Some(SensorPayload::SCHEMA_HASH)
    );
    assert_eq!(unpacked_payload, payload);

    // 7. Decode record back to verify exact fidelity
    let (decoded, consumed) =
        SensorPayload::decode_record_profile(unpacked_payload, OptimizationProfile::Balanced)
            .expect("Decode failed");

    assert_eq!(consumed, encoded_len);
    assert_eq!(decoded, processed);

    // 8. High-Level Adapter Verification: pack_record and unpack_record
    let mut adapter_scratch = [0u8; 128];
    let mut adapter_frame = [0u8; 256];
    let wire_len = pack_record(
        &processed,
        WireOptimizationProfile::Balanced,
        &mut adapter_scratch,
        &mut adapter_frame,
    )
    .expect("pack_record failed");

    let unpacked_record: SensorPayload =
        unpack_record(&adapter_frame[..wire_len]).expect("unpack_record failed");
    assert_eq!(unpacked_record, processed);
}

#[test]
fn test_wire_resync_recovery() {
    let wire_header = WireHeader {
        profile: WireOptimizationProfile::Direct,
        flags: WireFlags::new(false, false, true),
        payload_len: 4,
        schema_hash: Some(0x1234_5678),
    };

    let payload = [10, 20, 30, 40];
    let mut frame_buf = [0u8; 64];
    let frame_len = pack_frame(&wire_header, &payload, &mut frame_buf).expect("Packing failed");

    // Create a stream with preceding garbage bytes (line noise)
    let mut noisy_stream = [0u8; 128];
    noisy_stream[0..15].fill(0xEE); // Noise
    noisy_stream[15..15 + frame_len].copy_from_slice(&frame_buf[..frame_len]);

    // Resync parser should locate frame
    let (skipped, frame) =
        resync_frame(&noisy_stream[..15 + frame_len]).expect("Resync should succeed");
    assert_eq!(skipped, 15);
    assert_eq!(frame.header.payload_len, 4);
    assert_eq!(frame.header.schema_hash, Some(0x1234_5678));
    assert_eq!(frame.payload, &payload);
}

#[pipeline(
    context = TelemetryBlackboard,
    intent = SensorPayload,
    outcome = SensorPayload,
    error = &'static str,
    layers = [ValidationLayer],
    terminal = EgressTerminal,
    crate = stitch::core,
)]
struct SensorPipeline;

#[test]
fn test_attribute_newtype_pipeline_in_facade() {
    let mut ctx = TelemetryBlackboard::default();
    let mut pipe = SensorPipeline::new(ValidationLayer, EgressTerminal);

    let input = SensorPayload {
        sensor_id: 101,
        temperature_milli_c: -500,
        pressure_hpa: 1013,
    };
    let output = pipe
        .run_cycle(&mut ctx, input.clone())
        .expect("Traversal must succeed");
    assert_eq!(output, input);
    assert_eq!(ctx.processed_count, 1);
    assert_eq!(ctx.last_sensor_id, 101);
}
