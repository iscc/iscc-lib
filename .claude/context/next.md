# Next Work Package

## Step: Implement gen_instance_code_v0 with conformance tests

## Goal

Implement the first real `gen_*_v0` function — `gen_instance_code_v0` — and vendor the official
conformance test vectors to verify correctness against `iscc-core`. This unblocks all subsequent
function implementations by establishing the conformance testing pattern.

## Scope

- **Create**: `crates/iscc-lib/tests/data.json` (vendored from iscc-core)
- **Modify**: `Cargo.toml` (root — add `blake3` to workspace.dependencies, add `serde`/`serde_json`
    as workspace dev-dependencies), `crates/iscc-lib/Cargo.toml` (add `blake3` dep + dev-deps for
    test parsing), `crates/iscc-lib/src/lib.rs` (implement `gen_instance_code_v0`)
- **Reference**: `crates/iscc-lib/src/codec.rs` (for `encode_component`, `MainType`, `SubType`,
    `Version`), `iscc-core` Python reference via deepwiki

## Implementation Notes

### Algorithm (from iscc-core `code_instance.py`)

`gen_instance_code_v0(data, bits)` does:

1. Hash the entire input `data` with BLAKE3 → 32-byte digest
2. Call `encode_component(MainType::Instance, SubType::None, Version::V0, bits, &digest)`
3. Prefix the result with `"ISCC:"` and return

That's it — 3-5 lines of real logic. The function currently takes `&[u8]` (not a stream) which is
fine for the Rust API; streaming can be added later via a separate `InstanceHasher` struct.

### Dependency setup

Add to root `Cargo.toml` `[workspace.dependencies]`:

```toml
blake3 = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Add to `crates/iscc-lib/Cargo.toml`:

```toml
[dependencies]
blake3.workspace = true

[dev-dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
```

### Function implementation

Replace the stub in `src/lib.rs`:

```rust
pub fn gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<String> {
    let digest = blake3::hash(data);
    let component = codec::encode_component(
        codec::MainType::Instance,
        codec::SubType::None,
        codec::Version::V0,
        bits,
        digest.as_bytes(),
    )?;
    Ok(format!("ISCC:{component}"))
}
```

### Conformance test vectors

Download `data.json` from:
`https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`

Place it at `crates/iscc-lib/tests/data.json`.

The file structure for `gen_instance_code_v0` entries:

```json
{
  "gen_instance_code_v0": {
    "test_0000_empty_64": {
      "inputs": [
        "stream:",
        64
      ],
      "outputs": {
        "iscc": "ISCC:IAA26E2JXH27TING",
        "datahash": "...",
        "filesize": 0
      }
    }
  }
}
```

- `"stream:"` prefix means hex-encoded bytes (empty string after `stream:` = empty bytes)
- `"stream:abcd"` means `hex::decode("abcd")`
- First input is the data, second is bits
- Only verify the `iscc` output field (datahash/filesize are secondary metadata)

### Conformance test pattern

Write a test in `src/lib.rs` (or a separate integration test file) that:

1. Loads `data.json` via `include_str!("../tests/data.json")` for portability
2. Parses the JSON and extracts the `gen_instance_code_v0` section
3. For each test case: strips the `"stream:"` prefix, decodes hex to bytes, calls
    `gen_instance_code_v0(data, bits)`, asserts the `iscc` field matches
4. Use `serde_json::Value` for flexible parsing — no need for full struct deserialization

Update the existing `test_gen_instance_code_v0_stub` to verify real output instead of
`NotImplemented`.

### Known test vector (for quick sanity check)

Empty input with 64 bits → `"ISCC:IAA26E2JXH27TING"`

## Verification

- `cargo test -p iscc-lib` passes (all existing tests + new conformance tests)
- `gen_instance_code_v0(b"", 64)` returns `Ok("ISCC:IAA26E2JXH27TING")`
- All `gen_instance_code_v0` test vectors from `data.json` pass
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code added

## Done When

All verification criteria pass — `gen_instance_code_v0` produces correct ISCC Instance-Codes for
every conformance vector, and the conformance test infrastructure is in place for subsequent
functions.
