use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use stitch::prelude::*;
use stitch_benchmarks::scenarios::*;

fn bench_e2e_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_pipeline_telemetry");

    let count = 250;
    let (telemetry, _) = generate_sensor_telemetry(count);

    group.throughput(Throughput::Elements(count as u64));

    let mut columnar_batch_buf = vec![0u8; 64 * 1024];
    let mut framed_wire_buf = vec![0u8; 64 * 1024];

    // Full Pipeline: Monomorphic Batch Encode -> Wire Frame Pack (Castagnoli CRC32) -> FrameView Unpack
    group.bench_function("stitch_e2e_encode_frame_and_unpack_view", |b| {
        b.iter(|| {
            // 1. Columnar compression
            let payload_len = stitch_benchmarks::SensorTelemetry::encode_batch(
                criterion::black_box(&telemetry),
                criterion::black_box(&mut columnar_batch_buf),
            )
            .expect("encode columnar");

            let wire_header = WireHeader {
                profile: WireOptimizationProfile::Balanced,
                flags: WireFlags {
                    is_compressed: true,
                    is_chunked: false,
                    has_schema_hash: false,
                },
                payload_len,
                schema_hash: None,
            };

            // 2. Wire framing
            let frame_len = pack_frame(
                &wire_header,
                &columnar_batch_buf[..payload_len],
                &mut framed_wire_buf,
            )
            .expect("pack frame");

            // 3. Zero-copy parse & view verification
            let (_hdr, data) =
                unpack_frame(&framed_wire_buf[..frame_len]).expect("parse wire frame");

            criterion::black_box(data.len());
        })
    });

    group.finish();
}

criterion_group!(benches, bench_e2e_pipeline);
criterion_main!(benches);
