# Handoff

## 2026-07-24 — Enable WASM simd128 in the @iscc/wasm release + CI builds (issue #42)

**Done:** Pure build-flag change wiring WASM SIMD through the pipeline: step-level
`RUSTFLAGS: -C target-feature=+simd128` on the release `build-wasm` step and the CI `wasm` job's
test step, plus `--enable-simd` in the `wasm-opt` array so binaryen accepts the `v128` instructions
during the release build. No source changes (blake3 selects its wasm32 SIMD backend at compile time
via `target_feature = "simd128"`).

**Files changed:**

- `.github/workflows/release.yml`: `build-wasm` job's "Build WASM package" step gained
    `env: RUSTFLAGS: -C target-feature=+simd128`. `test-wasm` untouched (it smoke-tests the
    downloaded artifact, no rebuild).
- `.github/workflows/ci.yml`: `wasm` job's "Run tests" step gained the same step-level `env`.
- `crates/iscc-wasm/Cargo.toml`: `wasm-opt` array now
    `["-O3", "--enable-simd", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]` (taplo
    reformatted it to multi-line, matching the spec example in `specs/wasm-bindings.md` exactly).
- `crates/iscc-wasm/CLAUDE.md` (doc): release-profile paragraph updated to quote the new wasm-opt
    array and explain the RUSTFLAGS/simd128 mechanism.

**Verification:**

- All four greps pass: `target-feature=+simd128` in release.yml and ci.yml, `enable-simd` in
    Cargo.toml and CLAUDE.md.
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    — conformance 9/9 passed, unit 78/78 passed (byte-identical output under SIMD compile).
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release crates/iscc-wasm --features conformance`
    — exit 0, `wasm-opt` ran and accepted the SIMD input ("Your wasm pkg is ready to publish").
- `wasm-tools print crates/iscc-wasm/pkg/iscc_wasm_bg.wasm | grep -c 'v128'` → **1993** (spec
    "Verified when" #4 disassembly evidence; `wasm-tools` installed via
    `cargo binstall wasm-tools`).
- `mise run format` + `mise run check` — all 15 pre-commit hooks pass; tree contains only the four
    intended files (plus iterations.jsonl from the runner).

**Next:** Remaining v0.6.0 backlog, one per iteration: #43 Go ISCC-IDv1 encode/decode, "Restore
linux/aarch64 Python wheels", "Dependency review and refresh", and the two release-infra items
("Migrate npm publishing to OIDC", "Fix broken single-registry re-trigger"). #43 is the largest
remaining feature; the aarch64-wheels item is the most self-contained next pick.

**Notes:**

- Spec "Verified when" boxes in `specs/wasm-bindings.md` → "WASM SIMD (`simd128`)" left unchecked
    per next.md — review agent owns check-offs. Evidence for all four is above.
- The built `pkg/` directory is self-gitignored (wasm-pack writes `pkg/.gitignore` containing `*`) —
    no stray artifacts to manage.
- No `iscc-wasm` source changes, no new tests: next.md scoped this as a build-flag-only change and
    the existing 87 wasm tests exercised under the SIMD compile are the verification.
- Spec note "Applies to every published target (`web`, plus `bundler` if published)" — only `web` is
    published today, and it is covered. No bundler build exists in the workflows.
