# Next Work Package

## Step: Add Swift XCFramework build-and-publish job to release workflow

## Goal

Add the `build-xcframework` job and `swift` checkbox input to `release.yml` so that the release
pipeline can build, checksum-update, and publish the Swift XCFramework to GitHub Releases — enabling
zero-friction SPM installation for consumers.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**:
    - `.claude/context/specs/swift-bindings.md` (section "Release Workflow Integration" — has the
        exact YAML snippet)
    - `Package.swift` (understand the `releaseTag`/`releaseChecksum` sed targets on lines 16-17)
    - `scripts/build_xcframework.sh` (understand what the build step invokes)

## Not In Scope

- Adding `releaseTag` to `version_sync.py` — that's a separate follow-up step
- Updating `docs/howto/swift.md` to document SPM install — separate step after version sync
- Adding a Swift smoke test job — the XCFramework is tested via the existing `swift` CI job
    (`ci.yml`); release-time validation is via the checksum matching
- Modifying `Package.swift` locally — the `sed` commands run in CI on the `main` branch checkout,
    not as local file edits
- Modifying `ci.yml` — the existing Swift CI job is unchanged

## Implementation Notes

Follow the spec's YAML snippet in `.claude/context/specs/swift-bindings.md` section "Release
Workflow Integration" closely. Key details:

1. **Input**: Add `swift` boolean input to `workflow_dispatch.inputs` (matches pattern of existing
    `ffi`, `rubygems`, `nuget`, `maven-kotlin` inputs)

2. **Job `build-xcframework`**: Single macOS job that does everything (build → checksum → update
    Package.swift → commit → force-update tag → upload):

    - **Condition**: `startsWith(github.ref, 'refs/tags/v') || inputs.swift`
    - **Runner**: `macos-14` (Apple Silicon, same as existing Swift CI job)
    - **Permissions**: `contents: write` (needs to push commits and upload release assets)
    - **Checkout**: `fetch-depth: 0`, `ref: main` — must checkout main for the auto-commit
    - **Rust targets**: install all 5 Apple targets via `dtolnay/rust-toolchain@stable`
    - **Cache**: `actions/cache@v4` with key
        `xcf-${{ hashFiles('crates/iscc-*/src/**', 'Cargo.lock') }}`, path
        `target/ios/IsccLib.xcframework.zip`. Skip build on cache hit
    - **Build**: `./scripts/build_xcframework.sh --release` (only on cache miss)
    - **Checksum + sed**: Extract version from `GITHUB_REF_NAME`, compute checksum via
        `swift package compute-checksum`, use `sed -E -i ''` (macOS sed) to update both `releaseTag`
        and `releaseChecksum` in `Package.swift`
    - **Auto-commit**: `stefanzweifel/git-auto-commit-action@v5` with message
        `chore: update Swift XCFramework checksum`
    - **Force-update tag**: `git tag -fa ${{ github.ref_name }}` + `git push origin ... --force`
    - **Upload**: `softprops/action-gh-release@v2` with file `target/ios/IsccLib.xcframework.zip`

3. **macOS sed syntax**: Use `sed -E -i ''` (empty string backup extension) — macOS BSD sed differs
    from GNU sed. The existing build-ffi job uses `sed 's/...'` without `-i` so doesn't hit this,
    but here we need in-place editing.

4. **Tag name extraction**: `VERSION="${GITHUB_REF_NAME#v}"` strips the `v` prefix (e.g., `v0.4.0` →
    `0.4.0`). This matches the pattern used in other jobs.

5. **No `needs` dependencies** — this job is independent of all other release jobs. It builds
    `iscc-uniffi` from source on macOS, not from artifacts produced by other jobs.

6. **Rust cache**: Also add `Swatinem/rust-cache@v2` for Rust compilation caching (separate from the
    XCFramework zip cache).

## Verification

- `python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -q 'swift:' .github/workflows/release.yml` exits 0 (swift input present)
- `grep -q 'build-xcframework' .github/workflows/release.yml` exits 0 (job name present)
- `grep -q 'build_xcframework.sh' .github/workflows/release.yml` exits 0 (build script invoked)
- `grep -q 'swift package compute-checksum' .github/workflows/release.yml` exits 0 (checksum step)
- `grep -q 'force' .github/workflows/release.yml` exits 0 (force-update tag step present)
- `grep -q 'IsccLib.xcframework.zip' .github/workflows/release.yml` exits 0 (correct artifact name)
- `grep -q 'contents: write' .github/workflows/release.yml` exits 0 (write permission present)
- `grep -c 'type: boolean' .github/workflows/release.yml` returns 9 (was 8, now +swift)
- `mise run check` passes (YAML validation via pre-commit hooks)

## Done When

All verification criteria pass — `release.yml` contains a syntactically valid `build-xcframework`
job with `swift` input, XCFramework build, checksum update via sed, auto-commit, force-update tag,
and GitHub Release upload.
