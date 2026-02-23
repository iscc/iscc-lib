## 2026-02-23 — Add Node.js CI job to workflow

**Done:** Added a `nodejs` job to `.github/workflows/ci.yml` that builds the napi native addon and
runs the 46 JavaScript conformance tests. The job runs in parallel with the existing `rust` and
`python` jobs.

**Files changed:**

- `.github/workflows/ci.yml`: Added `nodejs` job with checkout, Rust toolchain + cache, Node.js 20
    setup, npm install, `npx napi build --platform`, and `npm test` steps

**Verification:**

- `cargo test -p iscc-lib`: 143 passed, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- YAML structure validated: 3 jobs (rust, python, nodejs), correct actions, no `needs:` dependency,
    `working-directory` used for all napi steps

**Next:** The CI workflow now covers all three language targets. Next steps could be: WASM bindings
(`@iscc/wasm` via wasm-bindgen), C FFI (cbindgen headers), documentation site, or CI/CD publishing
pipelines.

**Notes:** Used `npx napi build --platform` (debug mode) rather than `npm run build` (which uses
`--release`) for faster CI builds. The debug build is sufficient for conformance test verification.
The `working-directory` approach keeps the YAML clean per GitHub Actions best practices.
