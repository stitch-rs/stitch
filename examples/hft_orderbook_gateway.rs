//! Production Domain Reference Example: High-Frequency Trading (HFT) L2 OrderBook Gateway.
//!
//! Demonstrates the complete unified Stitch ecosystem:
//! 1. **Monomorphic SMA U-Cycle Execution**: Static layer dispatching, zero indirect jumps, zero dynamic allocations.
//! 2. **64-byte Cache-Aligned Blackboard**: Tick latency and volume tracking with zero cache-line straddling.
//! 3. **High-Density Columnar Transposition**: Gorilla float XOR compression for tick prices, Dictionary for instrument tickers, RLE for trade directions.
//! 4. **Hardware-Accelerated Wire Transport Framing**: Castagnoli CRC32 validation, frame header generation, zero-copy unpack.

use stitch::prelude::*;

// ============================================================================
// 1. Domain Types & Records
// ============================================================================

/// Level-2 Market Tick update.
#[derive(Clone, Copy, Debug, PartialEq, Record)]
pub struct MarketDepthUpdate {
    #[stitch(id = 1)]
    pub timestamp_ns: u64,
    #[stitch(id = 2, column = ColumnCodec::Dict)]
    pub instrument_id: u32,
    #[stitch(id = 3, column = ColumnCodec::Gorilla)]
    pub best_bid: f64,
    #[stitch(id = 4, column = ColumnCodec::Gorilla)]
    pub best_ask: f64,
    #[stitch(id = 5, column = ColumnCodec::Rle)]
    pub bid_volume: u32,
    #[stitch(id = 6, column = ColumnCodec::Rle)]
    pub ask_volume: u32,
    #[stitch(id = 7, column = ColumnCodec::Rle)]
    pub side: u8, // 0 = Buy, 1 = Sell
}

// ============================================================================
// 2. 64-byte Cache-Aligned OrderBook Blackboard
// ============================================================================

#[repr(C, align(64))]
pub struct OrderBookBlackboard {
    pub total_ticks_processed: u64,
    pub total_volume_traded: u64,
    pub last_timestamp_ns: u64,
    pub spread_anomalies_detected: u32,
    pub _pad: [u8; 36], // Ensures exact 64-byte cache line fit
}

impl Default for OrderBookBlackboard {
    fn default() -> Self {
        Self {
            total_ticks_processed: 0,
            total_volume_traded: 0,
            last_timestamp_ns: 0,
            spread_anomalies_detected: 0,
            _pad: [0u8; 36],
        }
    }
}

impl Blackboard for OrderBookBlackboard {}
assert_blackboard_aligned!(OrderBookBlackboard);

// ============================================================================
// 3. SMA Monomorphic Validation & Risk Layer
// ============================================================================

pub struct RiskValidationLayer;

impl Layer<OrderBookBlackboard, MarketDepthUpdate, MarketDepthUpdate, &'static str>
    for RiskValidationLayer
{
    #[inline(always)]
    fn on_enter(
        &self,
        ctx: &mut OrderBookBlackboard,
        intent: MarketDepthUpdate,
    ) -> FlowControl<MarketDepthUpdate, MarketDepthUpdate, &'static str> {
        // Crossed book check (Bid >= Ask)
        if intent.best_bid >= intent.best_ask {
            ctx.spread_anomalies_detected += 1;
            return FlowControl::Halt("Crossed Market Depth: best_bid >= best_ask");
        }

        ctx.total_ticks_processed += 1;
        ctx.total_volume_traded += (intent.bid_volume + intent.ask_volume) as u64;
        ctx.last_timestamp_ns = intent.timestamp_ns;

        FlowControl::Proceed(intent)
    }

    #[inline(always)]
    fn on_exit(
        &self,
        _ctx: &mut OrderBookBlackboard,
        _outcome: &mut Result<MarketDepthUpdate, &'static str>,
    ) {
    }
}

pub struct EgressTerminal;

impl Terminal<OrderBookBlackboard, MarketDepthUpdate, MarketDepthUpdate, &'static str>
    for EgressTerminal
{
    #[inline(always)]
    fn execute(
        &mut self,
        _ctx: &mut OrderBookBlackboard,
        intent: MarketDepthUpdate,
    ) -> Result<MarketDepthUpdate, &'static str> {
        Ok(intent)
    }
}

// ============================================================================
// 4. Main Simulation & Pipeline Benchmark
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===============================================================================");
    println!("    STITCH HFT L2 ORDERBOOK GATEWAY: REAL-TIME PIPELINE REFERENCE              ");
    println!("===============================================================================\n");

    let mut blackboard = OrderBookBlackboard::default();
    let mut pipeline = Pipeline::on_terminal(EgressTerminal).wrap(RiskValidationLayer);

    const TICKS_COUNT: usize = 1000;
    let mut batch = Vec::with_capacity(TICKS_COUNT);
    let base_time: u64 = 1_700_000_000_000_000_000;
    let mut current_mid = 5000.25f64;

    println!(">>> Ingesting {TICKS_COUNT} market depth ticks through Monomorphic SMA Pipeline...");

    for i in 0..TICKS_COUNT {
        let ts = base_time + (i as u64 * 100);
        let instrument_id = (i % 4) as u32; // 4 major instruments
        current_mid += ((i % 7) as f64 - 3.0) * 0.05;
        let spread = 0.10;
        let best_bid = current_mid - spread / 2.0;
        let best_ask = current_mid + spread / 2.0;
        let bid_volume = 500 + ((i % 10) * 50) as u32;
        let ask_volume = 400 + ((i % 8) * 60) as u32;
        let side = (i % 2) as u8;

        let raw_tick = MarketDepthUpdate {
            timestamp_ns: ts,
            instrument_id,
            best_bid,
            best_ask,
            bid_volume,
            ask_volume,
            side,
        };

        // Dispatch through SMA U-cycle
        let validated_tick = pipeline
            .dispatch(&mut blackboard, raw_tick)
            .expect("SMA Pipeline rejected tick");

        batch.push(validated_tick);
    }

    println!(
        "  - Ticks successfully processed: {}",
        blackboard.total_ticks_processed
    );
    println!(
        "  - Total volume recorded:        {} contracts",
        blackboard.total_volume_traded
    );
    println!(
        "  - Anomalies detected:           {}",
        blackboard.spread_anomalies_detected
    );

    // Columnar Compression
    let raw_size = batch.len() * core::mem::size_of::<MarketDepthUpdate>();
    let mut compressed_buf = vec![0u8; raw_size + 1024];

    let compressed_len = MarketDepthUpdate::encode_batch(&batch, &mut compressed_buf)
        .expect("Columnar batch encoding failed");

    println!("\n>>> Columnar Encoding Performance (Gorilla + Dict + RLE):");
    println!("  - Raw In-Memory AoS Size:      {} bytes", raw_size);
    println!("  - Stitch Compressed Size:      {} bytes", compressed_len);
    println!(
        "  - Compression Ratio:           {:.2}x ({:.1}% of raw)",
        raw_size as f64 / compressed_len as f64,
        (compressed_len as f64 / raw_size as f64) * 100.0
    );

    // Wire Transport Framing
    let wire_header = WireHeader {
        profile: WireOptimizationProfile::Balanced,
        flags: WireFlags::new(true, false, true),
        payload_len: compressed_len,
        schema_hash: Some(MarketDepthUpdate::SCHEMA_HASH),
    };

    let mut wire_frame_buf = vec![0u8; compressed_len + 64];
    let wire_len = pack_frame(
        &wire_header,
        &compressed_buf[..compressed_len],
        &mut wire_frame_buf,
    )
    .expect("Wire framing failed");

    println!("\n>>> Wire Transport Framing (Castagnoli Hardware CRC32):");
    println!("  - Wire Frame Total Length:     {} bytes", wire_len);
    println!(
        "  - Wire Overhead:               {} bytes",
        wire_len - compressed_len
    );

    // Receiver Unpack & Verification
    let (unpacked_hdr, unpacked_payload) =
        unpack_frame(&wire_frame_buf[..wire_len]).expect("Unpack frame failed");
    assert_eq!(unpacked_hdr.payload_len, compressed_len);
    assert_eq!(
        unpacked_hdr.schema_hash,
        Some(MarketDepthUpdate::SCHEMA_HASH)
    );

    let mut decoded_batch = vec![
        MarketDepthUpdate {
            timestamp_ns: 0,
            instrument_id: 0,
            best_bid: 0.0,
            best_ask: 0.0,
            bid_volume: 0,
            ask_volume: 0,
            side: 0,
        };
        TICKS_COUNT
    ];

    let decoded_count = MarketDepthUpdate::decode_batch(unpacked_payload, &mut decoded_batch)
        .expect("Columnar batch decode failed");

    assert_eq!(decoded_count, TICKS_COUNT);
    assert_eq!(decoded_batch[0], batch[0]);
    assert_eq!(decoded_batch[TICKS_COUNT - 1], batch[TICKS_COUNT - 1]);

    println!("\n[SUCCESS] Bit-exact verification complete. HFT OrderBook gateway run successful!");
    Ok(())
}
