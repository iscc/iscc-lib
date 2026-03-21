## 2026-03-21 — Review of: Create XCFramework build script and restructure root Package.swift

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding `scripts/build_xcframework.sh` and
restructuring root `Package.swift` with the Ferrostar-style variable toggle pattern. Both files
faithfully follow the spec at `.claude/context/specs/swift-bindings.md`. All 15 verification
criteria pass, all quality gates clean, no suppressions or scope violations.

**Verification:**

- [x] `test -x scripts/build_xcframework.sh` — script exists and is executable
- [x] `bash -n scripts/build_xcframework.sh` — valid shell syntax
- [x] `grep -q 'aarch64-apple-darwin'` — macOS arm64 target present
- [x] `grep -q 'x86_64-apple-ios'` — iOS simulator x86_64 target present
- [x] `grep -q 'xcodebuild -create-xcframework'` — XCFramework assembly present
- [x] `grep -q 'ditto'` — ditto zip present
- [x] `grep -q 'swift package compute-checksum'` — checksum computation present
- [x] `grep -q 'useLocalFramework' Package.swift` — variable toggle present
- [x] `grep -q 'binaryTarget' Package.swift` — uses binary target
- [x] `grep -q 'releaseTag' Package.swift` — release tag variable present
- [x] `grep -q 'releaseChecksum' Package.swift` — checksum variable present
- [x] `grep -q '.macOS' Package.swift` — platform constraints present
- [x] `diff packages/swift/Package.swift` — development Package.swift unchanged
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `cargo test -p iscc-lib` — all tests pass
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** Two findings, both about the intentional intermediate state:

1. **P1** (root Package.swift defaults to nonexistent remote XCFramework): Expected — the
    `PLACEHOLDER` checksum is temporary. The root Package.swift on `develop` isn't consumed by any
    tagged release yet. The v0.3.1 tag still has the old Package.swift. The release workflow (next
    step) will replace the placeholder with a real checksum and upload the XCFramework.
2. **P2** (`releaseTag` not in version sync): Explicitly deferred per next.md's "Not In Scope" —
    version sync entry is a planned follow-up step.

Both findings are valid observations about intermediate state, not defects.

**Next:** Continue the Swift XCFramework multi-step effort. Two remaining steps:

1. **Release workflow integration** (higher priority): Add `build-xcframework` job to `release.yml`
    with: macOS runner, XCFramework build + zip, `sed` to update `releaseTag`/`releaseChecksum` in
    Package.swift, auto-commit, force-update tag, upload to GitHub Release, `swift` checkbox input.
    Reference: spec section "Release Workflow Integration".
2. **Version sync**: Add `releaseTag` entry to `version_sync.py` so version bumps propagate to
    `Package.swift`. The `releaseChecksum` is CI-only (not in version sync).

After both: update `docs/howto/swift.md` to document zero-friction SPM install.

**Notes:**

- The Swift vend issue in issues.md stays open — this iteration completed step 1 (build script +
    Package.swift restructure) of a multi-step effort
- Script cannot be tested on Linux (requires macOS toolchain) — `bash -n` is the maximum
    verification possible. Real validation happens when the release workflow runs on macOS CI
- `releaseTag` is set to `"0.3.1"` (current version) and `releaseChecksum` to `"PLACEHOLDER"` — CI
    will replace both during the first release that includes Swift
