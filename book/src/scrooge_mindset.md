# The "Scrooge" Systems Mindset

In the Stitch ecosystem, every architectural decision is guided by the **"Scrooge" Systems Mindset** — a philosophy of pathological zero-waste and mechanical sympathy.

---

## 1. Zero Waste: Every Byte and Cycle Counts

Modern hardware executes billions of cycles per second, yet high-level abstractions routinely waste more than 90% of effective memory bandwidth on cache misses, pointer indirections, and allocator locks.

The Scrooge mindset rejects lazy engineering habits:

- **Cache Line Straddling**: On x86_64 and ARM64 CPUs, cache lines are 64 bytes wide. If a frequently read data structure occupies 68 bytes or crosses a 64-byte boundary unaligned, every memory access forces two separate L1D cache line fetches instead of one.
- **Padding Holes**: Structs without explicit field ordering often contain invisible padding bytes introduced by compiler alignment rules. In Stitch, fields are ordered descending by alignment or packed cleanly (`#[repr(C)]`).
- **Pointer-Chasing Graphs**: Heap-allocated node graphs (such as linked lists, boxed closures, or nested `Vec` containers) scatter memory addresses across DRAM. Stitch prioritizes flat, contiguous memory slices (`zerocopy`, contiguous ring buffers, columnar slabs).

---

## 2. Monomorphism Over Polymorphic Indirection

Dynamic dispatch (`dyn Trait`) creates three performance penalties:

1. An extra memory read to fetch the vtable pointer.
2. An indirect branch instruction (`call rax`), which frequently defeats hardware branch target predictors (BTB).
3. Complete defeat of compiler optimization: the LLVM optimizer cannot inline functions across dynamic trait objects, eliminating dead-code elimination, vectorization, and register promotion.

Stitch replaces dynamic trait objects with monomorphic compile-time composition:

```rust
// ANTI-PATTERN: Heap-allocated polymorphic pipeline
let pipeline: Vec<Box<dyn Layer>> = vec![
    Box::new(DecompressLayer),
    Box::new(ValidationLayer),
];

// STITCH PATTERN: Monomorphic zero-cost static pipeline
let pipeline = Pipeline::new()
    .with(DecompressLayer)
    .with(ValidationLayer);
// Inlines directly into linear machine code without indirect jumps.
```

---

## 3. Lossless Correctness Contracts

A common pitfall in high-throughput systems is introducing lossy heuristics (such as truncating identifiers, dropping low-order bits, or coalescing updates) into default paths.

In Stitch:

- Default configurations are **strictly lossless and 100% correct**.
- Any lossy optimization (such as coordinate quantization or floating-point truncation) must be an explicit, contractually specified opt-in flag.
- Over-approximation boundaries must be provably bounded and covered by unit tests.

---

## 4. Speculative Complexity & YAGNI

Never introduce complex, unproven data structures (like Eytzinger BFS arrays, succinct Elias-Fano, or multi-level radix trees) based solely on synthetic microbenchmarks.

Real production workloads are dominated by cache pollution, kernel boundaries, and memory bandwidth. If 20 lines of idiomatic, cache-aligned Rust or linear SWAR scanning outperforms a complex theoretical structure, choose simplicity. Always measure end-to-end latency with statistical rigor.
