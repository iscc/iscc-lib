## 2026-02-24 — Create Java conformance tests

**Done:** Created JUnit 5 conformance tests exercising all 9 `gen_*_v0` JNI functions against the
official `data.json` test vectors (46 vectors total). Added Gson test dependency and
maven-surefire-plugin with `java.library.path` pointing to `target/debug/` in `pom.xml`.

**Files changed:**

- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: New file with 9
    `@TestFactory` methods producing `DynamicTest` collections (one per vector), plus helper methods
    for JSON parsing, stream hex decoding, meta argument preparation, and array type conversions
- `crates/iscc-jni/java/pom.xml`: Added `com.google.code.gson:gson:2.11.0` (test scope) and
    `maven-surefire-plugin:3.5.2` with `-Djava.library.path` configuration

**Verification:** `cargo build -p iscc-jni` exits 0. `mvn test -f crates/iscc-jni/java/pom.xml`
reports `Tests run: 46, Failures: 0, Errors: 0, Skipped: 0 — BUILD SUCCESS`. All 9 gen function
groups pass with individual test vectors visible in output (e.g., `test_0001_title_only`).
`cargo clippy --workspace --all-targets -- -D warnings` clean. `mise run check` passes all 14
pre-commit hooks.

**Next:** Java CI job in `.github/workflows/ci.yml` to run `cargo build -p iscc-jni` followed by
`mvn test -f crates/iscc-jni/java/pom.xml`. Alternatively, the native library loader class for
`META-INF/native/` extraction, or address performance issues from `issues.md`.

**Notes:** The test structure mirrors the Node.js conformance tests closely. Meta argument handling
sorts keys via `TreeMap` before serializing with Gson. The `description` field uses empty string →
null mapping (matching napi-rs pattern where empty description is treated as absent). HexFormat
(Java 17+) handles stream hex decoding cleanly. All 46 vectors = 16 meta + 5 text + 3 image + 5
audio + 3 video + 2 mixed + 4 data + 3 instance + 5 iscc.
