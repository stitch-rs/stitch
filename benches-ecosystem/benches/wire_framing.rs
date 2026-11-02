use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use stitch::prelude::*;

fn bench_wire_packing(c: &mut Criterion) {
    let mut group = c.benchmark_group("wire_framing");

    let payload = vec![0xABu8; 1024]; // 1 KB payload
    let mut out_buffer = vec![0u8; 2048];

    let wire_header = WireHeader {
        profile: WireOptimizationProfile::Balanced,
        flags: WireFlags {
            is_compressed: true,
            is_chunked: false,
            has_schema_hash: false,
        },
        payload_len: payload.len(),
        schema_hash: None,
    };

    group.throughput(Throughput::Bytes(payload.len() as u64));

    group.bench_function("stitch_wire_pack_1kb", |b| {
        b.iter(|| {
            let written = pack_frame(
                criterion::black_box(&wire_header),
                criterion::black_box(&payload),
                criterion::black_box(&mut out_buffer),
            )
            .expect("pack frame");
            criterion::black_box(written);
        })
    });

    let frame_len = pack_frame(&wire_header, &payload, &mut out_buffer).expect("pack frame");
    let packed_bytes = &out_buffer[..frame_len];

    group.bench_function("stitch_wire_unpack_view_1kb", |b| {
        b.iter(|| {
            let (hdr, data) =
                unpack_frame(criterion::black_box(packed_bytes)).expect("parse frame view");
            criterion::black_box((hdr, data));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_wire_packing);
criterion_main!(benches);
