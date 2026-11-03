# Stitch Ecosystem Architecture (`stitch`) Reference

**Specification Identifier:** `ARCH-SPEC-STITCH-FACADE-001`  
**Classification:** Normative Ecosystem Meta-Architecture & Layer Integration Contracts  
**Status:** Canonical (Single Source of Truth)  

---

## 1. Normative Authority & Ecosystem Topology

`stitch` is the primary entry point and unified facade meta-crate for the Stitch Systems Architecture ecosystem.

```text
+-----------------------------------------------------------------------+
|                             stitch                                    |
|              (Unified Facade, Prelude & Meta-Crate)                   |
+-----------------------------------------------------------------------+
        │                               │                       │
        ▼                               ▼                       ▼
+-----------------------+   +-----------------------+   +---------------+
|    stitch-framework   |   |      stitch-data      |   |  stitch-wire  |
|  (Execution Kernel)   |   |   (Layout & Codecs)   |   |  (Transport)  |
+-----------------------+   +-----------------------+   +---------------+
        │                               │                       │
        └───────────────────────┬───────┴───────────────────────┘
                                ▼
        +───────────────────────────────────────────────+
        |                 stitch-core                   |
        |  (The Sewing Machine Architecture Core Engine)|
        +-----------------------------------------------+
```

---

## 2. Layer Taxonomy & Separation of Concerns

1. **`stitch-framework` (The Execution Engine)**:
   - Houses `stitch-core`, `stitch-macros`, and `stitch-async`.
   - Pure SMA compile-time monomorphism: `Layer`, `Terminal`, `Pipeline`, `Blackboard`, `FlowControl`.
   - Defines the universal U-cycle execution paradigm.

2. **`stitch-data` (The High-Density Data Engine)**:
   - Zero-allocation columnar transposition (AoS to SoA).
   - Monomorphic compression pipelines (`CompressionPipeline`, `DecompressionPipeline`).
   - Bitpacking, delta encoding, Gorilla floating point XOR, RLE, and dictionary codecs.
   - $O(1)$ random-access columnar slicing (`ColumnSliceIndex`, `read_bitpacked_u32_at`).

3. **`stitch-wire` (The Microsecond Transport Bus)**:
   - Physical 28-byte framing with hardware Castagnoli CRC-32C.
   - Microsecond packet serialization and SWAR resynchronization.
   - Zero socket leaks in core: pure SMA layers and `[IoSlice; 2]` scatter-gather descriptors.
   - Pluggable platform adapters (`udp`, `tokio`, `std`, `c`, `wasm`).

---

## 3. The Scrooge Systems Mindset Axioms

Every component exposed through `stitch::prelude` complies with:
- **Zero Dynamic Allocation**: Scratchpad buffers and stack arrays only.
- **Cache-Line Alignment**: Contexts and state machines align to 64 bytes (`#[repr(C, align(64))]`).
- **Monomorphic Dispatch**: Zero `Box<dyn ...>` in hot paths.
- **Strict Fail-Fast**: Typed domain errors (`StitchError`, `WireError`), zero silent fallbacks or masked drops.
