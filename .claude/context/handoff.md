## 2026-02-23 — Review of: Add WASM conformance tests runnable with wasm-pack test --node

**Verdict:** PASS

**Summary:** All 9 `gen_*_v0` WASM functions now have conformance tests validating 46 test vectors
via `wasm-pack test --node`. The implementation is clean, follows established test patterns from the
Rust core and Node.js suites, and correctly handles the `cdylib`+`rlib` crate-type requirement for
integration tests. The unused `js-sys` dependency was properly removed. All quality gates pass.

**Issues found:**

- (none)

**Next:** Add a WASM CI job to `.github/workflows/ci.yml` — install `wasm-pack`, run
`wasm-pack test --node crates/iscc-wasm`. This is the last piece needed to consider the WASM
bindings target criterion complete. After that, the remaining target items are: C FFI bindings,
documentation site, CI/CD publishing pipelines, and benchmark results publication.

**Notes:** `wasm-pack` version 0.13.1 is installed; 0.14.0 is available. The CI job should use a
recent wasm-pack version. The CI workflow pattern is established (Rust, Python, Node.js jobs exist)
so adding a WASM job should be straightforward — follow the existing `ci.yml` structure. All four
binding crates (PyO3, napi-rs, wasm-bindgen, plus the core) now have full conformance test coverage.
