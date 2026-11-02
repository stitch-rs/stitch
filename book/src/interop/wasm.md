# WebAssembly Linear Memory Zero-Copy Inspection

Modern telemetry dashboards, trading interfaces, and developer devtools run inside web browsers and WebAssembly runtimes.

`stitch-wire-adapter-wasm` compiles to `wasm32-unknown-unknown` under strict `#![no_std]`. It operates directly on the WebAssembly instance's linear memory without allocating intermediate JavaScript objects.

---

## 1. Linear Memory Architecture

When a browser worker receives binary WebSocket frames:

1. The raw `ArrayBuffer` is copied directly into the WebAssembly instance's linear memory.
2. The JavaScript frontend invokes exported functions:
   - `wasm_find_magic(offset, len)`: Scans for frame boundaries in WebAssembly memory.
   - `wasm_crc32(offset, len)`: Verifies hardware Castagnoli CRC32C.
   - `wasm_unpack_frame_view(offset, len, out_struct_offset)`: Decodes header fields and returns a pointer to the payload slice.

```javascript
// JavaScript Web Worker
const memory = wasmInstance.exports.memory.buffer;
const frameView = new DataView(memory, outOffset, 32);

const magicIndex = wasmInstance.exports.wasm_find_magic(rawBufOffset, rawLen);
const status = wasmInstance.exports.wasm_unpack_frame_view(magicIndex, rawLen - magicIndex, outOffset);

if (status === 0) {
    const payloadPtr = frameView.getUint32(24, true);
    const payloadLen = frameView.getUint16(2, true);
    // Zero-copy TypedArray view into decoded payload:
    const payload = new Uint8Array(memory, payloadPtr, payloadLen);
    renderChart(payload);
}
```

---

## 2. Benefits for Browser UIs

- **Zero JavaScript Garbage Collection**: Decoding 100,000 frames per second produces 0 GC pauses in V8 or SpiderMonkey.
- **Microsecond Rendering**: Telemetry charts and order book depth visuals update at 120 FPS without frame drops.
- **Portability**: The identical WebAssembly binary runs in Node.js, Cloudflare Workers, Fastly Compute@Edge, and browser web workers.
