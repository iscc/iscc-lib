# Spec: C FFI Developer Experience

Comprehensive developer experience for C/C++ consumers of iscc-lib. The target audience is
systems-level teams (e.g., instrument firmware, embedded C++ services) that need ISCC-SUM (Data-Code
\+ Instance-Code) via C interop. They should be able to evaluate and integrate the library without
installing a Rust toolchain.

## 1. How-To Guide — `docs/howto/c-cpp.md`

A dedicated how-to guide for C/C++ integration, following the same structure as the existing
per-language guides (`docs/howto/rust.md`, `docs/howto/python.md`, etc.).

**Sections:**

- **Overview**: what `iscc-ffi` provides, shared + static library, generated C header
- **Installation from pre-built release**: download tarball from GitHub Releases, extract library +
    header, link
- **Building from source**: `cargo build -p iscc-ffi --release`, cbindgen header generation
- **Build system integration**: CMake `find_library` / `FetchContent` snippet, pkg-config pattern,
    Meson example
- **ISCC-SUM quick start**: focused example showing `iscc_gen_sum_code_v0()` for one-shot file
    hashing — the primary use case
- **Streaming**: `DataHasher` + `InstanceHasher` walkthrough — create, feed chunks in a loop,
    finalize, free. Show both Data-Code and Instance-Code being generated from the same read loop.
    This is the "process object, throw bytes at it" pattern.
- **Composing ISCC-SUM manually**: using `iscc_gen_iscc_code_v0()` to combine individually streamed
    Data-Code + Instance-Code into a composite ISCC-CODE
- **Error handling**: `iscc_last_error()` pattern, NULL checks, thread safety notes
- **Memory management**: ownership rules, free functions, common pitfalls
- **Static vs dynamic linking**: when to use each, platform-specific notes
- **Cross-compilation**: building for ARM targets (relevant for embedded devices)
- **C++ RAII wrapper**: show a minimal `IsccDataHasher` C++ class wrapping the C lifecycle
    (constructor calls `_new`, destructor calls `_free`, move-only semantics)
- **Conformance verification**: using `iscc_conformance_selftest()` to validate the build

**Tone**: practical, task-oriented. Assume the reader is an experienced C++ developer who has never
seen ISCC before.

**Verified when:**

- [x] `docs/howto/c-cpp.md` exists and is linked in site navigation
- [x] Contains CMake integration snippet
- [x] Contains streaming DataHasher + InstanceHasher example
- [x] Contains ISCC-SUM one-shot example
- [x] Contains C++ RAII wrapper example
- [x] Contains error handling and memory management sections

## 2. Standalone Example Program

A focused, copy-pasteable example showing the exact Nanopore-style use case: streaming a file
through both hashers simultaneously, then composing the ISCC-SUM.

**Files:**

- `crates/iscc-ffi/examples/iscc_sum.c` — C example: open file, read chunks in a loop, feed both
    `DataHasher` and `InstanceHasher`, finalize both, compose via `iscc_gen_iscc_code_v0`, print
    result, free everything
- `crates/iscc-ffi/examples/CMakeLists.txt` — minimal CMake build that finds the library and
    compiles the example

The example should be self-contained: compile, run against any file, print the ISCC-SUM. Include
comments explaining each step.

**Verified when:**

- [ ] `crates/iscc-ffi/examples/iscc_sum.c` exists and compiles
- [ ] `crates/iscc-ffi/examples/CMakeLists.txt` exists and works with
    `cmake -B build && cmake   --build build`
- [ ] Example produces correct ISCC-SUM output for a test file
- [ ] Example demonstrates streaming (chunk loop), not just one-shot

## 3. Committed C Header with CI Freshness Check

The generated `iscc.h` header is the primary interface contract for C/C++ consumers. It should be
committed to the repo so developers can browse it on GitHub and so release artifacts include it
without requiring cbindgen.

**Implementation:**

- Generate header to `crates/iscc-ffi/include/iscc.h` (this path is already referenced in
    `cbindgen.toml` docs and the C test's `#include`)
- Remove `crates/iscc-ffi/include/` from `.gitignore` (currently the `include/` dir doesn't exist
    because the header is gitignored)
- Add a CI step to the C FFI job: regenerate the header, `git diff --exit-code` to ensure the
    committed version is up to date. If stale, CI fails with a message telling the developer to run
    `cbindgen` and commit the result.

**Verified when:**

- [ ] `crates/iscc-ffi/include/iscc.h` exists in the repo and is not gitignored
- [ ] CI C FFI job includes a freshness check that fails if the header is stale
- [ ] The C test program compiles against the committed header

## 4. Pre-Built FFI Release Artifacts

Add a release job that builds the FFI shared/static libraries for all supported platforms and
uploads them as GitHub Release assets. C/C++ consumers can download a platform tarball without
needing Rust.

**Implementation:**

Add to `release.yml`:

- A `build-ffi` job with the same platform matrix as the JNI/napi builds:
    - `x86_64-unknown-linux-gnu` (ubuntu-latest)
    - `aarch64-unknown-linux-gnu` (ubuntu-latest + cross-compiler)
    - `aarch64-apple-darwin` (macos-14)
    - `x86_64-apple-darwin` (macos-14)
    - `x86_64-pc-windows-msvc` (windows-latest)
- Each matrix entry: `cargo build -p iscc-ffi --release --target <target>`
- Package output: tarball (`.tar.gz` on Unix, `.zip` on Windows) containing:
    - Shared library (`libiscc_ffi.so` / `.dylib` / `.dll`)
    - Static library (`libiscc_ffi.a` / `.lib`)
    - `iscc.h` (copied from committed `include/iscc.h`)
    - `LICENSE`
- A `publish-ffi` job (needs `build-ffi`) that creates/updates the GitHub Release and uploads the
    tarballs as release assets
- Trigger: same as other release jobs (`startsWith(github.ref, 'refs/tags/v') || inputs.ffi`)
- Add `ffi` boolean input to `workflow_dispatch`

**Naming convention:** `iscc-ffi-v{version}-{target}.tar.gz` (e.g.,
`iscc-ffi-v0.0.4-x86_64-unknown-linux-gnu.tar.gz`)

**Verified when:**

- [x] `release.yml` includes `build-ffi` and `publish-ffi` jobs
- [x] `workflow_dispatch` has `ffi` boolean input
- [x] Tag push triggers FFI builds for all 5 platform targets
- [x] Each platform produces a tarball with shared lib + static lib + header + LICENSE
- [x] Tarballs are uploaded as GitHub Release assets

## 5. Feature Flags for Minimal Builds

Tracked separately as GitHub Issue #16. Allows `default-features = false` to build only Data-Code +
Instance-Code, stripping ~82K lines of serde/unicode dependencies. Relevant for embedded consumers
but not blocking for the DX improvements above.
