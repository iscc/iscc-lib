# Next Work Package

## Step: Create C/C++ how-to guide (issue #22)

## Goal

Create `docs/howto/c-cpp.md` — the dedicated how-to guide for C/C++ integration — and add it to site
navigation. This is the last documentation criterion blocking the C FFI DX target section.

## Scope

- **Create**: `docs/howto/c-cpp.md`
- **Modify**: `zensical.toml` (add `{ "C / C++" = "howto/c-cpp.md" }` to the How-to Guides nav
    section, after Java)
- **Reference**:
    - `.claude/context/specs/c-ffi-dx.md` §1 — required sections and tone
    - `docs/howto/rust.md` — structural model for howto guides (front matter, heading style, section
        flow)
    - `docs/c-ffi-api.md` — full API reference to link to (not duplicate)
    - `crates/iscc-ffi/examples/iscc_sum.c` — the committed example program to reference/embed
    - `crates/iscc-ffi/include/iscc.h` — the committed header for type signatures

## Not In Scope

- Pre-built FFI release tarballs (#25) — the "installation" section should cover building from
    source and mention that pre-built binaries are planned for future releases. Do NOT create CI
    jobs or release workflow changes.
- Modifying any C source code, header files, or Rust code
- Creating a separate C++ example program — the RAII wrapper section is illustrative code within the
    doc, not a compiled artifact
- Updating `docs/c-ffi-api.md` — that page is the API reference; the howto guide links to it

## Implementation Notes

**Front matter**: Use `icon: lucide/code` and a description matching the other howto guides' style
(`Guide to using iscc-lib from C/C++ — ...`).

**Required sections** (from spec §1):

1. **Overview** — what `iscc-ffi` provides (shared + static lib, generated C header), target
    audience (systems-level teams, embedded, C++ services)
2. **Building from source** — `cargo build -p iscc-ffi --release`, cbindgen header generation,
    output paths per platform (`libiscc_ffi.so` / `.dylib` / `iscc_ffi.dll`)
3. **Build system integration** — CMake `find_library()` snippet with `CMAKE_PREFIX_PATH`. Use
    `find_library()` pattern (NOT bare `CMAKE_LIBRARY_PATH` — flagged in review of #23). Include a
    brief pkg-config note.
4. **ISCC-SUM quick start** — focused example showing `iscc_gen_sum_code_v0()` for one-shot file
    hashing (can reference the committed example in `crates/iscc-ffi/examples/iscc_sum.c`)
5. **Streaming** — `DataHasher` + `InstanceHasher` walkthrough: create, feed chunks in a loop,
    finalize, free. Show dual-hasher pattern from same read loop
6. **Composing ISCC-SUM manually** — using `iscc_gen_iscc_code_v0()` to combine individually
    streamed Data-Code + Instance-Code
7. **Error handling** — `iscc_last_error()` pattern, NULL checks, thread safety note (thread-local
    storage)
8. **Memory management** — ownership rules table (which `_free` function for which return type),
    common pitfalls (double-free, use-after-free)
9. **Static vs dynamic linking** — when to use each, platform differences
10. **Cross-compilation** — building for ARM/embedded targets with `--target`
11. **C++ RAII wrapper** — minimal `IsccDataHasher` class (constructor→`_new`, destructor→`_free`,
    move-only, deleted copy)
12. **Conformance verification** — `iscc_conformance_selftest()` to validate a build

**Style guidelines**:

- Use fenced code blocks with `c` or `cpp` language tags
- Link to `c-ffi-api.md` for full API reference rather than duplicating function tables
- Tone: practical, task-oriented. Assume the reader is an experienced C/C++ developer who has never
    seen ISCC before
- Keep code examples self-contained and compilable (include necessary `#include` directives)
- Use admonition blocks (`!!! warning`, `!!! tip`) for important notes about memory management and
    thread safety

**Navigation**: Add after the Java entry in zensical.toml nav, using format
`{ "C / C++" = "howto/c-cpp.md" }`.

## Verification

- `test -f docs/howto/c-cpp.md` exits 0
- `grep -q 'c-cpp.md' zensical.toml` exits 0
- `grep -q 'iscc_gen_sum_code_v0' docs/howto/c-cpp.md` exits 0 (ISCC-SUM quick start present)
- `grep -q 'iscc_data_hasher' docs/howto/c-cpp.md` exits 0 (streaming section present)
- `grep -q 'iscc_last_error' docs/howto/c-cpp.md` exits 0 (error handling present)
- `grep -q 'iscc_free_string' docs/howto/c-cpp.md` exits 0 (memory management present)
- `grep -q 'RAII' docs/howto/c-cpp.md` exits 0 (C++ RAII wrapper present)
- `grep -q 'cmake' docs/howto/c-cpp.md` exits 0 (CMake integration present)
- `uv run zensical build` exits 0 (site builds successfully)

## Done When

All verification commands exit 0 — the C/C++ howto guide exists with all required sections and the
documentation site builds successfully.
