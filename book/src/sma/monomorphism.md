# Monomorphic Pipelines vs Dynamic Dispatch

Dynamic dispatch is the silent performance killer of low-latency systems. This chapter demonstrates how `stitch-core` achieves zero-cost pipeline composition through compile-time type-level nesting.

---

## 1. Type-Level Composition (`PipelineStack`)

Instead of storing layers in a slice of trait objects (`Vec<Box<dyn Layer>>`), `stitch-core` uses a recursive cons-list structure at the type level:

```rust
pub struct LayerCons<H, T> {
    pub head: H,
    pub tail: T,
}

pub struct LayerNil;
```

When you chain layers using the fluent builder API:

```rust
let pipeline = PipelineBuilder::new()
    .with(RateLimitLayer::new(100_000))
    .with(AuthValidationLayer)
    .with(ChecksumLayer)
    .build(TerminalCore);
```

The resulting pipeline type is concrete and fully known to the Rust compiler:

```rust
Pipeline<
    LayerCons<RateLimitLayer,
        LayerCons<AuthValidationLayer,
            LayerCons<ChecksumLayer, LayerNil>>>,
    TerminalCore
>
```

---

## 2. Zero-Cost LLVM Inlining

Because there are no trait objects or vtables:

1. The compiler monomorphizes the entire descent (`on_enter`) and ascent (`on_exit`) sequence.
2. If a layer's `on_exit` method is empty (the default no-op implementation), LLVM completely removes the function call and branch during dead-code elimination.
3. Intermediate registers are preserved across layer boundaries instead of being spilled to the stack to comply with standard C calling conventions.
4. Hot loops processing thousands of packets achieve performance identical to hand-written, monolithic assembly code.
