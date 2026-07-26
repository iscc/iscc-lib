# Learnings Archive

Archived learnings from completed project phases. Moved here to reduce per-iteration context
loading. Full history preserved in git. These entries are NOT loaded by CID agents — they are
reference-only for humans.

## Completed Binding Patterns

- IsccError enum was replaced with `iscc_lib::IsccResult<T>` (type alias for `Result<T, String>`)
    before the first binding was written — error strings carry enough context for debugging

- PyO3 requires `VIRTUAL_ENV` to be set via `env::var` or maturin won't find the venv; in CI, use
    `uv run maturin develop` which handles this automatically

- `maturin` discovers the PyO3 crate via `pyproject.toml` `[tool.maturin]` section — set
    `manifest-path` to `crates/iscc-py/Cargo.toml`

- PyO3 `#[pyo3(signature = (data, bits=64))]` syntax provides Python-side default arguments cleanly

- PyO3 constants registered via `m.add("CONST_NAME", value)?` in module init. Module-level constants
    in `__init__.py` must go AFTER imports (ruff E402)

- ty type checker needs `.pyi` stubs in the Python package directory alongside `__init__.py`, with
    identical function signatures. Without `.pyi` files, ty reports `Unknown` for all native
    bindings

- PyO3 type mappings: `&str` → Python `str`, `&[u8]` → `bytes`, `Vec<u8>` → `bytes`, `Vec<String>` →
    `list[str]`, `Option<&str>` → `str | None`, `&[Vec<i32>]` → `Sequence[Sequence[int]]`

- All 9 `gen_*_v0` PyO3 bindings follow the same pattern: receive args → call `iscc_lib::gen_*_v0` →
    `map_err(PyValueError::new_err)` → build `PyDict` from result struct fields

- Python conformance tests: load `data.json` from `crates/iscc-lib/tests/data.json`, use
    `@pytest.mark.parametrize` with computed test IDs from input data, decode `"stream:<hex>"`
    prefix for Data/Instance-Code tests

- CI workflow: `dtolnay/rust-toolchain@stable` before `Swatinem/rust-cache@v2` (action order matters
    — cache key uses Rust version from toolchain). Never use `mise` in CI — call tools directly

- `maturin` is declared as a build dependency in root `pyproject.toml`, NOT in
    `crates/iscc-py/pyproject.toml` — uv resolves it from the workspace root

- Python module name `iscc_lib._lowlevel` is set by `lib.name` in `crates/iscc-py/Cargo.toml` as
    `_lowlevel` and by `module-name` in `crates/iscc-py/pyproject.toml` as `iscc_lib._lowlevel`

- Criterion benchmarks: add `[[bench]]` section to Cargo.toml with `harness = false`. Individual
    benchmark files in `benches/` use `criterion_group!` and `criterion_main!` macros

- pytest-benchmark: Use `--benchmark-disable` flag by default in `pyproject.toml`
    `[tool.pytest.ini_options]` `addopts` to prevent benchmarks from running during normal test
    execution. Enable with `--benchmark-enable` when actually benchmarking

- napi-rs type mappings: `String` (owned, not `&str`), `Buffer` (for `&[u8]` and `Vec<u8>`),
    `Vec<Buffer>` for `Vec<Vec<u8>>`, `Vec<Vec<i32>>` maps directly

- napi bindings: `#[napi]` with `js_name` for snake_case → camelCase or custom names. Return
    `Result<T>` with `napi::Error::from_reason`. Use `Buffer::from(&slice[..])` for byte returns

- Node.js conformance tests: use `node:test` (`describe`/`it` structure), `node:assert` for
    assertions, `readFileSync` + `JSON.parse` for loading data.json. Sub-test IDs from `t.name` in
    `describe` callback. Stream hex decoding: `Buffer.from(hex, 'hex')`

- wasm-bindgen type mappings: `&str` → JS string, `&[u8]` → `Uint8Array`, `Vec<u8>` → `Uint8Array`,
    `Result<T, JsError>` → throws on error. Use `JsValue::from_serde` for complex returns, or
    individual field access via `Reflect::set` — prefer individual for type safety

- WASM crate (`iscc-wasm`) uses `cdylib` crate-type. Builds via `wasm-pack build` which handles
    wasm-bindgen glue generation. Target: `--target web` for ESM, `--target nodejs` for CJS

- `wasm-pack test --node crates/iscc-wasm --features conformance` — `--features` goes AFTER path,
    NOT after `--`. `--` passes args to wasm-bindgen-test-runner, not cargo

- WASM: `rlib` crate-type in Cargo.toml is needed alongside `cdylib` for `wasm-pack test` to work
    (tests compile as a library, not a cdylib). Without `rlib`, tests fail with linking errors

- WASM conformance: `include_str!("../../iscc-lib/tests/data.json")` embeds test data at compile
    time — no file I/O needed in WASM tests. Parse with `serde_json::from_str`

- C FFI: `thread_local!` stores last error string; `iscc_last_error` returns `*const c_char` to it.
    `iscc_free_string` frees owned `CString` pointers returned by other FFI functions. Caller must
    NOT free the error pointer (it's borrowed from thread-local)

- C FFI type mappings: `*const c_char` → C `const char*`, `*const u8` → `const uint8_t*`, `usize` →
    `size_t`, `bool` → `_Bool` (C11) or `stdbool.h`. Return owned `*mut c_char` for strings (caller
    frees via `iscc_free_string`)

- C test program: `gcc -o test_iscc tests/test_iscc.c -L../../target/debug -liscc_ffi` (Linux). Run
    with `LD_LIBRARY_PATH=../../target/debug ./test_iscc`. Header generated by
    `cbindgen --config cbindgen.toml --crate iscc-ffi --output tests/iscc.h`

- Zensical (MkDocs-based): `zensical.toml` for config, `docs/` for content. Build:
    `uv run zensical build`. Serve: `uv run zensical serve`. Deploy: GitHub Actions to `gh-pages`

- Rust `gen_*_v0` functions return named result structs (e.g., `MetaCodeResult`) with `iscc`,
    `iscc_id`, `name`, `description`, `meta` fields. The ISCC string is always in `.iscc`

- mkdocstrings-python + griffe: `show_source: false`, `show_root_heading: true`,
    `members_order: source` in `zensical.toml`. Requires `griffe` as explicit dependency (griffe is
    transitively required but must be pinned for compatibility)

- mdformat + mkdocs-material: `mdformat-mkdocs` plugin conflicts with `mdformat-gfm` — use
    `mdformat-mkdocs` only (it includes GFM support). Configure in `pyproject.toml`
    `[tool.mdformat]`

- napi-rs CI: no cross-compilation needed — `npx napi build --platform` builds native addon for
    current platform. For release, use `@napi-rs/cli` matrix builds per OS/arch

- WASM npm version fix:
    `node -e "const p = require('./package.json'); p.version = '0.0.1';   require('fs').writeFileSync('./package.json', JSON.stringify(p, null, 2)+'\n');"`
    — wasm-pack always writes `0.1.0` to package.json, so patching version in CI is needed

- `build_meta_data_url` helper is shared between `gen_meta_code_v0` and `json_to_data_url`. It
    builds `data:<mediatype>;base64,<payload>`. Media type is `application/ld+json` if `@context`
    key exists, else `application/json`. Payload is JCS-canonical → base64

- Python `IsccResult(dict)` subclass pattern: `result = IsccResult({...})` — supports both
    `result["key"]` (dict access) and `result.key` (attribute access via `__getattr__`). All 9
    `gen_*_v0` wrappers use this pattern

- `ty` type checker (PEP 695 aware) doesn't support `hasattr` for type narrowing — use `isinstance`
    instead. `hasattr(data, "read")` doesn't narrow to `BinaryIO` in ty, causing false positives

- Tier 1 symbols exposed via `pub use` in `crates/iscc-lib/src/lib.rs` (crate root re-exports). Tier
    2 symbols accessed via `iscc_lib::codec::*`, `iscc_lib::cdc::*`, etc. The module is `pub(crate)`
    so Tier 2 is crate-internal only

- Pre-push hooks: `cargo clippy -- -D warnings` runs in pre-push stage (not pre-commit). If clippy
    fails during push, fix locally and retry. Use `cargo clippy --fix -- -D warnings` for auto-fix

- PyO3 FFI boundary: `assert!`/`panic!` in Rust are caught by PyO3 and converted to
    `pyo3::PanicException`. For expected errors, use `Result<T, PyErr>` with `.map_err()` instead

- PyO3 streaming hasher pattern: `_DataHasher`/`_InstanceHasher` wrappers in `__init__.py` that hold
    a reference to the Rust-layer hasher. `gen_data_code_v0` Python wrapper detects
    `bytes`/`bytearray`/`memoryview` → direct call vs `BinaryIO` → chunked streaming with 64 KiB
    `_CHUNK_SIZE`. This two-path pattern (pass-through vs streaming) applies to both Data and
    Instance codes

- CID workflow: `mise run cid:run` executes up to 20 iterations. Each iteration runs 4 agents in
    sequence. The orchestrator reads context files between agents but doesn't modify them directly

- napi-rs build artifacts (`index.js`, `index.d.ts`, `*.node`, `node_modules/`) belong in the crate
    directory (napi-rs convention) — gitignore them via `crates/iscc-napi/.gitignore`

- napi streaming hashers use `JsFunction` callbacks (not `impl Fn` traits) — `this.call()` pattern
    with `Option<&str>` error + `Option<String>` result. Memory: Node.js GC handles JS objects; Rust
    `Box::new()` + `Box::into_raw()` for opaque pointers passed to JS via `External<T>`

- npm package naming: `@iscc/lib` (NOT `@iscc/iscc-lib`). WASM package: `@iscc/wasm` (NOT
    `@iscc/iscc-wasm`). Both are scoped packages under the `@iscc` org

- Zensical template overrides: `docs/.overrides/` directory. The `announce` partial at
    `docs/.overrides/partials/announce.html` adds a top banner. The CSS for it goes in
    `docs/stylesheets/extra.css`. In `zensical.toml`, add `extra_css = ["stylesheets/extra.css"]`

- JNI crate (`iscc-jni`): `crate-type = ["cdylib"]` produces `libiscc_jni.so` (Linux) /
    `libiscc_jni.dylib` (macOS) / `iscc_jni.dll` (Windows). Java wrapper class at
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` with native method
    declarations + `System.loadLibrary("iscc_jni")` in static initializer

- API hardening: when changing Tier 2 `pub(crate)` function signatures (e.g., adding validation,
    changing return type to `IsccResult`), all callers within the crate must be updated — usually
    just adding `?` propagation. The Tier 1 wrappers in `lib.rs` already use `IsccResult` so they
    propagate naturally

- Result struct pattern: each `gen_*_v0` returns a dedicated struct (e.g., `MetaCodeResult`,
    `TextCodeResult`) — not a generic struct. Each struct has only the fields relevant to that code
    type. All structs have `iscc: String` field; some add `iscc_id`, `name`, `description`, `meta`

- `gen_meta_code_v0` normalizes name/description via
    `text_trim(text_clean(input),   META_TRIM_NAME/DESCRIPTION)` BEFORE hashing — conformance
    vectors expect this normalization in the returned `name`/`description` fields

- `DataHasher` uses CDC for content-defined chunking + BLAKE3 for chunk hashing + MinHash for
    feature aggregation. The CDC → hash → MinHash pipeline runs in streaming fashion:
    `update(&chunk)` processes immediately, no buffering of the full input

- `DataHasher` buffer optimization: a persistent `buf: Vec<u8>` reuses across `update()` calls (no
    per-call `to_vec()`/`concat()`). Tail handling: `copy_within` + `truncate` shifts unfinished
    data to front of buffer. Benchmark: ~1.1 GiB/s at 64 KiB chunks

- `InstanceHasher` wraps BLAKE3 but outputs the ISCC-standard multihash format: 64-byte BLAKE3
    digest truncated to the requested bit_length. The hasher is stateful (accumulates data across
    multiple `update()` calls) and produces the code in `finalize()`

- `conformance_selftest` loads `data.json` at runtime (not compile time) and verifies every vector.
    Returns a human-readable string. The ISCC string comparison uses bitwise AND masking for
    truncated codes — do NOT compare full strings when bit_length < 256

- `decode_length` returns multiples of 32 bits for standard ISCC MainTypes (Meta, Semantic, Content,
    Data, Instance, Flake), multiples of 64 for ISCC-CODE, and multiples of 8 for ID —
    `bit_length / 8` in `iscc_decode` is always safe without remainder checking

## Publishing

- Packaging error "only one cdylib can be packaged" means Cargo.toml `[lib]` has
    `crate-type = ["cdylib", "rlib"]` but only one can ship to crates.io. Use `crate-type = ["lib"]`
    for the core crate (iscc-lib), keep `["cdylib"]` for binding crates

## Issue #21 — add_units/units Binding Patterns (COMPLETED)

- Python binding pattern: PyO3 wrapper accepts `&str` path → `Path::new(path)`, public wrapper adds
    `str | os.PathLike` via `os.fspath()`. `SumCodeResult(IsccResult)` class + `__all__` update.
    Wide mode test requires `bits=128` since 64-bit codes produce identical output in both modes
- Node.js binding pattern: `NapiSumCodeResult` struct with `#[napi(object)]` + `gen_sum_code_v0` fn
    with `Option<u32>`/`Option<bool>` params. Uses `i64` for filesize (napi-rs lacks u64 support).
    Tests use `node:test` + `node:assert` + temp files for I/O. Total: 135 tests (6 sum + 3 units)
- WASM binding pattern: `WasmSumCodeResult` struct with `#[wasm_bindgen(getter_with_clone)]` +
    `gen_sum_code_v0` fn accepting `&[u8]` (no filesystem in WASM). Uses `f64` for filesize (avoids
    `u64` → BigInt friction in JS). `add_units: Option<bool>` param + `units: Option<Vec<String>>`
    field (maps to `string[] | undefined` in TS). Total: 79 tests (9 conformance + 70 unit; 1 unit
    test behind `conformance` feature gate)
- C FFI binding pattern for units: `IsccSumCodeResult` uses `*mut *mut c_char` (NULL-terminated
    string array) for `units` — same representation as `iscc_decompose`/`iscc_sliding_window`.
    `vec_to_c_string_array` helper converts `Vec<String>` → C array; `iscc_free_string_array` cleans
    up. Error path frees `iscc` + `datahash` before returning null result. 85 Rust tests, 65 C tests
- JNI binding pattern: `SumCodeResult.java` (immutable, `String iscc`, `String datahash`,
    `long filesize`, `String[] units` nullable). JNI bridge returns `jobject` via `env.find_class` +
    `env.new_object` with signature `(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V`.
    `jboolean` is `u8` — compare `wide != 0`. Units via `build_string_array` →
    `unsafe { JObject::from_raw(arr) }`. 7 Maven sum tests. 65 total Maven tests
- Go binding pattern: `SumCodeResult` with `Units []string` + `addUnits bool` param. Pure Go (no
    FFI). Conditional `[]string{dataResult.Iscc, instanceResult.Iscc}` when `addUnits=true`, nil
    otherwise. 7 sum code tests total (4 existing + 3 units tests)

## Go Bindings — Pure Go Rewrite (COMPLETED)

- Go module path: `github.com/iscc/iscc-lib/packages/go`, package name `iscc`
- Conformance test path: `../../crates/iscc-lib/tests/data.json` (relative from packages/go)
- Go constants: `MetaTrimName`, `MetaTrimDescription`, `IoReadSize`, `TextNgramSize` are
    package-level `const` (idiomatic Go)
- `DecodeResult` struct: `Maintype`, `Subtype`, `Version`, `Length` (all `uint8`) + `Digest`
    (`[]byte`). Returned as `*DecodeResult` from `IsccDecode`
- Go uint32/uint64 arithmetic wraps naturally at overflow, matching Rust's wrapping_add/wrapping_mul
- Go `%` and `&` have equal precedence (both multiplicative), so `x % mprime & maxH` evaluates
    left-to-right as `(x % mprime) & maxH`, matching Rust
- Use `const` (not `var`) for scalar constants — Go supports constant expressions with bit shifts
- `golang.org/x/text/unicode/norm` for NFKC/NFD. `unicode.Is(unicode.C, c)` covers Cc, Cf, Co, Cs
- `TextRemoveNewlines` = `strings.Join(strings.Fields(text), " ")` (one-liner)
- `TextTrim` uses backward byte trimming until `utf8.ValidString` — simpler than Rust but identical
- CDC: `cdcGear` table is `var` not `const` (Go no const arrays). `min()` builtin since Go 1.21+
- MinHash: `mpa`/`mpb` arrays, `minhashFn` naming (avoids Go conflict). `maxi64`/`mprime`/`maxH` are
    `var` not `const` (Go uint64 shift limitation)
- SimHash: `AlgSimhash` returns `([]byte, error)`, `SlidingWindow` returns `([]string, error)`. Uses
    `[]rune` for Unicode-correct SlidingWindow
- DCT: `algDct` (unexported, `pub(crate)` in Rust). WTA-Hash: `AlgWtahash` (exported, `pub` in
    Rust). `wtaVideoIdPermutations` is `var` (Go no const arrays). All 7 algorithm modules complete
- DCT beta computation: Rust `/ cos / 2.0` vs Go `/ (cos * 2.0)` are numerically identical
    (verified) — multiplying cos ∈ [-1,1] by 2.0 is exact in IEEE 754
- Dependency order: codec → utils → algorithms → gen functions → streaming → conformance → cleanup
- Gen function test naming: `TestPureGo*` prefix is historical (from WASM coexistence phase). Could
    be renamed to `Test*` now that the WASM bridge is removed — cosmetic cleanup only
- JCS canonicalization: Go's `json.Marshal` suffices for string-only JSON values (sorted keys,
    compact format). A dedicated JCS library is needed only if float number formatting matters
- `SlidingWindow`/`AlgSimhash` error suppression (`_, _`) is safe in gen functions: width params are
    hardcoded valid constants (3 or 13), and AlgSimhash returns 32 zero bytes for empty input
- Go `DataHasher`/`InstanceHasher` Finalize is single-use (mutates internal state). Mirrors Python
    reference `_finalize()` which sets `self.tail = None`. Do not call Finalize twice
- Go pure rewrite is COMPLETE: 30/30 Tier 1 symbols, all 46 conformance vectors pass, zero WASM
    dependencies. Module deps: `github.com/zeebo/blake3`, `golang.org/x/text` (+ cpuid indirect)
- `DecodeResult` struct and algorithm constants (`MetaTrimName`, etc.) live in `codec.go` — the
    canonical location after WASM bridge removal

## gen_sum_code_v0 (COMPLETED)

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool)` is the 10th gen function
    and 32nd Tier 1 symbol. Single-pass file I/O feeds both `DataHasher` (CDC/MinHash) and
    `InstanceHasher` (BLAKE3) from the same buffer, then composes ISCC-CODE via `gen_iscc_code_v0`
- `SumCodeResult { iscc, datahash, filesize, units }` — `units: Option<Vec<String>>` contains
    `[Data-Code, Instance-Code]` ISCC strings when `add_units` is true. Borrow-before-move pattern:
    `gen_iscc_code_v0` borrows the strings, then they're moved into the vec (no clone needed)
- Binding propagation order: Python first (primary consumer), then Node.js/WASM/C FFI/Java, Go last
    (pure Go reimplementation needed — not a Rust wrapper)
- **`.pyi` stub must be updated alongside binding changes.** When adding/modifying parameters in
    `crates/iscc-py/src/lib.rs`, also update the corresponding signature in
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`. `ty check` (pre-push hook) will fail otherwise
- Binding-specific `add_units`/`units` patterns archived to `learnings-archive.md` (issue #21 fully
    resolved: all 7 bindings complete)

## Ruby Bindings — Magnus (COMPLETED)

- Magnus 0.7.1 works with Rust edition 2024 and Ruby 3.1.2. Magnus 0.8 requires Ruby 3.2+
- `extconf.rb` must be at crate root (not `ext/iscc_lib/`) — rb_sys `ExtensionTask` expects it next
    to `Cargo.toml`
- Cargo lib name must match package name (`iscc_rb`, not `iscc_lib`) — rb_sys derives the binary
    name from the package name. Ruby loads via `require_relative "iscc_lib/iscc_rb"`
- Root `.gitignore` has `lib/` pattern — need `!lib/` negation in `crates/iscc-rb/.gitignore`
- `bundler` not on PATH by default in devcontainer — need `$HOME/.local/share/gem/ruby/3.1.0/bin` on
    PATH
- Streaming classes use `#[magnus::wrap(class = "IsccLib::ClassName")]` + `RefCell<Option<inner>>`
    (Magnus gives `&self`, not `&mut self`). Ruby `class ClassName` inside `module IsccLib` reopens
    the native class. Method prefix `_update`/`_finalize` works; class prefix `_DataHasher` does NOT
    (Ruby constants must start with uppercase)
- `libclang-dev` required for rb-sys/bindgen to compile
- Standard Ruby linting: `standard` gem + `rubocop-minitest` plugin. Config at `.standard.yml` (not
    `.rubocop.yml`). `mise run check` now runs 15 hooks (incl. Ruby auto-fix). Pre-commit hook uses
    portable `ruby -e "puts Gem.user_dir"` for PATH resolution since `bundle` isn't on system PATH
- Ruby `JSON.generate` silently ignores `sort_keys: true` — use `meta_val.sort.to_h` before
    `JSON.generate` for sorted-key output. Python `json.dumps(sort_keys=True)` works as expected
- Gem dev-dependency work (iter 130): bundler has no `-C` flag — use
    `(cd crates/iscc-rb && bundle …)` with `$(ruby -e "puts Gem.user_dir")/bin` on PATH. CI's
    `ruby/setup-ruby` `bundler-cache: true` performs a **frozen** install, so prove Gemfile/lock
    consistency locally with `BUNDLE_FROZEN=true bundle install --local`

## Go/wazero Bridge (OBSOLETE)

- Go module path is `github.com/iscc/iscc-lib/packages/go`, package name `iscc`

- `text_clean` does NOT collapse double spaces within a line — use NFKC ligature normalization
    (e.g., fi ligature U+FB01 → "fi") for test cases instead of space-collapsing expectations

## CI/CD — Binding-Specific Release Details (archived iteration 5)

- **WASM conformance_selftest**: requires `--features conformance` in `wasm-pack build` — the export
    is gated behind `#[cfg(feature = "conformance")]` in the WASM crate. NAPI and Python export it
    unconditionally
- **NAPI js_name**: binding uses `#[napi(js_name = "conformance_selftest")]` — snake_case is
    preserved in the raw .node export. Smoke test can `require()` the .node file directly
- **RubyGems trusted publishing (OIDC)**: uses `rubygems/configure-rubygems-credentials@main` with
    `id-token: write` permission. No API keys needed. Configured on rubygems.org as trusted
    publisher
- **Ruby cross-gem action quirk**: `oxidize-rb/actions/cross-gem@v1` configure step greps
    `Gemfile.lock` in repo root (ignores `working-directory`). For subdirectory gems, symlink the
    lockfile: `ln -sf crates/iscc-rb/Gemfile.lock Gemfile.lock`
- **Ruby gem dev deps** (iter 130, archived iter 134 — dep-refresh slice 7 done): `rb_sys` stays
    pinned EXACTLY at 0.9.123 to match `tag: 0.9.123` of `oxidize-rb/actions/cross-gem` in
    `release.yml` (that rb_sys pins `rake-compiler-dock = 1.10.0`); `minitest ~> 5.0` is held
    because 6.0.x requires Ruby ≥ 3.2 vs the gem's declared 3.1.0 floor. Both reasons live in inline
    `# held:` comments in `crates/iscc-rb/Gemfile`
- **Windows GHA runners default to `pwsh`** (archived iter 154 — the cross-platform matrix is
    settled): any `run:` step using bash syntax (`$(...)`, `$GITHUB_OUTPUT`, `grep`, `sed`) in a
    cross-platform matrix MUST set `shell: bash`

## .NET Bindings (P/Invoke) — Completed Phase (Iteration 9)

- DLL name `"iscc_ffi"` — .NET auto-resolves to `libiscc_ffi.so` (Linux), `iscc_ffi.dll` (Windows),
    `libiscc_ffi.dylib` (macOS). No platform-specific code needed
- `[return: MarshalAs(UnmanagedType.U1)]` required for C `bool` → C# `bool` marshaling
- `dotnet test` requires `-e LD_LIBRARY_PATH=<path>` to pass library path to vstest host child
    process; shell-level env var alone is insufficient. CI needs `env:` on the test step
- .NET 8 SDK install in Dockerfile: Microsoft install script to `/usr/share/dotnet` (system-wide,
    before non-root user section)
- csbindgen (v1.9.7) in `build.rs` generates C# bindings from `extern "C"` functions. Uses
    `input_extern_file("src/lib.rs")` — parses `#[unsafe(no_mangle)]` (Rust 2024 edition) correctly.
    Unlike cbindgen (CLI tool), csbindgen runs in build.rs and writes directly to repo path
- `NativeMethods.g.cs` is `internal` class with 47 P/Invoke declarations, 6 struct types. Generated
    file uses `byte*` for C strings, `nuint` for `usize`, `[MarshalAs(UnmanagedType.U1)]` for bools
- `AllowUnsafeBlocks` required in `.csproj` for csbindgen's `byte*` pointer types
- Marshaling pattern: `ToNativeUtf8` (C# string → null-terminated UTF-8 `byte[]`),
    `ConsumeNativeString` (native `byte*` → managed `string` + `iscc_free_string`), `GetLastError`
    (reads `iscc_last_error()` without freeing — thread-local storage).
    `fixed (byte* p = nullArray)` sets pointer to null for optional parameters
- `dotnet test -e LD_LIBRARY_PATH=target/debug` with relative path fails in devcontainer — must use
    absolute path. CI is unaffected (uses `env:` which resolves correctly)
- **Empty span `fixed` null pointer**: C# `fixed (T* p = emptySpan)` produces NULL — FFI layer
    rejects NULL. Guard with `if (span.IsEmpty) { T sentinel; use &sentinel with length 0 }`.
    Applied to all 7 affected functions
- C# disallows pointer types (e.g., `byte**`) as generic type arguments — string array marshaling
    must be inlined per-method
- `IsccSumCodeResult` struct is at the namespace level in `NativeMethods.g.cs` (not nested)
- `ConsumeNativeStringArray`: shared helper for NULL-terminated `byte**` → `string[]` marshaling
- `IsccDecode` returns `DecodeResult` record; digest copied via `Span<byte>` before native free
- Streaming hashers (`IsccDataHasher`, `IsccInstanceHasher`): `SafeHandle` nested class +
    `IDisposable` on the outer class. `DangerousGetHandle()` is acceptable for single-threaded use
- **C# structured result records**: `Results.cs` holds all 11 sealed record types
- **NuGet .csproj README path**: `Include="../../README.md"` from `packages/dotnet/Iscc.Lib/`
    resolves to `packages/README.md` (wrong), not `packages/dotnet/README.md`. Use `../README.md`
- **NuGet native lib packaging**: cross-architecture find pattern must scope by target name
    (`-path "*-${target}/*"`) to avoid copying wrong-arch libraries when multiple targets share the
    same lib name (e.g., both linux-x64 and linux-arm64 produce `libiscc_ffi.so`)

## .NET Bindings

- Detailed P/Invoke patterns archived to `learnings-archive.md` (iteration 9 — .NET bindings
    completed). Key reference items preserved here for CI/release workflows only
- NuGet publish pipeline: 7 registry inputs total (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget). `build-ffi` shared between FFI and NuGet via `inputs.ffi || inputs.nuget`
- **Cross-architecture find bug pattern**: when extracting multi-target archives to the same CWD,
    `find -path "*/prefix-v*/*"` matches ALL targets. Scope by target name: `-path "*-${target}/*"`
- **.csproj relative paths**: `Include` paths are relative to csproj location, not project root.
    Count `../` carefully — `packages/dotnet/Iscc.Lib/../../README.md` = `packages/README.md` (NOT
    `packages/dotnet/README.md`)

## UniFFI Bindings (Kotlin-specific, archived iteration 5)

- Kotlin bindings: UniFFI generates `package uniffi.iscc_uniffi` — JVM project uses
    `src/main/kotlin/` (not KMP `src/commonMain/kotlin/`). Gradle wrapper (gradle-wrapper.jar ~44KB)
    and generated `iscc_uniffi.kt` (~112KB) both under 256KB large-file threshold
- Kotlin conformance tests: JUnit 5 + Gson deps in build.gradle.kts. JNA native lib loading requires
    `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env var — `java.library.path` alone does
    NOT work for JNA `Native.register()`. `HexFormat` requires Java 17+

## C++ Wrapper (archived from learnings.md — section fully met)

- C++ `std::vector<T>::data()` returns nullptr for empty vectors on some implementations
    (libstdc++). The `safe_data()` helper has two overloads (uint8_t and int32_t) in `detail`
    namespace. All nested vector loops now use `detail::safe_data()` — both top-level and inner
    elements are covered (11 total occurrences in iscc.hpp: 2 definitions + 9 call sites)
- C++ wrapper lives in `packages/cpp/` — header-only, depends on `iscc-ffi` shared library. No
    separate Rust crate. CMake references `iscc.h` from `crates/iscc-ffi/include/` via include paths
- C++ CI job: `cmake` needs explicit `apt-get install`, `g++` is pre-installed on `ubuntu-latest`.
    Uses `working-directory: packages/cpp` for cmake steps
- FFI tarball flat layout vs CMake include path: `iscc.hpp` and `iscc.h` are flat in FFI tarballs.
    CMake uses `#include <iscc/iscc.hpp>`. Tarball consumers use `#include "iscc.hpp"` with `-I`

## UniFFI Bindings (archived from learnings.md — section fully met)

- `crates/iscc-uniffi/` uses proc macros only — no UDL files, no `build.rs`. `uniffi.toml` only
    needed for binding gen customization, not compilation
- UniFFI requires owned types (`String`, `Vec<u8>`), no `usize` or `const` exports — use `u64` for
    sizes and getter functions for constants
- UniFFI Objects need `Send + Sync` — use `Mutex<Option<Inner>>` (not `RefCell`)
- Binding generation: `uniffi-bindgen.rs` (3-line entry point) +
    `[features] bindgen = ["uniffi/cli"]`+`[[bin]] required-features = ["bindgen"]` pattern
- Swift tests require macOS runner — cannot execute in Linux devcontainer

## Devcontainer Scripts (exec bit / Windows bind mount) — full incident (archived from learnings.md)

- The working tree lives on a Windows bind mount with `core.fileMode = false`, so git ignores
    on-disk exec bits and keeps the indexed mode (e.g. `100755`). When an agent rewrites a script
    via Edit/Write, the new on-disk file is `0644` (no exec bit) but `git status` stays clean — the
    lost exec bit is invisible to git. Commit `e848887` did exactly this to
    `.devcontainer/setup-codex.sh`, so `postCreateCommand` hit "Permission denied" (exit 126) when
    invoking it as `.devcontainer/setup-codex.sh`. Because of `&&` chaining, that aborted everything
    after it (mise trust → untrusted error, uv sync → missing venv) and left codex unseeded (login
    prompt).
- Rule: in `postCreateCommand`, invoke shell scripts via `bash .devcontainer/foo.sh`, never
    `.devcontainer/foo.sh` — `bash <file>` needs only read permission, so it is immune to the
    dropped exec bit. Make convenience steps (e.g. codex auth seeding) non-fatal
    (`{ bash ... || echo skipped; }`) so they can never abort the critical setup chain.

## PyO3 GIL release (`py.allow_threads`) — archived from learnings.md (#39 closed iter 91)

- Inject `py: Python<'_>` into a `#[pymethods]` `update()` — PyO3 auto-supplies it, so it's
    invisible to Python and `_lowlevel.pyi` stays unchanged (`ty check` confirms). Take the
    `&mut inner` borrow + finalized check BEFORE releasing; release only around the pure compute
    (keep `PyDict` build outside). `&[u8]`/`&mut *Hasher` are `Ungil + Send`, no copy needed. Sound
    because `__init__.py` coerces inputs to immutable `bytes` and `_lowlevel` is private (no public
    path hands a mutable buffer to the released borrow).

## Kotlin JAR Artifact Selection — archived from learnings.md (Kotlin bindings fully met)

- Gradle `withSourcesJar()` + `withJavadocJar()` produces 3 JARs in `build/libs/`. When uploading
    `*.jar` globs and then selecting with `ls | head -1`, alphabetical ordering picks `-javadoc.jar`
    before the runtime JAR. Always filter out classifier JARs (`-sources`, `-javadoc`) when
    selecting the runtime artifact.

## JNA / Kotlin Android — archived from learnings.md (Kotlin/Android bindings fully met)

- **JNA ARM32 resource prefix is `android-arm`, NOT `android-armv7`**: JNA 5.16.0's
    `Platform.getNativeLibraryResourcePrefix()` canonicalizes all `arm*` architectures to `arm`.
    Verified by decompiling `Platform.class`. Other Android prefixes are correct: `android-aarch64`,
    `android-x86-64`, `android-x86`
- `cargo-ndk` outputs to `target/<rust-triple>/release/` — same path convention as desktop builds,
    so artifact upload steps work unchanged

## Swift Package (archived iter 94 — Swift bindings fully met)

- Two `Package.swift` files coexist: root (SPM consumers) and `packages/swift/Package.swift` (CI/
    local dev). SPM reads root for dependency resolution; `cd packages/swift && swift build` uses
    the subdirectory one
- Docs site URL is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/`. Advance agents must
    use correct hostname when linking to howto guides

## Binding Propagation (archived iter 96 — all bindings met)

- Java `META_TRIM_*` constants are pure Java `public static final int` (no JNI call needed). Go
    constants are `const` in `codec.go`. Both follow existing pattern of `META_TRIM_DESCRIPTION`
- When adding FFI constants, update the algorithm constant count in the module docstring
    (`crates/iscc-ffi/src/lib.rs` line 5)
- **napi bundled single-package model (no `optionalDependencies`)**: `napi prepublish -t npm` is the
    *only* thing that injects per-platform `optionalDependencies` (`@iscc/lib-<triple>`) into
    `package.json` at publish time — never published, so they 404 on install and break `npm ci`. The
    bundled model ships all 5 `.node` in one tarball via `files: ["*.node"]`; the generated
    `index.js` loader `require`s the local `./iscc-lib.<triple>.node` first. Do NOT run prepublish.
    Revisit per-platform model only if tarball > ~30 MB (spec: `nodejs-bindings.md`). PyO3
    GIL-release detail archived (#39 closed)
- NAPI `index.js` and `index.d.ts` are gitignored (`crates/iscc-napi/.gitignore`) and auto-generated
    by `napi build`. CI runs `napi build` before `npm test`. Do NOT manually edit or commit these
    files — they regenerate with new constants automatically

## Documentation Maintenance (archived iter 98 — completed doc one-offs)

- After major architecture changes (e.g., WASM→pure Go), CI workflows, READMEs, and howto guides go
    stale simultaneously — group the cleanup into a single step targeting all affected files
- Java requires JDK 17+ (pom.xml `maven.compiler.source/target` = 17), not 11+. Always cross-check
    version claims in docs against actual build config files
- WASM tab snippets need `await init()` before any WASM call in standalone examples (omit only in
    sequential examples where init was already shown)
- **cbindgen `iscc_` prefix on types**: `cbindgen.toml` has `[export] prefix = "iscc_"` but
    `[fn] prefix = ""`. All type names in C code examples must use `iscc_`-prefixed forms
    (`iscc_FfiDataHasher`, `iscc_IsccSumCodeResult`, etc.) while function names are un-prefixed
    (`iscc_data_hasher_new`). The `c-ffi-api.md` reference page uses short names for exposition but
    howto code examples must be compilable

## Completed: PyO3 Migration Arc 0.23 → 0.29 (issue #1 closed, iter 105)

- `pyo3` lives only in root `Cargo.toml` `[workspace.dependencies]`, used by `iscc-py` alone.
    Migrated one minor per CID step. Per-hop recipe: bump pin → `cargo update -p pyo3` →
    build/clippy(`-D warnings`)/fmt → `uv run maturin develop` → `uv run pytest` (286 tests) — AND
    diff the macros-backend default-handling, not just compiler warnings.
- 0.23→0.24 and 0.24→0.25: ZERO source edits.
- **0.25→0.26 (FIRST edit hop)**: `Python::allow_threads` → `Python::detach` (pure rename, same
    GIL-release semantics; 7 sites) + `pyo3::PyObject` alias → `Py<PyAny>` return type (17 sites).
- **0.26→0.27 (SECOND edit hop)**: cast-family rename in `to_pylist` — `Bound::downcast` →
    `Bound::cast`, `downcast_into_unchecked` → `cast_into_unchecked` (identical signatures; error
    type `DowncastError` → `CastError` discarded by `if let Ok`).
- **0.27→0.28 (iter 104)**: compiled clean (zero deprecation edits) BUT carried a SILENT behavior
    change `-D warnings` does NOT catch — PyO3 0.28 flipped the unspecified `#[pymodule]` `gil_used`
    default `true` (macros-backend 0.27 `map_or(true,…)`) → `false` (0.28 `is_some_and(…)`). On
    free-threaded CPython source builds the module then imports WITHOUT re-enabling the GIL — unsafe
    for the raw borrowed `PyList_GetItem` pointers in `extract_frame_sigs`. Fix: explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) restores pre-0.28 semantics
    (no-op on GIL-enabled/abi3-published wheels; only matters for from-source free-threaded builds).
- **0.28→0.29 (FINAL, iter 105)**: ZERO source edits; lib.rs unchanged. 0.29.0 CHANGELOG ships both
    targeted RustSec advisory fixes (missing `Sync` on `PyCFunction::new_closure` #6096; OOB read in
    `BoundListIterator`/`BoundTupleIterator` `nth`/`nth_back` #6086). `pyo3-macros-backend` dropped
    its `pyo3-build-config` dep (internal, harmless). raw `pyo3::ffi::*` + `Bound::from_owned_ptr`
    stable through every hop. Advisory clearance could NOT be tool-confirmed —
    `cargo audit`/`cargo   deny` absent from devcontainer + CI; mechanical proxy used (lockfile
    resolves single 0.29.0).

## CI/CD — semver + coverage gates (archived iteration 108, fully landed)

- **`semver` CI job** (`ci.yml`, iter 93): `obi1kenobi/cargo-semver-checks-action@v2`,
    `package: iscc-lib`, baseline = last crates.io release. INFORMATIONAL pre-1.0 via
    `continue-on-error: true` — reports the post-0.4.0 `pub(crate)` narrowing as 2 major checks
    failed (expected, not a regression). `mise run semver` runs it locally. Becomes enforcing at
    v1.0.0 by dropping `continue-on-error`; `rust-core.md` line 372 checkbox stays `[ ]` until then.
- **`coverage` CI job** (`ci.yml`, iter 94, ci-cd.md Phase 1): standalone, no `needs:`, NO
    `continue-on-error`. `dtolnay/rust-toolchain@stable` w/ `components: llvm-tools-preview` →
    `taiki-e/install-action@v2` (`tool: cargo-llvm-cov`) →
    `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` → upload-artifact (`name: lcov`).
    `mise run coverage` mirrors it locally; `lcov.info` is gitignored (137KB / 5156 lines).

## CI/CD — CRAP gate full mechanics (archived iteration 108)

- **CRAP gate (iter 96 Phase 2 + iter 97 Phase 3, ci-cd.md)**: `Coverage + CRAP` job installs
    `cargo binstall -y --force cargo-crap@0.2.2` (`--force` LOAD-BEARING — rust-cache poisoning),
    runs report-only `--format github` + `--format sarif` (`upload-sarif@v3`, job-level
    `security-events: write`), then an ENFORCING final step
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` (NOT
    continue-on-error). `.crap-baseline.json` (repo root, COMMITTED, NOT gitignored — only
    `lcov.info`/`crap.sarif` are): envelope `{$schema, version, entries}`, 97 iscc-lib functions /
    10 files. `mise run crap:baseline` regenerates it byte-identical (idempotent).
    `.cargo-crap.toml` `threshold=30`, `missing="pessimistic"`, MUST list
    `crates/iscc-lib/benches/**` explicitly (the built-in `benches/**` default only matches
    repo-root, else `bench_cdc_chunks` leaks at CRAP 42).

## iai-callgrind perf gate — full saga (iter 107-109, #3 complete pending CI confirm)

- **STRIP zero-collection bug (iter 107 mis-diagnosed, iter 108 FIXED)**: `[profile.bench]` DOES
    inherit `strip = true` from root `[profile.release]` (Cargo: bench profile is based on release).
    With no override the bench binary is `stripped` / **0** `__iai_callgrind_wrapper` symbols →
    iai's `--toggle-collect=*::__iai_callgrind_wrapper_mod::*` matches nothing → every bench
    `summary: 0` while exiting 0 (FALSE GREEN; CI run 27742285656 had all-zero `.out`s). The
    iter-107 review's "bench doesn't inherit release strip" + "valgrind absent locally" claims were
    both WRONG. FIX: `[profile.bench] strip = false, debug = true` → `not stripped` / **11** symbols
    → real counts. CI guard `grep -rEq '^summary: [1-9]' target/iai/` fails the job on zero
    collection.
- **Perf job structure (iter 107)**: standalone `perf` job (no `needs:`, NO `continue-on-error`):
    apt valgrind → cargo-binstall → `cargo binstall -y --force iai-callgrind-runner@0.16.1`
    (`--force` load-bearing, rust-cache poisoning) → `cargo bench -p iscc-lib --bench iai_benches` →
    guard step → `Check perf regression` (slice 2b) → upload `target/iai/` (`if: always()`).
- **Slice 2b (iter 109)**: `scripts/iai_regression.py` (stdlib-only) + committed CI-sourced
    `.iai-baseline.json` (16 Ir entries, NOT gitignored) + `bench:iai:baseline`/`bench:iai:check`
    mise tasks. Local 1.96.0 vs CI-stable Ir agree within 1.66%. KNOWN false-green edges (filed as
    [review] issue): single-bench `summary: 0` reads as improvement & passes (guard only catches
    ALL-zero); a baselined bench that stops emitting `.out` only warns, never fails.

## CID Process (archived from learnings.md)

- **Advisor tool evaluated and deferred (2026-07, full rationale)**: the Claude Code advisor
    (`--advisor` / `advisorModel`) was assessed for the CID loop and rejected for now. A Fable 5
    main model accepts only a Fable advisor and Fable is not currently offered as one, so `advance`
    — the role that would benefit most — cannot use it. For the Opus roles the only pairing is
    Opus-advising-Opus, which duplicates what the review role and the Codex second opinion already
    provide, at extra cost (each advisor call re-reads the full transcript uncached and counts
    against subscription limits, with model-driven, uncappable timing). Revisit when Fable 5 becomes
    selectable as an advisor (`/advisor` picker no longer shows it as unavailable) — then Fable-main
    \+ Fable-advisor on `advance` is the configuration worth testing.

## CI/CD (archived from learnings.md)

- **`cargo binstall` + `Swatinem/rust-cache` poisoning (iter 100)**: rust-cache restores install
    metadata without the `~/.cargo/bin/<tool>` binary → plain `cargo binstall -y <tool>` skips and
    the next call dies `no such command` → CI RED. Fix: add `--force` (gate strengthening, not
    circumvention).

## Feature Flags — blake3 WASM SIMD backend (archived iter 119, #42 met)

- **blake3 WASM SIMD backend — RESOLVED (iters 117-118, #42)**: the `blake3/wasm32_simd` **Cargo
    feature** (direct `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep on iscc-wasm,
    feature-unification only) ACTIVATES the backend — build.rs emits `blake3_wasm32_simd` (→
    `Platform::detect()` = `WASM32_SIMD`) from `CARGO_FEATURE_WASM32_SIMD` on wasm32 only (native
    inert). Honest wiring proof:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` shows
    `wasm32_simd`. The global `-C target-feature=+simd128` RUSTFLAGS is NOT required to compile the
    backend (blake3's SIMD fns carry `#[target_feature(enable = "simd128")]`; a no-flag
    `wasm-pack build` compiles + emits `v128`) — it broadens simd128 to the whole crate (auto-vec of
    CDC/xxh32/minhash) and stays in CI/release; `--enable-simd` needed for wasm-opt. `v128`-opcode
    counting ALONE is a FALSE-POSITIVE for "backend active" (LLVM auto-vec emits it too)

## Algorithm — ISCC-IDv1 (archived iter 124, #43 met, Go-only)

- **ISCC-IDv1** (`gen_iscc_id_v1`, experimental, NOT in ISO 24138): 64-bit body
    `= (timestamp << 12) | hub_id`, timestamp = 52-bit µs-since-epoch (`< 2^52`), hub_id = low 12
    bits (0–4095). Header nibbles: MainType=ID(6), SubType=realm_id (0=test/1=operational),
    Version=1, length-index=0; body big-endian, base32, `ISCC:` prefix. Known vector:
    `ISCC:MAIGHFECJMOPMIAB` → realm 0, hub 1, ts 1751831876325218. Go-only port in
    `packages/go/iscc_id.go` (built via internal `encodeHeader`/`encodeLength`, NOT public
    `EncodeComponent`); other 11 bindings lack it. Rust core rejects ISCC-IDv1 at header level
    (`codec::Version` is V0-only)

## CI/CD — aarch64 Python wheels (archived iter 124, #49 met)

- **Adding a Python wheel target (iter 123, #49)**: only the `build-wheels` + `test-wheels` matrices
    need the new entry — `publish-pypi`'s "Download all artifacts" uses `pattern: wheels-*` +
    `merge-multiple: true`, so any `wheels-${{ matrix.os }}-${{ matrix.target }}` artifact is
    auto-collected and published (no publish-step edit). Native-ARM wheels build on
    `ubuntu-24.04-arm` (free GH runner, no QEMU/`container:`). release.yml-only changes can't be
    exercised by CID pushes (only `workflow_dispatch`+pypi) → verify statically: YAML parse + matrix
    presence + artifact-name consistency across the build→test→publish chain

## Documentation — "10 gen functions vs 9 conformance" (archived iter 124)

- **"10 gen functions" vs "9 conformance functions"**: iscc-lib has 10 `gen_*_v0` functions, but
    `data.json` conformance vectors cover only 9 (no gen_sum_code_v0). Files that test/benchmark
    against data.json should say "9"; general library descriptions should say "10". Avoid blanket
    "9→10" find-and-replace — it corrupts conformance-scoped files. iscc-core-ts also implements
    only 9 (no gen_sum_code_v0) — verify external projects' function tables before claiming "all 10"

## Feature Flags (archived iter 127 — section fully met, features stable since #42)

- `iscc-lib` features: `default = ["meta-code"]`, `text-processing` (unicode deps), `meta-code`
    (implies text-processing + JCS canonicalizer). Three deps are optional
- When gating `pub(crate)` functions behind features, their tests must also be gated — clippy
    `-D warnings` catches dead code in library builds even if test modules reference them
- Gate individual test functions with `#[cfg(feature = "...")]`, not the whole `mod tests` block,
    when the block contains both gated and ungated tests
- `serde_json` stays non-optional because `conformance.rs` uses it for parsing data.json vectors
- **`--no-default-features --all-targets` fails on the `benchmarks` bench** (pre-existing): benches
    import `gen_meta_code_v0`/`gen_text_code_v0` needing `meta-code`/`text-processing`. Lib + tests
    build fine. Scope clippy to the lib (`--no-default-features -- -D warnings`, no `--all-targets`)
    to avoid a false regression. CI never runs this combo
- **blake3 WASM SIMD backend — RESOLVED (#42)**: activated by the `blake3/wasm32_simd` Cargo feature
    (not `-C target-feature=+simd128`); `v128`-opcode counting alone is a FALSE-POSITIVE. Full
    recipe under "Feature Flags — blake3 WASM SIMD backend" above

## ISCC Algorithm Internals (archived iter 128 — all 10 gen\_\*\_v0 functions conformance-complete)

Second batch archived iter 131 — settled API-parameter facts, all re-derivable from
`crates/iscc-lib/src/`:

- `META_TRIM_META` validation: pre-decode check (`META_TRIM_META * 4/3 + 256`) applies to ALL meta
    strings (both Data-URL and JSON) as a fast-path optimization. Post-decode check on
    `payload.len()` guarantees correctness. JSON boundary test overhead: `{"x":""}` = 8 bytes

- `gen_image_code_v0` pixels parameter is a flat `&[u8]`, NOT `&[i32]`. Chromaprint provides `i32`
    audio fingerprints (for `gen_audio_code_v0`), not image pixels

- MainType Ord: MainType enum values are ordered for consistent processing. META=0, SEMANTIC=1,
    CONTENT=2, DATA=3, INSTANCE=4, ISCC=5, ID=6, FLAKE=7

- JSON `meta` parameter: uses JCS (RFC 8785) canonicalization. `@context` key triggers
    `application/ld+json` media type, otherwise `application/json`

- `alg_simhash` output length equals input digest length (e.g., 4 bytes for 4-byte digests). Returns
    32 zero bytes only for empty input. NOT always 256 bits

- `gen_instance_code_v0` accepts `bits` but ignores it — always produces 256-bit output (the hash of
    the full content). The `bits` parameter exists for API consistency only

- `gen_iscc_code_v0`: `wide` parameter determines 128-bit (default) or 256-bit combination. Data and
    Instance components are always included; content code is optional. Test vectors in data.json
    have no `wide` field — always pass `false`

- ST_ISCC SubType: for `gen_iscc_code_v0`, the SubType in the ISCC header is determined by the
    content code's SubType (TEXT/IMAGE/AUDIO/VIDEO/MIXED). When no content code is provided, SubType
    is NONE (0). SubType SUM (5) is used for `iscc_sum` (multi-asset aggregation, not in gen_iscc)

- `soft_hash_meta_v0` interleaves name and description features at the nibble level. Trim lengths
    are in bytes, not characters. The returned bytes are the raw SimHash digest

- `gen_text_code_v0` uses MinHash (not SimHash) for the content hash portion. `alg_minhash_256`
    produces 256 bits (32 bytes) from a set of n-gram features. Text n-gram size = 13 (characters)

- `gen_data_code_v0` uses MinHash on CDC chunk hashes. CDC splits binary data into content-defined
    chunks, each chunk is xxh32-hashed (not BLAKE3), the set of chunk hashes is MinHash'd

- `soft_hash_audio_v0` is a 3-stage hash: Chromaprint i32 array → 4-byte big-endian digests →
    SimHash (overall 4B + quarters 16B + sorted thirds 12B) = 32 bytes total

- `gen_mixed_code_v0` processes multiple content codes: sorts by MainType, groups by SubType,
    soft-hashes each group, then SimHash across groups. The input is a list of ISCC strings (units),
    not raw data

- `encode_units` produces a single bitfield encoding an ordered list of content components included
    in an ISCC-CODE. Used by `gen_iscc_code_v0` to record which units were combined

- DCT uses Nayuki's algorithm (not FFTW/scipy). Image-Code: 8×8 pixel blocks → per-block DCT →
    WTA-Hash across blocks. Video-Code: per-frame DCT → WTA-Hash per frame → SimHash across frames

## CI/CD — JVM test/publish + Gradle bind-mount flakes (archived iter 129, dep-refresh slice 5 done)

- **JVM test/publish gotchas** (iter 128): junit-jupiter ≥ 5.12 under Gradle 8.12.1 needs an
    explicit `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.x.y")` (Gradle injects a
    launcher predating platform 1.12 → "OutputDirectoryCreator not available"); Maven/surefire
    resolves the aligned launcher itself. `mvn -Prelease package -DskipTests` does **not** resolve
    `maven-gpg-plugin` (verify-phase; absent from `~/.m2`) — prove a plugin version exists with a
    `repo1.maven.org` `.pom` HTTP 200, not from build success.
    `./gradlew generatePomFileForMavenPublication` → `build/publications/maven/pom-default.xml`
    proves test-scope deps do not leak into the published artifact
- **Gradle flakes on the workspace bind mount**: `Unable to delete file …/build/kotlin/…` or
    `NoSuchFileException …/build/reports/tests/test/packages` are incremental-state races, not test
    failures (`build/test-results/test/*.xml` still showed `tests="9" failures="0"`). Re-run after
    `./gradlew clean` before concluding anything about a build

## CI/CD — Go module data-table differential (archived iter 130, dep-refresh slice 6 done)

- **Prove a `golang.org/x/text` bump is output-neutral, don't infer it from green vectors** (iter
    129): build a throwaway module in `/tmp` with
    `replace github.com/iscc/iscc-lib/packages/go => <repo>/packages/go`, dump
    `TextClean`/`TextCollapse` for all 1,112,032 code points, then re-run under
    `replace golang.org/x/text => golang.org/x/text v<old>` and `diff`. 0.34.0 → 0.40.0 was
    byte-identical (same `unicode/norm` tables; only invalid-rune bookkeeping changed). A new
    indirect (`golang.org/x/sys` via cpuid 2.4.0) is legitimate when `go mod tidy -diff` exits 0

## Release Pipeline (archived iter 133 — target section met)

- **Release pipeline pattern**: 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift) → build → smoke test → publish; 6 smoke-test jobs
    (test-wheels/napi/wasm/gem/jni/ffi) gate publish on the linux-x86_64 artifact; re-trigger a
    single registry with `--ref main`. `version_sync.py` manages 21 targets (`--check` exits 1 on
    mismatch). Adding a Python wheel target touches the build + test matrices only — `publish-pypi`
    collects wheels via `pattern: wheels-*`.
- **PyO3 0.29 upgrade note** (iter 133 prune): the explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` attribute is load-bearing — dropping it
    changes the module name PyO3 registers and breaks `iscc_lib._lowlevel` imports.

## Kotlin consumer floor — KGP bump evidence (archived iter 139, dep-refresh slice 5 done)

- KGP 2.1.10→2.4.10 stamps `mv=[2,4,0]` into the published jar; a `mavenLocal` consumer proved
    2.1.10 / 2.2.21 fail and 2.3.21 passes, which is how the "Kotlin 2.3 or newer" floor was fixed
    (iter 128). Reproduce with a throwaway Gradle consumer resolving `mavenLocal()` before accepting
    any future KGP bump.
- The floor is documented in four places that must move together: the root `README.md`,
    `packages/kotlin/README.md`, `docs/howto/kotlin.md` and `.claude/context/specs/`
    `kotlin-bindings.md`.

## ruff 0.16 adoption (archived iter 137 — slice 8 of the v0.6.0 dependency refresh, CLOSED)

- **Sliced by decision type, not by file** (iters 125 → 137). `uv lock --upgrade` (iter 125) could
    not take ruff 0.16: it added 104 new default-lint findings, so `ruff<0.16` went into
    `pyproject.toml` with an inline `# held:` reason. `uvx ruff@0.16.0 check .` previews an unpinned
    ruff without touching `uv.lock` — that is what let the hold-back survive four clean-up steps.
- **A** (iter 131) — 78 config-free findings: 36 lone `...` stub bodies deleted from `_lowlevel.pyi`
    (`PIE790` + `PYI048` double-report one line) and 6 `RUF059` unused unpackings `_`-prefixed. Stub
    change verified against `ty`, `mypy 1.18 --strict`, `pyright 1.1.407` because the wheel ships
    `py.typed`. **B** (iter 134) — `extend-select = ["S", "C901"]`; the 14 `# noqa: S603/S607`
    became *recognised* instead of `RUF100`-flagged; 2 genuinely dead directives deleted. **C**
    (iter 135) — the isort cluster: `[tool.ruff] src` + `combine-as-imports`, plus `I`, `RUF022`,
    `RUF100` added to `extend-select`. **D** (iter 136) — the last three: `itertools.pairwise`
    (RUF007), explicit `check=False` (PLW1510), exec bit on `tools/cid.py` (EXE001). **E** (iter
    137\) — pin dropped, `uv lock --upgrade-package ruff` → 0.16.0, zero findings and zero reformats
    at the flip.
- Live rules that outlived the slice stay in `learnings.md`: never blanket `--fix`, never `select`
    (use `extend-select`), and 0.16's `ruff format` reaches Python code blocks inside Markdown.
- **Two invocation gotchas** (archived iter 139 — the `[tool.ruff*]` config is settled; re-read
    before changing `src` / `exclude` / isort settings): (1) an unused `# noqa` is invisible unless
    `RUF100` is selected — prove a directive dead with
    `uv run ruff check --select <rule> --ignore-noqa` before deleting it (`S603` never fires on a
    fully static list-literal argv, only on dynamic argv). (2) the pre-commit `ruff-check` hook
    passes *filenames* and no `--force-exclude`, so hook-mode can disagree with `ruff check .` —
    re-probe per-file when `src`/`exclude`/isort settings change.
- **`_lowlevel.pyi` stub bodies are docstring-only — no trailing `...`** (iter 131; archived iter
    139 — Python bindings section met and both ruff hooks now gate the file locally). The wheel
    ships `py.typed`, so the stub is consumer-facing: check changes against `mypy 1.18 --strict` +
    `pyright 1.1.407` too, not just `ty`.

## prek hook-surface probing — formatter caveats (archived iter 140, gate parity closed iter 139)

- `ruff-check` deliberately skips Markdown: ruff 0.16 *formats* Python fences but does not *lint*
    them, so an `.md` path prints "No Python files found" and exits 0 — widening the lint hook to
    `markdown` buys zero enforcement and adds per-commit noise.
- Widening a *formatter* hook onto Markdown is safe even with pseudo-code fences: `ruff format`
    leaves a syntactically-invalid Python fence unchanged and exits 0 rather than erroring the run.
- The `.pyi` hole (prek types `.pyi` as `pyi`, not `python`) was found iter 138 and closed iter 139
    by adding `pyi` to both ruff hooks' `types_or`; CI's bare `ruff check`/`ruff format` always
    covered the published `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`.

## CI/CD — GitHub Action major-bump static verification (archived iter 141, dep-refresh slices 4+9 done)

- Statically verifiable far past "the tag exists" (iter 140, 9 refs / 73 lines in the unexercised
    `release.yml`): fetch each new major's `action.yml` at the tag ref
    (`raw.githubusercontent.com/<o>/<r>/<vN>/action.yml`) and assert with `yaml.safe_load` (never
    greps) that every `with:` key still appears under `inputs`, every `steps.<id>.outputs.<x>` the
    workflow reads still appears under `outputs` (`cache@v6` keeps `cache-hit`), and `runs.using` is
    runner-supported.
- Then read every intervening major's release notes for *default* changes: an input surviving is not
    its default surviving. Two that bite silently — `setup-node@v5+` auto-enables package-manager
    caching when `package.json` has a `packageManager` field and then *fails* with no lockfile (safe
    here: neither exists — re-check before adding either); `checkout@v6+` persists the auth token to
    `$RUNNER_TEMP` instead of `.git/config`, so plain `git push`/`fetch` still work
    (`prepare-release` is fine) but authenticated git inside a *Docker container action* needs
    runner ≥ 2.329.0.
- `download-artifact@v8` defaults to `digest-mismatch: error`; it is deliberately left strict for a
    publish pipeline (rationale → `decisions.md` 2026-07-25).

## CI/CD — release.yml registry-guard shape (archived iter 141, iter-139 fix landed)

- All 28 non-`prepare-release` jobs carry
    `if: ${{ !cancelled() && !failure() && (<registry cond>) }}`. A plain `if:` implies `success()`
    on `needs`, and GitHub propagates `prepare-release`'s *skip* transitively — that is why
    `-f <registry>=true` published nothing. `!failure()` still reads the job's own `needs`, so a
    failed build still blocks its publish; never substitute `always()`.
- **Invariant a new job must preserve:** its `needs` chain must be gated by the same registry flag
    or a superset (`build-ffi` is `ffi || nuget`) — else relaxing `success()` lets it run against
    artifacts never built. Full rationale → `decisions.md` 2026-07-25.

## Tooling — v0.6.0 dependency refresh + ruff 0.16 adoption (archived iter 142, both threads CLOSED)

- **Dependency refresh** ran as nine per-ecosystem slices, iters 124–140: Rust `Cargo.lock`, Python
    `uv.lock`, Rust direct pins, GHA refs in `ci.yml`/`docs.yml`, JVM manifests, Go module, Ruby
    manifests, ruff 0.16 adoption, GHA refs in `release.yml`. Per-slice detail lives in the
    `issues.md` entry; only human/major-gated bumps remain (xunit 3.x, Test.Sdk 18.x, Gradle
    wrapper, JUnit 6.x, `jni` 0.22, `magnus` 0.8).
- **Hold-back verification recipe** — a `# held:` comment's stated reason must be confirmed from
    registry metadata, never from the handoff prose: `cargo info <crate>@<ver>`,
    `gem specification <gem> -v <ver> --remote`, `https://rubygems.org/api/v1/versions/<gem>.json`.
    A wrong stated reason survives as folklore.
- **A tool bump can widen a gate's file discovery, not just its rules**: ruff 0.16 formats Python
    fences inside Markdown, so the bare `ruff format --check` used by `mise run lint` and CI went
    from 25 to 153 files. Diff the file count before/after a bump, then ask which *local* gate
    covers the newly discovered surface — the resulting local/CI parity gap was closed in iter 138
    by widening the prek `ruff-format` hook to `types_or: [python, pyi, markdown]`.
- **Single-package relock proof**: `uv lock --upgrade-package <pkg>` then
    `git diff -- uv.lock | grep -E '^[+-]name = '` must be empty (only the target moved).
- Every `[tool.ruff*]` setting in `pyproject.toml` carries its rationale as an inline comment.

## Tooling — git gotchas from the closed ruff/dep-refresh threads (archived iter 143)

- **A file-mode change needs `git update-index --chmod=+x`, not just `chmod`** (iter 136): with
    `core.fileMode=false` here a plain `chmod +x` is invisible to git — run both, prove it with
    `git ls-files -s <path>` → `100755`.
- **`cargo tree -i <crate>` prints "nothing to print" for proc-macro / target-specific deps** — add
    `--target all`. (The recurring `proc-macro-error2` future-incompat warning is dev-only and
    expected; full attribution → `issues.md` "Known constraint (verified iter 126)".)

## CI/CD — Swift release job tag dependency (archived iter 144)

- **Swift release job is tag-dependent**: `build-xcframework` derives the version from
    `GITHUB_REF_NAME` (not `Cargo.toml` like every other release job), so the `--ref main`
    re-trigger path breaks for Swift only. Release-day fact; `release.yml` is human-driven.

## ISCC Algorithm — settled vector/meta facts (archived iter 144)

- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing.
- Conformance vectors: `"stream:<hex>"` prefix in `data.json` denotes hex-encoded byte data; empty
    after the prefix = empty bytes. 50 vectors at iscc-core v1.3.0 (20+5+3+5+3+2+4+3+5).

## CID Process — concurrent-loop detection (archived iter 145; single-occurrence, iter 97)

- **Detect concurrent CID loops** (iter 97): context files changing mid-review, or `mise run check`
    reporting spurious "files were modified by this hook" on a file advance never touched, means a
    race. Confirm with `ps aux | grep -E 'cid:run|claude -p CID'`, then flag HUMAN REVIEW REQUESTED
    — do NOT kill processes yourself, and do NOT push. (Also carried in the review agent's
    `MEMORY.md`.)

## Ruff / prek hook scope (archived from learnings.md, iter 148)

- prek `types_or` at the close of the ruff-0.16 adoption: `[python, pyi, markdown]` for the format
    hook, `[python, pyi]` for the check hook — a strict superset of what CI runs (ruff 0.16 formats
    Python code blocks in Markdown but does not lint them).

## Python Binding Tooling Pins (archived iter 149 — Python bindings fully met)

- **PyO3 is `0.29`** (iscc-py only): keep `#[pymodule(name = "_lowlevel", gil_used = true)]`
    explicit. Per-hop upgrade recipe is in the PyO3 sections above.
- **`_lowlevel.pyi` is consumer-facing** (the wheel ships `py.typed`): stub bodies are
    docstring-only, and edits need `mypy 1.18 --strict` + `pyright 1.1.407`, not just `ty`.

## Codec rules (archived from learnings.md at iteration 150 — all pinned by tests)

- `conformance_selftest` masks truncated codes bitwise — never compare full strings below 256 bits
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`** — a loose
    guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`); enforced in Go
    `IsccDecode` + Rust `iscc_decode`. Composite `iscc_decompose` legitimately consumes trailing
    units — do NOT harden it
- `decode_length`: multiples of 32 bits for standard MainTypes, 64 for ISCC-CODE, 8 for ID (C FFI:
    length index for 64-bit codes is 1, not 0)
