## 2026-02-23 — Add C test program and CI job for iscc-ffi

**Done:** Created a self-contained C test program (`test_iscc.c`) that links against the `iscc-ffi`
shared library and validates 11 test cases covering all major FFI entrypoints. Added a `c-ffi` CI
job to the GitHub Actions workflow that builds the FFI crate, generates the C header via cbindgen,
compiles the test program with gcc, and runs it.

**Files changed:**

- `crates/iscc-ffi/tests/test_iscc.c`: C test program with 11 tests covering meta, text, image,
    instance, data code generation, error handling (NULL input), error clearing on success, and
    `iscc_free_string(NULL)` no-op safety
- `.github/workflows/ci.yml`: Added `c-ffi` job after the existing `wasm` job — installs cbindgen,
    builds `iscc-ffi`, generates header, compiles test with gcc, runs with `LD_LIBRARY_PATH`

**Verification:** All tests pass locally:

- C test program: 11 passed, 0 failed (meta x2, text, image, instance, data, error NULL, error
    non-NULL, error cleared, success after error, free NULL)
- `cargo test --workspace`: 163 passed, 0 failed (no regression)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo fmt --all --check`: clean

**Next:** Update `state.md` to reflect C test program completion and 5th CI job. Then tackle one of:
pytest-benchmark for Rust bindings (iscc_lib vs iscc-core comparison), documentation site
(lib.iscc.codes), or CI/CD publishing pipelines.

**Notes:** The C header is generated at CI time via `cbindgen` (not committed) — this is the
standard approach to avoid staleness. The test program uses `uint8_t empty = 0; &empty` for the
empty-data instance code test to provide a non-null pointer with zero length, matching the Rust test
behavior. System libraries `-lpthread -ldl -lm` are included in the gcc link flags for portability
across Linux variants.
