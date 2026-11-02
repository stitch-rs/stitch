# Run-Length Encoding (RLE) & SWAR Runs

Many real-world streams contain long sequences of identical repeated values:

- Status flags and bitmasks (e.g. `[1, 1, 1, 1, 1, ...]`).
- Visibility states in multiplayer game worlds.
- Unfilled order quantity thresholds in market depth ladders.

`stitch-data-transforms` implements **Run-Length Encoding (RLE)** optimized with SWAR (SIMD Within A Register) for instantaneous run counting.

---

## 1. Encoding Format

Each continuous sequence of identical elements is represented as a `(count, value)` pair:

```text
Input:  [42, 42, 42, 42, 42, 10, 10, 99]
Output: (5, 42), (2, 10), (1, 99)
```

The count is encoded using variable-length integer encoding (Varint) or fixed unsigned integers (`u16`/`u32`), ensuring minimal wire overhead for short runs while scaling smoothly to millions of identical items.

---

## 2. SWAR-Accelerated Run Detection

Instead of comparing array elements one by one, the encoder compares 64-bit blocks:

```rust
// Compare 8 bytes at a time
let word_a = u64::from_ne_bytes(slice[i..i+8].try_into().unwrap());
let repeated_word = value as u64 * 0x0101010101010101;

if word_a == repeated_word {
    // 8 identical elements confirmed in 1 CPU cycle
    count += 8;
    i += 8;
}
```

This approach allows `RleEncoder` to scan multi-megabyte buffers at memory bus speeds (>10 GB/s), delivering instantaneous compression on dense repetitive telemetry.
