# Next Work Package

## Step: Propagation slice 6 — Unicode boundary vectors in the C++ test

## Goal

Gate the `packages/cpp` C++17 wrapper on the 12 Unicode 16.0.0 boundary vectors, taking criterion 3
of the Unicode issue from **9 of 11** binding surfaces to **10 of 11** (only Swift left). Picks up
the `normal` `[human]` issue **"Declare and gate a Unicode data version (DECIDED)"**, remainder (b).

Not a bounce — iteration 159 passed review. This step **overturns state.md's "C++ is CI-proof-only
because `cmake` is absent"**: `cmake` is absent from `$PATH`, but the PyPI wheel resolves and runs —
`uv run --with cmake cmake --version` → **4.4.0** — and I built and ran the full C++ ASAN suite from
this container while scoping (**54 passed, 0 failed**), then compiled and ran a standalone probe of
exactly the code this step adds (**12/12 vectors PASS**, `-Wall -Wextra` clean, ASAN clean, exit 0).
So this slice is fully locally verifiable, not CI-proof-only.

## Scope

- **Create**: (nothing)
- **Modify**:
    - `packages/cpp/tests/test_iscc.cpp` — include the generated vectors header, add section 36
    - `packages/cpp/tests/CMakeLists.txt` — one test-only include directory
    - `docs/unicode.md` — extend the propagation paragraph to name the C++ test (doc)
    - `packages/cpp/CLAUDE.md` — record the generated header + do-not-hand-edit rule (doc)
- **Reference**:
    - `crates/iscc-ffi/tests/unicode_boundary_vectors.h` — the generated header being reused as-is
    - `crates/iscc-ffi/tests/test_iscc.c` lines 77-92 (`run_unicode_boundary_section`) and 474-490
        (section 29) — the C shape this mirrors
    - `packages/cpp/include/iscc/iscc.hpp` — `iscc::text_clean` (L251), `iscc::text_collapse` (L272)
    - `packages/cpp/CMakeLists.txt` — the **public** INTERFACE target (must stay untouched)
    - `.github/workflows/ci.yml` lines 214-235 — the `cpp` job (cmake + ASAN)
    - `crates/iscc-lib/tests/unicode_boundary.json` — canonical fixture (read only, do not edit)

## Not In Scope

- **Do NOT add the vectors include directory to `packages/cpp/CMakeLists.txt`.** state.md suggests
    line 19 of that file, but that is the *public* `iscc` INTERFACE target — a test-only header
    directory there is inherited by every consumer (vcpkg/conan/`install()`). Put it on the
    `test_iscc` target in `packages/cpp/tests/CMakeLists.txt` instead.
- Do NOT generate a second header, a C++-specific header, or a vendored JSON copy for C++. The
    existing `crates/iscc-ffi/tests/unicode_boundary_vectors.h` is plain C and already tracked.
- Do NOT touch `tests/test_vendored_fixtures.py` / `VENDORED_COPIES` — that table is for
    byte-identical *copies*; this slice adds none. The header stays a derived artifact gated by
    `tests/test_gen_ffi_boundary_vectors.py`.
- Do NOT regenerate, hand-edit or reformat `unicode_boundary_vectors.h`, and do not add, remove or
    rename a vector in `crates/iscc-lib/tests/unicode_boundary.json` — ten consumers assert exactly
    7 `text_clean` + 5 `text_collapse`.
- Do NOT touch `.github/workflows/ci.yml`; the `cpp` job already configures/builds/runs whatever the
    test CMakeLists declares.
- Do NOT start the Swift slice, and do not add a `swift`/`cmake` install step anywhere.
- Do NOT refresh `.crap-baseline.json` or `.iai-baseline.json`. No Rust source moves in this step,
    so neither the CI-only CRAP `--fail-regression` gate nor the 10 % Ir iai gate can move; touching
    a baseline here would be an unexplained diff.
- Do NOT copy the `delete_filter_output` oracles into the C++ test — plain equality against the
    expected value already reds on a delete-filter regression.

## Implementation Notes

Everything below was measured in this container while scoping; prefer it over re-deriving.

**1. Build plumbing (`packages/cpp/tests/CMakeLists.txt`).** Add one line next to the existing
`target_link_libraries(test_iscc PRIVATE iscc::iscc)`:

```cmake
# Generated Unicode 16.0.0 boundary vectors (test-only; not part of the public interface)
target_include_directories(test_iscc PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../../../crates/iscc-ffi/tests)
```

This mirrors the existing convention (relative cross-package paths live in CMake, sources use plain
includes) and keeps the header off the public interface. `${CMAKE_CURRENT_SOURCE_DIR}` here is
`packages/cpp/tests`, so three `..` reach the repo root.

**2. Source (`packages/cpp/tests/test_iscc.cpp`).** Add after `#include <iscc/iscc.hpp>`:

```cpp
#include "unicode_boundary_vectors.h"
```

Then a file-scope helper next to the other helpers (this exact signature compiles clean under
`g++ -std=c++17 -Wall -Wextra -fsanitize=address`; taking the address of the inline
`iscc::text_clean` / `iscc::text_collapse` is fine):

```cpp
/// Run one section of Unicode 16.0.0 boundary vectors through a wrapper text function.
static void run_unicode_boundary_section(const char* section,
                                         std::string (*fn)(const std::string&),
                                         const iscc_unicode_boundary_vector* vectors,
                                         size_t count) {
    for (size_t i = 0; i < count; ++i) {
        std::string name = std::string("unicode_boundary/") + section + "/" + vectors[i].name;
        assert_str_eq(fn(vectors[i].input), vectors[i].expected, name.c_str());
    }
}
```

and a new block **36** immediately before the `// Summary` block, mirroring C section 29 — three
metadata guards then the two sections:

```cpp
// 36. Unicode 16.0.0 boundary vectors — metadata guard + generated vectors
{
    assert_str_eq(ISCC_UNICODE_DATA_VERSION, "16.0.0",
                  "unicode_boundary metadata: version == 16.0.0");
    assert_eq(static_cast<size_t>(ISCC_TEXT_CLEAN_VECTOR_COUNT), 7,
              "unicode_boundary metadata: text_clean count == 7");
    assert_eq(static_cast<size_t>(ISCC_TEXT_COLLAPSE_VECTOR_COUNT), 5,
              "unicode_boundary metadata: text_collapse count == 5");
    run_unicode_boundary_section("text_clean", &iscc::text_clean,
                                 iscc_text_clean_vectors, ISCC_TEXT_CLEAN_VECTOR_COUNT);
    run_unicode_boundary_section("text_collapse", &iscc::text_collapse,
                                 iscc_text_collapse_vectors, ISCC_TEXT_COLLAPSE_VECTOR_COUNT);
}
```

Notes: the C test needed a `const char *unicode_version` local to dodge `-Waddress`; C++ needs no
such workaround because `assert_str_eq` takes `const std::string&`. Nothing is freed manually —
`iscc::text_clean` returns a `std::string` and `detail::UniqueString` already frees the FFI string,
so the ASAN/LSan run stays clean (verified). **Zero skips**: the C++ wrapper calls the same Rust
core, so all 12 vectors must pass.

**3. Local verification recipe** (run from the repo root, no `cd`; `build-uv/` is gitignored by
`.gitignore:12 build-*/`, so this leaves no tree diff):

```bash
cargo build -p iscc-ffi
uv run --with cmake cmake -S packages/cpp -B packages/cpp/build-uv \
    -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=$PWD/target/debug -DSANITIZE_ADDRESS=ON
uv run --with cmake cmake --build packages/cpp/build-uv
LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-uv/tests/test_iscc
```

Do **not** reuse the stale `packages/cpp/build/` directory — its `CMakeCache.txt` was written by
cmake 3.25 and the uv-provided cmake is 4.4.0. Local cmake is 4.4.0 vs CI's apt cmake; the project
declares `cmake_minimum_required(VERSION 3.14)`, which 4.4.0 accepts without a deprecation error
(verified). A `gmake[1]: warning: Clock skew detected` line may appear on this bind mount — it is
benign and unrelated.

**4. Docs.** In `docs/unicode.md`, the paragraph beginning "Both vector families are checked into
the repository" currently names the C FFI test program and its generated header — extend it so the
C++ test is named as a second consumer of the *same* generated header (no second artifact). Leave
the "All 11 native bindings" sentence below it alone. In `packages/cpp/CLAUDE.md`, add a bullet to
"Test Patterns" stating that the Unicode 16.0.0 boundary vectors come from
`crates/iscc-ffi/tests/unicode_boundary_vectors.h`, that it is generated
(`uv run --script scripts/gen_ffi_boundary_vectors.py`) and must never be hand-edited, and that the
include directory is set on the test target only — never on the public `iscc` INTERFACE target.

## Verification

- `cargo build -p iscc-ffi` exits 0
- `uv run --with cmake cmake -S packages/cpp -B packages/cpp/build-uv -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=$PWD/target/debug -DSANITIZE_ADDRESS=ON`
    exits 0, then `uv run --with cmake cmake --build packages/cpp/build-uv` exits 0 with no compiler
    warning from `test_iscc.cpp`
- `LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-uv/tests/test_iscc` prints
    **`69 passed, 0 failed`** (54 today + 3 metadata guards + 12 vectors) and exits 0
- `LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-uv/tests/test_iscc | grep -c '^PASS: unicode_boundary/'`
    prints **12**
- `grep -c 'iscc-ffi/tests' packages/cpp/CMakeLists.txt` prints **0** (the test-only include
    directory is not on the public INTERFACE target)
- `grep -c 'unicode_boundary_vectors.h' packages/cpp/tests/CMakeLists.txt` prints **1**
- `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py tests/test_vendored_fixtures.py` → **14
    passed** (6 + 8), and
    `git status --porcelain -- crates/iscc-ffi/tests/unicode_boundary_vectors.h crates/iscc-lib/tests/unicode_boundary.json tests/test_vendored_fixtures.py`
    prints nothing
- `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/ packages/cpp/CMakeLists.txt`
    prints nothing (no workflow, baseline, Rust/binding-source or public-CMake change)
- `grep -c 'C++' docs/unicode.md` prints **2 or more** (it is 1 today)
- `uv run zensical build` exits 0 and reports "No issues found"; `uv run scripts/check_docs_nav.py`
    prints `OK: 23 documentation pages consistent`
- `mise run check` — all hooks pass and no file is modified by the run (only the runner-owned
    `.claude/context/iterations.jsonl` may be dirty)

## Done When

All verification criteria pass — the C++ suite reports `69 passed, 0 failed` with 12
`unicode_boundary/` cases driven from the reused generated header, no public CMake / workflow /
baseline / fixture file has moved, and the docs name C++ as a consumer of that header.
