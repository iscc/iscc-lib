## 2026-02-24 — Review of: Create Java wrapper class and Maven build config

**Verdict:** PASS

**Summary:** Clean implementation of the Java-side JNI interface. All 29 native method declarations
in `IsccLib.java` exactly match the Rust JNI bridge function names in `lib.rs`. Maven compiles
successfully with JDK 17, pom.xml follows the spec precisely, and Dockerfile additions are minimal
and correct. No issues found.

**Verification:**

- [x] `sudo apt-get install -y openjdk-17-jdk-headless maven` succeeds — already present in
    devcontainer
- [x] `mvn compile -f crates/iscc-jni/java/pom.xml` exits 0 — BUILD SUCCESS
- [x] `grep -c 'native ' IsccLib.java` outputs `29` — exact match
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passes — no regressions
- [x] `.devcontainer/Dockerfile` contains `openjdk-17-jdk-headless` and `maven` — confirmed
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** Java conformance tests (`src/test/java/io/iscc/iscc_lib/IsccLibTest.java`) that load the
native library via `cargo build -p iscc-jni` and run against `data.json` vectors. This requires
building the Rust cdylib first and setting `java.library.path` to the build output directory.
Alternatively, define-next may prioritize performance issues from issues.md or the low-priority
correctness guards.

**Notes:** The `IsccLib.java` class includes a private constructor and good Javadoc coverage. The
`native` keyword count verification is clean (no false positives from comments). Maven `target/`
directory is covered by the root `.gitignore` `target/` pattern. The pom.xml has JUnit 5 ready for
the test step.
