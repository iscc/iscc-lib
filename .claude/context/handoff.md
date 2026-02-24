## 2026-02-24 — Review of: Create Java conformance tests

**Verdict:** PASS

**Summary:** Clean, well-structured JUnit 5 conformance test suite covering all 9 `gen_*_v0` JNI
functions against 46 official test vectors. The implementation mirrors the Node.js conformance tests
closely, uses `@TestFactory` + `DynamicTest` for readable data-driven output, and the pom.xml
changes are minimal and correct. All tests pass, clippy is clean, all 14 pre-commit hooks pass.

**Verification:**

- [x] `cargo build -p iscc-jni` exits 0 — native library built successfully
- [x] `mvn test -f crates/iscc-jni/java/pom.xml` passes all conformance tests — 46 tests, 0
    failures, BUILD SUCCESS
- [x] Test output shows all 9 `gen_*_v0` function groups with individual test vectors passing
- [x] `cargo clippy --workspace --all-targets -- -D warnings` remains clean
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- Minor: unused `import com.google.gson.JsonNull` — fixed by review agent (removed the import;
    `meta.isJsonNull()` is a method on `JsonElement`, not on `JsonNull`)

**Next:** Java CI job in `.github/workflows/ci.yml` to run `cargo build -p iscc-jni` followed by
`mvn test -f crates/iscc-jni/java/pom.xml`. Alternatively, address performance issues from issues.md
(DataHasher buffer allocation, codec header parsing), or the native library loader class for JAR
distribution.

**Notes:** The Java conformance test structure is consistent with all other binding conformance
tests (Python, Node.js, WASM, C). The `description` empty→null mapping in meta tests matches the JNI
bridge behavior. All 46 vectors accounted for: 16 meta + 5 text + 3 image + 5 audio + 3 video + 2
mixed + 4 data + 3 instance + 5 iscc.
