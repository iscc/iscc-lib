## 2026-03-02 — Create C/C++ how-to guide (issue #22)

**Done:** Created `docs/howto/c-cpp.md` with all 12 required sections (overview, building from
source, CMake integration, ISCC-SUM quick start, streaming, composing ISCC-SUM manually, error
handling, memory management, static vs dynamic linking, cross-compilation, C++ RAII wrapper,
conformance verification). Added navigation entry in `zensical.toml` after Java.

**Files changed:**

- `docs/howto/c-cpp.md`: Created — full C/C++ how-to guide with 12 sections, code examples,
    ownership rules table, admonition blocks for warnings/tips
- `zensical.toml`: Added `{ "C / C++" = "howto/c-cpp.md" }` to How-to Guides nav after Java

**Verification:** All 9 verification criteria pass:

- [x] `test -f docs/howto/c-cpp.md` — file exists
- [x] `grep -q 'c-cpp.md' zensical.toml` — nav entry present
- [x] `grep -q 'iscc_gen_sum_code_v0'` — ISCC-SUM quick start present
- [x] `grep -q 'iscc_data_hasher'` — streaming section present
- [x] `grep -q 'iscc_last_error'` — error handling present
- [x] `grep -q 'iscc_free_string'` — memory management present
- [x] `grep -q 'RAII'` — C++ RAII wrapper present
- [x] `grep -q 'cmake'` — CMake integration present
- [x] `uv run zensical build` — site builds successfully (4.46s)
- [x] `mise run check` — 14/14 hooks pass

**Next:** Issue #25 (pre-built FFI release tarballs) is the remaining C FFI DX target item. The
howto guide's "Building from source" section references planned pre-built binaries. Alternatively,
#21 (units support) or #16 (feature flags) are independent enhancements.

**Notes:** The guide uses `find_library()` for CMake integration (not `CMAKE_LIBRARY_PATH`), as
flagged in the prior review. The streaming section links to the committed example at
`crates/iscc-ffi/examples/iscc_sum.c` for full error handling. The RAII wrapper is illustrative code
within the doc (not a compiled artifact), as specified in the scope.
