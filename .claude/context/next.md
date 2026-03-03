# Next Work Package

## Step: Fix WASM conformance assertion count (16→20)

## Goal

Fix the CI-breaking WASM conformance test that still asserts `tested == 16` for `gen_meta_code_v0`
after data.json was updated to iscc-core v1.3.0 with 20 Meta-Code vectors. CI must be green before
starting Ruby bindings work.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-wasm/tests/conformance.rs` (line 66: change `16` to `20` and message)
- **Reference**: `crates/iscc-lib/tests/data.json` (to confirm 20 meta vectors)

## Not In Scope

- Updating any other assertion counts in the WASM conformance file (the other 8 functions already
    have correct counts: 5+3+5+3+2+4+3+5 matching v1.3.0 data.json)
- Starting Ruby bindings scaffold — that comes after CI is green
- Porting any new iscc-core v1.3.0 API additions (`iscc_validate` tightening, `iscc_nph_compare`)
- Changing conformance test structure or refactoring the test file

## Implementation Notes

Single-line change on line 66 of `crates/iscc-wasm/tests/conformance.rs`:

```rust
// Before:
assert_eq!(tested, 16, "expected 16 conformance tests to run");

// After:
assert_eq!(tested, 20, "expected 20 conformance tests to run");
```

This mirrors the identical fix already applied to `crates/iscc-lib/src/lib.rs` in commit `0c9b03b`
(the advance agent updated Rust core but forgot the WASM binding).

## Verification

- `cargo test -p iscc-wasm --test conformance` compiles without errors (wasm-pack not required for
    compilation check)
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `grep 'assert_eq!(tested, 20' crates/iscc-wasm/tests/conformance.rs` returns exactly one match
- `grep 'assert_eq!(tested, 16' crates/iscc-wasm/tests/conformance.rs` returns zero matches

## Done When

All four verification checks pass, confirming the WASM conformance assertion matches the v1.3.0
vector count.
