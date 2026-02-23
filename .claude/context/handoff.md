## 2026-02-23 — Create C FFI crate with extern "C" wrappers and cbindgen

**Done:** Created the `iscc-ffi` crate exposing all 9 `gen_*_v0` functions plus `iscc_free_string()`
and `iscc_last_error()` as `extern "C"` symbols. Includes cbindgen configuration that generates a
valid C header with all 11 exported symbols. The crate uses a thread-local error model (NULL return
\+ `iscc_last_error()`) and heap-allocated C strings freed by the caller.

**Files changed:**

- `crates/iscc-ffi/Cargo.toml`: new crate with `cdylib` + `staticlib` types, depends on `iscc-lib`
- `crates/iscc-ffi/cbindgen.toml`: cbindgen config with `ISCC_H` include guard and `iscc_` prefix
- `crates/iscc-ffi/src/lib.rs`: 11 `extern "C"` functions with null-pointer validation, UTF-8
    conversion, thread-local error storage, and 20 unit tests
- `Cargo.toml`: added `crates/iscc-ffi` to workspace members

**Verification:**

- `cargo build -p iscc-ffi` — succeeds (both cdylib and staticlib)
- `cargo clippy -p iscc-ffi -- -D warnings` — clean (0 warnings)
- `cargo fmt -p iscc-ffi --check` — clean
- `cbindgen --crate iscc-ffi` — generates valid C header with all 11 symbols
- `cargo test -p iscc-ffi` — 20 tests pass (null safety, known-value, error handling)
- `cargo test -p iscc-lib` — 143 existing tests still pass (no regression)

**Next:** Add a CI job for iscc-ffi in `.github/workflows/ci.yml` (build + test + cbindgen header
generation). After that, consider CI/CD publishing pipelines (OIDC trusted publishing for crates.io
and PyPI) or documentation site setup.

**Notes:** `iscc_last_error()` is not marked `unsafe` (it's a safe `extern "C"` fn) since it only
reads thread-local state via `RefCell::borrow()`. All other exported functions are
`unsafe extern "C"` because they dereference raw pointers. The helper functions `ptr_to_str` and
`ptr_to_optional_str` centralize null-check + UTF-8 validation to avoid repetition across the 9 gen
functions.
