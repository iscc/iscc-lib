# Next Work Package

## Step: Implement gen_meta_code_v0 meta object support

## Goal

Complete the 3 remaining conformance vectors for `gen_meta_code_v0` by handling JSON object and
Data-URL meta inputs. This finalizes the core crate at 100% conformance before starting bindings.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/Cargo.toml`, `crates/iscc-lib/src/simhash.rs`,
    `crates/iscc-lib/src/lib.rs`
- **Reference**: iscc-core `code_meta.py` via deepwiki, `crates/iscc-lib/tests/data.json`

## Implementation Notes

### 1. Add `serde_json` as regular dependency

Move `serde_json` from `[dev-dependencies]` to `[dependencies]` in `crates/iscc-lib/Cargo.toml`.
Keep `serde` as dev-dependency only (`serde_json::Value` parsing and serialization works without
`#[derive(Serialize)]`). `serde_json` is already in `[workspace.dependencies]` in root `Cargo.toml`.

### 2. Add `sliding_window_bytes` to `simhash.rs`

Add a bytes-oriented sliding window function:

```rust
pub(crate) fn sliding_window_bytes(data: &[u8], width: usize) -> Vec<&[u8]>
```

Same logic as the string `sliding_window` but operates on raw bytes, returning byte slices. Width
must be ≥ 2. If input is shorter than width, return a single slice of the full input. Add unit
tests.

### 3. Update meta hashing in `lib.rs`

The existing `soft_hash_meta_v0(name: &str, extra: Option<&str>)` handles text descriptions. Add a
new internal function for the bytes path:

```rust
fn soft_hash_meta_v0_with_bytes(name: &str, extra: &[u8]) -> Vec<u8>
```

- Name processing: same as existing (`text_collapse` → `sliding_window(name, 3)` → BLAKE3 → SimHash)
- Bytes extra processing: `sliding_window_bytes(extra, 4)` → BLAKE3 each n-gram → SimHash
- **Key difference from text path**: n-gram width is **4** (not 3), and NO `text_collapse` on bytes
- Interleave first 16 bytes of name SimHash and extra SimHash in 4-byte chunks (same interleaving
    logic as existing text path)
- Extract the interleaving logic into a shared helper to avoid duplication

### 4. Update `gen_meta_code_v0` to handle meta parameter

Remove the `NotImplemented` early return for `meta.is_some()`. When `meta` is `Some(meta_str)`:

**Data-URL path** (`meta_str` starts with `"data:"`):

- Parse the Data-URL: split on the first `,`, take everything after it
- Decode with `data_encoding::BASE64` → `payload: Vec<u8>`
- `data_encoding` is already a dependency (used for base32 in codec.rs)

**JSON path** (`meta_str` does NOT start with `"data:"`):

- Parse with `serde_json::from_str::<serde_json::Value>(meta_str)`
- Re-serialize with `serde_json::to_vec(&parsed)` → `payload: Vec<u8>`
- Without `preserve_order` feature, `serde_json::Map` uses `BTreeMap` (sorted keys) — this produces
    JCS-compatible output for ASCII keys
- Add a code comment noting this is sufficient for ASCII keys but not full RFC 8785 compliance

**Both paths** then:

- Call `soft_hash_meta_v0_with_bytes(&name, &payload)` for the similarity digest
- Call `multi_hash_blake3(&payload)` for the metahash
- Continue with existing `encode_component` + return logic

**When meta is None**: existing behavior unchanged (use description text path via
`soft_hash_meta_v0`)

### 5. Update conformance tests

Remove the skip logic for the 3 meta object test cases. Handle `meta_val` type dispatch:

- `serde_json::Value::Null` → pass `None` (existing path)
- `serde_json::Value::String(s)` → pass `Some(s.as_str())` (Data-URL case)
- `serde_json::Value::Object(_)` → serialize to string with `serde_json::to_string(&meta_val)`, pass
    `Some(&json_string)` (JSON object case)

Update assertion from `tested == 13` to `tested == 16`. Remove the `skipped` counter and skip logic
entirely.

### Edge cases

- Empty bytes payload after Data-URL decode: treat same as `meta: None` (use description path)
- Invalid base64 in Data-URL: return `InvalidInput` error
- Invalid JSON in meta string: return `InvalidInput` error

## Verification

- `cargo test -p iscc-lib` passes with **all 16** `gen_meta_code_v0` conformance vectors (0 skipped)
- Total test count ≥ 134 (may increase with new unit tests for `sliding_window_bytes`)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code

## Done When

All 16 `gen_meta_code_v0` conformance vectors pass and the full test suite is green with clippy
clean.
