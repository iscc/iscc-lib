# Plan: Restore linux/aarch64 Python wheels

## Context

iscc-lib published `manylinux_2_17_aarch64` Python wheels up to version 0.1.0, but the wheel build
matrix rewrite around 0.2.0 dropped the target. Versions 0.2.0 through 0.5.0 ship only macOS
universal2, `manylinux_2_17_x86_64`, and `win_amd64`. This blocks downstream consumers (iscc-sdk,
and the Amlet ingestion pipeline moving to AWS Graviton) from installing on Linux ARM without a Rust
toolchain for an sdist build.

Note the asymmetry: the napi (Node) and JNI jobs in `release.yml` still cross-compile
`aarch64-unknown-linux-gnu` — only the Python wheel matrix lost the target.

## Changes

### 1. Add aarch64 entry to the `build-wheels` matrix in `.github/workflows/release.yml`

The matrix (around line 172) currently has ubuntu-latest/x86_64, macos-14/universal2-apple-darwin,
and windows-latest/x64. Add:

```yaml
  - os: ubuntu-24.04-arm
    target: aarch64
    interpreter: python3.10
```

A native ARM runner (`ubuntu-24.04-arm`, free for public repos) is the simplest option — the
existing maturin-action step with `manylinux: auto` then picks the manylinux aarch64 container
automatically, and the build is fast since iscc-lib is plain Rust (no JVM/native-image, unlike
iscc-tika). Cross-compiling from x86_64 with QEMU also works but is slower; prefer the native
runner.

Check that the `before-script-linux` PATH hack (prepends `/opt/python/cp310-cp310/bin` so the abi3
wheel is tagged `cp310`, not `cp38`) behaves identically in the aarch64 manylinux container — the
path layout is the same, so it should.

### 2. Test the aarch64 wheel

If the release workflow has a wheel test/smoke job, extend it to install and import-test the aarch64
wheel on `ubuntu-24.04-arm` (e.g. `python -c "import iscc_lib; ..."` plus the Python test suite if
cheap). If wheels are only tested implicitly, add at least an install + selftest step for the new
target so a broken aarch64 wheel can't ship silently.

### 3. Optional: match the old tag width

Pre-0.2.0 wheels were `manylinux_2_17_aarch64.manylinux2014_aarch64`. `manylinux: auto` on the
current maturin-action will likely produce a comparable or newer tag — fine either way; just confirm
the resulting tag is installable on Amazon Linux 2023 / Ubuntu ARM (both glibc ≥ 2.34, so any
manylinux_2_17…2_34 tag works).

## Acceptance criteria

- Next release publishes a `manylinux*_aarch64` abi3 (cp310+) wheel alongside the existing three.
- CI installs and imports the aarch64 wheel on an ARM runner before publish.
- No change to the existing x86_64/macOS/Windows wheel outputs.
