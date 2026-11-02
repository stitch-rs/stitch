# Schema Evolution & Compile-Time SCHEMA_HASH

Distributed systems evolve over time. Microservices are deployed independently, requiring protocol messages to remain backward and forward compatible.

`stitch-data-core` enforces a rigorous protocol governance model based on **Field Descriptors**, the **`SchemaCompatibilityVerifier`**, and **Compile-Time `SCHEMA_HASH` Fingerprints**.

---

## 1. Field Descriptors & Schema Definition

In `stitch-data-core::schema`, every message type exposes its schema contract:

```rust
pub struct FieldDescriptor {
    pub id: u16,
    pub name: &'static str,
    pub bit_width: u16,
    pub is_nullable: bool,
}

pub struct SchemaDefinition {
    pub schema_name: &'static str,
    pub fields: &'static [FieldDescriptor],
}
```

---

## 2. The Compatibility Contract

The `SchemaCompatibilityVerifier` evaluates evolution rules between a writer's schema and a reader's schema:

- **Subset Compatibility (Forward Evolution)**: A reader can safely parse data produced by a newer writer if all fields expected by the reader exist with compatible bit-widths. Extra fields in the writer's schema are ignored.
- **Superset Compatibility (Backward Evolution)**: A writer with newer optional fields can produce messages readable by older consumers without binary deserialization crashes.
- **Type Safety**: Any alteration to a field's underlying bit width (e.g., changing `u16` to `u32` without an explicit field migration) produces a typed `CompatibilityError::IncompatibleBitWidth`.

---

## 3. The Compile-Time `SCHEMA_HASH`

To prevent mismatched schemas from being processed on hot paths, `stitch-wire` embeds an 8-byte `schema_hash` into every frame header:

```rust
// Calculated at compile time from field IDs, types, and sequence
pub const SCHEMA_HASH: u64 = 0xA4F2_8B19_3C00_E91D;
```

When a frame is received:

1. The reader compares `frame.schema_hash == Expected::SCHEMA_HASH`.
2. If the hashes match, zero dynamic schema verification is needed — the payload is unpacked in single-digit nanoseconds.
3. If the hashes differ, execution branches into the fallback evolution decoder or yields a structured schema error before executing corrupt byte interpretations.
