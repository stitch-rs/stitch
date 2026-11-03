# Stitch Facade (`stitch`) Roadmap

This roadmap tracks the development milestones, releases, and forward-looking capabilities for the `stitch` meta-crate.

---

## Current Release: v0.3.0 (Unified Ecosystem Integration)

- [x] **Unified Facade & Prelude**: Monomorphic re-export of `stitch-framework`, `stitch-data`, and `stitch-wire`.
- [x] **Zero Overhead Re-exports**: Feature-gated modules (`framework`, `data`, `wire`, `async`, `alloc`, `std`).
- [x] **End-to-End Exemplars**:
  - HFT OrderBook gateway (`hft_orderbook_gateway.rs`).
  - Multiplayer game snapshot delta synchronization (`game_snapshot_delta_sync.rs`).
  - Comprehensive telemetry streaming (`end_to_end_telemetry.rs`).
- [x] **Comprehensive mdBook Documentation**: Deep conceptual guides, interactive examples, and architecture references.

---

## v0.4.0 (Autonomous Codec Synthesis & IPC)

- [ ] **Procedural Pipeline Synthesis Macro**: Declare full SMA U-cycles with transport binding via single declarative macro.
- [ ] **Zero-Copy IPC Gateway**: Cross-process shared memory order routing and snapshot streaming.
