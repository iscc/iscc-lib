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

## JS / WASM marshalling (moved from MEMORY.md, iter 277)

- WASM returns (234): byte chunks → `Vec<js_sys::Uint8Array>` (d.ts `Uint8Array[]`), never
    `serde_wasm_bindgen::to_value` (gives `number[][]`/`any`). Probe d.ts via `wasm-pack build`
- JS ints (256): napi numeric params are `f64` narrowed by `to_u32`-style helpers, never
    `u8`/`u32`/`i32`. WASM (268): every number/bool/number-array param is `JsValue` + `number`/
    `opt_number`/`number_vec`/`boolean`/`opt_bool` (lib.rs top), TS type kept via
    `unchecked_param_type` / `unchecked_optional_param_type`; an `Option<String>` BEFORE such a
    param needs its own `unchecked_optional_param_type = "string | null"` or loses `?` (269). JS
    reads only via `catch` imports (`Reflect::get`/`get_u32`); never `Array::from`/`Array::get`/
    `try_iter` on caller values (269, ~2x slower, flagged). Tests: `num()`/`nums()`/`js()` (eval) in
    unit.rs. Probe from Node on a RELEASE
    `wasm-pack build --release --target nodejs --out-dir   /tmp/...`; `.d.ts` check = diff
    JSDoc-stripped files vs `git archive <base>` build

## Moved from MEMORY.md (iter 277)

- Ruby floor 3.3 (270): system Ruby 3.1 is refused by Bundler. Dev Ruby pinned ONCE in root
    `mise.toml` `[tools] ruby` (271): `mise exec -- bundle exec rake compile test` (`LANG=C.UTF-8`);
    hooks + docs name no `ruby@X` (pin test in `tests/test_ruby_gem_docs.py`, regex not `tomllib`:
    CI python-test runs 3.10). minitest 6 = 168 runs / 481 assertions
- Devcontainer (271): `devcontainer.json` is `skip-worktree` (disk = local array form); edit HEAD
    copy in /tmp, `git update-index --cacheinfo` (CLEARS the flag), re-set `--skip-worktree` before
    `mise run check`. Untracked `post-create.sh` mirrors the committed `postCreateCommand`
