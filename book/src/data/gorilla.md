# Gorilla Delta-of-Delta Float Compression

Floating-point numbers (`f32`, `f64`) are notoriously difficult to compress with standard entropy or dictionary compressors because the IEEE 754 representation distributes sign, exponent, and mantissa across specific bit ranges. Small mathematical changes flip lower mantissa bits unpredictably.

`stitch-data-transforms` provides an optimized implementation of the **Gorilla compression algorithm** (originally designed by Facebook for time-series telemetry).

---

## 1. Mathematical Mechanics

Gorilla compresses a stream of floating-point values by computing the bitwise XOR between successive values:

$$\Delta_i = v_i \oplus v_{i-1}$$

1. **Identical Values**: If $v_i = v_{i-1}$, $\Delta_i = 0$. This is stored as a single `0` bit.
2. **Similar Values**: If $v_i$ is close to $v_{i-1}$, their sign and exponent bits are identical, causing $\Delta_i$ to have leading zeros and trailing zeros.
3. **Variable-Length Bitpacking**:
   - If the leading and trailing zero count matches the previous XOR delta, write a `10` control bit followed by the meaningful bits.
   - If the zero counts differ, write a `11` control bit, the 5-bit leading zero count, the 6-bit meaningful length, and the meaningful bits.

---

## 2. Stitch Architecture Optimizations

The `stitch-data-transforms` implementation provides:

- **Zero Heap Allocations**: Encodes directly into a caller-supplied byte slice using a zero-copy bitwriter (`BitWriter`).
- **`f32` and `f64` Support**: Dedicated fast paths for 32-bit single-precision (used in game entity vectors) and 64-bit double-precision (used in financial asset prices).
- **Benchmark Performance**: Compresses over 40 million floating-point numbers per second per core, achieving 60–80% size reduction on typical sensor and telemetry streams.
