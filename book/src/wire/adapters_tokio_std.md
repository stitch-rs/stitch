# Streaming Adapters: Tokio & Std

To make `stitch-wire` seamlessly usable across diverse Rust runtimes, the ecosystem provides standalone adapter crates following the user-standard architecture established by `ipatlas` and `statefs-rs`.

---

## 1. Asynchronous Streaming: `stitch-wire-adapter-tokio`

For asynchronous microservices and TCP servers, `stitch-wire-adapter-tokio` provides `StitchWireCodec`, implementing `tokio_util::codec::Decoder` and `Encoder`:

```rust
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use stitch_wire_adapter_tokio::StitchWireCodec;
use futures::{StreamExt, SinkExt};

let stream = TcpStream::connect("127.0.0.1:8080").await?;
let mut framed = Framed::new(stream, StitchWireCodec::new());

// Receiving parsed, CRC-verified frames:
while let Some(frame_result) = framed.next().await {
    let frame = frame_result?;
    println!("Received frame seq={}: payload len={}", frame.seq, frame.payload.len());
}
```

### Features of the Tokio Adapter

- Automatically handles partial TCP reads and fragmented packets.
- Employs SWAR scanning to resynchronize if bytes were corrupted or dropped.
- Performs Castagnoli CRC32 validation before emitting `Frame` items.

---

## 2. Synchronous Blocking I/O: `stitch-wire-adapter-std`

For embedded systems, offline disk loggers, or blocking socket servers, `stitch-wire-adapter-std` provides `SyncFrameReader` and `SyncFrameWriter`:

```rust
use std::fs::File;
use stitch_wire_adapter_std::{SyncFrameReader, SyncFrameWriter};

// Writing frames directly to disk:
let mut writer = SyncFrameWriter::new(File::create("market_feed.bin")?);
writer.write_frame(flags, seq, schema_hash, &payload_bytes)?;

// Reading frames back sequentially:
let mut reader = SyncFrameReader::new(File::open("market_feed.bin")?);
while let Some(frame) = reader.read_next_frame()? {
    process_frame(&frame);
}
```

Both adapters wrap any standard `Read`/`Write` or `AsyncRead`/`AsyncWrite` stream without imposing heap buffers on inner algorithms.
