## 2026-02-23 — Implement conformance_selftest diagnostic function

**Done:** Implemented `conformance_selftest()` as a public Tier 1 API function in a dedicated
`conformance.rs` module. The function runs all 9 gen functions against every vendored conformance
vector from `data.json` and returns `true` if all produce correct output. This completes the Rust
core Tier 1 API (23/23 symbols).

**Files changed:**

- `crates/iscc-lib/src/conformance.rs`: Created module with `conformance_selftest()` public function
    and 9 private helper functions (`run_meta_tests`, `run_text_tests`, etc.), one per gen function.
    Each helper parses inputs from JSON, calls the gen function, compares `.iscc` output, and
    returns `false` on mismatch or error (with `eprintln!` diagnostics). Includes a unit test
    asserting the selftest passes. Uses `Option`-based error handling (closures returning
    `Option<bool>`) to avoid panics — any parse failure or gen function error counts as failure.
- `crates/iscc-lib/src/lib.rs`: Added `pub mod conformance;` and
    `pub use conformance::conformance_selftest;` re-export for flat crate-root imports.

**Verification:** All 230 tests pass (180 unit + 28 algo + 22 text), including the new
`conformance::tests::test_conformance_selftest_passes`. Clippy clean workspace-wide
(`--workspace --all-targets`). All pre-commit hooks pass. `iscc_lib::conformance_selftest` is
accessible as a public API symbol via the crate root re-export.

**Next:** All 23 Tier 1 symbols are implemented. `state.md` should be updated to reflect this
milestone. Remaining work includes adding `conformance_selftest` wrappers to all binding crates
(Python, Node.js, WASM, C FFI) and updating documentation. The streaming types (`DataHasher`,
`InstanceHasher`) also need binding wrappers.

**Notes:** The implementation uses per-function helpers rather than a generic dispatch approach
because each gen function has a unique signature and input decoding logic. The `gen_iscc_code_v0`
conformance vectors only include the codes array (no `wide` parameter), so `false` is passed for
wide mode (matching the existing test pattern). A `decode_stream` helper centralizes the
`"stream:<hex>"` parsing shared by data and instance tests.
