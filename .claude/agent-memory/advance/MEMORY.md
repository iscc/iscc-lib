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
- Python bindings return dict-like objects -- quick start uses `result['iscc']` (dict access) or
    `result.iscc` (attribute access via `__getattr__`)
- JNI error handling: all `unwrap()` calls replaced with `throw_and_default` pattern. Three forms:
    (1) nested match for `env.new_string().into_raw()`, (2) nested match for
    `env.byte_array_from_slice().into_raw()`, (3) early-return match + `if let Err` for loop bodies

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven runs from `crates/iscc-jni/java/` as working directory
- JDK 17 and Maven are pre-installed in devcontainer
- Gson 2.11.0 used as test-scope dependency for JSON parsing
- CI workflow at `.github/workflows/ci.yml` has 6 jobs: rust, python, nodejs, wasm, c-ffi, java
- Java CI uses `actions/setup-java@v4` with `distribution: temurin` + `java-version: '17'` (provides
    both JDK and Maven -- no separate Maven setup needed)
- All CI jobs share the same action triple: checkout@v4, rust-toolchain@stable, rust-cache@v2

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
