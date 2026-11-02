# Bitpacking & Dense Variable-Width Primitives

Standard hardware registers store integers in powers of two: 8, 16, 32, or 64 bits. However, domain values frequently occupy only a fraction of those bits:

- Percentage values (0–100) require only 7 bits, yet consume 8 or 16 bits in standard memory structs.
- Small delta timestamps (0–15 ms) require only 4 bits.
- Integer values up to 1,000 require only 10 bits instead of 32 bits.

`stitch-data-transforms` provides **Bitpacking (`BitpackEncoder` / `BitpackDecoder`)** to pack arbitrary bit-width integers into continuous byte streams without wasting padding bits.

---

## 1. Bitpacking Mechanics

Given a sequence of $N$ integers where every element fits in $K$ bits:

```text
Values (K=5 bits): [7, 12, 31, 2]
Binary: 00111, 01100, 11111, 00010

Packed stream:
Byte 0: 0 0 1 1 1 0 1 1  (7 + top 3 bits of 12)
Byte 1: 0 0 1 1 1 1 1 0  (bottom 2 bits of 12 + 31 + top 1 bit of 2)
Byte 2: 0 0 1 0 0 0 0 0  (remaining bits of 2 + zero padding)
```

Rather than using 16 bytes (four 32-bit integers), the payload consumes only **3 bytes** — a **5.3x reduction** in network bandwidth.

---

## 2. SIMD Acceleration

`stitch-data-transforms` leverages SIMD bit-shifting instructions to pack and unpack blocks of 32 values simultaneously.

Using AVX2/NEON instructions, unpacking runs at line rate (>15 GB/s), allowing decoders to reconstitute full integer arrays into CPU registers faster than standard DRAM can supply uncompressed data.
