---
name: bindings-detail
description: Ruby source gem, Ruby i32 marshalling, .NET/C++/npm binding gotchas moved out of the advance MEMORY.md index
metadata:
  type: project
---

# Binding detail (Ruby, .NET, C++, npm)

## Ruby Bindings (Magnus) → MEMORY-archive.md (Magnus 0.8 API, never `Vec<T>` params)

- Ruby source gem (239): ONLY via `scripts/build_ruby_source_gem.py --out DIR` (standalone manifest
    \+ `cargo package` core under `core/` + workspace Cargo.lock); gemspec `rb_sys ~> 0.9`.
    Clean-home probe: from /tmp, `GEM_HOME=GEM_PATH=/tmp/x gem install` (~40 s). `rb_sys` dep is
    conditional on `__dir__/core` (240): never unconditional — breaks cross-gem's unanchored lock
    grep. Root `[profile.release]` copied as verbatim key lines (241, no tomllib: py3.10); clean
    install on 3.4 ≈55 s, `.so` stripped
- ruff RUF100 strips `# noqa: S603` in `tests/` (S rules off there) — no noqa on test subprocess
- Ruby `i32` elements (229): only via `array_to_i32_vec` (i64 then `i32::try_from`); Magnus
    `RBignum::to_i32` truncates negative Bignums. Rebuild: `bundle exec rake compile:dev`

## Other Bindings — pointers

- .NET `packages/dotnet/` (P/Invoke over `iscc_ffi`, `dotnet-version: 8.0`); C++ `packages/cpp/`
    (header-only C++17, CMake + ASAN; no system cmake but `uv run --with cmake` works — build into
    gitignored `build-uv/`). Detail → MEMORY-archive.md
- Embedded NUL (213): .NET `ToNativeUtf8` / C++ `detail::checked_c_str` throw; route every new
    string input through them (bare `.c_str()` / `GetBytes` truncates at NUL)
- C++ empty vectors (273): `safe_data` sentinel REMOVED; every vector input passes `.data()` (FFI
    reads `(NULL, 0)` as empty since 255). Empty-inner-row probe: test 42b in `test_iscc.cpp`
- .NET jagged arrays (220): `PinnedRows` in `IsccLib.cs`, arrays sized `Count` (padding removed
    273). Filter tests: `--filter FullyQualifiedName~X`
- npm metadata (214): `@iscc/wasm` package.json comes from `crates/iscc-wasm/Cargo.toml` via
    wasm-pack (0.13.1 resolves `license.workspace`); `@iscc/lib` package.json is committed as-is.
    taplo aligns trailing comments to the longest line in a block — put comments on their own line
