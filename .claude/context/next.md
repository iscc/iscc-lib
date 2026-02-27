# Next Work Package

## Step: Implement `iscc_decode` Tier 1 function

## Goal

Add `iscc_decode` to the Rust core as the 29th of 30 Tier 1 symbols. This is the inverse of
`encode_component` — it decodes an ISCC unit string back into its header components and raw digest.
Needed by `iscc-sdk` for `code_sum()` and ISCC unit inspection (issue #7).

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/lib.rs` — add `pub fn iscc_decode` function and tests
- **Reference**:
    - `reference/iscc-core/iscc_core/codec.py` — `iscc_decode` (line 486), `iscc_clean` (line 644),
        `decode_header` (line 108)
    - `crates/iscc-lib/src/codec.rs` — `decode_base32`, `decode_header`, `decode_length`
    - Issue #7 in `.claude/context/issues.md`

## Not In Scope

- `iscc_normalize` (full multibase/hex normalization) — our `iscc_decode` only needs basic cleaning
    (strip "ISCC:" prefix, remove dashes), matching the pattern in `iscc_decompose`
- `iscc_clean` or `iscc_normalize` as separate public functions — not Tier 1 symbols
- `iscc_explain` — depends on `iscc_decode` but is not in the target API
- Propagating `iscc_decode` to binding crates — separate step
- Implementing `json_to_data_url` — the next step after this one
- Adding conformance vectors for `iscc_decode` — the existing data.json does not have
    decode-specific vectors; round-trip tests with `encode_component` provide sufficient coverage

## Implementation Notes

**Algorithm** (port from Python `iscc_decode` in `codec.py` line 486-497):

1. Strip optional `"ISCC:"` prefix (case-sensitive, matching `iscc_decompose` pattern)
2. Remove dashes (matching `iscc_clean` behavior for base32 input)
3. `codec::decode_base32(cleaned)` → raw bytes
4. `codec::decode_header(raw)` → `(MainType, SubType, Version, length_index, tail)`
5. `codec::decode_length(mtype, length_index, stype)` → `bit_length`
6. Truncate `tail` to exactly `bit_length / 8` bytes (cleaner than the Python ref which returns full
    tail and expects callers to truncate — our API returns the usable digest directly)
7. Return `(mtype as u8, stype as u8, version as u8, length_index as u8, truncated_digest)`

**Signature**: `pub fn iscc_decode(iscc: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>`

**Pattern**: Follow the same Tier 1 wrapper pattern as `encode_component` — takes/returns primitive
types (`u8` for enum fields), delegates to `codec::` module functions internally. Define the
function directly in `lib.rs` (not in `codec.rs`) to match the `encode_component` placement.

**Error cases**:

- Empty or whitespace-only string → propagated from `decode_base32`
- Invalid base32 characters → propagated from `decode_base32`
- Malformed header (unknown enum values) → propagated from `decode_header`
- Tail shorter than expected digest length → return `IsccError::InvalidInput` with descriptive
    message

**Tests** (add in the `#[cfg(test)]` module at the bottom of `lib.rs`):

1. **Round-trip with encode_component**: encode known digests with `encode_component`, decode with
    `iscc_decode`, verify all 5 tuple fields match
2. **With "ISCC:" prefix**: prepend "ISCC:" to an encoded component, decode, verify same result
3. **With dashes**: insert dashes into an encoded string, verify decode still works
4. **Various MainTypes**: test at least Meta (0), Content (2), Data (3), Instance (4) round-trips
5. **Error: invalid base32**: `iscc_decode("!!!INVALID!!!")` returns error
6. **Error: truncated input**: a too-short base32 string that decodes to fewer bytes than needed
7. **Known value**: pick a known ISCC from `data.json` test vectors and verify `iscc_decode` returns
    the expected MainType, SubType, Version, and digest length

## Verification

- `cargo test -p iscc-lib` passes (280 existing + new `iscc_decode` tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep -c 'pub fn iscc_decode' crates/iscc-lib/src/lib.rs` returns 1
- Round-trip: `encode_component(0, 0, 0, 64, &[0xaa;8])` → `iscc_decode(result)` returns
    `(0, 0, 0, 1, vec![0xaa;8])` (length_index is 1 because `encode_length(Meta, 64) = 64/32-1 = 1`)

## Done When

All verification criteria pass and `iscc_decode` is a public Tier 1 function in
`crates/iscc-lib/src/lib.rs` that correctly decodes any valid ISCC unit string into its header
components and truncated digest.
