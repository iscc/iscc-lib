# Next Work Package

## Step: Add input validation to alg_dct and alg_wtahash

## Goal

Fix two `[low]` correctness issues in the core crate: enforce power-of-two input length in `alg_dct`
(prevents silent incorrect output for non-power-of-two even lengths like 6, 10, 12) and add a
minimum-length guard to `alg_wtahash` (prevents index-out-of-bounds panic on vectors shorter than
380 elements).

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/dct.rs`, `crates/iscc-lib/src/wtahash.rs`,
    `crates/iscc-lib/src/lib.rs` (caller update for new `alg_wtahash` return type)
- **Reference**: issues.md (issue descriptions), `crates/iscc-lib/src/lib.rs:547` (sole
    `alg_wtahash` call site), `crates/iscc-lib/src/lib.rs:384,394` (`alg_dct` call sites — already
    handle `IsccResult`)

## Not In Scope

- Changing `alg_dct` or `alg_wtahash` visibility (they are `pub(crate)` by module visibility and
    should stay internal)
- Adding validation to other internal functions — scope is strictly these two issues
- Exposing `alg_wtahash` or `alg_dct` in the Tier 1 public API
- Fixing the other `[low]` issues (iscc-py, iscc-wasm, iscc-jni) — those are separate steps

## Implementation Notes

### `alg_dct` (dct.rs)

Replace the validation check at line 19:

```rust
// Current (wrong — accepts 6, 10, 12, etc.):
if n == 0 || (n > 1 && n % 2 != 0) {

// Fixed:
if !n.is_power_of_two() {
```

`n.is_power_of_two()` returns `false` for 0, `true` for 1, `true` for 2/4/8/16/32/…, and `false` for
all non-power-of-two values. This is a single-line change. Update the error message to say "power of
2" instead of "even length (or 1)". Update the docstring similarly.

Add test cases:

- `alg_dct(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])` → `Err` (length 6 is even but not power of 2)
- `alg_dct(&[1.0, 2.0])` → `Ok` (length 2 is valid)
- `alg_dct(&[1.0])` → `Ok` (length 1 is valid — already tested)

### `alg_wtahash` (wtahash.rs)

1. Change return type from `Vec<u8>` to `IsccResult<Vec<u8>>`
2. Add guard at function entry: `if vec.len() < 380 { return Err(InvalidInput("...")) }`
3. Also validate `bits`: must be > 0, divisible by 8, and ≤ 256 (the permutation table has 256
    entries)
4. Wrap the existing return in `Ok(...)`
5. Update the docstring to document the constraints

### Caller update (lib.rs)

At line 547, change:

```rust
Ok(wtahash::alg_wtahash(&vecsum, bits))
// →
Ok(wtahash::alg_wtahash(&vecsum, bits)?)
```

The caller `soft_hash_video_v0` already returns `IsccResult<Vec<u8>>`, so the `?` propagates
naturally.

Add test cases for `alg_wtahash`:

- Short input (length 100) → `Err`
- `bits = 0` → `Err`
- `bits = 7` (not divisible by 8) → `Err`
- `bits = 512` (exceeds 256 permutations) → `Err`

## Verification

- `cargo test -p iscc-lib` passes (all existing 261 tests + new validation tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `alg_dct(&[1.0; 6])` returns `Err` (new test confirms non-power-of-two rejection)
- `alg_wtahash(&[0i64; 100], 64)` returns `Err` (new test confirms short-vector rejection)
- All existing conformance tests still pass (no behavioral change for valid inputs)

## Done When

All verification criteria pass: existing tests unaffected, new validation tests confirm both
`alg_dct` and `alg_wtahash` reject invalid inputs with proper errors instead of producing wrong
output or panicking.
