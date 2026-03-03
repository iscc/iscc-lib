# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`
- Node.js: `crates/iscc-napi/src/lib.rs`
- WASM: `crates/iscc-wasm/src/lib.rs`
- C FFI: `crates/iscc-ffi/src/lib.rs`
- JNI: `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (Ruby wrapper + Result
    classes), lib/iscc_lib/version.rb, extconf.rb, Rakefile, Gemfile, iscc-lib.gemspec,
    test/test_smoke.rb. Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- Go pure: `packages/go/` — codec.go, utils.go, cdc.go, minhash.go, simhash.go, dct.go, wtahash.go,
    xxh32.go, code_content_text.go, code_meta.go, code_data.go, code_instance.go,
    code_content_image.go, code_content_audio.go, code_content_video.go, code_content_mixed.go,
    code_iscc.go, conformance.go. WASM bridge removed — pure Go only

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven POM is at `crates/iscc-jni/java/pom.xml` — run `mvn test` from `crates/iscc-jni/java/`
- CI workflow at `.github/workflows/ci.yml` has 9 jobs: version-check, rust, python, nodejs, wasm,
    c-ffi, java, go, bench. The `bench` job runs `cargo bench --no-run` (compile-only, no execution)
- `rust` CI job includes feature matrix testing: clippy + test for `--no-default-features`,
    `--all-features`, and `--no-default-features --features text-processing` (issue #16)
- `version-check` job: lightweight (checkout + setup-python only), runs
    `python scripts/version_sync.py --check` to catch manifest version drift
- Go CI job has zero Rust dependencies — only checkout, setup-go, test, vet (4 steps)
- Go CI uses `actions/setup-go@v5` with `go-version-file: packages/go/go.mod`
- Version sync: `scripts/version_sync.py` — `--check` mode exits 1 on mismatch
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds

## WASM/WASI

- `iscc-wasm` has `[features] conformance = []` — gates `conformance_selftest` WASM export
- wasm-pack `--features` must go AFTER the path, NOT after `--`

## Go Pure Go (Summary)

- Pure Go in `packages/go/` — all 10 gen functions + codec + algorithms. Zero WASM deps
- 155 Go tests total. CI: 4 steps (checkout, setup-go, test, vet) — no Rust deps
- Conformance: `//go:embed testdata/data.json`, `parseConformanceData()` with two-pass parsing

## gen_sum_code_v0

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>`
    in `lib.rs`
- Single-pass file I/O: opens file, reads in `IO_READ_SIZE` chunks, feeds both `DataHasher` and
    `InstanceHasher`, composes ISCC-CODE via `gen_iscc_code_v0`
- `SumCodeResult { iscc, datahash, filesize, units }` in `types.rs` — `#[non_exhaustive]`,
    `units: Option<Vec<String>>` contains `[Data-Code, Instance-Code]` when `add_units` is true
- File I/O errors mapped to `IsccError::InvalidInput("Cannot open/read file: {e}")`
- `iscc_decode` returns tuple `(u8, u8, u8, u8, Vec<u8>)` — use tuple destructuring in tests, not
    field access. `MainType` is `pub(crate)` in `codec` module, not accessible from test module
- 32nd and final Tier 1 symbol for Rust core — all 32 symbols now implemented
- All 7 bindings implement `gen_sum_code_v0` with `add_units`/`units` (issue #21 complete): Python
    (PyO3), Node.js (napi-rs, `i64` filesize), WASM (`f64` filesize, `&[u8]` input), C FFI
    (NULL-terminated `*mut *mut c_char` for units), JNI (nullable `String[]`), Go (`code_sum.go`,
    single-pass file I/O)

## Benchmarks

- `crates/iscc-lib/benches/benchmarks.rs` — all 10 `gen_*_v0` + DataHasher streaming + CDC chunks
- `bench_sum_code` uses `tempfile::NamedTempFile` since `gen_sum_code_v0` takes `&Path` (not
    `&[u8]`)
- Temp files created outside bench closure (setup cost excluded from measurement)
- `tempfile` is a dev-dependency only (workspace dep `tempfile = "3"`)

## Codec Internals

- `decode_header` and `decode_varnibble_from_bytes` operate directly on `&[u8]` with bitwise
    extraction — no intermediate `Vec<bool>`. `get_bit`/`extract_bits` helpers (MSB-first)
- `encode_header` still uses `Vec<bool>` internally (encode path less performance-sensitive)

## Streaming

- `DataHasher`: persistent `buf: Vec<u8>` reused across `update()` calls. CDC → BLAKE3 chunk hash →
    MinHash pipeline. Tail: `copy_within` + `truncate`. ~1.1 GiB/s at 64 KiB chunks
- `InstanceHasher`: wraps BLAKE3, outputs ISCC multihash format (64-byte digest truncated)

## API Design

- Video API uses `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), other
    bindings pass `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields + validates with
    `TryFrom<u8>`. Delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix and dashes, returns exact digest bytes (not full tail)
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key

## Documentation

- Tabbed syntax: `=== "Language"` with 4-space indent, blank line before code block

- Tab order for tutorial: Python, Rust, Node.js, Java, Go, WASM (6 tabs)

- Landing page (`docs/index.md`) tab order: Rust, Python, Node.js, Java, Go, WASM

- mdformat reformats JS imports to multi-line `import { ... } from` style — run format before commit

- Landing page Go example updated to pure Go API (`result, _ := iscc.GenTextCodeV0(...)` pattern)

- Node.js/Java/WASM gen functions return plain strings; Python/Rust/Go return result objects

- `docs/architecture.md` and `docs/development.md` share identical directory trees and crate summary
    tables — keep them in sync when editing either file

- Go shown in Mermaid diagram as standalone disconnected node with green style (not connected to
    CORE) — reflects pure Go reimplementation. Five Rust-dependent binding crates shown with arrows

- Java API reference: `docs/java-api.md` — hand-written, follows C FFI page structure adapted for
    Java (no manual memory mgmt except streaming hasher handles)

- All 4 Reference pages complete: Rust API, Python API, C FFI, Java API

## Binding Constant Export Patterns

- NAPI: `#[napi(js_name = "CONST_NAME")] pub const CONST_NAME: u32 = iscc_lib::CONST_NAME as u32;`
- WASM: `#[wasm_bindgen(js_name = "CONST_NAME")] pub fn const_name() -> u32 { ... }` (getter fn, not
    const — wasm-bindgen limitation)
- C FFI: `#[unsafe(no_mangle)] pub extern "C" fn iscc_const_name() -> u32 { ... }` + inline
    `#[test]` in same file. cbindgen auto-generates the C header
- NAPI JS tests: `describe('CONST_NAME', () => { it('equals X'); it('is a number'); })`
- WASM tests: `#[wasm_bindgen_test]` in `tests/unit.rs` (requires wasm-pack to run)
- C tests: `ASSERT_EQ(iscc_const_name(), value, "label")` in `tests/test_iscc.c`
- 5 constants currently exported: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META,
    IO_READ_SIZE, TEXT_NGRAM_SIZE

## C FFI Documentation

- `docs/howto/c-cpp.md` — C/C++ how-to guide with 12 sections (overview, build, cmake, quick start,
    streaming, composing, error handling, memory mgmt, static/dynamic, cross-compile, RAII,
    conformance)
- `docs/c-ffi-api.md` — full API reference (types, constants, code gen, text utils, algorithms,
    codec, streaming, diagnostics, memory mgmt, error handling)
- zensical.toml nav: howto guides list includes `{ "C / C++" = "howto/c-cpp.md" }` after Java
- CMake integration uses `find_library()` pattern (not `CMAKE_LIBRARY_PATH`)

## Feature Flags

- `crates/iscc-lib/Cargo.toml` defines: `default = ["meta-code"]`, `text-processing` (unicode deps),
    `meta-code` (implies text-processing + JCS canonicalizer)
- `text-processing` gates: `text_clean`, `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`
- `meta-code` gates: META_TRIM constants, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests` in conformance, `sliding_window_bytes`
- `conformance` module is always available (not feature-gated). `conformance_selftest()` skips
    disabled code types (meta, text) via `#[cfg]` blocks — does not fail for missing features
- When gating `pub(crate)` functions, their tests must also be gated — dead-code lint fires in
    library builds even if test modules use them
- Integration tests in `crates/iscc-lib/tests/test_text_utils.rs` also need per-function gating
- `serde_json` stays as a regular (non-optional) dep because `conformance.rs` uses it for parsing
    `data.json`. Gating it requires restructuring conformance (future work)

## Ruby Bindings (Magnus)

- Magnus 0.7.1 used (not 0.8) — Magnus 0.8 requires Ruby 3.2+, devcontainer has Ruby 3.1.2. Magnus
    0.7.1 works with Rust edition 2024 and Ruby 3.1
- `function!` macro does NOT accept `&Ruby` parameter — use `Ruby::get().expect("called from Ruby")`
    inside the function body to get the Ruby runtime handle
- rb_sys `ExtensionTask.new("iscc-rb")` — task name must match Cargo package name (not lib name).
    Binary derived as `"iscc-rb".tr("-", "_")` = `"iscc_rb"`. Cargo `[lib] name` must match
- `extconf.rb` must be at Cargo manifest directory (crate root), not `ext/iscc_lib/`. rb_sys
    hardcodes `File.join(cargo_metadata.manifest_directory, "extconf.rb")`
- Root `.gitignore` has `lib/` pattern (from Python template) — blocks all `lib/` directories. Ruby
    crate's `.gitignore` needs `!lib/` negation to re-include `crates/iscc-rb/lib/`
- Bundler must use local vendor path (`bundle config set --local path vendor/bundle`) since system
    gem path `/var/lib/gems/3.1.0` is not writable by dev user
- PATH for bundle commands: `/home/dev/.local/share/gem/ruby/3.1.0/bin` must be in PATH
- `bundle exec rake compile` builds release profile by default (rb_sys sets `RB_SYS_CARGO_PROFILE`)
- Gen functions prefixed with `_` in Rust bridge (e.g., `_gen_meta_code_v0`), Ruby wrapper provides
    keyword-arg public API (e.g., `gen_meta_code_v0(name, description: nil, ...)`)
- Ruby `Result < Hash` enables both `result["iscc"]` and `result.iscc` access via `method_missing`
- Constants set via `module.const_set("NAME", value)` in Magnus init
- Binary data: Magnus `String` validates UTF-8 — use `RString` param + `unsafe { data.as_slice() }`
    for functions accepting arbitrary bytes (e.g., `encode_base64`, `encode_component`). Copy bytes
    immediately before any Ruby API calls. Return binary data via `RString::from_slice(&bytes)`
- Returning arrays: use `ruby.ary_new_capa(n)` + `arr.push(val)?` for mixed-type arrays (e.g.,
    `iscc_decode` returns `[u8, u8, u8, u8, RString]`)
- 25/32 Tier 1 symbols exposed: all 10 gen functions (meta/text/image/audio/video/mixed/data/
    instance/iscc/sum_code_v0), 4 text utils, 5 constants, encode_base64, iscc_decompose,
    encode_component, iscc_decode, json_to_data_url, conformance_selftest
- `gen_image_code_v0` requires exactly 1024 pixels (32×32 image) — use `RString` for binary input
- `gen_audio_code_v0` takes `Vec<i32>` — Magnus auto-converts Ruby Array of integers
- `gen_video_code_v0`: `RArray` → `Vec<Vec<i32>>` via `into_iter()` +
    `TryConvert::try_convert(val)`. Magnus 0.7.1 `RArray::each()` is deprecated — use `into_iter()`.
    `TryConvert` is an associated fn, not a method — import trait, call
    `TryConvert::try_convert(val)` not `val.try_convert()`
- `gen_video_code_v0` requires ≥380 elements per frame (WTA-Hash minimum). Small test arrays error
- `gen_mixed_code_v0`: `Vec<String>` auto-converts, then `Vec<&str>` for core. Only Content-Codes
    (Text/Image/Audio/Video) accepted — Meta-Codes cause "all codes must be Content-Codes" error
- `gen_data_code_v0`/`gen_instance_code_v0`: same `RString` + `unsafe { data.as_slice() }` pattern
- `gen_iscc_code_v0`: `Vec<String>` + `bool` wide — same pattern as `gen_mixed_code_v0`
- `gen_sum_code_v0`: `String` path → `std::path::Path::new(&path)`, 4 positional args. `units` is
    conditionally included in hash (only when `Some`). Tests use `Tempfile` with ensure cleanup

## Gotchas

- JNI package underscore encoding: `iscc_lib` → `iscc_1lib` in function names
- After adding new symbols to `crates/iscc-py/src/lib.rs`, MUST rebuild the `.so` with
    `uv run maturin develop -m crates/iscc-py/Cargo.toml` before `pytest` will work
- data.json `_metadata` key (v1.3.0+): top-level metadata section with flat string values, not test
    vectors. Rust `serde_json::Value` ignores it naturally; Go needed explicit skip logic
