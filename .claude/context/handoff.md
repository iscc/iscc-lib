## 2026-03-21 — Create XCFramework build script and restructure root Package.swift

**Done:** Created `scripts/build_xcframework.sh` — a portable bash script that cross-compiles
`iscc-uniffi` for 5 Apple targets, creates fat binaries via `lipo`, assembles an XCFramework via
`xcodebuild`, zips with `ditto`, and prints the SHA256 checksum. Restructured root `Package.swift`
to use the Ferrostar-style variable toggle pattern with `useLocalFramework`, `releaseTag`, and
`releaseChecksum`, replacing the old `.target`/`.linkedLibrary` approach with `.binaryTarget`.

**Files changed:**

- `scripts/build_xcframework.sh`: New file — complete XCFramework build pipeline (cross-compile,
    lipo, stage headers, xcodebuild, ditto zip, compute-checksum)
- `Package.swift`: Rewritten — variable toggle (`useLocalFramework`), conditional `binaryTarget`
    (local path vs GitHub Releases URL), platform constraints (macOS 13, iOS 16), placeholder
    `releaseTag`/`releaseChecksum`

**Verification:**

- All 12 grep-based verification criteria pass (5 script checks + 5 Package.swift checks + unchanged
    dev Package.swift + platform check)
- `bash -n` syntax check passes
- Script is executable (`chmod +x`)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `cargo test -p iscc-lib` — 317 tests pass (261 unit + 31 integration + 24 text utils + 1 doctest)
- `mise run check` — all 15 pre-commit hooks pass

**Next:** Two follow-up steps remain before Swift XCFramework is end-to-end functional: (1) Add
`build-xcframework` job to `release.yml` with force-update tag pattern and XCFramework upload, and
add `swift` checkbox input. (2) Add `releaseTag` entry to `version_sync.py` so version bumps
propagate to `Package.swift`. Documentation update (`docs/howto/swift.md`) should follow after the
release workflow is verified working on macOS CI.

**Notes:**

- `releaseTag` placeholder set to `"0.3.1"` (current version) and `releaseChecksum` to
    `"PLACEHOLDER"` — these will be updated by CI during the first release that includes Swift
- The script cannot be tested in the Linux devcontainer (requires macOS toolchain: `lipo`,
    `xcodebuild`, `ditto`, `swift`). Syntax validation via `bash -n` is the maximum verification
    possible here
- The root `Package.swift` no longer has a `testTarget` — tests remain exclusively in
    `packages/swift/Package.swift` for CI, matching the existing pattern noted in agent memory
- `packages/swift/Package.swift` is completely unchanged (verified via git diff)
