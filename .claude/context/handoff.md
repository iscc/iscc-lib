## 2026-02-24 — Add Java CI job

**Done:** Added a `java` job to `.github/workflows/ci.yml` that builds the JNI native library with
`cargo build -p iscc-jni` and runs Maven conformance tests with
`mvn test -f crates/iscc-jni/java/pom.xml`. The job follows the exact same pattern as the existing 5
CI jobs (checkout, Rust toolchain, Rust cache) plus `actions/setup-java@v4` with Temurin JDK 17.

**Files changed:**

- `.github/workflows/ci.yml`: Added `java` job (lines 101-115) with 6 steps: checkout,
    rust-toolchain, rust-cache, setup-java, cargo build, mvn test

**Verification:**

- [x] `cargo build -p iscc-jni` exits 0
- [x] `mvn test -f crates/iscc-jni/java/pom.xml` passes all 46 conformance tests (0 failures, BUILD
    SUCCESS)
- [x] YAML is valid (`python3 -c "import yaml; yaml.safe_load(...)"` exits 0)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` remains clean
- [x] `mise run check` — all 14 pre-commit hooks pass

**Next:** Address performance issues from issues.md (DataHasher buffer allocation, codec header
parsing), the native library loader class for JAR distribution, or Go bindings.

**Notes:** Straightforward single-file change. The `pom.xml` Surefire `argLine` with
`${project.basedir}/../../../target/debug` resolves correctly for both local and CI — no special
library path configuration needed in the CI job itself.
