## 2026-02-24 — Create Java wrapper class and Maven build config

**Done:** Created the Java-side source tree for JNI bindings: `IsccLib.java` with all 29 native
method declarations matching the Rust JNI bridge signatures, a minimal Maven `pom.xml` for
compilation (JDK 17, JUnit 5 test dep), and added `openjdk-17-jdk-headless maven` to the
devcontainer Dockerfile.

**Files changed:**

- `crates/iscc-jni/java/pom.xml`: Maven build config (groupId `io.iscc`, artifactId `iscc-lib`,
    version `0.0.1-SNAPSHOT`, JDK 17, JUnit 5, UTF-8, maven-compiler-plugin)
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: Java class with 29
    `public static native` methods organized into sections (conformance, gen functions, text
    utilities, encoding, codec, sliding window, algorithm primitives, streaming hashers). Each
    method has Javadoc with `@param`, `@return`, and `@throws` tags. Static initializer loads
    `iscc_jni` native library.
- `.devcontainer/Dockerfile`: Added `openjdk-17-jdk-headless maven` to the system packages
    `apt-get install` line.

**Verification:**

- [x] `sudo apt-get install -y openjdk-17-jdk-headless maven` succeeds
- [x] `mvn compile -f crates/iscc-jni/java/pom.xml` exits 0 (BUILD SUCCESS)
- [x] `grep -c 'native ' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` outputs
    `29`
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passes (no Rust changes, no
    regression)
- [x] `.devcontainer/Dockerfile` contains `openjdk-17-jdk-headless` and `maven`
- [x] `mise run check` — all 14 pre-commit hooks pass

**Next:** Java tests (`src/test/java/`) that load the native library and run conformance checks
against `data.json` vectors. This requires building the Rust cdylib first
(`cargo build -p iscc-jni`) and setting `java.library.path` to point to the build output.
Alternatively, define-next may prioritize the performance issues from issues.md or the low-priority
correctness guards.

**Notes:** The verification criterion `grep -c 'native '` required careful attention — the initial
class Javadoc contained "static native calls" which added a false match. Adjusted the comment
wording to "static JNI calls" to get the exact count of 29. Maven downloads all plugins on first run
(~80KB of output), but compile itself is fast. The `target/` directory from Maven build lands inside
the `crates/iscc-jni/java/` tree and should be gitignored if it becomes an issue (currently not
tracked by git since it's generated output).
