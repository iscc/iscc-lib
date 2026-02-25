# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

## Code Locations

- JNI Java wrapper: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- JNI Rust bridge: `crates/iscc-jni/src/lib.rs`
- JNI Java tests: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- JNI Maven config: `crates/iscc-jni/java/pom.xml`
- Conformance vectors: `crates/iscc-lib/tests/data.json` (46 total: 16+5+3+5+3+2+4+3+5)
- Node.js conformance tests: `crates/iscc-napi/__tests__/conformance.test.mjs`
- Per-crate READMEs: `crates/iscc-lib/README.md`, `crates/iscc-py/README.md`,
    `crates/iscc-napi/README.md`, `crates/iscc-wasm/README.md`, `crates/iscc-jni/README.md`
- Root README: `README.md` — covers all languages (Rust, Python, Node.js, Java, WASM)

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
- JNI error handling: all `unwrap()` calls replaced with `throw_and_default` pattern. Three forms:
    (1) nested match for `env.new_string().into_raw()`, (2) nested match for
    `env.byte_array_from_slice().into_raw()`, (3) early-return match + `if let Err` for loop bodies

## WASM/WASI

- `iscc-ffi` compiles as wasm32-wasip1 from existing `crate-type = ["cdylib", "staticlib"]` — no
    Cargo.toml changes needed. The cdylib target produces the `.wasm` file
- `iscc_alloc`/`iscc_dealloc` are the WASM host memory management pair — host allocates via
    `iscc_alloc`, writes data, calls FFI functions, then frees via `iscc_dealloc`
- Debug WASM binary is ~10.5MB; release + wasm-opt would reduce significantly
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
- Maven coordinates: `io.iscc:iscc-lib:0.0.1` (pom.xml has `0.0.1-SNAPSHOT`, README uses `0.0.1`)

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
- WASM quick start must use ESM `import`/`await init()` (not CommonJS `require()`) -- wasm-bindgen
    requires async WASM initialization
- README template: 6 H2 sections (What is ISCC, Installation, Quick Start, API Overview, Links,
    License), 70-80 lines each, identical "What is ISCC" paragraph and Links section across all
    crates. All 5 publishable crates now have READMEs; iscc-ffi is not published and has no README
- Root README "What is iscc-lib" paragraph (line ~46) still says "Python, Node.js, WebAssembly, and
    C" without Java -- Key Features line was updated but this paragraph was out of scope
- Java quick start must pass all parameters explicitly (no default arguments in Java) --
    `genMetaCodeV0("...", null, null, 64)` not `genMetaCodeV0("...")`
- Python `__init__.py` module-level constants must go AFTER imports -- ruff E402 (module level
    import not at top of file) fires if a constant is placed between import groups
