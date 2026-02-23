# Next Work Package

## Step: Promote alg_cdc_chunks to Tier 1 public API

## Goal

Promote `alg_cdc_chunks` from `pub(crate)` to `pub` as the last of the 4 algorithm primitives in the
Tier 1 API. This brings the promoted symbol count from 16 to 17 of 22 and completes the algorithm
primitives group.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` (change `pub(crate) mod cdc` → `pub mod cdc`, add
    `pub use cdc::alg_cdc_chunks` re-export), `crates/iscc-lib/src/cdc.rs` (change `alg_cdc_chunks`
    from `pub(crate) fn` → `pub fn`), `crates/iscc-lib/tests/test_algorithm_primitives.rs` (add CDC
    integration tests)
- **Reference**: `crates/iscc-lib/src/cdc.rs` (current implementation),
    `crates/iscc-lib/tests/test_algorithm_primitives.rs` (existing test pattern),
    `crates/iscc-lib/src/lib.rs` (re-export pattern)

## Implementation Notes

Apply the established Tier 1 promotion pattern (used 3 times already for utils, simhash, minhash):

1. In `lib.rs`: change `pub(crate) mod cdc;` → `pub mod cdc;`
2. In `lib.rs`: add `pub use cdc::alg_cdc_chunks;` to the re-exports block
3. In `cdc.rs`: change `pub(crate) fn alg_cdc_chunks` → `pub fn alg_cdc_chunks`
4. Keep `alg_cdc_params`, `alg_cdc_offset`, and `DATA_AVG_CHUNK_SIZE` as `pub(crate)` — these are
    internal helpers, not part of the Tier 1 API. Since `cdc` becomes a `pub` module, `pub(crate)`
    items remain invisible outside the crate (no leakage risk — confirmed in learnings).

Add integration tests to `test_algorithm_primitives.rs` following the existing pattern:

- **Basic chunking**: verify `alg_cdc_chunks` returns expected chunk count for known data
- **Empty input**: verify it returns a single empty chunk (matches existing unit test, but via
    public API path)
- **Reassembly**: verify chunks concatenate back to original data
- **UTF-32 alignment**: verify `utf32=true` produces 4-byte-aligned chunk boundaries
- **Different avg sizes**: verify smaller avg_chunk_size produces more chunks
- **Flat import path**: verify `iscc_lib::alg_cdc_chunks` works
- **Module path import**: verify `iscc_lib::cdc::alg_cdc_chunks` works

Run workspace-wide clippy (`cargo clippy --workspace --all-targets -- -D warnings`) before
finishing, per learnings about newer lints only surfacing in `--all-targets` mode.

## Verification

- `cargo test -p iscc-lib` passes (all existing 182 tests + new CDC integration tests)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `iscc_lib::alg_cdc_chunks` is callable from integration tests (flat import)
- `iscc_lib::cdc::alg_cdc_chunks` is callable from integration tests (module path)
- Internal helpers (`alg_cdc_params`, `alg_cdc_offset`, `DATA_AVG_CHUNK_SIZE`) remain invisible
    outside the crate

## Done When

All verification criteria pass: tests green, clippy clean, `alg_cdc_chunks` accessible via both
import paths, and internal helpers remain properly encapsulated.
