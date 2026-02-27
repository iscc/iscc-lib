# Next Work Package

## Step: Add algorithm constants and Tier 1 encode_component wrapper

## Goal

Add 5 of the 7 missing Tier 1 symbols to the Rust core: 4 algorithm constants (`META_TRIM_NAME`,
`META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) and a Tier 1 `encode_component` wrapper
that accepts `u8` enum parameters. This unblocks binding crates and addresses issues #6 and #8.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` (add `pub const` values, add Tier 1 `encode_component`
    wrapper function with doc comments and tests)
- **Reference**: `crates/iscc-lib/src/codec.rs` (existing `encode_component` with Rust enum params,
    `MainType`/`SubType`/`Version` enums with `TryFrom<u8>` impls),
    `reference/iscc-core/iscc_core/options.py` (constant values),
    `.claude/context/specs/rust-core.md` (Tier 1 API spec)

## Not In Scope

- Implementing `iscc_decode` — that's a separate step requiring new decode logic
- Implementing `json_to_data_url` — separate step requiring base64 encoding logic
- Propagating new symbols to any binding crate (Python, Node.js, WASM, C FFI, JNI, Go)
- Adding Python `IntEnum` wrappers (`MT`, `ST`, `VS`) or `core_opts` — those are binding-layer work
- Changing the existing `codec::encode_component` signature — the Tier 2 version stays as-is
- Updating documentation site or README

## Implementation Notes

### Algorithm Constants

Add 4 `pub const` values directly in `lib.rs` (near the top, after the `pub use` re-exports):

```rust
/// Max UTF-8 byte length for name metadata trimming.
pub const META_TRIM_NAME: usize = 128;

/// Max UTF-8 byte length for description metadata trimming.
pub const META_TRIM_DESCRIPTION: usize = 4096;

/// Buffer size in bytes for streaming file reads (4 MB).
pub const IO_READ_SIZE: usize = 4_194_304;

/// Character n-gram width for text content features.
pub const TEXT_NGRAM_SIZE: usize = 13;
```

**Note on `IO_READ_SIZE`**: The spec says `4_194_304` (4 MB). The Python reference `options.py` says
`2_097_152` (2 MB). Follow the spec — Titusz authored both the spec and the issues, and this
constant is advisory (used by SDK for buffer sizing, not by core algorithms). If in doubt, use the
spec value `4_194_304`.

Optionally replace the hardcoded magic numbers `128`, `4096`, and `13` in the existing gen functions
(`gen_meta_code_v0` lines ~170-181, `gen_text_code_v0` line ~270) with the new constants. This is
good practice but secondary — the constants must exist as `pub const` regardless.

### Tier 1 `encode_component` Wrapper

Add a new `pub fn encode_component` at crate root (in `lib.rs`) that takes `u8` for enum fields and
delegates to `codec::encode_component`. There is no naming conflict because
`codec::encode_component` is NOT re-exported at crate root.

```rust
/// Encode a raw digest into an ISCC unit string.
///
/// Takes integer type identifiers (matching `MainType`, `SubType`, `Version` enum values)
/// and a raw digest, returns a base32-encoded ISCC unit string.
///
/// # Errors
///
/// Returns `IsccError::InvalidInput` if enum values are out of range, if `mtype` is
/// `MainType::Iscc` (5), or if `digest.len() < bit_length / 8`.
pub fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: &[u8],
) -> IsccResult<String> {
    let mt = codec::MainType::try_from(mtype)?;
    let st = codec::SubType::try_from(stype)?;
    let vs = codec::Version::try_from(version)?;
    // Validate digest length before delegating
    let needed = (bit_length / 8) as usize;
    if digest.len() < needed {
        return Err(IsccError::InvalidInput(format!(
            "digest length {} < bit_length/8 ({})", digest.len(), needed
        )));
    }
    codec::encode_component(mt, st, vs, bit_length, digest)
}
```

**Key details:**

- The `TryFrom<u8>` impls already exist on `MainType`, `SubType`, and `Version` in `codec.rs`
- The wrapper adds an explicit `digest.len() < bit_length / 8` check (the spec says to reject short
    digests rather than silently truncating)
- The existing `codec::encode_component` already rejects `MainType::Iscc` — that check cascades
    through
- All internal callers in `lib.rs` already use `codec::encode_component(MainType::..., ...)` — they
    don't need to change

### Tests

Add tests in the `#[cfg(test)]` module at the bottom of `lib.rs`:

1. **Constants tests**: Assert each constant equals its expected value
2. **encode_component round-trip**: Encode a known digest and verify the output matches the codec
    version
3. **encode_component rejects Iscc**: `encode_component(5, 0, 0, 64, &[0;8])` returns error
4. **encode_component rejects short digest**: `encode_component(0, 0, 0, 64, &[0;4])` returns error
5. **encode_component matches codec**: `encode_component(3, 0, 0, 64, &digest)` ==
    `codec::encode_component(MainType::Data, SubType::None, Version::V0, 64, &digest)`

## Verification

- `cargo test -p iscc-lib` passes (all 269 existing + new tests for constants and encode_component)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep -c 'pub const META_TRIM_NAME' crates/iscc-lib/src/lib.rs` returns 1
- `grep -c 'pub const IO_READ_SIZE' crates/iscc-lib/src/lib.rs` returns 1
- `grep -c 'pub fn encode_component' crates/iscc-lib/src/lib.rs` returns 1

## Done When

All verification criteria pass — the 4 algorithm constants and the Tier 1 `encode_component` wrapper
with `u8` parameters are public at the `iscc_lib` crate root, tested, and clippy-clean.
