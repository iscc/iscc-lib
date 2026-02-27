# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

## Code Locations

- JNI Java wrapper: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- JNI NativeLoader: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`
- JNI Rust bridge: `crates/iscc-jni/src/lib.rs`
- JNI Java tests: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- JNI Maven config: `crates/iscc-jni/java/pom.xml`
- Conformance vectors: `crates/iscc-lib/tests/data.json` (46 total: 16+5+3+5+3+2+4+3+5)
- Node.js conformance tests: `crates/iscc-napi/__tests__/conformance.test.mjs`
- Per-crate READMEs: `crates/iscc-lib/README.md`, `crates/iscc-py/README.md`,
    `crates/iscc-napi/README.md`, `crates/iscc-wasm/README.md`, `crates/iscc-jni/README.md`,
    `crates/iscc-ffi/README.md`, `packages/go/README.md`
- Root README: `README.md` — covers all languages (Rust, Python, Java, Go, Node.js, WASM, C)

## Implementation Patterns

- JUnit 5 conformance tests use `@TestFactory` + `DynamicTest` for data-driven tests from JSON
- Gson `JsonObject`/`JsonArray` for traversing data.json (test scope dependency)
- Meta argument prep: JsonObject -> TreeMap sort -> Gson serialize; JsonNull -> null; string ->
    as-is
- Stream hex decoding: `HexFormat.of().parseHex()` (Java 17+) after stripping `"stream:"` prefix
- Empty description in meta tests maps to null (not empty string) to match JNI bridge behavior
- `gen_iscc_code_v0` test vectors have no `wide` field -- always pass `false`
- Maven Surefire plugin `-Djava.library.path` points to `${project.basedir}/../../../target/debug`
- data.json relative path from Maven test CWD: `../../iscc-lib/tests/data.json`
- napi bindings return strings directly (`.map(|r| r.iscc)`) -- Node.js quick start uses plain
    string assignment, not object property access
- napi-rs `Buffer` implements `From<&[u8]>` directly — no need for `.to_vec()` intermediate
    allocation when converting borrowed slices to JS Buffer objects
- napi-rs `package.json` needs a `"files"` allowlist because `.gitignore` excludes `index.js` and
    `index.d.ts` — without `"files"`, `npm publish` uses `.gitignore` as fallback and omits
    entrypoints
- `npx napi build --platform` regenerates `index.js`/`index.d.ts` with version from `package.json` —
    use this to fix version skew when `package.json` version changes
- Python bindings return dict-like objects -- quick start uses `result['iscc']` (dict access) or
    `result.iscc` (attribute access via `__getattr__`)
- Python `__init__.py` bytes-like input pattern: `isinstance(data, (bytes, bytearray, memoryview))`
    for stream detection (NOT `hasattr(data, "read")` — ty type checker doesn't support it). Inner
    `isinstance(data, bytes)` check converts bytearray/memoryview to `bytes` for Rust FFI
- Python streaming: `_CHUNK_SIZE = 65536` (64 KiB) for chunked reads from BinaryIO, uses Rust-layer
    `_DataHasher`/`_InstanceHasher` for true streaming (no unbounded `.read()`)
- `from __future__ import annotations` is active in `__init__.py` — use `|` union syntax in type
    annotations, do NOT import `Union` from typing (ruff flags it as unused)
- JNI error handling: `throw_and_default` for `IllegalArgumentException` (invalid input),
    `throw_state_error` for `IllegalStateException` (invalid object state, e.g., finalized hashers).
    Three return forms: (1) nested match for `env.new_string().into_raw()`, (2) nested match for
    `env.byte_array_from_slice().into_raw()`, (3) early-return match + `if let Err` for loop bodies
- JNI Java-side Javadoc (`IsccLib.java`) still says `@throws IllegalArgumentException` for hasher
    update/finalize methods — the Rust side throws `IllegalStateException` but Java declarations are
    cosmetically mismatched (tests verify correct runtime behavior)
- JNI `isccDecode` returns `jobject` (not `jstring`): construct Java object via `env.find_class` +
    `env.new_object` with constructor signature `(IIII[B)V`. The `JValue::Object` takes a reference
    to `JByteArray` (which derefs to `JObject`). Class path uses `/` separators
- JNI `encodeComponent` validates jint ranges (0-255 for mtype/stype/version, ≥0 for bitLength)
    before casting to u8/u32, using `throw_and_default` for out-of-range values
- JNI constants are `public static final int` in Java (no JNI function needed — compile-time
    literals). Placed at top of `IsccLib.java` before static initializer block
- `IsccDecodeResult.java`: separate file in same package (`io.iscc.iscc_lib`), public final fields,
    single constructor `(int, int, int, int, byte[])`. Auto-compiled by Maven (no pom.xml changes)
- JNI `extern "system"` count verification: `grep -c 'extern "system"'` returns N+1 because of doc
    comment on line 3 mentioning the string. Actual function count = grep result - 1

## WASM/WASI

- `iscc-wasm` has `[features] conformance = []` — gates `conformance_selftest` WASM export and its
    unit test. CI enables it via `wasm-pack test --node crates/iscc-wasm --features conformance`
- wasm-pack `--features` must go AFTER the path, NOT after `--`. `--` in wasm-pack passes args to
    the test runner (wasm-bindgen-test-runner), not to cargo. Correct:
    `wasm-pack test --node crates/iscc-wasm --features conformance`. Wrong:
    `wasm-pack test --node crates/iscc-wasm -- --features conformance`
- `iscc-ffi` compiles as wasm32-wasip1 from existing `crate-type = ["cdylib", "staticlib"]` — no
    Cargo.toml changes needed. The cdylib target produces the `.wasm` file
- `iscc_alloc`/`iscc_dealloc` are the WASM host memory management pair — host allocates via
    `iscc_alloc`, writes data, calls FFI functions, then frees via `iscc_dealloc`
- WASM binary in `packages/go/iscc_ffi.wasm` is tracked in git (release build, ~683KB). Must be
    rebuilt and recommitted whenever FFI exports change. Build:
    `cargo build -p iscc-ffi --target wasm32-wasip1 --release` →
    `cp target/wasm32-wasip1/release/iscc_ffi.wasm packages/go/`
- Debug WASM binary is ~10.5MB; release + wasm-opt reduces significantly
- wasm-opt release config in `crates/iscc-wasm/Cargo.toml`:
    `[package.metadata.wasm-pack.profile.release]` with
    `wasm-opt = ["-O", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]`. Rust's LLVM
    emits `memory.copy` (bulk-memory) and `i32.trunc_sat_f64_s/u` (nontrapping-float-to-int) — both
    must be enabled for wasm-opt to accept the binary. If future Rust versions emit more post-MVP
    ops, add specific `--enable-*` flags (or switch to `--enable-all` as last resort)
- Install target: `rustup target add wasm32-wasip1`
- Build: `cargo build -p iscc-ffi --target wasm32-wasip1`
- Output: `target/wasm32-wasip1/debug/iscc_ffi.wasm`

## Go/wazero Bridge

- Go module: `packages/go/` with package name `iscc`, module path
    `github.com/iscc/iscc-lib/packages/go`
- wazero v1.11.0 is the pure-Go WASM runtime — no CGO required
- WASM binary embedded via `//go:embed iscc_ffi.wasm` — must be pre-built and copied to
    `packages/go/iscc_ffi.wasm` before `go test`
- wazero function calls return `[]uint64` — cast to `uint32` for WASM32 pointers
- String marshaling pattern: `writeString` allocs + writes UTF-8 + null terminator, `readString`
    reads byte-by-byte until null, `freeString` calls `iscc_free_string`
- `iscc_last_error` returns borrowed pointer — do NOT free it
- Each `NewRuntime()` call compiles the ~11MB WASM module (~0.6s). For test suites with many tests,
    consider `CompileModule` once + `InstantiateModule` per test
- `wazero.NewModuleConfig().WithStdout(io.Discard).WithStderr(io.Discard)` suppresses WASI noise
- Use `r.InstantiateWithConfig(ctx, wasmModule, cfg)` (not separate compile+instantiate) for simpler
    single-module loading
- `text_clean` does NOT collapse double spaces within a line — it does NFKC normalization, control
    char removal, newline normalization, consecutive empty line collapse, and leading/trailing
    whitespace stripping. Use NFKC test cases (e.g., fi ligature U+FB01) for testing
- WASM32 empty slice alignment: `iscc_alloc(0)` returns NonNull::dangling (ptr=1, alignment 1). This
    is fine for `*const u8` but NOT for `*const i32` (needs alignment 4). For empty i32 slices,
    allocate minimum 4 bytes to get a properly aligned pointer from the allocator. `writeI32Slice`
    returns (ptr, allocSize, count) because allocSize may differ from count\*4 for this reason
- Memory helpers: `writeBytes` for `[]byte → *const u8 + len`, `writeI32Slice` for
    `[]int32 → *const i32 + len`, `writeStringArray` for `[]string → **c_char + count`,
    `writeI32ArrayOfArrays` for `[][]int32 → **i32 + *usize + count` (video frame signatures)
- All 9 gen\_\*\_v0 Go wrappers follow the same pattern: marshal args → call FFI → callStringResult
    (check NULL, readString, freeString) → return
- String-array-returning functions (SlidingWindow, IsccDecompose) use `callStringArrayResult` which
    reads a null-terminated array of u32 pointers from WASM32 memory (4 bytes each, little-endian),
    calls `readString` for each non-zero pointer, then `iscc_free_string_array` to free the entire
    array. Pattern mirrors `callStringResult` for single strings
- Go Runtime has 48 methods total: 27 public (Close, ConformanceSelftest, TextClean,
    TextRemoveNewlines, TextCollapse, TextTrim, EncodeBase64, SlidingWindow, IsccDecompose,
    AlgSimhash, AlgMinhash256, AlgCdcChunks, SoftHashVideoV0, 9 gen\_\*\_v0, NewDataHasher,
    NewInstanceHasher, JsonToDataUrl, EncodeComponent, IsccDecode) + 21 private helpers
- Go `DecodeResult` struct: public struct with `Maintype`, `Subtype`, `Version`, `Length` (all
    `uint8`) and `Digest` (`[]byte`). Returned as `*DecodeResult` (pointer) from `IsccDecode`
- Go `IsccDecode` uses sret ABI: 16-byte `IsccDecodeResult` struct. Layout: ok(1B) + maintype(1B) +
    subtype(1B) + version(1B) + length(1B) + padding(3B) + digest.data(4B) + digest.len(4B).
    `iscc_free_decode_result` takes sret pointer (single i32 param) on wasm32
- Go constants: `MetaTrimName`, `MetaTrimDescription`, `IoReadSize`, `TextNgramSize` are
    package-level `const` (idiomatic Go). No enum types — use plain `int`/`uint8`
- Go streaming hasher pattern: `DataHasher`/`InstanceHasher` structs hold `rt *Runtime` +
    `ptr   uint32` (opaque WASM pointer). Factory methods on Runtime call `iscc_*_hasher_new()` and
    check for NULL. `Update` writes bytes via `writeBytes`, calls `iscc_*_hasher_update` (returns
    i32 as bool: 0=error, nonzero=ok). `UpdateFrom` reads from `io.Reader` in 64 KiB chunks and
    delegates to `Update`. `Finalize` calls `iscc_*_hasher_finalize` (returns string pointer) and
    uses `callStringResult`. `Close` calls `iscc_*_hasher_free` and zeroes `h.ptr` to prevent
    double-free (fire-and-forget, safe to call multiple times). No sret ABI needed — all streaming
    hasher FFI functions use simple i32 params/returns
- Byte-buffer-returning WASM functions use sret ABI: caller allocates 8 bytes (IsccByteBuffer or
    IsccByteBufferArray struct), passes ptr as first arg. Function writes struct fields to that ptr.
    The free functions (iscc_free_byte_buffer, iscc_free_byte_buffer_array) take the struct by
    pointer (1 i32 param), so the sret ptr can be reused directly — no extra alloc for free call.
    IsccByteBuffer is {data_ptr: i32, len: i32} = 8 bytes. IsccByteBufferArray is {buffers_ptr: i32,
    count: i32} = 8 bytes. Each buffer in the array is at offset i\*8
- `writeByteArrayOfArrays` follows same pattern as `writeI32ArrayOfArrays` but for `[][]byte` input
    (digests). Used by `AlgSimhash` which takes `*const *const u8` + `*const usize` + count
- `writeU32Slice` is identical to `writeI32Slice` but with `uint32` Go type (same 4-byte encoding)
- Go conformance test path to data.json: `../../crates/iscc-lib/tests/data.json` (relative from
    packages/go test working directory)
- Meta test vectors: dict meta values need json.Marshal before passing to FFI; null maps to nil
    `*string`; empty description `""` is passed as pointer to empty string (not nil)

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven runs from `crates/iscc-jni/java/` as working directory
- JDK 17 and Maven are pre-installed in devcontainer
- Gson 2.11.0 used as test-scope dependency for JSON parsing
- CI workflow at `.github/workflows/ci.yml` has 7 jobs: rust, python, nodejs, wasm, c-ffi, java, go
- Java CI uses `actions/setup-java@v4` with `distribution: temurin` + `java-version: '17'` (provides
    both JDK and Maven -- no separate Maven setup needed)
- All CI jobs share the same action triple: checkout@v4, rust-toolchain@stable, rust-cache@v2
- Go CI job uses `actions/setup-go@v5` with `go-version-file: packages/go/go.mod` (version from
    go.mod, not hardcoded). Build chain: cargo build WASM → cp to packages/go → go test → go vet.
    Uses `CGO_ENABLED=0` and `-count=1` to prevent caching

## Registry/Publishing

- Cargo.toml `readme` field in `crates/iscc-lib/` points to `"README.md"` (crate-local)
- pyproject.toml `readme` field in `crates/iscc-py/` set to `"README.md"`
- npm auto-detects `README.md` in package directory -- no package.json change needed
- Package names on registries: `iscc-lib` (crates.io), `iscc-lib` (PyPI), `@iscc/lib` (npm),
    `@iscc/wasm` (npm/WASM), `io.iscc:iscc-lib` (Maven Central)
- iscc-wasm and iscc-jni both have `publish = false` in Cargo.toml -- no `readme` field needed (they
    publish via npm and Maven respectively, not crates.io)
- Maven coordinates: `io.iscc:iscc-lib:0.0.1` (pom.xml version synced to bare `0.0.1`)
- Version sync script: `scripts/version_sync.py` — reads workspace version from root `Cargo.toml`
    via regex `r'^version\s*=\s*"(.+?)"'`, updates `package.json` (json stdlib) and `pom.xml` (regex
    replacement). Supports `--check` flag. mise tasks: `version:sync`, `version:check`

## Release Workflow

- PR merge: `gh pr merge N --merge` (merge commit, not squash) preserves commit history
- Tag on main: `git tag vX.Y.Z && git push origin vX.Y.Z` triggers `.github/workflows/release.yml`
- Release workflow matches pattern `push: tags: [v*.*.*]`
- After tagging, switch back to develop: `git checkout develop`
- If local changes block branch switch, `git stash push -m "reason" <file>` then `git stash pop`
    after switching back
- OIDC trusted publishing for crates.io requires crate to exist first (first publish needs API
    token). PyPI supports pending trusted publishers. npm uses `NPM_TOKEN` secret
- Release workflow has 4 `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (all
    boolean, default false). Jobs: `publish-crates-io`, `build-wheels`, `build-sdist`,
    `publish-pypi`, `build-napi`, `publish-npm-lib`, `build-wasm`, `publish-npm-wasm`, `build-jni`,
    `assemble-jar`
- `build-jni` matrix: 5 platforms with `native-dir` and `lib-name` matrix vars matching NativeLoader
    conventions. Artifacts named `jni-{native-dir}` (e.g., `jni-linux-x86_64`)
- `assemble-jar` downloads `jni-*` artifacts into `jni-staging/`, iterates subdirectories to copy
    native libs to `src/main/resources/META-INF/native/{native-dir}/`. Uses
    `mvn package -DskipTests` and uploads JAR as `iscc-lib-jar` artifact
- `actions/download-artifact@v4` default behavior (no `merge-multiple`) creates per-artifact
    subdirectories named after the artifact — useful for iterating platform-specific downloads

## Documentation

- Ecosystem page: `docs/ecosystem.md` — covers official (iscc-core, iscc-lib) and community
    (iscc-core-ts) implementations. Uses `icon: lucide/globe`. Nav entry in `zensical.toml` placed
    between "Explanation" and "Reference" as a top-level entry
- `branciard/iscc-core-ts`: TypeScript port, Apache-2.0, v0.3.0, all 9 gen\_\*\_v0 and
    gen_iscc_id_v0/v1 and gen_flake_code_v0. Vendors official `data.json` (66KB). 263 tests across
    18 suites. Author: François Branciard. NGI Zero Core / NLnet funded. Status: active development,
    not production-ready
- How-to guide structure: YAML front matter (`icon`, `description`) → title → intro → installation →
    code generation (9 subsections: Meta, Text, Image, Audio, Video, Mixed, Data, Instance,
    ISCC-CODE) → streaming → text utilities → conformance testing → error handling
- Python uses `icon: lucide/terminal`, Node.js uses `icon: lucide/hexagon`, Go uses
    `icon: lucide/package`
- Go how-to emphasizes the `Runtime` lifecycle pattern (NewRuntime/Close) since it's unique to Go
- Go `bits` parameter is `uint32` (not `int32`) — verified in `packages/go/iscc.go`
- `GenIsccCodeV0` in Go does not expose a `bits` parameter (wide is hardcoded to false)
- Java uses `icon: lucide/coffee`
- zensical.toml nav: How-to Guides order is Rust → Python → Node.js → WebAssembly → Go → Java
- Go and Java guides include algorithm primitives section (SlidingWindow, AlgMinhash256,
    AlgCdcChunks, AlgSimhash) not present in Python/Node.js guides
- All 6 how-to guides have Codec operations + Constants sections. Python, Node.js, WASM, Go all have
    them; Java has them too. Python uniquely documents `core_opts` SimpleNamespace and IntEnum
    return types from `iscc_decode`. WASM constants are exported as uppercase getter functions
    (`META_TRIM_NAME()` etc.) via `js_name` attributes. Node.js Codec section uses `require()` style
    imports per next.md spec
- All 6 how-to guides complete: Rust (356 lines), Python (~420), Node.js (~350), WASM (~410), Go
    (463), Java (~390)
- `docs/architecture.md` and `docs/development.md` include all 6 binding crates (Python, Node.js,
    WASM, C FFI, JNI, Go) in diagrams, layout trees, and tables. Go uses dotted arrow (`-.->`) in
    Mermaid to indicate indirect WASM dependency via `iscc-ffi`
- Java guide key differences from Go: no runtime object (static methods), "Setup" section replaces
    "Runtime setup", streaming uses opaque `long` handles with try-finally (not defer),
    `genIsccCodeV0` exposes `boolean wide` parameter (Go hardcodes to false)
- docs/index.md landing page: 6 Quick Start tabs (Rust, Python, Node.js, Java, Go, WASM) + 7
    Available Bindings table rows. All tabs use `gen_text_code_v0("Hello World")`. mdformat
    auto-reformats JS imports to multi-line style in code blocks inside tabbed markdown
- WASM how-to (`docs/howto/wasm.md`) uses `@iscc/wasm` throughout (20 occurrences). Always verify
    npm package names against `docs/index.md` and `crates/*/README.md` — the wasm-pack howto
    originally had `@iscc/iscc-wasm` (wrong)

## Node.js Binding — Tier 1 Propagation

- napi-rs `#[napi]` on `pub const` works directly (no getter function fallback needed). `usize` to
    `u32` cast is safe for all 4 algorithm constants (all fit within u32 range)
- `IsccDecodeResult` uses `#[napi(object)]` struct with named fields (`maintype`, `subtype`,
    `version`, `length`, `digest`) — JavaScript has no tuples, so return an object instead
- `iscc_decode` napi wrapper destructures the Rust tuple `(u8, u8, u8, u8, Vec<u8>)` into
    `IsccDecodeResult` struct fields, converting `Vec<u8>` to `Buffer` via `.into()`
- napi-rs `#[napi(js_name = "...")]` on constants uses the original SCREAMING_SNAKE_CASE name to
    prevent napi-rs auto-conversion to camelCase
- Total Node.js test count after 7 new symbols: 124 (103 existing + 21 new across 7 describe blocks)

## WASM Binding — Tier 1 Propagation

- wasm-bindgen does NOT support `#[wasm_bindgen]` on `pub const` — use getter functions with
    `#[wasm_bindgen(js_name = "SCREAMING_CASE")]` instead. Safe `as u32` cast (all values fit)
- `IsccDecodeResult` WASM struct uses `#[wasm_bindgen(getter_with_clone)]` because `Vec<u8>` is not
    `Copy`. The `digest` field maps to `Uint8Array` in JS
- wasm-bindgen accepts `&str` and `&[u8]` directly (like PyO3, unlike napi-rs which needs owned
    `String`/`Buffer`). No `.as_deref()` or `.as_ref()` conversion needed for these types
- Total WASM test count after 7 new symbols: 59 unit + 1 conformance_selftest (with feature) + 9
    conformance, from 40 unit previously
- `#[wasm_bindgen` annotation count in lib.rs: 35 (was 25, +10 for 7 functions + 2 impl blocks + 1
    struct)

## C FFI Binding — Tier 1 Propagation

- Constants exposed as `extern "C"` getter functions (not `pub static` — avoids cbindgen `usize` → C
    type mapping issues). All are infallible (no error handling, no `clear_last_error`)
- `iscc_json_to_data_url` follows the standard string-in/string-out pattern (same as
    `iscc_text_clean`)
- `iscc_encode_component` takes raw `*const u8` + `usize` for digest, with the standard null-check +
    `from_raw_parts` pattern from `iscc_gen_data_code_v0`
- `IsccDecodeResult` is `#[repr(C)]` struct with `ok: bool` discriminant,
    `maintype/subtype/version/   length: u8`, and `digest: IsccByteBuffer`. Reuses existing
    `IsccByteBuffer` and helpers (`null_byte_buffer`, `vec_to_byte_buffer`)
- `iscc_free_decode_result` delegates to `iscc_free_byte_buffer` for digest cleanup
- `ptr_to_str` in FFI crate takes `param_name: &str` arg for error messages (not just `ptr` like
    next.md pseudocode suggested) — all new functions use this pattern
- Length index for 64-bit codes is 1 (not 0): `decode_length` uses `(length_index + 1) * 32` for
    standard MainTypes. Index 0 = 32-bit, index 1 = 64-bit
- Generated `iscc.h` header is NOT committed — CI generates it dynamically via `cbindgen`
- Total `#[unsafe(no_mangle)]` count after propagation: 44 (was 35, +9: 4 constants +
    json_to_data_url
    - encode_component + iscc_decode + iscc_free_decode_result + the existing ones)
- Total Rust unit tests: 77 (62 existing + 15 new). Total C test assertions: 49 (30 existing + 19
    new)

## Tier 1 API Surface

- Algorithm constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`)
    are `pub const` at crate root in `lib.rs`, placed after `pub use` re-exports
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields, validates with
    `TryFrom<u8>`, adds explicit digest length check (`digest.len() < bit_length / 8`), then
    delegates to `codec::encode_component`. No naming conflict because `codec::encode_component` is
    NOT re-exported at crate root
- Magic numbers 128, 4096, 13 in gen functions replaced with constants `META_TRIM_NAME`,
    `META_TRIM_DESCRIPTION`, `TEXT_NGRAM_SIZE` respectively
- `IO_READ_SIZE` uses spec value 4_194_304 (4 MB), not Python reference value 2_097_152 (2 MB)
- `iscc_decode` Tier 1 wrapper in `lib.rs` takes `&str`, returns `(u8, u8, u8, u8, Vec<u8>)` —
    strips "ISCC:" prefix and dashes, delegates to `codec::decode_base32` → `codec::decode_header` →
    `codec::decode_length`, truncates tail to exact digest bytes. Unlike Python ref which returns
    full tail, our API returns usable digest directly
- `json_to_data_url` in `lib.rs` combines `parse_meta_json` + `build_meta_data_url` private helpers
    into one public API. Defined directly in `lib.rs` (not in a submodule), so no `pub use`
    re-export needed. Deps: `serde_json`, `serde_json_canonicalizer`, `data_encoding` — all already
    present. Output differs from conformance vector test_0016 in two ways: no `charset=utf-8`
    parameter, and payload is JCS-canonical (spaces removed)

## Python Binding — Tier 1 Propagation

- PyO3 `iscc_decode` wrapper needs `py: Python<'_>` param to wrap `Vec<u8>` in `PyBytes::new()`.
    Returns `PyObject` using `.into_pyobject(py)?.into()` for the tuple
- PyO3 constants registered with `m.add("NAME", value)?` in module init (not `wrap_pyfunction!`)
- Python `__init__.py` `__all__` had 34 entries before Tier 1 propagation, not 35 as estimated. The
    count after adding 7 new symbols is 41 (34 + 7)
- Constants and simple functions (encode_component, iscc_decode, json_to_data_url) are direct
    re-exports in `__init__.py` — no wrapper logic needed (unlike gen_data_code_v0 which adds
    streaming)
- Type stubs (`_lowlevel.pyi`) place constants at top (before function stubs), with inline
    docstrings. Constants use `int` type annotation
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` works; bare `maturin develop` fails (command
    not found in devcontainer PATH — needs `uv run` prefix)

## Codec Internals

- `decode_header` and `decode_varnibble_from_bytes` operate directly on `&[u8]` with bitwise
    extraction — no intermediate `Vec<bool>` allocation. Helpers: `get_bit(data, bit_pos)` reads a
    single bit, `extract_bits(data, bit_pos, count)` reads N bits as u32 (both MSB-first)
- `encode_header` and `encode_varnibble` still use `Vec<bool>` internally (encode path is less
    performance-sensitive)
- `bytes_to_bits` and `bits_to_u32` are `#[cfg(test)]` only — used by test helpers but not
    production code
- `bits_to_bytes` is still in production code (used by `encode_header`)
- Rust 1.93+ clippy lints: `collapsible_if` and `manual_div_ceil` (use `n.div_ceil(d)` instead of
    `(n + d - 1) / d`)

## Streaming Optimization

- `DataHasher` uses a persistent `buf: Vec<u8>` that is reused across `update()` calls — no per-call
    allocations. Pattern: `extend_from_slice` → CDC → hash complete chunks → extract `tail_len` as
    `usize` (releases borrow) → `copy_within` + `truncate` to shift tail to front
- Borrow checker constraint: CDC chunks borrow from `self.buf`, so extract `tail_len` before
    `drop(chunks)`, then mutate `self.buf`. Explicit `drop(chunks)` makes the borrow release visible
- Benchmark: `DataHasher` streaming at 64 KiB chunks on 1 MB data achieves ~1.1 GiB/s throughput

## API Generics

- Video API (`soft_hash_video_v0`, `gen_video_code_v0`) uses `<S: AsRef<[i32]> + Ord>` instead of
    concrete `&[Vec<i32>]`. This allows FFI to pass `&[&[i32]]` (zero-copy borrows) while other
    bindings continue passing `&[Vec<i32>]` unchanged. `AsRef<[i32]>` gives slice access, `Ord`
    enables `BTreeSet` deduplication. Body uses `.as_ref()` for element access
- FFI video wrappers use `Vec<&[i32]>` (1 remaining `.to_vec()` in FFI crate is for
    `alg_cdc_chunks`)

## Go Pure Go Rewrite

- Pure Go codec module: `packages/go/codec.go` — type enums (`MainType`, `SubType`, `Version` as
    typed constants with `iota`), varnibble header encoding/decoding, base32/base64, length
    encoding/decoding, unit encoding/decoding, `EncodeComponent`, `IsccDecompose`, `IsccDecode`.
    Zero external dependencies (Go standard library only)
- Go type enum naming: `MTMeta`..`MTFlake` (not `MainTypeMeta` etc.), `STNone`..`STWide`,
    `STText = STNone` alias, `VSV0 Version = 0`
- Go codec internal helpers are unexported (lowercase): `encodeHeader`, `decodeHeader`,
    `encodeLength`, `decodeLength`, `encodeBase32`, `decodeBase32`, `encodeVarnibble`,
    `decodeVarnibbleFromBytes`, `getBit`, `extractBits`, `bitsToBytes`, `encodeUnits`,
    `decodeUnits`, `encodeComponentInternal`, `popcount`
- Go codec public API: `EncodeBase64`, `EncodeComponent` (takes `uint8` for enum fields),
    `IsccDecompose`, `IsccDecode` (returns `*DecodeResult` from `iscc.go`)
- `IsccDecode` reuses existing `DecodeResult` struct from `iscc.go` (same package, no duplication)
- Go base32 uses `base32.StdEncoding.WithPadding(base32.NoPadding)` — RFC 4648 uppercase, no padding
- Go base64 uses `base64.RawURLEncoding` — RFC 4648 §5 URL-safe, no padding
- Go codec test file: `packages/go/codec_test.go` with `TestCodec*` naming convention. All 48 tests
    independent of WASM binary (pure Go). Conformance tests load data.json via
    `os.ReadFile("../../crates/iscc-lib/tests/data.json")`
- Dependency chain for pure Go rewrite: codec (done) → text utils → algorithms → gen functions →
    streaming hashers → conformance selftest

## Gotchas

- `pop_local_frame` is `unsafe` in jni crate v0.21 (Rust 2024 edition) — must wrap in `unsafe {}`
    with SAFETY comment. For helper functions returning `Result`, use `?` after the unsafe block.
    For `throw_and_default` error paths, use `let _ =` to discard the pop result (JVM cleans up on
    native return)
- JNI package underscore encoding: `iscc_lib` -> `iscc_1lib` in function names
- Java `byte` is signed -- casting int (0-255) to byte works correctly for pixel data
- HexFormat requires Java 17+ (already set as Maven compiler target)
- mdformat auto-formats markdown files in pre-commit -- write READMEs with compatible formatting (no
    smart dashes, use `--` not em-dashes in markdown text)
- mdformat-mkdocs + `--wrap 100` crashes ("renders to different HTML") when long backtick chains
    (e.g., func_a/func_b/func_c in backticks) cross the wrap boundary. Keep backtick expressions
    short or restructure as abbreviated references. Also avoid `char *` (backtick + asterisk) in
    prose -- the asterisk can be mis-parsed as emphasis during wrapping
- WASM quick start must use ESM `import`/`await init()` (not CommonJS `require()`) -- wasm-bindgen
    requires async WASM initialization
- README template: 6 H2 sections (What is ISCC, Installation, Quick Start, API Overview, Links,
    License), 70-80 lines each, identical "What is ISCC" paragraph and Links section across all
    crates. Go README adds an extra Architecture section (wazero/no-cgo details). All 7
    crates/packages now have READMEs. iscc-ffi README has Building (not Installation) + Memory
    Management sections
- Root README "What is iscc-lib" paragraph uses "language bindings" (not just "bindings") to ensure
    mdformat (wrap=100) puts the full language list on one grep-matchable line. Careful with
    rewording — mdformat rewrapping can split the list across lines
- Java quick start must pass all parameters explicitly (no default arguments in Java) --
    `genMetaCodeV0("...", null, null, 64)` not `genMetaCodeV0("...")`
- NativeLoader pattern: JAR extraction from `META-INF/native/{os}-{arch}/{libname}` → temp dir →
    `System.load()`, with `System.loadLibrary("iscc_jni")` fallback. `synchronized` + `volatile`
    guard. Uses `NativeLoader.class.getResourceAsStream()` (not ClassLoader) for fat JAR/OSGi
    compatibility. CI uses the fallback path via Surefire `-Djava.library.path`
- Python `__init__.py` module-level constants must go AFTER imports -- ruff E402 (module level
    import not at top of file) fires if a constant is placed between import groups
- `__version__` in `__init__.py` uses `importlib.metadata.version("iscc-lib")` — reads from
    installed package metadata, which maturin populates from Cargo.toml via `dynamic = ["version"]`.
    Place it after `from __future__ import annotations` and `from importlib.metadata import version`
- When `maturin develop` installs a version, it persists in the venv — if the workspace version
    changes in Cargo.toml, must rebuild with `maturin develop` to sync the installed version
- Dict meta pattern in `gen_meta_code_v0` Python wrapper: `import json as _json` (underscore alias
    to avoid namespace pollution), `isinstance(meta, dict)` →
    `_json.dumps(meta, separators=(",",   ":"), ensure_ascii=False)` → `json_to_data_url()`. The
    Rust `json_to_data_url` handles JCS canonicalization internally, so the Python side only needs
    compact JSON serialization
- PIL pixel data pattern in `gen_image_code_v0` Python wrapper: widen signature to
    `bytes | bytearray | memoryview | Sequence[int]`, use
    `if not isinstance(pixels, bytes):   pixels = bytes(pixels)`. The `bytes()` constructor handles
    bytearray, memoryview, and Sequence[int] (including PIL's ImagingCore from `Image.getdata()`)
    uniformly. No Rust changes needed — conversion is Python-wrapper-only. This same pattern applies
    to any future function that accepts `&[u8]` in Rust but needs wider input types in Python
- Python IntEnum classes (`MT`, `ST`, `VS`) in `__init__.py`: pure Python, no Rust dependency. `ST`
    has `TEXT = 0` alias for `NONE` (IntEnum allows duplicate values as aliases — first definition
    wins). `iscc_decode` wrapper converts raw integers to IntEnum types. `core_opts` is a
    `SimpleNamespace` mapping attribute names to existing constants. Total `__all__` entries: 45 (41
    \+ MT, ST, VS, core_opts)
