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

## Go/wazero Bridge (OBSOLETE)

- Go module path is `github.com/iscc/iscc-lib/packages/go`, package name `iscc`

- `text_clean` does NOT collapse double spaces within a line — use NFKC ligature normalization
    (e.g., fi ligature U+FB01 → "fi") for test cases instead of space-collapsing expectations
