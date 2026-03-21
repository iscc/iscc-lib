## 2026-03-21 — Review of: Add root Package.swift and fix Swift install documentation

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, well-scoped iteration adding a root `Package.swift` for SPM URL resolution and
updating all three Swift install docs to version 0.3.1 with honest build-from-source instructions.
All 11 verification criteria pass, all quality gates pass. One minor fix applied (wrong docs
hostname). The SPM install instructions issue is fully resolved; the native library vending issue is
addressed via documentation (downgraded to low).

**Verification:**

- [x] `test -f Package.swift` — root Package.swift exists
- [x] `head -1 Package.swift | grep -q 'swift-tools-version'` — valid Swift manifest header
- [x] `grep -q 'packages/swift/Sources/IsccLib' Package.swift` — paths point to subdirectory
- [x] `grep -q 'packages/swift/Sources/iscc_uniffiFFI' Package.swift` — FFI target path correct
- [x] `! grep -q 'testTarget' Package.swift` — no test target in root manifest
- [x] `grep -q '0.3.1' README.md` — Swift version updated
- [x] `! grep -q 'from: "0.3.0"' README.md` — old version removed
- [x] `! grep -q 'from: "0.3.0"' packages/swift/README.md` — old version removed
- [x] `! grep -q 'from: "0.3.0"' docs/howto/swift.md` — old version removed
- [x] `grep -q 'cargo build' packages/swift/README.md` — build-from-source documented
- [x] `grep -q 'cargo build' docs/howto/swift.md` — build-from-source documented
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- Fixed: Wrong docs hostname in README.md Swift section (`iscc-lib.iscc.io` → `lib.iscc.codes`)
- Deleted issue "Swift SPM install instructions are incorrect" — fully resolved
- Updated issue "Swift package does not vend the native library" — docs now in place, downgraded to
    `low`. XCFramework remains future work

**Codex review:** Three findings, all assessed:

1. **P1 — version 0.3.1 tag doesn't include root Package.swift**: Valid observation. The published
    `v0.3.1` tag predates this commit, so `from: "0.3.1"` won't resolve until the next release.
    This is expected — the docs are forward-looking for the next release. SPM `from:` means
    "minimum version" so it will work once a new tag is cut. Not actionable now.
2. **P2 — wrong docs hostname**: Valid and fixed. Changed `iscc-lib.iscc.io` to `lib.iscc.codes` in
    README.md.
3. **P2 — iOS 16+ claim vs host-only cargo build**: Valid pre-existing issue. The `iOS 16+` in
    packages/swift/README.md requirements predates this iteration. Building for iOS requires
    cross-compilation (XCFramework). Not in scope for this step.

**IDLE**: All remaining issues are low priority — no actionable work for CID.

**Next:** Both remaining issues are `low` priority (XCFramework distribution, docs site logos) — CID
should idle. The next actionable work would likely come from human direction: XCFramework binary
distribution for Swift, a new release cut (to include root Package.swift), or new target.md goals.

**Notes:**

- Root Package.swift adds 24 lines, mirrors subdirectory manifest with adjusted paths. Two manifests
    coexist without conflict — SPM reads root for dependency resolution, CI uses subdirectory
- The `from: "0.3.1"` version in all docs will only work after the next release tag is cut. The
    current v0.3.1 tag doesn't include the root Package.swift
- `packages/swift/README.md` still claims `iOS 16+` support but build instructions only cover macOS.
    iOS would require XCFramework distribution — tracked in the remaining low-priority issue
