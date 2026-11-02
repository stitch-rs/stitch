# Container Specification & Layout Invariants

`stitch-wire` defines the binary protocol container format used for low-latency network communication, shared memory IPC, and file persistence.

---

## 1. Frame Binary Layout

Every frame on the wire begins with a deterministic, fixed-size header, followed by the payload:

```text
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Magic ('S','T','C','H')                 |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|    Version    |     Flags     |          Payload Length       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Sequence                            |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Schema Hash                          |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                     CRC32C (Castagnoli)                       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                       Payload (Variable)                      |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

---

## 2. Header Fields & Specifications

| Field | Type | Size | Description |
| :--- | :---: | :---: | :--- |
| **Magic** | `[u8; 4]` | 4 bytes | Protocol synchronization marker (`0x53, 0x54, 0x43, 0x48` -> `"STCH"`). |
| **Version** | `u8` | 1 byte | Protocol wire version (`0x01`). |
| **Flags** | `u8` | 1 byte | Bitmask: compression flags, priority, and heartbeat markers. |
| **Payload Length** | `u16` | 2 bytes | Exact length in bytes of the following payload (up to 65,535 bytes). |
| **Sequence** | `u64` | 8 bytes | Monotonically increasing datagram sequence ID for loss detection. |
| **Schema Hash** | `u64` | 8 bytes | Compile-time 64-bit schema fingerprint ensuring layout compatibility. |
| **CRC-32C** | `u32` | 4 bytes | Hardware Castagnoli polynomial checksum protecting header and payload. |

Total Header Size: **28 bytes** (dense packed) or **64 bytes** (when configured in cache-line aligned mode).

---

## 3. Invariants & Zero-Copy Unpacking

1. **Little-Endian Standard**: All integer fields (`u16`, `u32`, `u64`) are encoded in canonical little-endian byte order (`#[inline] to_le_bytes()`).
2. **Zero-Copy Views**: `stitch-wire` can unpack frames directly from a borrowed byte slice (`&'a [u8]`) without allocating intermediate buffers on the heap.
3. **Payload Isolation**: The header and payload are contiguous in memory, allowing `zerocopy::FromBytes` casting when structures are `repr(C)`.
