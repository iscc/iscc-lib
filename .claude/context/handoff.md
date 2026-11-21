## 2026-11-21 — JNI loader tests pass in the release layout, and CI runs the suite in that layout

**Done:** The child-JVM loader tests now get a fallback directory that holds a real library in
either layout: the `java.library.path` entry that holds it (CI), or else a temp copy of the library
bundled for the host as a resource (release). Neither found fails the test. Added the both-fail test
(`loaderReportsBothFailures`), extended `LoaderProbe` to print the cause and suppressed classes, and
added a release-layout `mvn test` step to the CI `java` job.

**Files changed:**

- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: shared launcher
    `runLoaderProbe(classpath, property, libraryDir)` → `ProbeRun` record (exit, stdout, stderr);
    `libraryPathEntryWithLibrary()`, `extractBundledLibrary()`, `deleteTree()`,
    `writeBrokenBundledLibrary()` helpers; `assertLoaderProbePasses` uses the fallback directory and
    deletes the extracted copy in `finally`; test `loaderReportsBothFailures`. The three
    `loaderFallsBack*` tests keep their bodies (broken-library test now uses the shared helpers).
    `allocationFailureSurfacesAsOutOfMemoryError` unchanged
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/LoaderProbe.java`: after a throwable, prints
    `cause: <class>` (if any) and one `suppressed: <class>` line per suppressed exception
- `.github/workflows/ci.yml`: job `java` step "Run Maven tests (release layout: bundled library
    only)" — `mkdir -p …/META-INF/native/linux-x86_64`, `cp target/debug/libiscc_jni.so` there, `mv`
    the original to `$RUNNER_TEMP`, `mvn test -f crates/iscc-jni/java/pom.xml`

**Verification:**

- CI layout: `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` → exit 0;
    surefire: IsccLibTest tests=114 skipped=0 failures=0, DataEdgeTest 8/0/0; testcases
    `loaderFallsBackOnBrokenBundledLibrary`, `…UnknownOperatingSystem`, `…UnknownArchitecture`,
    `loaderReportsBothFailures` all present
- Release layout + CI step body: throwaway tree (`git ls-files crates/iscc-jni/java …data*.json` via
    tar, so it carries the uncommitted edits; `diff` confirmed the test file matches) +
    `target/debug/libiscc_jni.so`; step `run` extracted with PyYAML, run as
    `RUNNER_TEMP=$T/rt bash --noprofile --norc -eo pipefail step.sh` → exit 0,
    `Tests run: 122, Failures: 0, Errors: 0, Skipped: 0`; afterwards `target/debug` empty, `.so` in
    `$RUNNER_TEMP`; same four loader testcases in surefire, skipped=0
- Mutation: in that tree, `error.addSuppressed(bundledError);` commented out in `NativeLoader.java`,
    `mvn test -Dtest='IsccLibTest#loader*'` → exit 1, `Tests run: 4, Failures: 1`,
    `loaderReportsBothFailures … expected: <3> but was: <2>` (stdout lines)
- Both-fail assertions match `docs/howto/java.md`: `UnsatisfiedLinkError`, message contains
    `'/META-INF/native/<os>-<arch>/<lib>'`, `cause: java.lang.UnsatisfiedLinkError`, exactly one
    `suppressed: java.lang.UnsatisfiedLinkError`
- `mise run check` → exit 0
- Cleanup: probe trees removed, `rmdir /tmp/iscc-jni-[0-9]*` run; no `iscc-fallback-native*`,
    `iscc-broken-native*` or `iscc-empty-libpath*` dirs left in `/tmp`

**Next:** push the batch (`8a99a3c` + this commit) and confirm both `java` steps green in CI. Then
the docs-only batch: `docs/architecture.md` sections plus the `ecosystem.md` / `howto/nodejs.md` "10
standard functions" leftovers.

**Notes:**

- The CI step body is proven locally only; the first CI run on push is its real proof
- `Swatinem/rust-cache` saves `target/` after the job, now without `target/debug/libiscc_jni.so`;
    cargo relinks a missing output on the next run, so the cache stays usable (not run in CI yet)
- `loaderReportsBothFailures` uses `writeBrokenBundledLibrary`'s `assumeTrue` (host without a
    bundled-library slot) like the existing broken-library test; it never skips on linux-x86_64
- Binding-only Java + CI change: no CRAP, iai or core-API impact. `ci-cd.md` job table unaffected
    (no job added or renamed)
