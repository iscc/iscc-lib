# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

## Scope Calibration

- Java bindings follow the established multi-step pattern: JNI bridge → wrapper class → tests → CI
    job → loader → docs. Each step is independently verifiable. The review agent confirmed this
    progression works well (three PASS verdicts so far).
- CI job additions are small, single-file changes that provide high value (makes existing local
    tests CI-verified). Good candidate for quick iterations. Pattern: copy existing job structure,
    swap language-specific setup action and build/test commands.
- After CI job: the next logical Java steps are native library loader (for JAR distribution), then
    README/docs updates. Go bindings are a separate track that can start in parallel.
- Per-crate READMEs: batch into groups of 2-3 to stay within scope limits. Batch 1 = iscc-lib,
    iscc-py, iscc-napi (primary publishable crates) — done. Batch 2 = iscc-wasm, iscc-jni (two
    remaining publishable crates) — done. iscc-ffi is not published to any registry, so its README
    is lower priority. Batch 3 = packages/go (after Go bindings exist).
- README files are "create" operations (greenfield), not "modify" — they're less risky than code
    changes. Manifest updates are trivial one-liners. Combined, 3 creates + 2 modifies is a
    reasonable single step for documentation work.
- Normal performance issues (codec Vec<bool>, DataHasher copying, FFI video frame allocation) are
    non-blocking optimizations. They don't affect correctness or user-facing functionality. Starting
    Go bindings (largest feature gap) takes priority over clearing optimization issues.
- Root README updates: single-file modifications with clear verification. Java sections follow
    established patterns (Rust/Python/Node.js/WASM already present). Section ordering should match
    target.md: Rust, Python, Java, Node.js, WASM. Insert Java after Node.js and before WASM in both
    Installation and Quick Start sections.
- After root README Java: next candidates are (1) docs/howto/java.md (create + zensical.toml nav
    update), (2) Java native loader class, (3) Go bindings. The howto guide is a natural follow-up
    since it continues the documentation story.
- Critical issues always take priority regardless of feature trajectory. The JNI unwrap() issue is a
    safety fix — pure mechanical replacement (21 calls, 3 patterns, 1 file). Good scope: small,
    well-defined, zero behavioral change, easy to verify with `grep -c 'unwrap()'`.
- Python binding issues (bytes-like misclassification + unbounded .read()) are a natural pair: same
    4 call sites, same file, interrelated fix logic. Good scope for a single iteration — one
    production file + one test file, clear verification via grep + pytest. The handoff and state.md
    both recommended this as the highest-priority next step after the JNI safety fix.
- JNI jint validation + local reference overflow are a natural pair: same file, both robustness
    fixes, no behavioral change for valid inputs. Combined scope is ~40 lines of Rust changes + ~15
    lines of Java tests. 2 files modified (1 Rust + 1 Java test), well within 3-file limit.
- Multiple small issues in the same crate are a natural batch. The 3 napi issues (version skew, npm
    packaging, unnecessary clone) touch only 2 files (`package.json` + `src/lib.rs`) and are all
    quick fixes. The version skew requires a `napi build` regeneration step (not a code change)
    which the advance agent must run. Gitignored generated files (`index.js`) don't count toward the
    3-file limit since they're build artifacts.
- After napi cleanup: WASM silent-null is the next single-crate fix (2 files: lib.rs + unit.rs).
    Mechanical: change return type to `Result<JsValue, JsError>`, swap `.unwrap_or(JsValue::NULL)`
    to `.map_err(...)`, add `.unwrap()` in tests. Every other WASM function already uses this
    pattern so consistency is the primary motivation.
- Go bindings multi-step plan: (1) WASI build + alloc/dealloc ✅ → (2) Go module scaffold + wazero
    bridge + memory helpers ✅ → (3) gen\_\*\_v0 wrappers + conformance tests ✅ → (4) CI job
    (current) → (5) remaining 12 Tier 1 wrappers (text utils, algo primitives, streaming hashers) →
    (6) README/docs. CI before remaining wrappers ensures regression protection as the API grows.
- Go scaffold scoping: the WASM binary (~10.5 MB debug) is NOT checked into git. Uses `//go:embed`
    with a gitignored binary built by `cargo build -p iscc-ffi --target wasm32-wasip1`. TestMain
    skips gracefully if binary is missing.
- Go gen\_\*\_v0 wrappers need 4 memory helper categories: (1) writeString (existing), (2)
    writeBytes for raw byte data (image/data/instance), (3) writeI32Slice for int32 arrays (audio),
    (4) writeStringArray + writeI32ArrayOfArrays for pointer-to-pointer patterns (mixed/iscc/video).
    WASM32 is little-endian for all integer writes. Pointer arrays use uint32 elements (4 bytes
    each).
- Go module path is `github.com/iscc/iscc-lib/packages/go` with package name `iscc`. This matches
    the `go get` path in target.md.
- Go/wazero requires WASI instantiation (`wasi_snapshot_preview1.MustInstantiate`) because iscc-ffi
    targets `wasm32-wasip1`. Without WASI, the module won't instantiate.
- `iscc_last_error()` returns a borrowed pointer (not owned) — the Go bridge must NOT call
    `iscc_free_string` on it. This differs from `iscc_text_clean` etc. which return owned strings.
- Go installation via mise is the cleanest approach — add `go = "latest"` to `[tools]` section in
    `mise.toml`. This avoids Dockerfile changes and works in both devcontainer and CI contexts.
- Go CI job is unique among CI jobs: it has a cross-compilation build step (Rust → wasm32-wasip1)
    before the language-specific test phase. The WASM binary is ~10.5 MB debug. Use
    `dtolnay/rust-toolchain@stable` with `targets: wasm32-wasip1` to avoid a separate rustup step.
    Use `actions/setup-go@v5` with `go-version-file` instead of hardcoded version.

## Architecture Decisions

- Java conformance tests use `data.json` from `crates/iscc-lib/tests/data.json` (shared across all
    bindings) via relative path from Maven's working directory.
- Maven Surefire plugin sets `java.library.path` to `target/debug/` for finding the native cdylib.
    This means `cargo build -p iscc-jni` must run before `mvn test`.
- Gson chosen as JSON parsing library for Java tests — handles nested arrays (`int[][]` for video
    frame sigs) cleanly and is a well-known, lightweight test dependency.
- Go bindings use WASM/wazero (pure Go, no cgo) per target spec. The WASM module is built from
    iscc-ffi targeting `wasm32-wasip1`. The Go wrapper embeds the `.wasm` binary via `//go:embed`.
    Alloc/dealloc helpers are needed because the WASM host must allocate memory inside the module to
    pass strings and byte buffers. `iscc_alloc(size) -> *mut u8` and `iscc_dealloc(ptr, size)` are
    the standard pattern for WASM FFI memory management.
- The FFI crate's existing `crate-type = ["cdylib", "staticlib"]` works for wasm32-wasip1 — cargo
    produces a `.wasm` from the cdylib target. No Cargo.toml changes needed for the build.
- `thread_local!` in the FFI crate (for error storage) should work on wasm32-wasip1 since WASM is
    single-threaded. The macro compiles but degenerates to a simple static.

## Registry README Patterns

- napi-rs `gen_*_v0` functions return `String` (not structured objects) — Node.js quick start
    examples must show string return, not `result.iscc` pattern.

- Python bindings return `dict` (via PyO3 `PyDict`) — quick start uses `result['iscc']`.

- Rust core returns typed `*CodeResult` structs with `.iscc` field.

- `crates/iscc-lib/Cargo.toml` currently has `readme = "../../README.md"` — must change to
    `"README.md"` when per-crate README is created.

- `crates/iscc-py/pyproject.toml` has no `readme` field — needs `readme = "README.md"` added.

- npm auto-detects `README.md` in the package directory — no `package.json` change needed.

- WASM binding (`iscc-wasm`) also returns strings from gen functions (same as napi-rs). It publishes
    to npm as `@iscc/wasm` via wasm-pack, not to crates.io (Cargo.toml has `publish = false`).

- JNI binding (`iscc-jni`) publishes to Maven Central as `io.iscc:iscc-lib`, not crates.io
    (Cargo.toml has `publish = false`). Java method names use camelCase (e.g., `genMetaCodeV0`).
    Java gen functions return `String` (ISCC code string) — quick start shows direct string result.

- Batch 1 READMEs landed at 70-75 lines each — slightly under the originally suggested 80-120 range
    but the review agent confirmed they were "complete and well-structured." Target 70-80 lines for
    batch 2.

## CI Workflow Patterns

- All CI jobs share a common preamble: `actions/checkout@v4` → `dtolnay/rust-toolchain@stable` →
    `Swatinem/rust-cache@v2`, then language-specific setup and build/test commands.
- Language-specific setup actions: `actions/setup-python@v5`, `actions/setup-node@v4`,
    `actions/setup-java@v4` (with `distribution: 'temurin'`), `actions/setup-go@v5` (with
    `go-version-file`).
- Never use `mise` in CI — call tools directly per learnings.
- Maven Surefire's `${project.basedir}` resolves to the pom.xml directory, so
    `${project.basedir}/../../../target/debug` reaches the workspace root's build output.
- Go CI job is the only one with a cross-compilation pre-step: Rust → wasm32-wasip1 → copy to
    packages/go/ before running Go tests. All other binding jobs build native (same-platform) Rust.

## napi-rs Packaging

- `index.js` and `index.d.ts` are auto-generated by `npx napi build` and gitignored. CI regenerates
    them each run. The version embedded in `index.js` comes from `package.json`'s `version` field at
    build time.
- `npm publish` falls back to `.gitignore` when no `"files"` field or `.npmignore` exists. Since
    `.gitignore` excludes `index.js`/`index.d.ts`, a `"files"` allowlist in `package.json` is
    required for correct publishing.
- napi-rs `Buffer::from()` accepts `Vec<u8>` directly. For `&[u8]` slices (from `alg_cdc_chunks`),
    check if `From<&[u8]>` is implemented in napi v3; if not, `.to_vec()` is still needed but the
    code should use `into_iter()` for clarity.

## Recurring Patterns

- All binding conformance tests follow the same structure: load data.json, iterate per-function test
    groups, decode inputs per function signature, compare `.iscc` output field. The Node.js test
    (`conformance.test.mjs`) is the cleanest template to mirror.
- `gen_iscc_code_v0` test vectors have no `wide` parameter in data.json — always pass `false` (the
    Python default).
- `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data for `gen_data_code_v0` and
    `gen_instance_code_v0`. Empty after prefix = empty bytes.

## Python Binding Patterns

- `ty` type checker does NOT support `hasattr()` narrowing — must use `isinstance` inversion for
    stream detection. Pattern: `if not isinstance(data, (bytes, bytearray, memoryview))` narrows to
    BinaryIO; `elif not isinstance(data, bytes)` narrows to bytearray|memoryview.
- For stream inputs in `gen_data_code_v0`/`gen_instance_code_v0`: use \_DataHasher/\_InstanceHasher
    with chunked reads (64 KiB) instead of unbounded `.read()`. This avoids memory exhaustion and
    exercises the streaming Rust code path.
- For `DataHasher.update`/`InstanceHasher.update` stream inputs: chunked read loop feeding the inner
    Rust hasher. The constructor delegates to update(), so only update() needs the fix.

## JNI Safety Patterns

- JNI `extern "system"` functions must never panic — with `panic = "abort"` in release, a panic
    aborts the entire JVM. All JNI env operations (`new_string`, `byte_array_from_slice`,
    `set_object_array_element`, etc.) return `jni::errors::Result` and must be handled.
- The `throw_and_default` helper is the standard error-handling pattern: throws a Java exception and
    returns `T::default()` (null for pointer types, 0 for primitives, false for booleans).
- There are 3 unwrap patterns in the JNI crate: (A) `env.new_string().unwrap().into_raw()` for
    string returns, (B) `env.byte_array_from_slice().unwrap().into_raw()` for byte array returns,
    (C) loop-body unwraps in `algCdcChunks`. All follow the same fix: match + throw_and_default.

## Gotchas

- JNI function names encode Java package underscores as `_1` (e.g., `iscc_lib` → `iscc_1lib`). The
    Java `native` method names must match the Rust `extern "system"` function names exactly after
    the JNI name-mangling prefix.
- `gen_image_code_v0` pixels in data.json are JSON int arrays (0-255) that need casting to Java
    `byte` (signed). Java's `byte` range is -128 to 127, so values 128-255 will wrap — this is fine
    because the JNI bridge handles the conversion correctly.
- Maven's working directory is the pom.xml parent directory, not the workspace root. All relative
    paths in Java tests must be calculated from `crates/iscc-jni/java/`.
- ISCC Foundation URL is `https://iscc.io` — not iscc.foundation or other variants.
