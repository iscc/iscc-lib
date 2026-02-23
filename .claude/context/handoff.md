## 2026-02-23 — Review of: Promote text utilities to Tier 1 public API

**Verdict:** PASS

**Summary:** Clean visibility promotion of 4 text utility functions from `pub(crate)` to `pub`, with
module-level `pub mod utils` and flat crate-root re-exports. 22 integration tests cover all
specified scenarios including edge cases (UTF-8 boundary truncation, NFKC fullwidth chars, CRLF,
diacritics). All 165 tests pass, clippy clean, docs generate, no gate circumvention.

**Issues found:**

- (none)

**Next:** Continue Tier 1 API expansion — promote algorithm primitives from `simhash`, `minhash`,
and `cdc` modules. The next batch should be `sliding_window` and `alg_simhash` (from `simhash`),
`alg_minhash_256` (from `minhash`), and `alg_cdc_chunks` (from `cdc`). These follow the same
`pub(crate)` → `pub` promotion pattern established here. After algorithm primitives, the remaining
Tier 1 symbols are: `encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher` streaming
types, and `conformance_selftest`.

**Notes:** The `utils` module is now `pub`, but `multi_hash_blake3` remains `pub(crate)` (invisible
outside the crate) and `is_c_category`/`is_cmp_category` remain private. No leakage. When promoting
the algorithm modules (`simhash`, `minhash`, `cdc`), the same pattern applies: make the module
`pub`, promote specific functions to `pub`, add `pub use` re-exports at crate root. The binding
crates will also need wrappers for the newly public symbols in a future step.
