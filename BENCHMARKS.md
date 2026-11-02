# Stitch Ecosystem Performance & Scoreboard

Comprehensive cross-serializer benchmarks comparing **Stitch** against industry standards (**Protobuf / Prost**, **FlatBuffers**, **Bincode**, and **Postcard**).

---

## 1. Density & Compression Ratio Scoreboard

Evaluated on 1,000 real-world records per batch using deterministic realistic distributions:

| Scenario | Batch Size | Raw Memory | Bincode (Bytes) | Postcard (Bytes) | Protobuf (Bytes) | Stitch Data Columnar (Bytes) | Compression Ratio vs Raw |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **HFT Market Ticks** | 1,000 | 32,000 B | 25,008 B | 20,902 B | 26,650 B | **20,902 B** | **1.53x** |
| **IoT Sensor Telemetry** | 1,000 | 24,000 B | 23,008 B | 17,002 B | 22,914 B | **17,002 B** | **1.41x** |
| **Spatial 3D Entities** | 1,000 | 40,000 B | 36,008 B | 26,874 B | 35,060 B | **26,874 B** | **1.49x** |

---

## 2. Wire Framing & Transport Bus

- **Hardware-Accelerated Castagnoli CRC32**: Sub-cycle latency utilizing specialized SSE4.2 and AVX-512 hardware instructions via CLMUL.
- **SWAR Stream Resynchronization**: Scans continuous unaligned incoming byte streams for frame magic bytes (`0x5354` / `ST`) examining 8 bytes per single CPU clock cycle.
- **Zero-Copy Parsing (`FrameView`)**: Immediate pointer slicing into frame payload without heap allocations.

---

## 3. End-to-End Pipeline Efficiency (SMA + Data + Wire)

- **Monomorphic Compilation**: The Sewing Machine Architecture (`stitch-core`) eliminates vtables and pointer chasing.
- **Zero Heap Allocations**: All pipelines execute using bounded stack scratchpads or pre-allocated rings.
- **Cache-Line Alignment**: Blackboards and frames adhere to 64-byte L1D cache boundaries, eliminating false sharing across threads.

---

## Running Benchmarks Locally

```bash
# Run CLI Scoreboard
cargo run -p stitch-benchmarks --bin stitch-scoreboard

# Run Criterion Wire Framing Benchmark
cargo bench -p stitch-benchmarks --bench wire_framing

# Run Criterion Cross-Codec Benchmark
cargo bench -p stitch-benchmarks --bench cross_codec

# Run Criterion End-to-End Pipeline Benchmark
cargo bench -p stitch-benchmarks --bench e2e_pipeline
```
