# Summary

[Introduction](introduction.md)

- [The "Scrooge" Systems Mindset](scrooge_mindset.md)

# Part I: The Sewing Machine Architecture (SMA)

- [Foundations & The U-Cycle](sma/foundations.md)
- [Monomorphic Pipelines vs Dynamic Dispatch](sma/monomorphism.md)
- [Zero-Allocation Flow Control & Blackboards](sma/flow_control.md)
- [Cache-Line Engineering & 64-Byte Alignment](sma/cache_alignment.md)

# Part II: Network Wire Framing (stitch-wire)

- [Container Specification & Layout Invariants](wire/invariants.md)
- [SIMD / SWAR Magic Byte Resynchronization](wire/swar_magic.md)
- [Hardware-Accelerated Castagnoli CRC-32C](wire/crc32c.md)
- [Vectored Scatter-Gather I/O](wire/vectored_io.md)
- [Streaming Adapters: Tokio & Std](wire/adapters_tokio_std.md)

# Part III: Columnar Data & Compression (stitch-data)

- [Row-Oriented vs Columnar Data Layouts](data/columnar_overview.md)
- [Gorilla Delta-of-Delta Float Compression](data/gorilla.md)
- [Run-Length Encoding (RLE) & SWAR Runs](data/rle.md)
- [Dictionary Compression for Low Cardinality](data/dictionary.md)
- [Bitpacking & Dense Variable-Width Primitives](data/bitpacking.md)
- [Schema Evolution & Compile-Time SCHEMA_HASH](data/schema_evolution.md)

# Part IV: Cross-Language Interop & Runtimes

- [C-ABI Library & Panic-Safe FFI](interop/c_abi.md)
- [WebAssembly Linear Memory Zero-Copy Inspection](interop/wasm.md)

# Part V: Production Domain Exemplars

- [HFT L2 OrderBook Matching Gateway](exemplars/hft_orderbook.md)
- [Distributed Game Engine State Synchronizer](exemplars/game_delta_sync.md)

# Part VI: Benchmarks & Empirical Proof

- [Comparative Evaluation: Stitch vs Protobuf vs Bincode vs Postcard](benchmarks/comparative.md)
- [Reproducing the Ecosystem Scoreboard](benchmarks/reproducing.md)
