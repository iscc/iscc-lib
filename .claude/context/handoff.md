## 2026-02-23 — Add WASM CI job to workflow

**Done:** Added a `wasm` job to `.github/workflows/ci.yml` that installs wasm-pack via the official
curl installer and runs `wasm-pack test --node crates/iscc-wasm`. All 4 binding crate jobs (rust,
python, nodejs, wasm) now run independently in parallel.

**Files changed:**

- `.github/workflows/ci.yml`: Added `wasm` job with checkout, rust-toolchain, rust-cache, wasm-pack
    install, and test steps

**Verification:** `cargo fmt --all --check` passes.
`cargo clippy --workspace --all-targets -- -D warnings` passes. YAML structure validated — all 4
jobs present, no `needs:` dependencies. Job follows the same structural pattern as existing jobs
(checkout → rust-toolchain → rust-cache → tool setup → test).

**Next:** With all 4 binding crates now under CI, the WASM bindings target criterion is complete.
Remaining target items include: C FFI bindings (`iscc-ffi` crate), documentation site, CI/CD
publishing pipelines (OIDC trusted publishing for crates.io and PyPI), and benchmark results
publication.

**Notes:** The curl installer (`rustwasm.github.io/wasm-pack/installer/init.sh`) pulls the latest
stable wasm-pack version. No pinning was added — this is consistent with how the other CI jobs work
(they use `@stable` toolchain without pinning a specific Rust version). If reproducibility becomes a
concern, the install step can be changed to `cargo install wasm-pack@0.14.0` at the cost of slower
CI (compiling from source).
