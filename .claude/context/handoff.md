## 2026-03-21 — Add Swift XCFramework build-and-publish job to release workflow

**Done:** Added `swift` boolean input (9th) to `workflow_dispatch.inputs` and `build-xcframework`
job to `release.yml`. The job builds the XCFramework on macOS-14, computes the checksum via
`swift package compute-checksum`, updates `Package.swift` with `sed`, auto-commits, force-updates
the tag, and uploads the zip to GitHub Releases.

**Files changed:**

- `.github/workflows/release.yml`: Added `swift` input (lines 40-43) and `build-xcframework` job
    (lines 1170-1215) with XCFramework build, checksum update, auto-commit, force-update tag, and
    release asset upload

**Verification:** All 10 verification criteria from next.md pass:

- [x] YAML valid (`python -c "import yaml; yaml.safe_load(...)"` exits 0)
- [x] `swift:` input present
- [x] `build-xcframework` job name present
- [x] `build_xcframework.sh` script invoked
- [x] `swift package compute-checksum` step present
- [x] `force` tag update present
- [x] `IsccLib.xcframework.zip` artifact name correct
- [x] `contents: write` permission present
- [x] 9 boolean inputs (was 8, +1 for swift)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Next:** Two remaining steps in the Swift XCFramework multi-step effort:

1. Add `releaseTag` entry to `version_sync.py` so version bumps propagate to root `Package.swift`
2. Update `docs/howto/swift.md` to document SPM install using the zero-friction binary target

**Notes:**

- The job faithfully follows the spec's YAML snippet from `.claude/context/specs/swift-bindings.md`
    section "Release Workflow Integration"
- No `needs` dependencies — this job runs independently of all other release jobs
- Uses macOS BSD `sed -E -i ''` syntax (empty backup extension), not GNU `sed -i`
- Includes both `Swatinem/rust-cache@v2` (Rust compilation caching) and `actions/cache@v4`
    (XCFramework zip caching) — the zip cache skips the entire native build on hit
- This is a YAML-only change with no Rust code modifications, so no tests to write beyond the
    verification criteria above
