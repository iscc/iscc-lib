## 2026-02-25 — Review of: Fix WASM release build (wasm-opt bulk-memory)

**Verdict:** PASS

**Summary:** The advance agent added a `[package.metadata.wasm-pack.profile.release]` section to
`crates/iscc-wasm/Cargo.toml` with two wasm-opt feature flags. The change is minimal (3 lines of
TOML), correctly scoped, and verified end-to-end with a successful `wasm-pack build --release`. The
additional `--enable-nontrapping-float-to-int` flag (beyond what next.md anticipated) was a
justified discovery during testing.

**Verification:**

- [x] `grep -q 'enable-bulk-memory' crates/iscc-wasm/Cargo.toml` exits 0 — PASS
- [x] `grep -q 'wasm-pack.profile.release' crates/iscc-wasm/Cargo.toml` exits 0 — PASS
- [x] `cargo check -p iscc-wasm` passes — Cargo.toml valid
- [x] `wasm-pack build --target web --release crates/iscc-wasm` succeeds — completed in 29.36s
- [x] `mise run check` passes — all 14 pre-commit hooks clean
- [x] No quality gate circumvention — diff contains only Cargo.toml config + context files

**Issues found:**

- (none)

**Next:** The WASM release build is now fixed on `develop`. To get this fix into a release: (1)
merge `develop` → `main` via PR, (2) tag and push a new version (or re-trigger the release workflow
manually). Human-gated tasks remain: configure `NPM_TOKEN` repository secret, configure crates.io
OIDC trusted publishing. If no new target.md goals have been added, the next CID iteration could
address the low-priority TypeScript port evaluation (issues.md) or wait for new direction from the
human.

**Notes:** The advance agent discovered that `--enable-nontrapping-float-to-int` is also needed
(Rust's LLVM emits `i32.trunc_sat_f64_s/u` from DCT/codec float-to-int conversions). This is
documented in both the handoff and agent memory. The explicit flag approach (vs `--enable-all`) is
preferred for traceability.
