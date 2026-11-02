# SIMD / SWAR Magic Byte Resynchronization

In streaming protocols (TCP streams or reassembled UDP packet buffers), network disruptions, byte-dropping proxies, or socket boundary splits can desynchronize the reader from frame boundaries.

`stitch-wire` implements high-speed **SWAR (SIMD Within A Register)** pattern matching to scan incoming byte streams and locate the 4-byte magic marker (`"STCH"`) in single-digit nanoseconds.

---

## 1. The SWAR Technique

Scanning byte-by-byte (`stream.windows(4)`) in a scalar loop requires 4 load operations and branch evaluations per byte.

SWAR processes 8 bytes (a 64-bit word) per CPU instruction:

```rust
#[inline]
pub fn find_magic_swar(buf: &[u8]) -> Option<usize> {
    const MAGIC: u32 = u32::from_le_bytes(*b"STCH");
    let mut i = 0;

    // Process 8-byte chunks using 64-bit register arithmetic
    while i + 8 <= buf.len() {
        let chunk = u64::from_le_bytes(buf[i..i + 8].try_into().unwrap());
        
        // Fast-path check: check low and high 32-bit halves
        if (chunk as u32) == MAGIC {
            return Some(i);
        }
        if ((chunk >> 32) as u32) == MAGIC {
            return Some(i + 4);
        }
        
        // Advance by alignment boundary
        i += 4;
    }

    // Scalar fallback for remaining tail bytes
    while i + 4 <= buf.len() {
        if &buf[i..i + 4] == b"STCH" {
            return Some(i);
        }
        i += 1;
    }

    None
}
```

---

## 2. Benefits in Production

- **Throughput**: Processes gigabytes of stream buffer per second without triggering L1 cache spills.
- **Resilience**: Even if 1,000 corrupted bytes arrive over an unstable connection, the parser recovers framing immediately upon reading the next intact frame header.
