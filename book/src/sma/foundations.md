# Foundations & The U-Cycle Lifecycle

The **Sewing Machine Architecture (SMA)** is the architectural pattern that powers `stitch-core`. It treats data processing as a physical stitch needle piercing cleanly through a stack of fabric layers and returning back up in a bounded **U-cycle**.

---

## 1. The Anatomy of a Stitch

Traditional middleware architectures (like tower or axum) rely on asynchronous futures or recursive closures (`next(req)`), which allocate memory or produce complex state machines that cannot be fully flattened by compiler optimizers.

SMA decomposes processing into two strictly bounded phases:

1. **Descent (`on_enter`)**: The needle pierces downward through ordered pipeline layers. Each layer inspects, validates, or mutates the shared blackboard context.
2. **Terminal Execution (`TerminalHandler`)**: The needle reaches the lowest point of the stroke, where the core domain logic or state transition executes.
3. **Ascent (`on_exit`)**: The needle ascends back upward through the layers in reverse topological order. Layers perform post-processing, metric recording, or cleanup.

```text
    Down (Descent)                           Up (Ascent)
         │                                        ▲
         ▼                                        │
  ┌──────────────┐                         ┌──────────────┐
  │ Layer 1      │ -- on_enter() ------->  │ Layer 1      │ -- on_exit()
  └──────┬───────┘                         └──────▲───────┘
         │                                        │
         ▼                                        │
  ┌──────────────┐                         ┌──────────────┐
  │ Layer 2      │ -- on_enter() ------->  │ Layer 2      │ -- on_exit()
  └──────┬───────┘                         └──────▲───────┘
         │                                        │
         └─────────────► ┌────────────────┐ ◄─────┘
                         │ Terminal Target│
                         └────────────────┘
```

---

## 2. The Monomorphic Layer Contract

In `stitch-core`, every layer implements the `Layer` trait:

```rust
pub trait Layer<C: ?Sized> {
    /// Executed during needle descent.
    /// Returns FlowControl to determine whether descent continues.
    fn on_enter(&mut self, context: &mut C) -> FlowControl;

    /// Executed during needle ascent in reverse pipeline order.
    fn on_exit(&mut self, context: &mut C);
}
```

Because the traversal order is resolved at compile time, the compiler completely eliminates recursion and call-stack growth, inlining both passes into a straight sequence of instructions.
