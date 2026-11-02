# C-ABI Library & Panic-Safe FFI Boundaries

While the core Stitch ecosystem is authored in pure Rust, high-throughput systems frequently integrate with external ecosystems: C, C++, Python, Go, and legacy game engines.

`stitch-wire-adapter-c` exports a pure C-ABI dynamic and static library (`.dll`, `.so`, `.dylib`, `.a`) that allows any foreign language to produce and consume Stitch wire frames.

---

## 1. Safety & The Panic Boundary

Rust panics must never cross the foreign function interface (FFI) boundary. A panic unwinding into C or C++ stack frames results in undefined behavior and crashes the host process.

`stitch-wire-adapter-c` wraps every exported symbol with `std::panic::catch_unwind`:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn stitch_unpack_frame(
    buf: *const u8,
    len: usize,
    out_frame: *mut StitchFrameView,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if buf.is_null() || out_frame.is_null() {
            return -1; // Invalid pointer
        }
        
        let slice = unsafe { std::slice::from_raw_parts(buf, len) };
        match unpack_frame(slice) {
            Ok(frame) => {
                unsafe {
                    (*out_frame).version = frame.version;
                    (*out_frame).flags = frame.flags;
                    (*out_frame).payload_len = frame.payload.len() as u16;
                    (*out_frame).payload_ptr = frame.payload.as_ptr();
                }
                0 // Success
            }
            Err(_) => -2, // Malformed frame or CRC mismatch
        }
    });

    result.unwrap_or(-99) // Catch-unwind panic barrier
}
```

---

## 2. The C Header (`include/stitch_wire.h`)

The crate includes an ANSI C-compatible header file:

```c
#ifndef STITCH_WIRE_H
#define STITCH_WIRE_H

#include <stdint.h>
#include <stddef.h>

typedef struct {
    uint8_t version;
    uint8_t flags;
    uint16_t payload_len;
    uint64_t seq;
    uint64_t schema_hash;
    uint32_t crc32;
    const uint8_t* payload_ptr;
} StitchFrameView;

int32_t stitch_pack_frame(
    uint8_t flags,
    uint64_t seq,
    uint64_t schema_hash,
    const uint8_t* payload,
    uint16_t payload_len,
    uint8_t* out_buf,
    size_t out_capacity,
    size_t* out_written
);

int32_t stitch_unpack_frame(
    const uint8_t* buf,
    size_t len,
    StitchFrameView* out_frame
);

uint32_t stitch_crc32(const uint8_t* buf, size_t len);
int64_t stitch_find_magic(const uint8_t* buf, size_t len);

#endif // STITCH_WIRE_H
```

Using this header, C and C++ developers achieve zero-copy framing, hardware CRC32 calculations, and SWAR magic resynchronization with zero runtime dependencies.
