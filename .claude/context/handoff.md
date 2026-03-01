## 2026-03-01 — Export META_TRIM_META in Node.js, WASM, and C FFI bindings

**Done:** Added `META_TRIM_META = 128_000` constant export to all three Rust-based binding crates
(Node.js/napi, WASM/wasm-bindgen, C FFI), following the exact pattern of the existing
`META_TRIM_DESCRIPTION` export in each crate. Added corresponding tests in all test files.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added `#[napi]` const `META_TRIM_META`
- `crates/iscc-wasm/src/lib.rs`: Added `#[wasm_bindgen]` getter function `meta_trim_meta()`
- `crates/iscc-ffi/src/lib.rs`: Added `extern "C"` getter function `iscc_meta_trim_meta()` + inline
    Rust test
- `crates/iscc-napi/__tests__/functions.test.mjs`: Added `META_TRIM_META` import and describe block
    (value=128000, type='number')
- `crates/iscc-wasm/tests/unit.rs`: Added `wasm_bindgen_test` for `meta_trim_meta()` value
- `crates/iscc-ffi/tests/test_iscc.c`: Added `ASSERT_EQ` for `iscc_meta_trim_meta() == 128000`

**Verification:**

- `cargo test -p iscc-napi`: compiles clean (0 Rust-side tests, as expected)
- `cargo test -p iscc-ffi`: 78 tests passed (77 existing + 1 new `test_meta_trim_meta`)
- `cargo test -p iscc-wasm`: compiles clean (wasm_bindgen_test requires wasm-pack runtime)
- `cargo clippy -p iscc-napi -- -D warnings`: clean
- `cargo clippy -p iscc-wasm -- -D warnings`: clean
- `cargo clippy -p iscc-ffi -- -D warnings`: clean
- `mise run check`: all 14 pre-commit hooks passed

**Next:** Export `META_TRIM_META` in the remaining 2 binding crates: Java (JNI) and Go. Go already
has `MetaTrimMeta` patterns in `codec.go` — verify if it needs updating. Java needs a new JNI
constant in the Java binding class.

**Notes:** Issue #18 is now 4/5 tasks done (Rust core ✅, validation ✅, Python ✅, Node.js/WASM/C FFI
✅). Only Java and Go remain. The NAPI JS tests and C tests can only run in their respective build
environments (Node.js with native addon, C with compiled shared library) — the Rust compilation and
clippy checks confirm correctness at the Rust level.
