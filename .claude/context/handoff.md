## 2026-02-23 — Promote alg_cdc_chunks to Tier 1 public API

**Done:** Promoted `alg_cdc_chunks` from `pub(crate)` to `pub` as the last of the 4 algorithm
primitives in the Tier 1 API. Added 7 integration tests covering basic chunking, empty input,
reassembly, UTF-32 alignment, different average sizes, and both import paths. This brings the
promoted symbol count from 16 to 17 of 22.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: changed `pub(crate) mod cdc` → `pub mod cdc`, added
    `pub use cdc::alg_cdc_chunks` re-export
- `crates/iscc-lib/src/cdc.rs`: changed `alg_cdc_chunks` from `pub(crate) fn` → `pub fn`
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: added 5 CDC behavior tests + 2 import path
    tests (flat and module path), updated module docstring and existing flat import test

**Verification:** All 188 tests pass (143 unit + 23 algorithm primitives + 22 text utils).
`cargo clippy --workspace --all-targets -- -D warnings` clean. `mise run check` clean (all
pre-commit hooks pass). Both `iscc_lib::alg_cdc_chunks` and `iscc_lib::cdc::alg_cdc_chunks` import
paths verified. Internal helpers (`alg_cdc_params`, `alg_cdc_offset`, `DATA_AVG_CHUNK_SIZE`) remain
`pub(crate)` and are not accessible from integration tests.

**Next:** The remaining Tier 1 symbols to promote are: `encode_base64`, `iscc_decompose`,
`DataHasher`/`InstanceHasher` streaming types, and `conformance_selftest`. The codec utilities
(`encode_base64`, `iscc_decompose`) are likely the next straightforward promotion targets.

**Notes:** Mechanical application of the established promotion pattern — no surprises or decisions
needed. All 4 algorithm primitives (sliding_window, alg_simhash, alg_minhash_256, alg_cdc_chunks)
are now part of the Tier 1 public API.
