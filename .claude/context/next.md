# Next Work Package

## Step: Add SHA512 checksums to vcpkg portfile

## Goal

Replace `SKIP_SHA512` in `packages/cpp/portfile.cmake` with actual SHA512 checksums for each of the
5 platform tarballs from the v0.2.0 release, strengthening supply-chain integrity for vcpkg
consumers. This resolves the "vcpkg portfile skips SHA512 verification" normal-priority issue.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/cpp/portfile.cmake` — add per-platform `ISCC_SHA512` variable in each if/elseif
        branch; replace `SKIP_SHA512` with `SHA512 "${ISCC_SHA512}"`
- **Reference**:
    - `packages/cpp/portfile.cmake` — current structure with 5 platform branches and
        `vcpkg_download_distfile` call
    - v0.2.0 release assets (download via `gh release download v0.2.0`)

## Not In Scope

- Release workflow automation to compute SHA512 on future releases — that's a follow-up step
- Adding SHA512 verification to the Conan recipe (`conanfile.py`) — no issue filed, separate concern
- Creating a separate SHA512 manifest file or lookup table — keep it simple in the portfile itself
- Modifying `vcpkg.json` or any other file — this is a single-file change to `portfile.cmake`

## Implementation Notes

### 1. Compute SHA512 checksums

Download all 5 v0.2.0 release tarballs and compute SHA512 for each:

```bash
gh release download v0.2.0 --pattern 'iscc-ffi-v0.2.0-*' --dir /tmp/iscc-ffi-checksums
sha512sum /tmp/iscc-ffi-checksums/*
```

The 5 files are:

- `iscc-ffi-v0.2.0-x86_64-unknown-linux-gnu.tar.gz`
- `iscc-ffi-v0.2.0-aarch64-unknown-linux-gnu.tar.gz`
- `iscc-ffi-v0.2.0-aarch64-apple-darwin.tar.gz`
- `iscc-ffi-v0.2.0-x86_64-apple-darwin.tar.gz`
- `iscc-ffi-v0.2.0-x86_64-pc-windows-msvc.zip`

### 2. Update portfile.cmake

Add a `set(ISCC_SHA512 "...")` line inside each platform's if/elseif block, right after the existing
`set()` calls. Example structure:

```cmake
if(VCPKG_TARGET_IS_LINUX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-unknown-linux-gnu")
    set(ISCC_LIB_NAME "libiscc_ffi.so")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "<computed-hash-here>")
elseif(...)
    ...
    set(ISCC_SHA512 "<computed-hash-here>")
```

### 3. Replace SKIP_SHA512

Change the `vcpkg_download_distfile` call from:

```cmake
    SKIP_SHA512
```

to:

```cmake
    SHA512 "${ISCC_SHA512}"
```

### 4. Important details

- The SHA512 hash is of the tarball/zip **file itself** (not of its contents)
- Use lowercase hex for the SHA512 hash (vcpkg convention)
- Do NOT change anything else in the portfile — no structural refactoring

## Verification

- `! grep -q 'SKIP_SHA512' packages/cpp/portfile.cmake` exits 0 (SKIP_SHA512 removed)
- `grep -c 'ISCC_SHA512' packages/cpp/portfile.cmake` returns 6 (5 `set()` + 1 `SHA512` usage)
- `grep -q 'SHA512 "\${ISCC_SHA512}"' packages/cpp/portfile.cmake` exits 0 (SHA512 variable used in
    download call)
- Each SHA512 hash is 128 hex characters:
    `grep -cP 'set\(ISCC_SHA512 "[0-9a-f]{128}"\)' packages/cpp/portfile.cmake` returns 5
- `cmake -P packages/cpp/portfile.cmake 2>&1 | head -1` does NOT error on CMake syntax (may fail on
    vcpkg functions, but should not fail on syntax)

## Done When

All verification criteria pass — `SKIP_SHA512` replaced with real SHA512 checksums for all 5
platform tarballs, and the portfile uses the `SHA512 "${ISCC_SHA512}"` pattern.
