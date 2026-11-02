# Dictionary Compression for Low Cardinality

In many domains, high-volume datasets contain values drawn from a very small set of distinct options (low cardinality):

- Currency pairs and stock ticker symbols (`"BTC-USDT"`, `"ETH-USDT"`).
- Entity archetypes and team IDs in games (`TEAM_RED`, `TEAM_BLUE`).
- Geographic region codes and HTTP status codes.

`stitch-data-transforms` provides **Dictionary Compression (`DictEncoder` / `DictDecoder`)** to replace repeated strings or integers with compact integer indices.

---

## 1. Dictionary Mechanism

The dictionary operates in two phases:

1. **Dictionary Header**: A compact table listing all unique symbols or values observed in the batch:
   `["AAPL", "GOOGL", "MSFT"]` -> Indexes `[0, 1, 2]`
2. **Payload Indices**: The original sequence is replaced by indices:
   `["AAPL", "AAPL", "MSFT", "GOOGL"]` -> `[0, 0, 2, 1]`

When the cardinality is under 256 unique items, indices are stored as single bytes (`u8`), reducing a 16-character string symbol to a single byte on the wire (a **16x compression factor**).

---

## 2. In-Memory Hashless Lookup

For hot paths, `DictEncoder` uses a fixed-size, stack-allocated direct-mapped table or linear cache to resolve index mappings without calling heap allocators (`std::collections::HashMap`).

Combined with `stitch-data-derive`, dictionary compression can be applied declaratively:

```rust
#[derive(ColumnarRecord)]
pub struct Trade {
    pub timestamp: u64,
    #[column(codec = "Dict")]
    pub symbol: &'static str,
    #[column(codec = "Gorilla")]
    pub price: f64,
}
```
