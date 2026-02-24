# Next Work Package

## Step: Fix `alg_cdc_chunks` infinite loop when `utf32=true`

## Goal

Fix a critical bug where `alg_cdc_chunks` enters an infinite loop when `utf32=true` and the
remaining buffer is smaller than 4 bytes. This is a Tier 1 public API and any external caller using
`utf32=true` with certain inputs will hang the process.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/cdc.rs` (fix alignment logic + add tests)
- **Reference**: `reference/iscc-core/iscc_core/cdc.py` (lines 45-50),
    `reference/iscc-core/tests/test_cdc.py` (lines 83-98, `test_data_chunks_utf32`)

## Not In Scope

- Fixing the same bug upstream in iscc-core (the Python reference has the identical issue at
    `cdc.py:47` — `cut_point -= cut_point % 4` can also produce 0 in Python, but that's a separate
    upstream issue to file later)
- Porting the reference `test_data_chunks_utf32` conformance test with `static_bytes` and BLAKE3
    hash verification — that's a separate step requiring the `static_bytes` helper
- Fixing other issues from issues.md (the normal/low priority issues wait)
- Changing the function signature or return type
- Refactoring the CDC module beyond the minimal fix

## Implementation Notes

The bug is at `cdc.rs:130-132`:

```rust
if utf32 {
    cut_point -= cut_point % 4;
}
```

When `cut_point < 4` (e.g., the remaining buffer is 1-3 bytes), `cut_point % 4 == cut_point`, so
`cut_point` becomes 0. This means `pos` never advances and the loop at line 125 runs forever.

**Fix**: After the UTF-32 alignment subtraction, if `cut_point` is 0, set it to the minimum of 4 and
the remaining data length. This guarantees forward progress while maintaining 4-byte alignment when
possible:

```rust
if utf32 {
    cut_point -= cut_point % 4;
    if cut_point == 0 {
        cut_point = remaining.len().min(4);
    }
}
```

Using `remaining.len()` (which equals `data.len() - pos`) handles the edge case where fewer than 4
bytes remain — we consume whatever is left rather than trying to align.

**Tests to add** (inside the existing `mod tests` block in `cdc.rs`):

1. `test_alg_cdc_chunks_utf32_small_buffer` — input of 3 bytes (not 4-byte aligned) with
    `utf32=true`. Must terminate and return chunks that reassemble to the original data.
2. `test_alg_cdc_chunks_utf32_exact_4_bytes` — input of exactly 4 bytes with `utf32=true`. Must
    return one chunk of 4 bytes.
3. `test_alg_cdc_chunks_utf32_7_bytes` — input of 7 bytes (4+3) with `utf32=true`. Verifies handling
    of a non-aligned tail.
4. `test_alg_cdc_chunks_utf32_reassembly` — larger input (~4096 bytes, 4-byte aligned) with
    `utf32=true`. Chunks must reassemble to original data.
5. `test_alg_cdc_chunks_utf32_empty` — empty input with `utf32=true`. Must not loop.

All tests should verify: (a) the function terminates, (b) chunks reassemble to original input, (c)
for 4-byte-aligned inputs, all chunks except possibly the last are 4-byte aligned.

## Verification

- `cargo test -p iscc-lib` passes (all 184+ existing tests + 5 new utf32 tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib -- test_alg_cdc_chunks_utf32` passes (the 5 new tests specifically)
- The 3-byte input test proves the infinite loop is fixed (it would hang/timeout if the bug
    remained)

## Done When

All verification criteria pass, confirming `alg_cdc_chunks` with `utf32=true` no longer loops on
small or non-aligned inputs.
