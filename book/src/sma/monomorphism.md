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

---

## 3. The Canonical DX Standard: `#[pipeline(...)]` Newtype Attribute

To eliminate complex nested type declarations and DSL macro friction, Stitch provides the canonical `#[pipeline(...)]` attribute macro:

```rust
use stitch::prelude::*;

#[pipeline(
    context = TelemetryBlackboard,
    intent = SensorPayload,
    outcome = SensorPayload,
    error = &'static str,
    layers = [RateLimitLayer, AuthValidationLayer, ChecksumLayer],
    terminal = TerminalCore,
)]
pub struct SensorPipeline;
```

### Architectural Guarantees

1. **Natural Forward Ordering**: Layers in `layers = [A, B, C]` are listed in top-down descent order ($A \to B \to C \to \text{Terminal}$).
2. **`#[repr(transparent)]` Tuple Struct**: The macro expands into a concrete newtype struct wrapping `Pipeline<...>` with zero memory overhead.
3. **Ergonomic Construction**: Automatically generates `SensorPipeline::new(...)` taking each layer and terminal as typed arguments.
4. **IDE & Tooling Support**: Unlike procedural function-like macros, the pipeline is a first-class named struct indexed by `rust-analyzer` and documented cleanly in `rustdoc`.
