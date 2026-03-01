# Next Work Package

## Step: Add META_TRIM_META constant and payload validation to Rust core

## Goal

Add the `META_TRIM_META: usize = 128_000` constant to the Rust core Tier 1 API and enforce payload
size validation in `gen_meta_code_v0`, preventing unbounded memory/compute from oversized meta
parameters. This addresses issue #18.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/lib.rs` — add `META_TRIM_META` constant next to existing `META_TRIM_*`
        constants; add pre-decode and post-decode size checks in `gen_meta_code_v0`
- **Reference**:
    - `.claude/context/specs/rust-core.md` lines 254–291 (Algorithm Constants spec with validation
        formula)
    - `reference/iscc-core/iscc_core/code_meta.py` (Python reference for meta handling)
    - `crates/iscc-lib/src/lib.rs` lines 276–378 (current `gen_meta_code_v0` implementation)

## Not In Scope

- Exposing `META_TRIM_META` in any binding (Python, Node.js, WASM, C FFI, Java, Go) — that's a
    separate step after the Rust core is done
- Adding `gen_sum_code_v0` or `SumCodeResult` — that's issue #15, a separate larger feature
- Truncating payloads to the limit instead of rejecting — the spec says reject with error
- Updating documentation pages or READMEs for the new constant — wait until bindings are also done

## Implementation Notes

**Constant definition** (add after `META_TRIM_DESCRIPTION` at line ~32):

```rust
/// Max decoded payload size in bytes for the meta element.
pub const META_TRIM_META: usize = 128_000;
```

**Pre-decode fast check** — Before decoding the Data-URL or parsing JSON, check string length.
Base64 encoding inflates by ~4/3, plus there's a media type header. The spec formula:

```rust
const PRE_DECODE_LIMIT: usize = META_TRIM_META * 4 / 3 + 256;
```

Apply this check when `meta_str` is provided (both Data-URL and JSON paths), before any decoding
work. Return `IsccError::InvalidInput` with a descriptive message.

**Post-decode check** — After `decode_data_url()` or `parse_meta_json()` returns the payload bytes,
check `payload.len() > META_TRIM_META`. Return `IsccError::InvalidInput`.

**Where to insert in `gen_meta_code_v0`** — The current match block at lines 299–303:

```rust
let meta_payload: Option<Vec<u8>> = match meta {
    Some(meta_str) if meta_str.starts_with("data:") => Some(decode_data_url(meta_str)?),
    Some(meta_str) => Some(parse_meta_json(meta_str)?),
    None => None,
};
```

Add the pre-decode check before the match, and the post-decode check after. Pattern:

```rust
// Pre-decode fast check: reject obviously oversized meta strings
if let Some(meta_str) = meta {
    const PRE_DECODE_LIMIT: usize = META_TRIM_META * 4 / 3 + 256;
    if meta_str.len() > PRE_DECODE_LIMIT {
        return Err(IsccError::InvalidInput(format!(
            "meta string exceeds size limit ({} > {PRE_DECODE_LIMIT} bytes)",
            meta_str.len()
        )));
    }
}

let meta_payload: Option<Vec<u8>> = match meta {
    Some(meta_str) if meta_str.starts_with("data:") => Some(decode_data_url(meta_str)?),
    Some(meta_str) => Some(parse_meta_json(meta_str)?),
    None => None,
};

// Post-decode check: reject payloads exceeding META_TRIM_META
if let Some(ref payload) = meta_payload {
    if payload.len() > META_TRIM_META {
        return Err(IsccError::InvalidInput(format!(
            "decoded meta payload exceeds size limit ({} > {META_TRIM_META} bytes)",
            payload.len()
        )));
    }
}
```

**Tests to add** (in the existing `mod tests` block):

1. `test_meta_trim_meta_value` — assert `META_TRIM_META == 128_000`
2. `test_gen_meta_code_v0_meta_at_limit` — create a JSON payload of exactly 128,000 bytes, verify it
    succeeds
3. `test_gen_meta_code_v0_meta_over_limit` — create a JSON payload of 128,001 bytes, verify it
    returns `IsccError::InvalidInput`
4. `test_gen_meta_code_v0_data_url_pre_decode_reject` — create a Data-URL string exceeding the
    pre-decode limit, verify rejection before decoding

**Edge case**: Existing conformance tests with meta payloads are small (well under 128K) — they must
continue to pass unchanged.

## Verification

- `cargo test -p iscc-lib` passes (all existing tests + 4 new tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `iscc_lib::META_TRIM_META` is importable and equals `128_000`
- `gen_meta_code_v0("test", None, Some(<128K+ payload>), 64)` returns `Err(IsccError::InvalidInput)`
- All existing conformance vector tests still pass

## Done When

All verification criteria pass: the `META_TRIM_META` constant exists at crate root,
`gen_meta_code_v0` rejects oversized payloads with pre-decode and post-decode checks, new boundary
tests pass, and all existing tests remain green.
