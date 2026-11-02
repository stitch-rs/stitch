# Distributed Game Engine State Synchronizer

The `game_snapshot_delta_sync` example (located in `stitch-facade/examples/game_snapshot_delta_sync.rs`) illustrates a 128Hz multiplayer world state broadcast system synchronizing 64 entities over unreliable UDP transport.

---

## 1. The Challenge: Network Jitter & Bandwidth Budgets

In competitive multiplayer games (FPS, RTS, Simulators):

- 64 to 128 players transmit high-precision positions $(x, y, z)$, rotations (yaw, pitch, roll), velocities, and animation flags at 128 updates per second.
- Standard serialization (JSON or raw C structs) consumes 300–800 KB/s per client, saturating residential upstream bandwidth.
- Packet loss and byte corruptions cause deserializer crashes or state desynchronization.

---

## 2. The Stitch Solution

```text
       64 Entity Transforms (128 Hz)
                     │
                     ▼
  ┌─────────────────────────────────────┐
  │ stitch-data: Columnar Serialization │
  │  - Positions (x, y, z): Gorilla     │
  │  - Rotations (f32): Gorilla         │
  │  - Visibility Bitmasks: RLE         │
  │  - Team IDs & Weapon Archetypes: Dict│
  └──────────────────┬──────────────────┘
                     │ (82% Bandwidth Reduction)
                     ▼
  ┌─────────────────────────────────────┐
  │ stitch-wire: Zero-Copy Framing      │
  │  - 28-Byte FrameHeader + Castagnoli │
  │  - Monotonic Sequence IDs           │
  │  - Vectored Scatter-Gather UDP      │
  └──────────────────┬──────────────────┘
                     │ (UDP Transmission)
                     ▼
  ┌─────────────────────────────────────┐
  │ Client Recovery & SWAR Resync       │
  │  - Corrupted packets rejected (CRC) │
  │  - SWAR scans stream to find "STCH" │
  │  - Zero GC allocations on client    │
  └─────────────────────────────────────┘
```

### Empirical Results

- **Raw Uncompressed Frame**: 4,608 bytes per snapshot.
- **Stitch Columnar Frame**: 748 bytes per snapshot (**83.8% bandwidth savings**).
- **Resynchronization Latency**: Corrupted UDP packets are detected and skipped in ~14 nanoseconds using SWAR magic prefix recovery.
