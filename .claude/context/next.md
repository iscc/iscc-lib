# Next Work Package

## Step: Implement encode_base64 Tier 1 function

## Goal

Add `encode_base64` as a public Tier 1 API function in the codec module. This is the simplest of the
4 remaining Tier 1 symbols — a one-line wrapper around `data_encoding::BASE64URL_NOPAD` matching
iscc-core's `encode_base64` (RFC 4648 base64url, no padding).

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/codec.rs` (add function + tests), `crates/iscc-lib/src/lib.rs`
    (add re-export)
- **Reference**: `reference/iscc-core/iscc_core/codec.py` (lines 307-313: `encode_base64`),
    `crates/iscc-lib/src/codec.rs` (lines 378-391: existing `encode_base32`/`decode_base32` pattern)

## Implementation Notes

1. **Function**: Add `pub fn encode_base64(data: &[u8]) -> String` in `codec.rs`, right after the
    existing base32 section (after line 391). Use `data_encoding::BASE64URL_NOPAD.encode(data)` —
    this matches Python's `base64.urlsafe_b64encode(data).decode("ascii").rstrip("=")` exactly. Add
    a short docstring: "Encode bytes as base64url (RFC 4648 §5, no padding)."

2. **Re-export**: In `lib.rs`, add `codec::encode_base64` to the module's public surface. The codec
    module is already `pub mod codec`, so the function just needs to be `pub fn`. Also add a flat
    re-export: `pub use codec::encode_base64;` alongside the existing `pub use` lines.

3. **Tests**: Add tests in the `codec.rs` `#[cfg(test)] mod tests` section:

    - Empty input: `encode_base64(&[])` → `""`
    - Known value: `encode_base64(&[0, 1, 2, 3])` → verify against Python (`AAECAT` — run
        `base64.urlsafe_b64encode(bytes([0,1,2,3])).decode().rstrip("=")` to confirm)
    - Roundtrip with `data_encoding::BASE64URL_NOPAD.decode()` for random-ish data
    - Padding-free: verify output contains no `=` characters

4. **Pattern**: Follow the exact pattern of `encode_base32` — public function, one-line body,
    concise docstring. No error return needed (encoding always succeeds).

## Verification

- `cargo test -p iscc-lib` passes (all 193+ tests including new encode_base64 tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `iscc_lib::encode_base64(&[0, 1, 2, 3])` returns the correct base64url string
- `iscc_lib::encode_base64(&[])` returns `""`
- Output matches iscc-core `encode_base64` for identical inputs (base64url, no padding)
- Function is importable as `iscc_lib::encode_base64` (flat re-export)

## Done When

All verification criteria pass — `encode_base64` is a public Tier 1 function producing base64url
output matching iscc-core, with tests proving correctness.
