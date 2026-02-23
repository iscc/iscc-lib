## 2026-02-23 — Promote simhash and minhash modules to Tier 1

**Done:** Promoted `sliding_window`, `alg_simhash` (simhash module), and `alg_minhash_256` (minhash
module) to public Tier 1 API. Changed module visibility from `pub(crate)` to `pub` for both modules,
promoted 3 functions from `pub(crate)` to `pub`, and added flat `pub use` re-exports at crate root.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Changed `pub(crate) mod simhash` → `pub mod simhash` and
    `pub(crate) mod minhash` → `pub mod minhash`; added `pub use minhash::alg_minhash_256` and
    `pub use simhash::{alg_simhash, sliding_window}` re-exports
- `crates/iscc-lib/src/simhash.rs`: Changed `alg_simhash` and `sliding_window` from `pub(crate)` to
    `pub`; kept `sliding_window_bytes` as `pub(crate)`
- `crates/iscc-lib/src/minhash.rs`: Changed `alg_minhash_256` from `pub(crate)` to `pub`; kept
    `minhash()` and `minhash_compress()` as private
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: Created 17 integration tests covering
    sliding_window (5 tests), alg_simhash (5 tests), alg_minhash_256 (4 tests), and import path
    verification (3 tests)

**Verification:** All 182 tests pass (143 unit + 17 algorithm integration + 22 text integration).
Clippy clean with `-D warnings`. All pre-commit hooks pass. Both flat (`iscc_lib::sliding_window`)
and module-path (`iscc_lib::simhash::sliding_window`) imports verified working. Internal helpers
(`minhash`, `minhash_compress`, `sliding_window_bytes`) remain invisible outside the crate.

**Next:** Promote `alg_cdc_chunks` from the `cdc` module to Tier 1 using the same pattern. After
that, the remaining Tier 1 symbols are: `encode_base64`, `iscc_decompose`,
`DataHasher`/`InstanceHasher` streaming types, and `conformance_selftest`.

**Notes:** The `alg_simhash` function uses `impl AsRef<[u8]>` in argument position, which means
turbofish syntax (`alg_simhash::<Vec<u8>>`) cannot be used — callers must provide type-annotated
slices instead. This is fine for the public API but worth noting for documentation.
