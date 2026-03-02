# Next Work Package

## Step: Add pre-built FFI release artifacts to release.yml (issue #25)

## Goal

Add `build-ffi` and `publish-ffi` jobs to `.github/workflows/release.yml` so that C/C++ consumers
can download pre-built platform tarballs from GitHub Releases without needing a Rust toolchain. This
completes the last unmet C FFI DX target criterion (spec §4).

## Scope

- **Create**: none
- **Modify**: `.github/workflows/release.yml`
- **Reference**:
    - `.claude/context/specs/c-ffi-dx.md` §4 (exact requirements)
    - `crates/iscc-ffi/Cargo.toml` (crate-type: cdylib + staticlib)
    - Existing `build-jni` job in `release.yml` (template for 5-platform matrix)
    - Existing `build-napi` job in `release.yml` (cross-compilation pattern)

## Not In Scope

- Testing the workflow end-to-end (requires tag push or manual dispatch on GitHub — verified by
    maintainer)
- Modifying `ci.yml` or any other workflow file
- Adding FFI tarball download instructions to `docs/howto/c-cpp.md` — that's a future doc update
- Changing `crates/iscc-ffi/` source code or build configuration
- Version bump or release — this step only adds the CI jobs

## Implementation Notes

### workflow_dispatch input

Add an `ffi` boolean input alongside the existing `crates-io`, `pypi`, `npm`, `maven` inputs:

```yaml
ffi:
  description: Build and publish FFI tarballs to GitHub Releases
  type: boolean
  default: false
```

### build-ffi job

Model directly on the existing `build-jni` job. Same 5-platform matrix:

| os             | target                    | lib-shared        | lib-static    |
| -------------- | ------------------------- | ----------------- | ------------- |
| ubuntu-latest  | x86_64-unknown-linux-gnu  | libiscc_ffi.so    | libiscc_ffi.a |
| ubuntu-latest  | aarch64-unknown-linux-gnu | libiscc_ffi.so    | libiscc_ffi.a |
| macos-14       | aarch64-apple-darwin      | libiscc_ffi.dylib | libiscc_ffi.a |
| macos-14       | x86_64-apple-darwin       | libiscc_ffi.dylib | libiscc_ffi.a |
| windows-latest | x86_64-pc-windows-msvc    | iscc_ffi.dll      | iscc_ffi.lib  |

Steps per matrix entry:

1. Checkout + Rust toolchain + rust-cache (same as build-jni)
2. Install cross-compiler for aarch64-unknown-linux-gnu (same pattern)
3. `cargo build -p iscc-ffi --release --target ${{ matrix.target }}`
4. Get version from root `Cargo.toml` (same `grep` pattern as other jobs)
5. Stage artifacts into a staging directory: shared lib + static lib +
    `crates/iscc-ffi/include/iscc.h` + `LICENSE`
6. Create tarball: `tar czf iscc-ffi-v{version}-{target}.tar.gz` on Unix,
    `Compress-Archive ... iscc-ffi-v{version}-{target}.zip` on Windows
7. Upload artifact with name `ffi-${{ matrix.target }}`

**Windows library files**: Cargo produces `iscc_ffi.dll` (DLL), `iscc_ffi.dll.lib` (import lib for
DLL), and `iscc_ffi.lib` (static lib). Include all three in the Windows archive.

**Tarball contents**: Files placed in a directory named `iscc-ffi-v{version}-{target}/` so
extraction creates a named subdirectory (standard tarball practice).

### publish-ffi job

Depends on `build-ffi`. Triggers on same condition:
`startsWith(github.ref, 'refs/tags/v') || inputs.ffi`

Steps:

1. Checkout (for version extraction)
2. Download all `ffi-*` artifacts
3. Get version from root `Cargo.toml`
4. Use `softprops/action-gh-release@v2` to upload tarballs/zips as release assets

**Permissions**: needs `contents: write` at job level (top-level is `contents: read`). The action
creates the release if it doesn't exist (for tag pushes) or uploads to existing release.

**Tag resolution**: For tag pushes, `github.ref` is the tag. For workflow_dispatch, use
`v${{ steps.version.outputs.version }}` as the tag name for the release.

### Conditional logic

The `if` condition follows the same pattern as all other jobs:
`startsWith(github.ref, 'refs/tags/v') || inputs.ffi`

## Verification

- `grep -q 'build-ffi' .github/workflows/release.yml` exits 0 (build job exists)
- `grep -q 'publish-ffi' .github/workflows/release.yml` exits 0 (publish job exists)
- `grep -q 'ffi:' .github/workflows/release.yml` exits 0 (workflow_dispatch input exists)
- `grep -c 'x86_64-unknown-linux-gnu' .github/workflows/release.yml` returns ≥4 (FFI + JNI + napi
    matrices all have this target)
- `grep -q 'iscc-ffi' .github/workflows/release.yml` exits 0 (builds iscc-ffi crate)
- `grep -q 'iscc.h' .github/workflows/release.yml` exits 0 (header included in tarball)
- YAML is valid: `python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
    exits 0

## Done When

The advance agent is done when all verification criteria pass and `release.yml` contains
well-structured `build-ffi` and `publish-ffi` jobs matching the spec §4 requirements.
