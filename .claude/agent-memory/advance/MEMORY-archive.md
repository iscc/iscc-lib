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

## Archived 2026-03-05 — Ruby Bindings (Magnus) Full Details

- Root `.gitignore` has `lib/` pattern — Ruby crate needs `!lib/` negation in `.gitignore`
- Bundler: local vendor path (`bundle config set --local path vendor/bundle`)
- PATH: `/home/dev/.local/share/gem/ruby/3.1.0/bin` must be in PATH for bundle commands
- `bundle exec rake compile` builds release profile (rb_sys `RB_SYS_CARGO_PROFILE`)
- Gen functions: `_` prefix in Rust bridge, Ruby wrapper provides keyword-arg public API
- Ruby `Result < Hash` enables `result["iscc"]` and `result.iscc` via `method_missing`
- Constants: `module.const_set("NAME", value)` in Magnus init
- Binary data: `RString` param + `unsafe { data.as_slice() }` — copy bytes before Ruby API calls
- Returning arrays: `ruby.ary_new_capa(n)` + `arr.push(val)?` for mixed-type arrays
- Test files: `test/test_smoke.rb`, `test/test_iscc_lib.rb`, `test/test_conformance.rb`
