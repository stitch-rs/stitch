# Vectored Scatter-Gather I/O

In high-frequency network applications, assembling a complete packet by copying the header and multiple payload slices into a single contiguous buffer is an unnecessary memory copy.

`stitch-wire` supports **Vectored Scatter-Gather I/O**, allowing the operating system kernel to directly gather distinct memory regions into a single network packet.

---

## 1. Zero-Copy Header & Payload Assembly

When dispatching a batch of messages:

1. The **Header** resides in a small, stack-allocated or ring-allocated 28-byte array.
2. The **Payload** resides in a separate columnar buffer or order book record.
3. Rather than allocating a new `Vec<u8>` or performing `memcpy`, `stitch-wire` prepares a 2-segment `IoSlice`:

```rust
use std::io::IoSlice;
use stitch_wire::udp::pack_frame_vectored;

let header_buf: [u8; 28]; // Computed on stack
let payload_slice: &[u8]; // Borrowed from columnar compressor

let slices: [IoSlice; 2] = pack_frame_vectored(
    &mut header_buf,
    flags,
    seq,
    schema_hash,
    payload_slice,
);

// Dispatched via OS sendmsg / sendmmsg directly to the network NIC ring:
socket.send_vectored(&slices)?;
```

---

## 2. Kernel-Level Advantage

On Linux and modern OS kernels, `sendmsg` / `sendmmsg` with `iovec` writes memory directly to the Network Interface Card (NIC) transmit descriptors via DMA (Direct Memory Access).

This achieves true **0-copy transmission**: zero heap allocations, zero user-space intermediate byte copies, and minimal cache pollution.
