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

## Go Pure Go Rewrite

- Pure Go codec: `packages/go/codec.go` — type enums (`MainType`, `SubType`, `Version` with `iota`),
    varnibble header encoding/decoding, base32/base64, `EncodeComponent`, `IsccDecompose`,
    `IsccDecode`. Zero external dependencies
- Go type naming: `MTMeta`..`MTFlake`, `STNone`..`STWide`, `STText = STNone`, `VSV0 Version = 0`
- Internal helpers are unexported (lowercase): `encodeHeader`, `decodeHeader`, etc.
- `IsccDecode` uses `DecodeResult` struct defined in `codec.go`
- Base32: `base32.StdEncoding.WithPadding(base32.NoPadding)`. Base64: `base64.RawURLEncoding`
- Pure Go text utils: `TextClean` (NFKC + control-char + empty-line collapse), `TextCollapse` (NFD +
    lowercase + filter C/M/P + NFKC), `TextTrim` (UTF-8 byte-boundary), `TextRemoveNewlines`
    (strings.Fields join). Uses `golang.org/x/text/unicode/norm`
- CDC: `cdcGear` table is `var` not `const` (Go no const arrays). `min()` builtin Go 1.21+
- MinHash: `minhashFn` naming (avoids conflict). `maxi64`/`mprime`/`maxH` are `var` not `const`
- SimHash: `AlgSimhash` returns `([]byte, error)`, `SlidingWindow` returns `([]string, error)`. Uses
    `[]rune` for Unicode-correct SlidingWindow
- CDC integer ceiling: `(minSize + 1) / 2` (Go has no div_ceil method)
- DCT: `algDct` (unexported) + `dctRecursive` helper. Only uses `math` stdlib. Nayuki recursive
    divide-and-conquer. Input must be power of 2 — checked via `n > 0 && n&(n-1) == 0`
- WTA-Hash: `AlgWtahash` (exported) + `wtaVideoIdPermutations` `[256][2]int` table. No external deps
- Gen functions: `code_content_text.go` (GenTextCodeV0 + softHashTextV0), `code_meta.go`
    (GenMetaCodeV0 + metaNameSimhash + softHashMetaV0 + softHashMetaV0WithBytes + interleaveDigests
    \+ slidingWindowBytes + decodeDataURL + parseMetaJSON + jsonHasContext + buildMetaDataURL +
    multiHashBlake3), `code_data.go` (GenDataCodeV0 + DataHasher with Push/Finalize),
    `code_instance.go` (GenInstanceCodeV0 + InstanceHasher with Push/Finalize),
    `code_content_image.go` (GenImageCodeV0 + softHashImageV0 + transposeMatrix + flatten8x8 +
    computeMedian), `code_content_audio.go` (GenAudioCodeV0 + softHashAudioV0 + arraySplit[T]).
    Result types: `TextCodeResult`, `MetaCodeResult`, `DataCodeResult`, `InstanceCodeResult`,
    `ImageCodeResult`, `AudioCodeResult`, `VideoCodeResult`, `MixedCodeResult`, `IsccCodeResult`
- xxh32: `xxh32.go` — standalone xxHash32 impl (~80 lines). Used by softHashTextV0 for n-gram
    feature hashing. Unexported: `xxh32(data, seed)`, `xxh32Round`, `rotl32`, `readU32LE`
- JCS canonicalization: uses Go stdlib `json.Marshal` (sorts keys, compact format). Works for
    string/null values in conformance vectors. For full RFC 8785 float compliance, would need a
    dedicated library
- BLAKE3 dependency: `github.com/zeebo/blake3` (SIMD-optimized). `blake3.Sum256(data)` returns
    `[32]byte`
- Test naming for gen functions: `TestPureGo*` prefix (historical — could be renamed to `Test*` in
    future cleanup)
- Go docs: `packages/go/README.md` and `docs/howto/go.md` describe pure Go API (no WASM/wazero).
    Examples use `iscc.Function(...)` pattern with typed result structs (`*MetaCodeResult`, etc.)
- Image-Code helpers: `transposeMatrix`, `flatten8x8`, `computeMedian` are unexported in
    `code_content_image.go`. `bitsToBytes` reused from `codec.go`
- Audio-Code: `arraySplit[T any]` is generic (Go 1.18+), used for splitting digests into quarters/
    thirds. `AlgSimhash` on 4-byte digests returns 4 bytes (output = input digest length)
- `sort.Slice` for int32: `func(i, j int) bool { return s[i] < s[j] }` (no built-in int32 sort)
- Video-Code: `SoftHashVideoV0` exported (matching Rust `pub fn`). Dedup via
    `fmt.Sprintf("%v", sig)` string keys in `map[string][]int32`. Column-wise int64 sums →
    `AlgWtahash`
- Mixed-Code: `softHashCodesV0` unexported (matching Rust non-pub). Preserves first header byte for
    type info in SimHash entries. Uses `decodeHeader`/`decodeLength` to validate Content MainType
    and bit length. `AlgSimhash` error safely discarded (all entries identical length)
- Go module dependencies: `github.com/zeebo/blake3` (BLAKE3, SIMD), `golang.org/x/text` (Unicode).
    No wazero or WASM dependencies. `github.com/klauspost/cpuid/v2` indirect (blake3 SIMD detection)
- Test naming: `TestCodec*`, `TestUtils*`, `TestCdc*`, `TestMinhash*`, `TestSimhash*`,
    `TestAlgDct*`, `TestAlgWtahash*`, `TestPermutation*`
- Conformance tests (per-function): `os.ReadFile("../../crates/iscc-lib/tests/data.json")`
- Conformance selftest: `//go:embed testdata/data.json` in conformance.go.
    `ConformanceSelftest()   (bool, error)` — package-level function (no receiver). Uses
    `vectorEntry` struct + 9 `run*Tests` section runners. `decodeStream` shared helper for
    Data/Instance hex decoding

## gen_sum_code_v0

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` in `lib.rs`
- Single-pass file I/O: opens file, reads in `IO_READ_SIZE` chunks, feeds both `DataHasher` and
    `InstanceHasher`, composes ISCC-CODE via `gen_iscc_code_v0`
- `SumCodeResult { iscc, datahash, filesize }` in `types.rs` — same `#[non_exhaustive]` pattern
- File I/O errors mapped to `IsccError::InvalidInput("Cannot open/read file: {e}")`
- `units: Vec<String>` field deferred (not in scope for initial core implementation)
- 32nd and final Tier 1 symbol for Rust core — all 32 symbols now implemented
- Python binding: PyO3 wrapper in `crates/iscc-py/src/lib.rs` accepts `&str` path, `SumCodeResult`
    class in `__init__.py`, public wrapper accepts `str | os.PathLike` via `os.fspath()`, 6 tests in
    `tests/test_smoke.py`. Remaining bindings: Node.js, WASM, C FFI, Java, Go (5 crates)

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
