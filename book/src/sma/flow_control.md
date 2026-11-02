# Zero-Allocation Flow Control & Blackboards

In high-throughput systems, returning `Result<T, E>` through deeply nested handler stacks often incurs heap allocation (via `Box<dyn Error>`) or bloats enum return registers across call boundaries.

SMA solves this through two complementary constructs: **Zero-Allocation FlowControl** and the **Blackboard Pattern**.

---

## 1. FlowControl Primitives

`FlowControl` is a compact enum that instructs the SMA needle driver how to proceed during descent:

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowControl {
    /// Continue downward descent to the next layer or terminal handler.
    Continue = 0,
    /// Immediately abort downward descent and begin ascending (U-turn).
    Break = 1,
}
```

Because `FlowControl` is represented as a single byte (`#[repr(u8)]`), it is returned directly in a CPU general-purpose register (e.g. `AL` on x86_64) without stack writes.

### Early Short-Circuiting (The U-Turn)

When any layer encounters an invalid condition (e.g., rate limit exceeded or invalid signature), it returns `FlowControl::Break`:

1. The needle immediately ceases descent; subsequent downstream layers and the terminal handler are skipped.
2. The needle ascends back through all layers that have already executed `on_enter`, invoking their `on_exit` handlers.
3. This guarantees that any acquired state, transaction counters, or metrics are strictly and deterministically unwound without memory leaks.

---

## 2. The Blackboard Pattern

Rather than passing payloads as values that move across function arguments, SMA adopts the **Blackboard Pattern**.

A single mutable reference (`&mut Context`) is passed down and up the pipeline:

```rust
pub struct RequestBlackboard {
    pub raw_bytes: [u8; 1500],
    pub payload_len: usize,
    pub client_id: u32,
    pub is_authenticated: bool,
    pub execution_status: u16,
}
```

All layers operate in-place upon this memory region. This ensures:

- **Zero Allocations**: Memory is allocated once (on the thread stack or pre-allocated ring buffer).
- **Cache Warmth**: The blackboard stays resident in the CPU L1 data cache throughout the entire duration of the stroke.
