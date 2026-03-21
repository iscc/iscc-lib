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
