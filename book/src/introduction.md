# Introduction: The Stitch Ecosystem

Welcome to **The Stitch Architecture Book**, the comprehensive technical manual for building extreme-throughput, zero-allocation data processing pipelines and low-latency network applications in Rust.

---

## 1. The Core Problem: The Modern Systems Tax

Modern network services and real-time processing engines suffer from accumulated latency layers:

- **Dynamic Dispatch Overhead**: Trait objects (`Box<dyn Layer>`, `Box<dyn Service>`) introduce indirect vtable lookups, instruction cache misses, and inhibit inline compilation.
- **Heap Allocation Jitter**: Unbounded object allocations (`Vec<u8>`, `String`, heap-allocated messages) trigger global memory allocator contention and non-deterministic GC pauses or page faults.
- **Serialization Redundancy**: Common interchange formats (JSON, schema-heavy Protobuf) spend precious CPU cycles traversing complex pointer graphs, copying bytes, and executing variable-length zigzag decoders.
- **Memory Subsystem Abuse**: Cache lines (64 bytes on modern x86_64/ARM64 architectures) are frequently straddled or underutilized due to arbitrary field reordering and memory padding holes.

---

## 2. The Stitch Solution: Unified Architecture

The Stitch architecture addresses these challenges across three foundational layers:

```text
+-------------------------------------------------------------------+
|                        stitch Meta-Facade                         |
|         Unified ergonomic API, preludes, and domain exemplars      |
+---------------------------------+---------------------------------+
                                  |
         +------------------------+------------------------+
         |                                                 |
+-----------------------------------+             +----------------------------------+
|           stitch-core             |             |           stitch-data            |
| Monomorphic SMA Pipeline & Layers |             | Columnar Codecs & Transformations|
| Zero-Cost U-Cycle Traversal       |             | Gorilla, Dict, RLE, Bitpacking   |
+-----------------------------------+             +----------------------------------+
         |                                                 |
         +------------------------+------------------------+
                                  |
+---------------------------------+---------------------------------+
|                            stitch-wire                            |
| Zero-Copy Wire Framing, 64-Byte Cache Alignment, HW CRC32C        |
| Adapters: Tokio, Std, C-ABI, and WebAssembly                      |
+-------------------------------------------------------------------+
```

1. **`stitch-core`**: The Sewing Machine Architecture (SMA). A compile-time, zero-allocation pipeline design featuring monomorphic static dispatch, strict U-cycle traversal, and 64-byte aligned blackboards.
2. **`stitch-data`**: High-performance columnar encoding featuring Gorilla float delta-of-delta, SIMD-accelerated RLE, dictionary compression, and bitpacking with compile-time schema hashing (`SCHEMA_HASH`).
3. **`stitch-wire`**: Low-level framing and transport container specification with hardware Castagnoli CRC-32C, SWAR (SIMD Within A Register) magic resynchronization, and zero-copy streaming adapters.
4. **`stitch` (Meta-Facade)**: Unified developer facade combining pipeline primitives, columnar transforms, wire framing, and procedural derive macros (`#[derive(Record)]`, `#[derive(ColumnarRecord)]`).

---

## 3. Core Architectural Invariants

Throughout this book and codebase, four invariant rules are strictly enforced:

1. **`#![no_std]` Foundations**: Core crates (`stitch-core`, `stitch-wire-core`, `stitch-data-core`) do not depend on the standard library or dynamic memory allocation.
2. **Deterministic Latency**: No hidden allocations on the hot path. All scratch spaces and ring buffers are statically sized or caller-allocated.
3. **Mechanical Sympathy**: Physical memory layouts are engineered directly around hardware cache lines (64 bytes).
4. **Explicit Failure**: No silent error suppression, truncation, or dummy fallbacks. All boundary errors are represented as typed domain results.
