# Changelog

All notable changes to `stitch` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] - 2026-11-03

### Added
- **Unified Prelude**: Single-import access to execution kernel, data engine, and wire framing.
- **End-to-End Exemplars**:
  - `hft_orderbook_gateway.rs`: Sub-microsecond market data framing and delta decoding.
  - `game_snapshot_delta_sync.rs`: Realtime physics entity state compression and datagram sync.
  - `end_to_end_telemetry.rs`: Multi-sensor telemetry streaming with Castagnoli CRC-32C.
- **Ecosystem mdBook**: Complete documentation suite covering SMA design axioms, data layout, wire framing, and mechanical sympathy.

### Changed
- Integrated composable SMA wire framing pipeline (`WireBlackboard`, `WirePresets`).
- Updated all domain exemplars to canonical `WireFlags::new`.
