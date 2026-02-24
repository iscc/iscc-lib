# Next Work Package

## Step: Validate Content-Code length in `soft_hash_codes_v0`

## Goal

Add length validation to `soft_hash_codes_v0` so that Content-Codes with fewer bits than the
requested `bits` parameter are rejected with `IsccError::InvalidInput`, matching the Python
reference behavior. Currently the function silently zero-pads short codes, producing results that
diverge from `iscc-core`.

## Scope

- **Modify**: `crates/iscc-lib/src/lib.rs` (add validation in `soft_hash_codes_v0`, add tests)
- **Reference**: `reference/iscc-core/iscc_core/code_content_mixed.py` lines 88–90 (Python length
    check), `crates/iscc-lib/src/codec.rs` line 334 (`decode_length` function)

## Not In Scope

- Fixing `gen_meta_code_v0` empty Data-URL routing — separate issue, separate step
- Fixing `alg_simhash` panic on mismatched digest sizes — separate step
- Fixing `sliding_window` panic on width < 2 — separate step
- Changing `decode_header` or `decode_length` internals — use them as-is
- Removing the zero-padding logic entirely — after validation passes, the padding path is dead code
    for valid inputs, but removing it is a cosmetic cleanup that can wait
- Removing the issue from `issues.md` — the review agent handles that after verifying

## Implementation Notes

The fix is in `crates/iscc-lib/src/lib.rs` inside `soft_hash_codes_v0` (line ~574). After
`decode_header` returns `(mtype, stype, _ver, blen, body)`, call
`codec::decode_length(mtype, blen, stype)` to get the unit's bit length, then check it against
`bits`.

Reference Python (`code_content_mixed.py:88-90`):

```python
unit_lengths = [ic.decode_length(t[0], t[3]) for t in code_tuples]
if not all(ul >= bits for ul in unit_lengths):
    raise AssertionError(f"Code to short for {bits}-bit length")
```

Rust implementation — add after the `mtype != MainType::Content` check (around line 590):

```rust
let unit_bits = codec::decode_length(mtype, blen, stype);
if unit_bits < bits {
    return Err(IsccError::InvalidInput(
        format!("Content-Code too short for {bits}-bit length (has {unit_bits} bits)"),
    ));
}
```

The existing `decode_header` destructure on line 585 needs to capture `stype` (currently `_stype`)
and `blen` (currently `_blen`):

```rust
let (mtype, stype, _ver, blen, body) = codec::decode_header(raw)?;
```

**Tests to add** (≥3 new test functions in the existing `#[cfg(test)] mod tests` in `lib.rs`):

- `test_soft_hash_codes_v0_rejects_short_code` — construct two Content-Codes where one has fewer
    bits than the default 64. Use `codec::encode_component` with `MainType::Content` and a body
    shorter than 8 bytes (e.g., 32-bit = 4 bytes body). Verify `Err(IsccError::InvalidInput(_))`.
- `test_soft_hash_codes_v0_accepts_exact_length` — two Content-Codes with exactly 64 bits each.
    Verify `Ok(...)`.
- `test_soft_hash_codes_v0_accepts_longer_codes` — two Content-Codes with 128 bits each, requesting
    64 bits. Verify `Ok(...)`.

To construct test Content-Codes, use `codec::encode_component` which produces raw bytes (header +
body). For a 32-bit Content-Code:
`codec::encode_component(MainType::Content, SubType::Text, Version::V0, 32, &body_4bytes)`. For
64-bit: same with 64-bit body (8 bytes).

## Verification

- `cargo test -p iscc-lib` passes (all 244+ existing tests + ≥3 new tests, 0 failures)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib -- test_soft_hash_codes` passes (new validation tests)
- The new rejection test asserts `Err(IsccError::InvalidInput(_))` containing "too short"

## Done When

All verification criteria pass — `soft_hash_codes_v0` rejects Content-Codes shorter than the
requested bit length with a descriptive error, matching the Python reference behavior.
