## 2026-02-23 — Promote text utilities to Tier 1 public API

**Done:** Changed 4 text utility functions (`text_clean`, `text_remove_newlines`, `text_trim`,
`text_collapse`) from `pub(crate)` to `pub` in `utils.rs`, made the `utils` module public in
`lib.rs`, and added flat re-exports at the crate root. Created 22 dedicated integration tests
verifying behavior and public accessibility.

**Files changed:**

- `crates/iscc-lib/src/utils.rs`: Changed visibility of 4 functions from `pub(crate)` to `pub`,
    enhanced docstrings to better match Python reference descriptions
- `crates/iscc-lib/src/lib.rs`: Changed `pub(crate) mod utils` to `pub mod utils`, added
    `pub use utils::{text_clean, text_collapse, text_remove_newlines, text_trim}`
- `crates/iscc-lib/tests/test_text_utils.rs`: New file with 22 integration tests covering all 4
    functions plus crate-root and module-path import verification

**Verification:** All 165 tests pass (143 existing + 22 new). `cargo clippy -D warnings` clean.
`cargo doc --no-deps` succeeds. All 4 functions are importable via both `iscc_lib::text_clean` and
`iscc_lib::utils::text_clean`. All pre-commit hooks pass. No existing behavior changed.

**Next:** Continue Tier 1 API expansion — promote algorithm primitives (`sliding_window`,
`alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), then `encode_base64`, `iscc_decompose`,
streaming types (`DataHasher`, `InstanceHasher`), and `conformance_selftest`. The `simhash` and
`minhash` modules follow the same `pub(crate)` → `pub` promotion pattern used here.

**Notes:** The `utils` module also contains `multi_hash_blake3`, `is_c_category`, and
`is_cmp_category` which remain at their original visibility (private or `pub(crate)`). Only the 4
text utility functions were promoted per the work package scope. The `multi_hash_blake3` function is
`pub(crate)` and the helper functions `is_c_category`/`is_cmp_category` are private — these should
be evaluated in a future step if they belong in the public API.
