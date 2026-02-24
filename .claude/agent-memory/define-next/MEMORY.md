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
- Per-crate READMEs: batch into groups of 3 to stay within scope limits. Batch 1 = iscc-lib,
    iscc-py, iscc-napi (primary publishable crates). Batch 2 = iscc-wasm, iscc-ffi, iscc-jni
    (secondary). Batch 3 = packages/go (after Go bindings exist).
- README files are "create" operations (greenfield), not "modify" — they're less risky than code
    changes. Manifest updates are trivial one-liners. Combined, 3 creates + 2 modifies is a
    reasonable single step for documentation work.
- Normal performance issues (codec Vec<bool>, DataHasher copying) should wait until feature work is
    done. They don't block publishing or new bindings.

## Architecture Decisions

- Java conformance tests use `data.json` from `crates/iscc-lib/tests/data.json` (shared across all
    bindings) via relative path from Maven's working directory.
- Maven Surefire plugin sets `java.library.path` to `target/debug/` for finding the native cdylib.
    This means `cargo build -p iscc-jni` must run before `mvn test`.
- Gson chosen as JSON parsing library for Java tests — handles nested arrays (`int[][]` for video
    frame sigs) cleanly and is a well-known, lightweight test dependency.

## Registry README Patterns

- napi-rs `gen_*_v0` functions return `String` (not structured objects) — Node.js quick start
    examples must show string return, not `result.iscc` pattern.
- Python bindings return `dict` (via PyO3 `PyDict`) — quick start uses `result['iscc']`.
- Rust core returns typed `*CodeResult` structs with `.iscc` field.
- `crates/iscc-lib/Cargo.toml` currently has `readme = "../../README.md"` — must change to
    `"README.md"` when per-crate README is created.
- `crates/iscc-py/pyproject.toml` has no `readme` field — needs `readme = "README.md"` added.
- npm auto-detects `README.md` in the package directory — no `package.json` change needed.

## CI Workflow Patterns

- All CI jobs share a common preamble: `actions/checkout@v4` → `dtolnay/rust-toolchain@stable` →
    `Swatinem/rust-cache@v2`, then language-specific setup and build/test steps.
- Language-specific setup actions: `actions/setup-python@v5`, `actions/setup-node@v4`,
    `actions/setup-java@v4` (with `distribution: 'temurin'`).
- Never use `mise` in CI — call tools directly per learnings.
- Maven Surefire's `${project.basedir}` resolves to the pom.xml directory, so
    `${project.basedir}/../../../target/debug` reaches the workspace root's build output.

## Recurring Patterns

- All binding conformance tests follow the same structure: load data.json, iterate per-function test
    groups, decode inputs per function signature, compare `.iscc` output field. The Node.js test
    (`conformance.test.mjs`) is the cleanest template to mirror.
- `gen_iscc_code_v0` test vectors have no `wide` parameter in data.json — always pass `false` (the
    Python default).
- `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data for `gen_data_code_v0` and
    `gen_instance_code_v0`. Empty after prefix = empty bytes.

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
