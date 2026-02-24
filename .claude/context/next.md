# Next Work Package

## Step: Harden `iscc_decompose` against truncated input

## Goal

Add bounds checks to `iscc_decompose` so that malformed or truncated base32 input returns
`IsccError::InvalidInput` instead of panicking. This is the highest-impact open code correctness
issue — it affects a Tier 1 public API bound to all four languages.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` (add bounds checks in `iscc_decompose`, add tests)
- **Reference**: `reference/iscc-core/iscc_core/codec.py` lines 376–421 (Python `iscc_decompose`)

## Not In Scope

- Fixing the other normal-priority robustness issues (`soft_hash_codes_v0`, `gen_meta_code_v0`,
    `alg_simhash`, `sliding_window`) — each is a separate step
- Changing `decode_header` or `decode_length` internals — only validate `body` length after they
    return
- Adding fuzz testing infrastructure — a future step
- Removing the issue from `issues.md` — the review agent handles that after verifying

## Implementation Notes

The function at `codec.rs:463–526` has five unchecked slice operations that panic on truncated
`body` vectors returned by `decode_header`:

1. **Line 475**: `&body[..nbytes]` — standard unit path. Guard: `body.len() >= nbytes`.
2. **Line 486**: `&body[..16]` — wide mode Data-Code. Guard: `body.len() >= 16`.
3. **Line 488**: `&body[16..32]` — wide mode Instance-Code. Guard: `body.len() >= 32`.
4. **Line 501**: `&body[idx * 8..]` — dynamic unit loop. Guard: `body.len() >= (idx + 1) * 8` (each
    unit needs 8 bytes).
5. **Lines 511/518**: `&body[body.len() - 16..]` and `&body[body.len() - 8..]` — static units.
    Guard: `body.len() >= main_types.len() * 8 + 16` (dynamic units + Data + Instance = total
    body).

For each failing guard, return:

```rust
Err(IsccError::InvalidInput("truncated ISCC body: expected N bytes, got M".into()))
```

The wide-mode path (items 2+3) can use a single check: `body.len() >= 32`. The non-wide ISCC-CODE
path (items 4+5) can use a single upfront check: total expected body size is
`main_types.len() * 8 + 16` bytes (dynamic units × 8 + Data 8 + Instance 8).

**Python comparison**: Python slicing silently truncates on out-of-bounds (`body[16:32]` returns
fewer bytes if body is short), so the reference doesn't crash — it produces wrong output. The Rust
fix is strictly better: reject invalid input early.

**Tests to add** (≥5 new test functions):

- `test_decompose_truncated_standard_unit` — single unit with body shorter than `nbytes`
- `test_decompose_truncated_wide_mode` — ISCC-CODE with Wide subtype and body < 32 bytes
- `test_decompose_truncated_dynamic_units` — ISCC-CODE with body too short for dynamic units
- `test_decompose_truncated_static_units` — ISCC-CODE with body too short for trailing Data+Instance
- `test_decompose_empty_body` — valid header but zero body bytes
- `test_decompose_valid_still_works` — ensure at least one existing valid decompose still passes
    (regression guard; existing tests already cover this, but an explicit one is nice)

To craft truncated inputs: use `encode_header` to produce a valid header, append fewer body bytes
than expected, then `encode_base32` the result. Alternatively, take a known-valid base32 ISCC string
and truncate characters from the end.

## Verification

- `cargo test -p iscc-lib` passes (all 237+ existing tests + ≥5 new tests, 0 failures)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib -- test_decompose_truncated` passes (new truncation tests)
- Every new test asserts that the result is `Err(IsccError::InvalidInput(_))`, not a panic

## Done When

All verification criteria pass — `iscc_decompose` returns `Err` instead of panicking for every
truncated-body scenario, with test coverage for each guard.
