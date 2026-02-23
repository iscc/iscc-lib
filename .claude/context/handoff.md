## 2026-02-23 — Add npm publishing jobs to release workflow

**Done:** Added 4 new jobs to `release.yml` for building and publishing `@iscc/lib` (napi-rs native
addon) and `@iscc/wasm` (wasm-bindgen package) to npm. Updated `package.json` with cross-platform
triples configuration.

**Files changed:**

- `.github/workflows/release.yml`: Added `build-napi` (5-target matrix: linux-x64, linux-arm64,
    macos-arm64, macos-x64, windows-x64), `publish-npm-lib` (downloads artifacts, runs napi
    prepublish, npm publish), `build-wasm` (wasm-pack build + package.json name/version patching),
    `publish-npm-wasm` (downloads WASM pkg artifact, npm publish). All publish jobs gated with
    `startsWith(github.ref, 'refs/tags/v')`, use `NPM_TOKEN` secret, and have `id-token: write`
    permission for provenance attestation.
- `crates/iscc-napi/package.json`: Added `"defaults": true` and
    `"additional": ["aarch64-unknown-linux-gnu"]` to napi triples configuration.

**Verification:** All verification criteria from next.md pass:

- YAML valid, all 8 jobs present (4 existing + 4 new)
- napi build matrix covers linux-x64, linux-arm64, macos-arm64, macos-x64, windows-x64
- WASM build uses `wasm-pack build --target web --release`
- npm publish uses `--access public` and `--provenance`
- `NPM_TOKEN` secret referenced, `id-token: write` on publish jobs
- Publish jobs have tag guard (`startsWith(github.ref, 'refs/tags/v')`)
- Existing crates.io and PyPI jobs unchanged
- npm jobs independent of crates.io/PyPI (parallel execution)
- package.json has platform triples configured
- `cargo clippy`, `cargo fmt --check`, `cargo test` all pass

**Next:** The target state notes that the only remaining gap after this step is GitHub Pages
enablement (admin action, not code). The project is at code-complete status. Consider running the
full release workflow via `workflow_dispatch` (build-only, publish jobs skipped) to validate the CI
configuration end-to-end before the first real release.

**Notes:** The aarch64-linux cross-compilation uses `gcc-aarch64-linux-gnu` installed via apt-get,
with `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER` env var set on the build step (harmlessly
ignored on non-arm64 targets). The WASM version patching uses a Node.js heredoc script to read the
workspace version from root `Cargo.toml` and set it in the wasm-pack generated `package.json`. The
`napi prepublish -t npm` command in the publish job organizes platform-specific `.node` binaries
into npm packages before publishing.
