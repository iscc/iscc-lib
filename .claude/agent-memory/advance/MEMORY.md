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

## Implementation Patterns

- JUnit 5 conformance tests use `@TestFactory` + `DynamicTest` for data-driven tests from JSON
- Gson `JsonObject`/`JsonArray` for traversing data.json (test scope dependency)
- Meta argument prep: JsonObject → TreeMap sort → Gson serialize; JsonNull → null; string → as-is
- Stream hex decoding: `HexFormat.of().parseHex()` (Java 17+) after stripping `"stream:"` prefix
- Empty description in meta tests maps to null (not empty string) to match JNI bridge behavior
- `gen_iscc_code_v0` test vectors have no `wide` field — always pass `false`
- Maven Surefire plugin `-Djava.library.path` points to `${project.basedir}/../../../target/debug`
- data.json relative path from Maven test CWD: `../../iscc-lib/tests/data.json`

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven runs from `crates/iscc-jni/java/` as working directory
- JDK 17 and Maven are pre-installed in devcontainer
- Gson 2.11.0 used as test-scope dependency for JSON parsing
- CI workflow at `.github/workflows/ci.yml` has 6 jobs: rust, python, nodejs, wasm, c-ffi, java
- Java CI uses `actions/setup-java@v4` with `distribution: temurin` + `java-version: '17'` (provides
    both JDK and Maven — no separate Maven setup needed)
- All CI jobs share the same action triple: checkout@v4, rust-toolchain@stable, rust-cache@v2

## Gotchas

- JNI package underscore encoding: `iscc_lib` → `iscc_1lib` in function names
- Java `byte` is signed — casting int (0-255) to byte works correctly for pixel data
- HexFormat requires Java 17+ (already set as Maven compiler target)
