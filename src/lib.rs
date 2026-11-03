//! # Stitch (`stitch`)
//!
//! **The Unified Systems Architecture Ecosystem for High-Performance Rust.**
//!
//! `stitch` is the primary entry point and facade meta-crate for the Stitch ecosystem,
//! seamlessly integrating:
//! - **`stitch-core`**: The Sewing Machine Architecture (SMA) — zero-cost monomorphic U-cycle execution pipelines.
//! - **`stitch-data`**: High-density in-memory layout engine, zero-allocation transformations, and columnar codecs.
//! - **`stitch-wire`**: Microsecond transport framing bus, hardware-accelerated Castagnoli CRC32, and streaming resynchronization.
//!
//! ## Architectural Axioms
//! - **Zero Dynamic Allocations**: Hot paths execute strictly bounded in scratchpads and stack nodes without heap churn.
//! - **Mechanical Sympathy**: 64-byte L1D cache-line alignment prevents false sharing and cache-line straddling.
//! - **Strict `#![no_std]`**: All core computational and framing engines run on bare metal, embedded, and WASM.
//! - **Monomorphic Composition**: Static dispatch eliminates vtable indirect jumps, enabling aggressive compiler inlining.
//!
//! ## Unified Prelude
//!
//! ```rust
//! use stitch::prelude::*;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

// --- Sub-crates and Facades ---

#[cfg(feature = "framework")]
pub mod framework {
    //! The Sewing Machine Architecture (SMA) execution kernel, traits, and procedural macros.
    pub use stitch_core::*;
    pub use stitch_macros::*;

    #[cfg(feature = "async")]
    pub use stitch_async as r#async;
}

#[cfg(feature = "framework")]
pub use framework as core;

#[cfg(feature = "data")]
pub mod data {
    //! High-density data layout, zero-allocation transformation engine, and columnar codecs.
    pub use stitch_data::*;
}

#[cfg(feature = "wire")]
pub mod wire {
    //! Microsecond transport framing bus, Castagnoli CRC32, and streaming resync parser.
    pub use stitch_wire::*;
}

// --- Unified Prelude ---

/// Canonical prelude aggregating SMA execution primitives, data layout traits, and wire framing.
pub mod prelude {
    // 1. Framework & SMA Execution Primitives
    #[cfg(feature = "framework")]
    pub use crate::framework::{
        // Ports, Adapters, Providers
        Adapter,
        Blackboard,
        // CQS
        Command,
        CommandExecutor,
        CommandPermit,
        // Domain intent accounting
        Data,
        Entity,
        Event,
        // Flow Control
        FlowControl,
        // SMA Nodes & Pipelines
        Layer,
        // Outbox
        Outbox,
        Pipeline,
        PipelineChain,
        Port,
        PropRead,
        PropWrite,
        Provider,
        Query,
        QueryExecutor,
        // Identity & Tokens
        RawId,
        RawToken,
        StackNode,
        StatelessLayer,
        StatelessLayerTable,
        StitchId,
        StitchToken,
        Terminal,
        TerminalNode,
        ValueObject,
        // Memory & Cache Alignment
        assert_blackboard_aligned,
        blackboard,
        command,
        data,
        define_layer_enum,
        // Macros
        define_pipeline,
        entity,
        event,
        // Hash
        fnv1a_32,
        fnv1a_64,
        id,
        layer,
        pipeline,
        query,
        terminal,
        token,
        value_object,
        wire_pipeline,
    };

    // 2. Data & Layout Engine
    #[cfg(feature = "data")]
    pub use crate::data::{
        // Transforms & Codecs
        BitPacker,
        // Record traits & Codecs
        ColumnCodec,
        ColumnarBlockBuilder,
        CompressionPipeline,
        DecompressionPipeline,
        DeltaTransformer,
        FieldCodec,
        GorillaCompressor,
        Record,
        RecordBatchBuilder,
        RleTransformer,
        // Error
        StitchError,
        // BitStream & Bit IO
        bitstream::{BitReader, BitWriter},
        // Varint & ZigZag
        varint::{decode_prefix_varint, encode_prefix_varint},
        zigzag::{decode_zigzag32, decode_zigzag64, encode_zigzag32, encode_zigzag64},
    };

    // 3. Transport & Wire Bus
    #[cfg(feature = "wire")]
    pub use crate::wire::{
        FrameReader,
        FrameView,
        // Resync
        ResyncStats,
        StreamResyncParser,
        VectoredFrame,
        WIRE_MAGIC,
        WIRE_VERSION_1,
        // Wire Error
        WireError,
        WireFlags,
        // Framing
        WireHeader,
        checked_frame_total_len,
        // Hardware CRC32 Castagnoli
        crc::{
            crc32_castagnoli, crc32_castagnoli_finalize, crc32_castagnoli_init,
            crc32_castagnoli_slices, crc32_castagnoli_update,
        },
        find_magic,
        find_magic_swar,
        pack_frame,
        pack_frame_vectored,
        resync_frame,
        // SPSC Lock-free Ring Buffer
        ring::{CachePaddedAtomicUsize, SpscByteRing},
        unpack_frame,
    };

    #[cfg(all(feature = "wire", feature = "std"))]
    pub use crate::wire::udp::{UdpBatchReceiver, UdpBatchSender};

    // OptimizationProfile & Wire Adapters
    #[cfg(feature = "data")]
    pub use crate::data::OptimizationProfile;

    #[cfg(all(feature = "wire", feature = "data"))]
    pub use crate::wire::OptimizationProfile as WireOptimizationProfile;

    #[cfg(all(feature = "wire", not(feature = "data")))]
    pub use crate::wire::OptimizationProfile;

    #[cfg(all(feature = "wire", feature = "data"))]
    pub use crate::wire::data_adapter::{pack_record, unpack_record};

    // Macro derive re-export
    #[cfg(feature = "data")]
    pub use stitch_data::Record as DeriveRecord;
}
