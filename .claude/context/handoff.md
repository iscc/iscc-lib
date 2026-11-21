## 2026-11-21 — JNI `NativeLoader` reaches the `java.library.path` fallback on any platform

**Done:** `detectOs()`/`detectArch()` return `null` for an unrecognized value instead of throwing;
`load()` skips strategy 1 then and goes to `System.loadLibrary`. Strategy 1 also falls through when
the extracted library fails to load (`UnsatisfiedLinkError`, not only `IOException`). If strategy 2
fails too, the final error has the strategy-2 error as cause and the bundled-library error as
suppressed. Three child-JVM loader tests (`LoaderProbe`) cover unknown arch, unknown OS and a broken
bundled library.

**Files changed:**

- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`: null-returning detection,
    `UnsatisfiedLinkError` fallthrough + `addSuppressed`/`initCause`, `failureMessage(resourcePath)`
    helper (raw `os.name`/`os.arch`; says "No bundled library exists for …" when strategy 1 was
    skipped), Javadoc updated. `libraryFileName` default now throws `IllegalArgumentException`
    (unreachable: only called with a non-null `detectOs()` result, after the null check)
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/LoaderProbe.java` (new): child-JVM `main`
    printing `conformanceSelftest()` or `Class: message`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: helper
    `assertLoaderProbePasses(classpath, property)` (asserts exit 0 and stdout `true`, stderr in the
    message) + `loaderFallsBackOnUnknownArchitecture` (`-Dos.arch=riscv64`),
    `loaderFallsBackOnUnknownOperatingSystem` (`-Dos.name=FreeBSD`),
    `loaderFallsBackOnBrokenBundledLibrary` (garbage at `META-INF/native/<os>-<arch>/<lib>` in a
    temp dir prepended to the child classpath; `assumeTrue` only for an unknown host)
- `docs/howto/java.md` (Setup), `crates/iscc-jni/README.md`, `crates/iscc-jni/CLAUDE.md` (Native
    Library Loading): describe the fallback for unknown platforms and unloadable bundled libraries

**Verification:**

- `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` → exit 0; surefire:
    IsccLibTest 113 run, 0 failures/errors/skips (the three `loader*` tests included), DataEdgeTest
    8
- `grep -n 'throw new UnsatisfiedLinkError("Unsupported' …/NativeLoader.java` → no output
- Against HEAD: `git archive HEAD` into a `mktemp -d` tree, symlinked `target` to the workspace
    `target/` (pom's `java.library.path`), copied in only the new `IsccLibTest.java` +
    `LoaderProbe.java`, `mvn -q test -Dtest='IsccLibTest#loader*'` → `Tests run: 3, Failures: 3`:
    - BrokenBundledLibrary:
        `but was: <java.lang.UnsatisfiedLinkError: /tmp/iscc-jni-…/libiscc_jni.so: … file too short>`
    - UnknownOperatingSystem: `but was: <java.lang.UnsatisfiedLinkError: Unsupported OS: FreeBSD>`
    - UnknownArchitecture:
        `but was: <java.lang.UnsatisfiedLinkError: Unsupported architecture: riscv64>` Throwaway tree
        removed afterwards.
- Both-fail message, checked by hand (`-Djava.library.path=/nonexistent`): riscv64 →
    `… No bundled library exists for os.name=Linux, os.arch=riscv64; tried System.loadLibrary("iscc_jni"). …`;
    broken bundled lib → `Tried JAR resource '/META-INF/native/linux-x86_64/libiscc_jni.so' …`,
    `Suppressed: … file too short`, `Caused by: … no iscc_jni in java.library.path: /nonexistent`
- `git grep -nE 'other operating system or architecture fails|before it tries either strategy' -- docs crates/iscc-jni`
    → no output
- `mise run check` → exit 0

**Next:** the docs-only batch: the `docs/architecture.md` entries plus the `ecosystem.md` /
`howto/nodejs.md` "10 standard functions" leftovers (scope with `git grep -nE '10 .gen_' -- docs`).

**Notes:**

- Loading the garbage `.so` makes HotSpot print a "might have disabled stack guard" warning on the
    child's stderr; harmless, the test checks only stdout and the exit code
- `.claude/context/specs/java-bindings.md` → "Native Library Loading" was not in scope and states no
    throw-before-fallback, so it still holds; it does not mention the bundled-load fallthrough
- Binding-only Java change: no CRAP/iai impact
