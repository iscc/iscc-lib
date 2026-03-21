## 2026-03-21 — Review of: Add Swift CI job on macOS runner

**Verdict:** PASS

**Summary:** The advance agent added a clean 21-line `swift:` CI job to `ci.yml` that builds
`iscc-uniffi` and runs `swift build` + `swift test` on `macos-14` with correct `-Xlinker` flags for
library discovery. The implementation matches all next.md requirements exactly, touches only the
specified file, and follows established CI job patterns (debug build, `actions/checkout@v4`,
`dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`).

**Verification:**

- [x] `grep -q 'macos-14' .github/workflows/ci.yml` exits 0 — PASS
- [x] `grep -q 'swift test' .github/workflows/ci.yml` exits 0 — PASS
- [x] `grep -q 'cargo build -p iscc-uniffi' .github/workflows/ci.yml` exits 0 — PASS
- [x] YAML validation (`yaml.safe_load`) exits 0 — PASS
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- [x] `mise run check` — 15/15 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex launched but produced no output (0 bytes after 5+ minutes on a 21-line YAML
diff). Skipped — the diff is trivial and fully validated by manual review.

**Next:** Push to verify the Swift CI job passes on GitHub Actions — this is the first real
execution of the Swift conformance tests. If CI is green, the next work package should be one of:

1. **Swift docs + README integration** — `docs/howto/swift.md` how-to guide, README Swift
    install/quickstart tab, `packages/swift/CLAUDE.md`, version sync (`Constants.swift` +
    `version_sync.py`)
2. If the CI job fails (e.g., `-rpath` doesn't work for XCTest runner), fix it — add
    `DYLD_LIBRARY_PATH` env fallback to the test step

After Swift is fully complete (CI green, docs done), the Swift issue in issues.md can be closed and
Kotlin bindings can begin.

**Notes:**

- CI job count is now 15 (14 named jobs + 1 python status gate). The `swift:` job is placed between
    `cpp:` and `bench:`, keeping binding jobs grouped
- Swift tests cannot be validated locally (Linux devcontainer) — the first CI run on GitHub is the
    real validation. The `-rpath` approach is preferred over `DYLD_LIBRARY_PATH` because SIP strips
    `DYLD_*` vars on macOS
- The spec's example CI YAML included `--release`, cross-compile targets, and a
    `uniffi-bindgen   generate` step — all correctly omitted by the advance agent (debug build
    matches other CI jobs, cross-compile is for XCFramework release which is out of scope, and
    bindings are already committed to git)
