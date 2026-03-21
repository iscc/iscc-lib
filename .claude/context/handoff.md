## 2026-03-21 — Add Swift CI job on macOS runner

**Done:** Added a `swift:` CI job to `.github/workflows/ci.yml` that runs on `macos-14` (Apple
Silicon M1). The job builds the UniFFI native library (`cargo build -p iscc-uniffi`), builds the
Swift package (`swift build` with linker flags), and runs conformance tests (`swift test` with
`-rpath` for runtime library discovery).

**Files changed:**

- `.github/workflows/ci.yml`: Added `swift:` job between `cpp:` and `bench:` jobs. Uses `macos-14`
    runner, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`, debug build, `-Xlinker -L`
    for link-time library path, `-Xlinker -rpath` for runtime library path. CI now has 15 jobs
    total.

**Verification:**

- `grep -q 'macos-14' .github/workflows/ci.yml` — PASS
- `grep -q 'swift test' .github/workflows/ci.yml` — PASS
- `grep -q 'cargo build -p iscc-uniffi' .github/workflows/ci.yml` — PASS
- YAML validation via `yaml.safe_load()` — PASS
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — PASS (clean)
- `mise run check` — 15/15 hooks pass

**Next:** Push to `develop` and verify the Swift CI job passes on GitHub Actions. If `-rpath` alone
doesn't work for the XCTest runner, add `DYLD_LIBRARY_PATH` env fallback. After CI is green:
`docs/howto/swift.md`, README Swift sections, version sync integration (`Constants.swift` +
`version_sync.py`), and `packages/swift/CLAUDE.md`.

**Notes:**

- The CI job count is now 15 (was 14 after cpp was added; memory says 13 which is stale — it was 13
    before dotnet+cpp were added). Jobs: version-check, rust, python-test, python, nodejs, wasm,
    c-ffi, dotnet, java, go, ruby, cpp, swift, bench = 14 named jobs + python status gate = 15 total
- Swift tests cannot be validated locally (Linux devcontainer). The first CI run on GitHub will be
    the real validation. The `-rpath` approach is preferred over `DYLD_LIBRARY_PATH` because SIP can
    strip `DYLD_*` vars on macOS
- No tests written locally since the only change is a CI workflow YAML addition — verification is
    via grep checks and YAML parsing
