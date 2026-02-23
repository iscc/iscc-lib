# Next Work Package

## Step: Promote simhash and minhash modules to Tier 1

## Goal

Make `sliding_window`, `alg_simhash` (from `simhash` module), and `alg_minhash_256` (from `minhash`
module) publicly accessible as Tier 1 API, following the same promotion pattern established for text
utilities. This exposes 3 of the 4 target algorithm primitives.

## Scope

- **Create**: `crates/iscc-lib/tests/test_algorithm_primitives.rs` — integration tests
- **Modify**: `crates/iscc-lib/src/lib.rs` — change module visibility + add `pub use` re-exports
- **Modify**: `crates/iscc-lib/src/simhash.rs` — change `alg_simhash` and `sliding_window` from
    `pub(crate)` to `pub`
- **Modify**: `crates/iscc-lib/src/minhash.rs` — change `alg_minhash_256` from `pub(crate)` to `pub`
- **Reference**: `crates/iscc-lib/tests/test_text_utils.rs` — established integration test pattern
- **Reference**: `reference/iscc-core/iscc_core/simhash.py` — Python reference for `sliding_window`
    and `alg_simhash`
- **Reference**: `reference/iscc-core/iscc_core/minhash.py` — Python reference for `alg_minhash_256`

## Implementation Notes

Follow the exact promotion pattern from the text utils step (see learnings):

1. **lib.rs module visibility** — change these two lines:

    - `pub(crate) mod simhash;` → `pub mod simhash;`
    - `pub(crate) mod minhash;` → `pub mod minhash;`

2. **lib.rs re-exports** — add flat crate-root re-exports after the existing `pub use utils::...`
    line:

    ```rust
    pub use minhash::alg_minhash_256;
    pub use simhash::{alg_simhash, sliding_window};
    ```

3. **simhash.rs** — change visibility on exactly two functions:

    - `pub(crate) fn alg_simhash(...)` → `pub fn alg_simhash(...)`
    - `pub(crate) fn sliding_window(...)` → `pub fn sliding_window(...)`
    - Keep `sliding_window_bytes` as `pub(crate)` — it's not a Tier 1 target

4. **minhash.rs** — change visibility on exactly one function:

    - `pub(crate) fn alg_minhash_256(...)` → `pub fn alg_minhash_256(...)`
    - Keep `minhash()` and `minhash_compress()` as private — they're internal helpers

5. **Integration tests** — create `test_algorithm_primitives.rs` with tests verifying:

    - **`sliding_window`**: basic n-grams ("hello" width 3 → ["hel", "ell", "llo"]), input shorter
        than width (returns single element), Unicode correctness (CJK characters work), width=2
        minimum
    - **`alg_simhash`**: empty input returns 32 zero bytes, single digest returns itself, multiple
        identical digests return same digest, different digests produce meaningful hash
    - **`alg_minhash_256`**: empty features produces 32 bytes, single feature produces 32 bytes,
        deterministic (same input → same output), different features produce different digests
    - Both import paths work: `iscc_lib::sliding_window` (flat) and
        `iscc_lib::simhash::sliding_window` (module path)
    - Verify `sliding_window_bytes` is NOT accessible via `iscc_lib::simhash::sliding_window_bytes` —
        this would be a compile-time check but since we can't do negative compile tests easily, just
        note it for manual verification

## Verification

- `cargo test -p iscc-lib` passes with all existing 165 tests + new integration tests (expect ~180+
    total)
- `cargo clippy -p iscc-lib -- -D warnings` is clean
- `iscc_lib::sliding_window`, `iscc_lib::alg_simhash`, and `iscc_lib::alg_minhash_256` are
    importable from crate root in integration tests
- `iscc_lib::simhash::sliding_window` and `iscc_lib::simhash::alg_simhash` are importable via module
    path
- `iscc_lib::minhash::alg_minhash_256` is importable via module path
- Internal helpers (`minhash`, `minhash_compress`, `sliding_window_bytes`) remain invisible outside
    the crate

## Done When

All verification criteria pass — the 3 algorithm functions are publicly accessible via both flat and
module-path imports, integration tests confirm correct behavior, and all existing tests still pass.
