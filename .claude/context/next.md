# Next Work Package

## Step: Implement conformance_selftest diagnostic function

## Goal

Implement the last remaining Tier 1 symbol — `conformance_selftest()` — a public diagnostic function
that runs all 9 gen functions against every vendored conformance vector from `data.json` and returns
`true` if all produce correct output. This completes the Rust core Tier 1 API (23/23 symbols).

## Scope

- **Create**: `crates/iscc-lib/src/conformance.rs` — new module containing the selftest logic
- **Modify**: `crates/iscc-lib/src/lib.rs` — add `pub mod conformance;` and
    `pub use conformance::conformance_selftest;` re-export
- **Reference**:
    - `reference/iscc-core/iscc_core/conformance.py` — Python reference implementation
    - `crates/iscc-lib/src/lib.rs` lines 889–1400 — existing conformance test patterns for all 9 gen
        functions (input decoding, output comparison)
    - `crates/iscc-lib/tests/data.json` — vendored conformance vectors
    - `.claude/context/specs/rust-core.md` lines 161–175 — spec for this function

## Implementation Notes

**Signature:** `pub fn conformance_selftest() -> bool`

**Pattern:** Port from `reference/iscc-core/iscc_core/conformance.py`. The function:

1. Loads `data.json` via `include_str!("../tests/data.json")` (compile-time embedding, same as
    existing tests)
2. Parses as `serde_json::Value`
3. Iterates through all 9 function sections: `gen_meta_code_v0`, `gen_text_code_v0`,
    `gen_image_code_v0`, `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`,
    `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`
4. For each test case, decodes inputs (handling `"stream:"` hex prefix for byte data), calls the
    corresponding gen function, and compares the `.iscc` field of the result against expected
    output
5. Returns `true` if all pass, `false` if any mismatch or error

**Input decoding rules** (same as existing tests):

- `"stream:<hex>"` → `hex::decode(hex_part)` to get `Vec<u8>` / `&[u8]`
- String values → `&str`
- Numbers → `u64` cast to `u32` for bits, `bool` for wide
- Null → `None`
- Objects (in meta field) → `serde_json::to_string()` for JSON string
- Arrays of arrays (video frame_sigs) → `Vec<Vec<i32>>`

**Output comparison:** Compare only the `.iscc` field from each result struct against
`outputs["iscc"]`. This is sufficient for conformance — the existing per-field tests in `lib.rs`
already verify all struct fields individually.

**Error handling:** Any `Err` from a gen function should count as a failure (set `passed = false`),
not panic. The function should be resilient — it catches all errors and continues through remaining
test cases. Use `eprintln!` for failure diagnostics (matching Python's `log.error`).

**Reuse existing patterns:** The input decoding and dispatch logic for each function is already
proven in the `#[test]` functions in `lib.rs` (lines 889–1400). Extract and adapt that logic into
the selftest. The key difference is: tests use `assert_eq!` (panics on failure), but selftest uses
comparison and returns `false` (no panic).

**Module placement:** Create a dedicated `conformance.rs` module rather than adding to the already
large `lib.rs`. This keeps the diagnostic separate from core algorithms.

## Verification

- `cargo test -p iscc-lib` passes (all 229+ existing tests still pass)
- A new test calls `conformance_selftest()` and asserts it returns `true`
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `iscc_lib::conformance_selftest` is accessible as a public API symbol
- The function handles all 9 gen function sections in `data.json` without panicking

## Done When

All verification criteria pass — `conformance_selftest()` returns `true` when called, is exported as
public Tier 1 API, and all existing tests continue to pass.
