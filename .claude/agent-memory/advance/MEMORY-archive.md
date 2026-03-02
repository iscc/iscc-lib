# Advance Agent Memory — Archive

Archived implementation patterns from completed project phases. Moved here to reduce per-invocation
context loading. Full history preserved in git.

See MEMORY.md for current active entries.

## Archived 2026-03-02 — Documentation Sweep Patterns

- "N gen" count references exist in: READMEs (9 files), docs/ (14 files), howto/ (6 files), crate
    CLAUDE.md files (5), notes/ (2), source comments (.rs, .py, .mjs, .pyi), benchmarks/ (2)
- The Edit tool requires a full Read call (not offset/limit) before the first edit per file
- mdformat auto-reformats after edits — always run `mise run format` twice after doc changes
- iscc-core-ts is external and may have different function counts than iscc-lib

## Archived 2026-03-02 — C FFI Examples

- `crates/iscc-ffi/examples/iscc_sum.c` — streaming ISCC-SUM example (read file → dual hashers →
    compose → print). C89/C99 compatible style (variables declared at block start)
- `crates/iscc-ffi/examples/CMakeLists.txt` — minimal cmake build targeting `iscc_ffi` library
- gcc compile:
    `gcc -o out iscc_sum.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
- Run: `LD_LIBRARY_PATH=target/debug ./out <filepath>`

## Archived 2026-03-02 — C FFI Release Artifacts

- `release.yml` has `build-ffi` (5-platform matrix) + `publish-ffi` (uploads to GitHub Releases)
- Trigger: `startsWith(github.ref, 'refs/tags/v') || inputs.ffi` (same pattern as other jobs)
- Tarball naming: `iscc-ffi-v{version}-{target}.tar.gz` (Unix), `.zip` (Windows)
- Windows includes 3 files: `iscc_ffi.dll`, `iscc_ffi.dll.lib` (import lib), `iscc_ffi.lib` (static)
- Unix includes 2 files: shared lib + static lib. Both also include `iscc.h` + `LICENSE`
- `publish-ffi` needs `contents: write` (top-level is `contents: read`)
- Uses `softprops/action-gh-release@v2` with tag_name ternary for tag push vs manual dispatch
