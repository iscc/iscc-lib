# Next Work Package

## Step: Add C++ documentation (README, howto, root README)

## Goal

Add comprehensive C++ documentation: a per-package `README.md` for `packages/cpp/`, update the
`docs/howto/c-cpp.md` guide to showcase `iscc.hpp` as the recommended C++ approach, and add C++
install + quickstart sections to the root `README.md`. This makes the C++ wrapper discoverable and
usable by developers.

## Scope

- **Create**: `packages/cpp/README.md`
- **Modify**: `docs/howto/c-cpp.md`, `README.md`
- **Reference**: `packages/cpp/include/iscc/iscc.hpp` (C++ API surface), `packages/dotnet/README.md`
    (per-package README pattern), `docs/howto/dotnet.md` (howto guide pattern)

## Not In Scope

- Package manager manifests (`vcpkg.json`, `conanfile.py`, `pkg-config/iscc.pc.in`) — separate step
- Adding `gen_mixed_code_v0` test to `test_iscc.cpp` — separate step
- Nested vector null-safety hardening (`safe_data` for inner elements) — separate step
- Swift/Kotlin documentation placeholders — those bindings don't exist yet
- Modifying `iscc.hpp` or any C++ source code — this is a docs-only step

## Implementation Notes

### `packages/cpp/README.md` (create)

Follow the `packages/dotnet/README.md` pattern:

- Badges: CI status, license
- Tagline: "Idiomatic C++17 header-only wrapper for ISO 24138 (ISCC)"
- "What is ISCC" section (reuse standard text from other READMEs)
- Installation: explain two paths — (1) download pre-built FFI release tarball from GitHub Releases,
    (2) build from source with `cargo build -p iscc-ffi --release`
- Quick start: show `#include <iscc/iscc.hpp>` with `iscc::gen_meta_code_v0("Title")`
- API overview: table of all 10 gen functions + streaming + utilities (matching .NET README format)
- Links: docs site, howto guide, repository, ISCC specification
- License: Apache-2.0

### `docs/howto/c-cpp.md` (modify)

The existing guide (433 lines) covers the raw C FFI thoroughly. The C++ section currently (lines
327-411) shows a **hand-written** RAII wrapper class for `DataHasher`. This must be replaced with
content showing the **actual `iscc.hpp` wrapper** that now exists.

Changes needed:

1. **Update the intro** (lines 8-11) to mention `iscc.hpp` as the recommended C++ approach

2. **Update the pre-built binaries tip** (line 40-44) to mention `iscc.hpp` is included in tarballs

3. **Replace the "C++ RAII wrapper" section** (lines 327-411) with comprehensive `iscc.hpp`
    documentation:

    - Explain include path conventions: tarball users use `#include "iscc.hpp"` (flat layout);
        CMake/source users use `#include <iscc/iscc.hpp>` (via include directory setup)
    - Quick start: `iscc::gen_meta_code_v0()` returning a result struct with `.iscc`
    - Gen functions: show a few key ones (meta, text, sum) with `iscc::` namespace
    - Streaming: `iscc::DataHasher` with RAII — `update()` / `finalize()` pattern. Show dual-hasher
        streaming as C++ equivalent of the C streaming section above
    - Error handling: `iscc::IsccError` exception class (replaces NULL checks)
    - Codec/utility: `iscc::iscc_decode()`, `iscc::iscc_decompose()`
    - Conformance: `iscc::conformance_selftest()`

4. Keep ALL existing C sections unchanged (building, CMake, pkg-config, C quick start, C streaming,
    C error handling, C memory management, static/dynamic linking, cross-compilation, C
    conformance)

### `README.md` (modify)

1. **Update "Polyglot" bullet** (line 26): Add "C++" to the list — currently says "Python, Java, Go,
    Ruby, C#, Node.js, WASM, and C FFI"

2. **Update "What is iscc-lib" paragraph** (line 50): Add "C++" to the ecosystem list

3. **Add C++ installation section** between "C# / .NET" and "WASM":

    ```markdown
    ### C / C++

    Pre-built release tarballs are attached to each
    [GitHub Release](https://github.com/iscc/iscc-lib/releases). Download for your platform —
    includes shared library, static library, `iscc.h` header, and `iscc.hpp` C++ wrapper.
    ```

4. **Add C++ quickstart** between "C# / .NET" and "WASM":

    ```cpp
    #include <iscc/iscc.hpp>
    #include <iostream>

    int main() {
        auto result = iscc::gen_meta_code_v0("ISCC Test Document!");
        std::cout << "Meta-Code: " << result.iscc << std::endl;
    }
    ```

## Verification

- `grep -c 'iscc.hpp' docs/howto/c-cpp.md` returns at least 5 (multiple references to the header)
- `grep -c 'iscc::gen_meta_code_v0' docs/howto/c-cpp.md` returns at least 1
- `grep -c 'iscc::DataHasher' docs/howto/c-cpp.md` returns at least 1
- `grep 'C++' README.md | head -5` shows C++ mentioned in install and quickstart sections
- `grep -c 'iscc.hpp' README.md` returns at least 1
- `test -f packages/cpp/README.md` exits 0
- `grep -c 'gen_meta_code_v0' packages/cpp/README.md` returns at least 1
- `grep -c 'What is ISCC' packages/cpp/README.md` returns 1
- `mise run check` passes (formatting, lint, pre-commit hooks)

## Done When

All verification criteria pass — three documentation files exist with correct C++ wrapper content,
and `mise run check` is clean.
