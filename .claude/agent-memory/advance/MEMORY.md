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
- Go Runtime has 45 methods total: 24 public (Close, ConformanceSelftest, TextClean,
    TextRemoveNewlines, TextCollapse, TextTrim, EncodeBase64, SlidingWindow, IsccDecompose,
    AlgSimhash, AlgMinhash256, AlgCdcChunks, SoftHashVideoV0, 9 gen\_\*\_v0, NewDataHasher,
    NewInstanceHasher) + 21 private helpers
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
- All 6 how-to guides complete: Rust (356 lines), Python (353), Node.js (281), WASM (338), Go (388),
    Java (321)
- Java guide key differences from Go: no runtime object (static methods), "Setup" section replaces
    "Runtime setup", streaming uses opaque `long` handles with try-finally (not defer),
    `genIsccCodeV0` exposes `boolean wide` parameter (Go hardcodes to false)
- docs/index.md landing page: 6 Quick Start tabs (Rust, Python, Node.js, Java, Go, WASM) + 7
    Available Bindings table rows. All tabs use `gen_text_code_v0("Hello World")`. mdformat
    auto-reformats JS imports to multi-line style in code blocks inside tabbed markdown
- WASM how-to (`docs/howto/wasm.md`) uses `@iscc/wasm` throughout (20 occurrences). Always verify
    npm package names against `docs/index.md` and `crates/*/README.md` — the wasm-pack howto
    originally had `@iscc/iscc-wasm` (wrong)

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
