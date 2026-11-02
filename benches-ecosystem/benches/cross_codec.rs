use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use prost::Message;
use stitch::prelude::*;
use stitch_benchmarks::scenarios::*;

fn bench_cross_codec(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_codec_hft_market_ticks");

    let count = 500;
    let (stitch_ticks, prost_batch) = generate_market_ticks(count);

    group.throughput(Throughput::Elements(count as u64));

    // 1. Stitch Columnar Encode
    let mut stitch_out = vec![0u8; 64 * 1024];
    group.bench_function("stitch_encode_columnar", |b| {
        b.iter(|| {
            let written = stitch_benchmarks::MarketTick::encode_batch(
                criterion::black_box(&stitch_ticks),
                criterion::black_box(&mut stitch_out),
            )
            .expect("encode columnar");
            criterion::black_box(written);
        })
    });

    // 2. Protobuf (Prost) Encode
    let mut prost_out = Vec::with_capacity(64 * 1024);
    group.bench_function("protobuf_prost_encode", |b| {
        b.iter(|| {
            prost_out.clear();
            prost_batch.encode(&mut prost_out).expect("encode prost");
            criterion::black_box(prost_out.len());
        })
    });

    // 3. Bincode Encode
    let mut bincode_out = Vec::with_capacity(64 * 1024);
    group.bench_function("bincode_encode", |b| {
        b.iter(|| {
            bincode_out.clear();
            bincode::serialize_into(&mut bincode_out, criterion::black_box(&stitch_ticks))
                .expect("bincode encode");
            criterion::black_box(bincode_out.len());
        })
    });

    // 4. Postcard Encode
    let mut postcard_out = vec![0u8; 64 * 1024];
    group.bench_function("postcard_encode", |b| {
        b.iter(|| {
            let slice = postcard::to_slice(criterion::black_box(&stitch_ticks), &mut postcard_out)
                .expect("postcard encode");
            criterion::black_box(slice.len());
        })
    });

    group.finish();
}

criterion_group!(benches, bench_cross_codec);
criterion_main!(benches);
