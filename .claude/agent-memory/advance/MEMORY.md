# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (46 total: 16+5+3+5+3+2+4+3+5)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`
- Node.js: `crates/iscc-napi/src/lib.rs`
- WASM: `crates/iscc-wasm/src/lib.rs`
- C FFI: `crates/iscc-ffi/src/lib.rs`
- JNI: `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Go pure: `packages/go/` — codec.go (types + constants + DecodeResult + codec functions), utils.go,
    cdc.go, minhash.go, simhash.go, dct.go, wtahash.go, xxh32.go, code_content_text.go,
    code_meta.go, code_data.go, code_instance.go, code_content_image.go, code_content_audio.go,
    code_content_video.go, code_content_mixed.go, code_iscc.go, conformance.go (+ test files,
    testdata/data.json embedded via go:embed). WASM bridge removed — pure Go only

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven POM is at `crates/iscc-jni/java/pom.xml` — run `mvn test` from `crates/iscc-jni/java/`
- CI workflow at `.github/workflows/ci.yml` has 9 jobs: version-check, rust, python, nodejs, wasm,
    c-ffi, java, go, bench. The `bench` job runs `cargo bench --no-run` (compile-only, no execution)
- `version-check` job: lightweight (checkout + setup-python only), runs
    `python scripts/version_sync.py --check` to catch manifest version drift
- Go CI job has zero Rust dependencies — only checkout, setup-go, test, vet (4 steps)
- Go CI uses `actions/setup-go@v5` with `go-version-file: packages/go/go.mod`
- Version sync: `scripts/version_sync.py` — `--check` mode exits 1 on mismatch
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds

## WASM/WASI

- `iscc-wasm` has `[features] conformance = []` — gates `conformance_selftest` WASM export
- wasm-pack `--features` must go AFTER the path, NOT after `--`
- wasm-opt release flags: `[package.metadata.wasm-pack.profile.release]` with
    `wasm-opt = ["-O", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]`

## Go Pure Go Rewrite (Summary)

- Pure Go in `packages/go/` — all 10 gen functions + codec + algorithms. Zero WASM deps
- Dependencies: `github.com/zeebo/blake3`, `golang.org/x/text`. Indirect: `cpuid/v2`
- Go idioms: unexported helpers (lowercase), `var` for arrays/large uint64 (Go const limitations),
    `[]rune` for Unicode SlidingWindow, generics for `arraySplit[T]`
- Conformance: `//go:embed testdata/data.json`, per-function tests use
    `os.ReadFile("../../crates/iscc-lib/tests/data.json")`
- 151 Go tests total. CI: 4 steps (checkout, setup-go, test, vet) — no Rust deps

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
- Python binding: PyO3 wrapper in `crates/iscc-py/src/lib.rs` accepts `&str` path + `add_units` bool
    param, `SumCodeResult` class in `__init__.py` with `units: list[str] | None`, public wrapper
    accepts `str | os.PathLike` via `os.fspath()` + `add_units: bool = False`. 9 tests in
    `tests/test_smoke.py` (6 existing + 3 units tests). `add_units=True` sets `"units"` dict key,
    `False` omits it (matching iscc-core optional field pattern)
- Node.js binding: `NapiSumCodeResult` struct (`#[napi(object)]`) with `units: Option<Vec<String>>`
    - `gen_sum_code_v0` napi fn with `add_units: Option<bool>` param in `crates/iscc-napi/src/lib.rs`.
        Uses `i64` for `filesize` (napi-rs no u64 support). 9 tests in `__tests__/functions.test.mjs`
        (6 existing + 3 units tests). `Option<Vec<String>>` maps to `string[] | undefined` in TS
        automatically. 135 total NAPI tests
- WASM binding: `WasmSumCodeResult` struct (`#[wasm_bindgen(getter_with_clone)]`) with
    `units: Option<Vec<String>>` + `gen_sum_code_v0` fn with `add_units: Option<bool>` param in
    `crates/iscc-wasm/src/lib.rs`. Accepts `&[u8]` (no filesystem in WASM). Uses `f64` for
    `filesize` (wasm-bindgen `u64` maps to `BigInt`, awkward for JS). Composes internally via
    `DataHasher` + `InstanceHasher` + `gen_iscc_code_v0` (borrow-before-move pattern for units). 9
    tests in `tests/unit.rs` (6 existing + 3 units). 78 total WASM tests (9 conformance + 69 unit; 1
    behind `conformance` feature gate). `Option<Vec<String>>` maps to `string[] | undefined` in TS
- C FFI binding: `IsccSumCodeResult` repr(C) struct with `ok`, `iscc`, `datahash`, `filesize`,
    `units: *mut *mut c_char` (NULL-terminated array or NULL).
    `iscc_gen_sum_code_v0(path, bits,   wide, add_units)` extern "C" function +
    `iscc_free_sum_code_result` (frees units via `iscc_free_string_array`). 7 Rust sum tests + 5 C
    sum tests. 85 total Rust tests, 65 total C test assertions
- JNI binding: `SumCodeResult.java` (immutable, `String iscc`, `String datahash`, `long filesize`)
    - `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` in `crates/iscc-jni/src/lib.rs`. Returns `jobject`
        via `env.find_class("io/iscc/iscc_lib/SumCodeResult")` + `env.new_object()` with signature
        `(Ljava/lang/String;Ljava/lang/String;J)V`. 4 Maven tests. 62 total Maven tests
- Go binding: `packages/go/code_sum.go` — `SumCodeResult` struct (`Iscc`, `Datahash`, `Filesize`) +
    `GenSumCodeV0(path string, bits uint32, wide bool)`. Single-pass file I/O with `os.Open` +
    `DataHasher` + `InstanceHasher` + `GenIsccCodeV0`. 4 tests in `code_sum_test.go`. 151 total Go
    tests. ALL 7 bindings complete for issue #15

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

## Documentation Sweep Patterns

- "N gen" count references exist in: READMEs (9 files), docs/ (14 files), howto/ (6 files), crate
    CLAUDE.md files (5), notes/ (2), source comments (.rs, .py, .mjs, .pyi), benchmarks/ (2)
- The Edit tool requires a full Read call (not offset/limit) before the first edit per file
- mdformat auto-reformats after edits — always run `mise run format` twice after doc changes
- iscc-core-ts is external and may have different function counts than iscc-lib

## C FFI Documentation

- `docs/howto/c-cpp.md` — C/C++ how-to guide with 12 sections (overview, build, cmake, quick start,
    streaming, composing, error handling, memory mgmt, static/dynamic, cross-compile, RAII,
    conformance)
- `docs/c-ffi-api.md` — full API reference (types, constants, code gen, text utils, algorithms,
    codec, streaming, diagnostics, memory mgmt, error handling)
- zensical.toml nav: howto guides list includes `{ "C / C++" = "howto/c-cpp.md" }` after Java
- CMake integration uses `find_library()` pattern (not `CMAKE_LIBRARY_PATH`)

## C FFI Examples

- `crates/iscc-ffi/examples/iscc_sum.c` — streaming ISCC-SUM example (read file → dual hashers →
    compose → print). C89/C99 compatible style (variables declared at block start)
- `crates/iscc-ffi/examples/CMakeLists.txt` — minimal cmake build targeting `iscc_ffi` library
- gcc compile:
    `gcc -o out iscc_sum.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
- Run: `LD_LIBRARY_PATH=target/debug ./out <filepath>`

## C FFI Header

- Committed header: `crates/iscc-ffi/include/iscc.h` (generated by cbindgen, tracked in git)
- Test artifact: `crates/iscc-ffi/tests/iscc.h` (gitignored, CI-generated)
- Regenerate:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`
- CI freshness check: regenerates to include/, `git diff --exit-code` fails if stale
- C test uses `-I crates/iscc-ffi/include` (not tests/) for the committed header

## C FFI Release Artifacts

- `release.yml` has `build-ffi` (5-platform matrix) + `publish-ffi` (uploads to GitHub Releases)
- Trigger: `startsWith(github.ref, 'refs/tags/v') || inputs.ffi` (same pattern as other jobs)
- Tarball naming: `iscc-ffi-v{version}-{target}.tar.gz` (Unix), `.zip` (Windows)
- Windows includes 3 files: `iscc_ffi.dll`, `iscc_ffi.dll.lib` (import lib), `iscc_ffi.lib` (static)
- Unix includes 2 files: shared lib + static lib. Both also include `iscc.h` + `LICENSE`
- `publish-ffi` needs `contents: write` (top-level is `contents: read`)
- Uses `softprops/action-gh-release@v2` with tag_name ternary for tag push vs manual dispatch

## Gotchas

- JNI package underscore encoding: `iscc_lib` → `iscc_1lib` in function names
- mdformat auto-formats markdown — keep backtick expressions short to avoid wrapping crashes
- `from __future__ import annotations` in `__init__.py` — use `|` union syntax, not `Union`
- Python `__all__` has 48 entries (32 API + 11 result types + `__version__` + MT, ST, VS, core_opts)
- `gen_sum_code_v0` wide mode only differs from normal when `bits >= 128` (wide requires 128-bit+
    codes)
- After adding new symbols to `crates/iscc-py/src/lib.rs`, MUST rebuild the `.so` with
    `uv run maturin develop -m crates/iscc-py/Cargo.toml` before `pytest` will work
- JSON `{"x":""}` overhead is 8 bytes (not 7) — relevant for boundary tests on META_TRIM_META
- META_TRIM_META validation: pre-decode check uses `META_TRIM_META * 4/3 + 256` (base64 inflation +
    media type header), post-decode check uses `META_TRIM_META` directly
