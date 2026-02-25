## 2026-02-25 — Fix WASM release build (wasm-opt bulk-memory)

**Done:** Added `[package.metadata.wasm-pack.profile.release]` section to
`crates/iscc-wasm/Cargo.toml` with
`wasm-opt = ["-O", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]`. This configures
wasm-opt to accept the WebAssembly post-MVP features that Rust's LLVM backend emits during release
builds.

**Files changed:**

- `crates/iscc-wasm/Cargo.toml`: Added `[package.metadata.wasm-pack.profile.release]` section with
    wasm-opt flags

**Verification:**

- `grep -q 'enable-bulk-memory' crates/iscc-wasm/Cargo.toml` — PASS
- `grep -q 'wasm-pack.profile.release' crates/iscc-wasm/Cargo.toml` — PASS
- `cargo check -p iscc-wasm` — PASS (Cargo.toml valid)
- `wasm-pack build --target web --release crates/iscc-wasm` — PASS (completed in 29.81s)
- `mise run check` — PASS (all 14 pre-commit hooks clean)

**Next:** The WASM release build is fixed. The next steps to unblock npm publishing are: (1) set up
`NPM_TOKEN` repository secret for @iscc org, (2) manually trigger the release workflow with npm
checkbox to publish @iscc/wasm and @iscc/lib, or re-tag a new version. The crates.io OIDC publishing
remains a separate human-gated task.

**Notes:** The original next.md anticipated only `--enable-bulk-memory` was needed (based on the
original `memory.copy` error). Testing locally revealed that Rust's LLVM also emits
`i32.trunc_sat_f64_s` and `i32.trunc_sat_f64_u` instructions (from float-to-integer conversions in
the DCT and codec code), which require the `--enable-nontrapping-float-to-int` feature. Both flags
were added explicitly rather than using `--enable-all`, following the principle of documenting
exactly which features are needed. If future Rust compiler versions emit additional post-MVP WASM
instructions, additional `--enable-*` flags can be added to this same section.
