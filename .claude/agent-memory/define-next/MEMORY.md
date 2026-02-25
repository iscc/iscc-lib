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
- Root README updates: single-file modifications with clear verification. Java sections follow
    established patterns (Rust/Python/Node.js/WASM already present). Section ordering should match
    target.md: Rust, Python, Java, Node.js, WASM. Insert Java after Node.js and before WASM in both
    Installation and Quick Start sections.
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
- Go bindings multi-step plan: all 8 steps completed (WASI build → module scaffold → gen wrappers →
    CI job → README → remaining 12 wrappers → streaming hashers → root README Go section). Follow-up
    items: Go `io.Reader` streaming (optional, not explicitly in verified-when), docs/howto guides.
- Java how-to guide: differs from Go in key ways — no runtime object (static methods via
    `IsccLib.*`), streaming uses opaque `long` handles (not structs) requiring try-finally for
    `*Free` calls, build-from-source installation since Maven Central publishing isn't wired yet.
- Java native loader: NativeLoader.java is a well-known JNI pattern (sqlite-jdbc, netty-tcnative).
    Two-phase loading: (1) try JAR resource extraction from `META-INF/native/{os}-{arch}/`, (2) fall
    back to `System.loadLibrary`. Existing CI still works via fallback since no native libs are
    bundled in the JAR yet. This is complete — created in iteration 16.
- **Post-feature optimization phase**: All binding targets met (Rust, Python, Node.js, WASM, C FFI,
    Java partial, Go partial). Remaining target gaps are publishing infrastructure (OIDC, Maven
    Central, npm pipelines) which are complex multi-file CI tasks. Normal issues (codec Vec<bool>,
    DataHasher copying, FFI video allocation) are good single-file optimization steps.
- **Codec Vec<bool> optimization complete** (iteration 18): direct bitwise extraction in
    decode_header, verified by review. DataHasher buffer optimization is next natural optimization
    step.
- **DataHasher buffer optimization** (iteration 19, PASS): persistent `Vec<u8>` replacing per-call
    `to_vec()`/`concat()`. Completed successfully.
- **FFI video frame allocation**: the core video API takes `&[Vec<i32>]` but FFI must construct from
    raw pointers. Using Rust generics `<S: AsRef<[i32]> + Ord>` makes the API accept both
    `&[Vec<i32>]` (existing callers unchanged) and `&[&[i32]]` (FFI can pass borrowed slices). This
    is backward-compatible and limits changes to 2 files (core + FFI). The `Ord` bound is needed for
    `BTreeSet` deduplication inside `soft_hash_video_v0`. Both `Vec<i32>` and `&[i32]` implement
    `AsRef<[i32]> + Ord`.
- **Generic API for backward-compatible optimization**: when an internal optimization requires
    accepting a broader type (e.g., borrowed slices instead of owned Vecs), prefer generic bounds
    (`AsRef<T> + OtherTraits`) over concrete type changes. This avoids cascading modifications
    across all binding crates.
- **Low-priority internal validation fixes**: `pub(crate)` functions with incorrect or missing
    validation are good candidates for batching — same crate, no binding changes, clear tests. The
    `alg_dct` + `alg_wtahash` pair is 3 files (dct.rs, wtahash.rs, lib.rs caller) and purely
    additive (new error paths, no behavioral change for valid inputs). When changing a function's
    return type (e.g., `Vec<u8>` → `IsccResult<Vec<u8>>`), check all callers — if the caller already
    returns `IsccResult`, adding `?` is trivial.
- **Low-priority housekeeping phase**: All [normal] issues resolved, only [low] issues remain. These
    are good small iteration targets: WASM conformance feature gate (3 non-test files), stale
    CLAUDE.md updates (1 file), JNI exception type mapping (1 file + test). Feature gates involving
    CI changes count as 3 files (Cargo.toml + lib.rs + ci.yml) — test files don't count against the
    limit. Group related [low] issues in the same crate only if they share files.

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
    pass strings and byte buffers.
- The FFI crate's existing `crate-type = ["cdylib", "staticlib"]` works for wasm32-wasip1 — cargo
    produces a `.wasm` from the cdylib target. No Cargo.toml changes needed for the build.
- `thread_local!` in the FFI crate (for error storage) should work on wasm32-wasip1 since WASM is
    single-threaded. The macro compiles but degenerates to a simple static.

## Registry README Patterns

- napi-rs `gen_*_v0` functions return `String` (not structured objects) — Node.js quick start
    examples must show string return, not `result.iscc` pattern.
- Python bindings return `dict` (via PyO3 `PyDict`) — quick start uses `result['iscc']`.
- Rust core returns typed `*CodeResult` structs with `.iscc` field.
- WASM binding (`iscc-wasm`) also returns strings from gen functions (same as napi-rs). It publishes
    to npm as `@iscc/wasm` via wasm-pack, not to crates.io (Cargo.toml has `publish = false`).
- JNI binding (`iscc-jni`) publishes to Maven Central as `io.iscc:iscc-lib`, not crates.io
    (Cargo.toml has `publish = false`). Java method names use camelCase (e.g., `genMetaCodeV0`).
- Go README: Go module proxy (pkg.go.dev) renders README.md. Go gen functions return
    `(string, error)` — quick start uses `iscc, err := rt.GenMetaCodeV0(ctx, ...)` pattern.

## CI Workflow Patterns

- All CI jobs share a common preamble: `actions/checkout@v4` → `dtolnay/rust-toolchain@stable` →
    `Swatinem/rust-cache@v2`, then language-specific setup and build/test commands.
- Language-specific setup actions: `actions/setup-python@v5`, `actions/setup-node@v4`,
    `actions/setup-java@v4` (with `distribution: 'temurin'`), `actions/setup-go@v5` (with
    `go-version-file`).
- Never use `mise` in CI — call tools directly per learnings.
- Go CI job is the only one with a cross-compilation pre-step: Rust → wasm32-wasip1 → copy to
    packages/go/ before running Go tests. All other binding jobs build native (same-platform) Rust.

## Recurring Patterns

- All binding conformance tests follow the same structure: load data.json, iterate per-function test
    groups, decode inputs per function signature, compare `.iscc` output field.
- `gen_iscc_code_v0` test vectors have no `wide` parameter in data.json — always pass `false` (the
    Python default).
- `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data for `gen_data_code_v0` and
    `gen_instance_code_v0`. Empty after prefix = empty bytes.

## Gotchas

- JNI function names encode Java package underscores as `_1` (e.g., `iscc_lib` → `iscc_1lib`).
- `gen_image_code_v0` pixels in data.json are JSON int arrays (0-255) that need casting to Java
    `byte` (signed). Java's `byte` range is -128 to 127, so values 128-255 will wrap — this is fine
    because the JNI bridge handles the conversion correctly.
- Maven's working directory is the pom.xml parent directory, not the workspace root.
- ISCC Foundation URL is `https://iscc.io` — not iscc.foundation or other variants.
