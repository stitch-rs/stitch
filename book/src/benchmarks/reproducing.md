# Reproducing the Ecosystem Scoreboard

To ensure full reproducibility, the `stitch-facade` repository includes the automated `stitch-scoreboard` CLI tool and Criterion benchmark suite in `benches-ecosystem`.

---

## 1. Running the Automated Scoreboard

Clone the repository and run the scoreboard CLI directly via Cargo:

```bash
cd stitch-facade
cargo run --release -p stitch-benchmarks --bin stitch-scoreboard
```

The tool executes:

1. **Wire Framing Test**: 1,000,000 frame packs and unpacks measuring throughput and Castagnoli CRC32 validation.
2. **Columnar Codecs Test**: Gorilla, Dict, RLE, and Bitpacking compression ratios and decode timings.
3. **End-to-End Pipeline**: Full SMA U-cycle execution passing through order validation and outbound network framing.

---

## 2. Running Criterion Microbenchmarks

For statistical analysis with confidence intervals, p-values, and HTML reports:

```bash
cd stitch-facade
cargo bench -p stitch-benchmarks
```

Criterion outputs comprehensive HTML reports and SVG performance graphs to:
`stitch-facade/target/criterion/report/index.html`

### Verifying Zero Allocations with Valgrind / DHAT

To verify that the hot path incurs exactly zero heap allocations:

```bash
cargo build --release -p stitch-benchmarks --example hft_orderbook_gateway
valgrind --tool=dhat ./target/release/examples/hft_orderbook_gateway
```

Expected result:

```text
== Total bytes allocated at exit: 0 bytes in 0 blocks (Hot Path)
```
