# Cache-Line Engineering & 64-Byte Alignment

Hardware memory subsystems do not read individual bytes from DRAM; they fetch chunks of **64 contiguous bytes** known as cache lines.

When an access spans across two cache lines, or when two CPU cores write to independent fields located within the same 64-byte line (false sharing), latency increases dramatically.

---

## 1. The 64-Byte Invariant

In the Stitch ecosystem, hot blackboards, headers, and ring buffer slots are aligned to 64 bytes using `#[repr(C, align(64))]`:

```rust
#[repr(C, align(64))]
pub struct OrderBookBlackboard {
    pub symbol_id: u32,
    pub sequence: u64,
    pub best_bid: u64,
    pub best_ask: u64,
    pub bid_qty: u32,
    pub ask_qty: u32,
    pub flags: u16,
    pub _reserved: [u8; 30], // Pads exactly to 64 bytes
}
```

### Why Exact Alignment Matters

1. **Single Cache-Line Access**: A 64-byte aligned struct is guaranteed to fit entirely within a single L1D cache line. Any read or write hits the same cache block.
2. **Elimination of False Sharing**: In multi-threaded worker pools, aligning per-core blackboards or ring buffer queues prevents false sharing cache invalidations between CPU cores.

---

## 2. Compile-Time Verification

Stitch uses compile-time assertions to guarantee size and alignment invariants:

```rust
const _: () = {
    assert!(core::mem::size_of::<OrderBookBlackboard>() == 64);
    assert!(core::mem::align_of::<OrderBookBlackboard>() == 64);
};
```

If a developer adds a field without adjusting padding, or if compiler alignment shifts, compilation fails immediately at build time with a clear error.
