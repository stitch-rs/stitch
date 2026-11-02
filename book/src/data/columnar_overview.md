# Row-Oriented vs Columnar Data Layouts

Traditional serialization formats (JSON, MessagePack, Bincode, and Protobuf) encode records in **Row-Oriented** order (Array of Structures — AoS).

When transmitting or persisting large batches of homogeneous records (financial ticks, sensor feeds, or game state updates), row-oriented layouts severely limit compression ratios and throughput.

---

## 1. The Fundamental Compression Limit of AoS

Consider a tick structure:

```rust
struct Tick {
    timestamp: u64,
    price: f64,
    volume: u32,
}
```

In a row-oriented batch:
`[Timestamp 1, Price 1, Volume 1, Timestamp 2, Price 2, Volume 2, ...]`

Adjacent bytes have completely different statistical properties: timestamps grow monotonically, prices fluctuate around a narrow decimal range, and volumes are discrete integers. Generic compressors (like gzip, zstd, or lz4) must maintain large sliding dictionary windows and struggle to find repetition across alternating types.

---

## 2. Columnar Transformation (SoA)

`stitch-data` restructures records into **Structure of Arrays (SoA)** batches:

```text
Timestamps: [1000000001, 1000000002, 1000000003, ...]  -> Delta / Bitpacked
Prices:     [100.25,     100.26,     100.25,     ...]  -> Gorilla XOR Delta
Volumes:    [500,        500,        500,        ...]  -> Run-Length Encoding (RLE)
```

By grouping identical types contiguously:

1. **Domain-Specific Compression**: Each column uses its optimal encoding algorithm (e.g. Gorilla for floats, RLE for repeating masks).
2. **Extreme Compression Ratios**: Compression factors routinely reach 5x to 15x higher than row-oriented Protobuf or Bincode.
3. **SIMD Vectorization**: Contiguous integer and float arrays can be processed directly by CPU vector registers (AVX2, AVX-512, ARM NEON).
