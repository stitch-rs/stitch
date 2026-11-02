//! Production Domain Reference Example: Real-time Multiplayer Game Engine Snapshot Synchronizer.
//!
//! Demonstrates:
//! 1. ECS Entity Position & Kinematics delta encoding using Gorilla XOR float compression.
//! 2. RLE compression for player animation status masks and weapon IDs.
//! 3. Wire transport framing with hardware Castagnoli CRC32.
//! 4. High-loss UDP transport packet simulation with streaming resynchronization recovery.

use stitch::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Record)]
pub struct EntityKinematics {
    #[stitch(id = 1)]
    pub entity_id: u32,
    #[stitch(id = 2, column = ColumnCodec::Gorilla)]
    pub pos_x: f32,
    #[stitch(id = 3, column = ColumnCodec::Gorilla)]
    pub pos_y: f32,
    #[stitch(id = 4, column = ColumnCodec::Gorilla)]
    pub pos_z: f32,
    #[stitch(id = 5, column = ColumnCodec::Gorilla)]
    pub yaw: f32,
    #[stitch(id = 6, column = ColumnCodec::Rle)]
    pub health: u8,
    #[stitch(id = 7, column = ColumnCodec::Rle)]
    pub active_weapon: u8,
    #[stitch(id = 8, column = ColumnCodec::Rle)]
    pub state_flags: u16, // InAir, Crouching, Shooting, Dead
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===============================================================================");
    println!("    STITCH MULTIPLAYER GAME ENGINE ECS WORLD SNAPSHOT SYNCHRONIZER             ");
    println!("===============================================================================\n");

    const PLAYERS: usize = 64;
    const TICKS: usize = 128; // Simulation over 128 server ticks (~1 second at 128Hz)

    let mut world_state: Vec<EntityKinematics> = (0..PLAYERS)
        .map(|id| EntityKinematics {
            entity_id: id as u32,
            pos_x: (id as f32) * 2.5,
            pos_y: 64.0,
            pos_z: -(id as f32) * 5.0,
            yaw: 0.0,
            health: 100,
            active_weapon: (id % 5) as u8,
            state_flags: 0x0001, // OnGround
        })
        .collect();

    println!(
        ">>> Simulating 128Hz multiplayer world state broadcast ({} entities)...",
        PLAYERS
    );

    let raw_snapshot_size = PLAYERS * core::mem::size_of::<EntityKinematics>();
    let mut total_raw_bytes = 0;
    let mut total_compressed_wire_bytes = 0;

    let mut scratch = vec![0u8; 32 * 1024];
    let mut wire_buf = vec![0u8; 32 * 1024];

    for tick in 0..TICKS {
        // Physics update step
        for e in &mut world_state {
            e.pos_x += 0.05;
            e.pos_z += 0.02;
            e.yaw = ((e.yaw + 1.0) as u32 % 360) as f32;
        }

        total_raw_bytes += raw_snapshot_size;

        // 1. Columnar Compression
        let payload_len = EntityKinematics::encode_batch(&world_state, &mut scratch)
            .expect("Columnar encode batch failed");

        // 2. Wire transport framing
        let header = WireHeader {
            profile: WireOptimizationProfile::Realtime,
            flags: WireFlags {
                is_compressed: true,
                is_chunked: false,
                has_schema_hash: true,
            },
            payload_len,
            schema_hash: Some(EntityKinematics::SCHEMA_HASH),
        };

        let frame_len =
            pack_frame(&header, &scratch[..payload_len], &mut wire_buf).expect("Framing failed");
        total_compressed_wire_bytes += frame_len;

        // Verify receiver decoding every 32 ticks
        if tick % 32 == 0 {
            let (unpacked_hdr, unpacked_payload) =
                unpack_frame(&wire_buf[..frame_len]).expect("Unpack frame");
            assert_eq!(
                unpacked_hdr.schema_hash,
                Some(EntityKinematics::SCHEMA_HASH)
            );

            let mut client_entities = vec![
                EntityKinematics {
                    entity_id: 0,
                    pos_x: 0.0,
                    pos_y: 0.0,
                    pos_z: 0.0,
                    yaw: 0.0,
                    health: 0,
                    active_weapon: 0,
                    state_flags: 0,
                };
                PLAYERS
            ];

            let count = EntityKinematics::decode_batch(unpacked_payload, &mut client_entities)
                .expect("Decode client snapshot");
            assert_eq!(count, PLAYERS);
            assert_eq!(client_entities[0].entity_id, world_state[0].entity_id);
        }
    }

    println!("\n>>> 128Hz Simulation Results:");
    println!("  - Total Raw AoS Volume:        {} bytes", total_raw_bytes);
    println!(
        "  - Total Wire Frame Volume:     {} bytes",
        total_compressed_wire_bytes
    );
    println!(
        "  - Bandwidth Compression Ratio: {:.2}x ({:.1}% of raw bandwidth)",
        total_raw_bytes as f64 / total_compressed_wire_bytes as f64,
        (total_compressed_wire_bytes as f64 / total_raw_bytes as f64) * 100.0
    );

    println!("\n>>> UDP Corrupted Stream Resynchronization Test (SWAR Parser)...");
    let mut degraded_stream = Vec::new();
    // Prepend 100 bytes of garbage noise
    degraded_stream.extend_from_slice(&[0xEE; 100]);
    // Append valid snapshot frame
    degraded_stream.extend_from_slice(&wire_buf[..total_compressed_wire_bytes / TICKS]);
    // Trailing garbage noise
    degraded_stream.extend_from_slice(&[0xAA; 50]);

    let mut resync_parser = StreamResyncParser::new();
    let mut recovered = 0;
    let mut offset = 0;

    while offset < degraded_stream.len() {
        if let Some((consumed, frame)) = resync_parser.parse_next(&degraded_stream[offset..]) {
            assert_eq!(
                frame.header.schema_hash,
                Some(EntityKinematics::SCHEMA_HASH)
            );
            recovered += 1;
            offset += consumed;
        } else {
            break;
        }
    }

    println!("  - Packets Recovered from Noise: {}", recovered);
    println!(
        "  - Bytes Discarded:             {} bytes",
        resync_parser.stats().bytes_discarded
    );
    println!(
        "  - Resync Events:               {}",
        resync_parser.stats().resync_events
    );

    println!("\n[SUCCESS] Game engine world snapshot synchronizer simulation completed cleanly!");
    Ok(())
}
