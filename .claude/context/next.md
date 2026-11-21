# Next Work Package

## Step: JNI loader tests pass in the release layout, and CI runs the suite in that layout

## Goal

Fix the `critical` issue "JNI loader tests fail in `release.yml` → `test-jni` and block the Maven
publish". This is a repair of iteration 278's tests (the loader itself stays as it is). It also adds
a CI step that runs the Java suite in the release layout, so the next test that needs the fallback
library cannot break `publish-maven` without anyone seeing it.

## Alternatives Considered

- **Chosen:** test repair + release-layout CI step — a `critical` issue takes priority, and without
    it the unpushed loader commit `8a99a3c` cannot ship
- **Rejected:** add a `cargo build`/copy to `target/debug` in `release.yml` → `test-jni` — no run
    can prove it (`workflow_dispatch`-only), and the tests would still depend on a dev-only path
- **Rejected:** docs batch (`architecture.md` + "10 `gen_`" leftovers) — `normal`; next after this

## Scope

- **Modify**: `.github/workflows/ci.yml` (job `java`: one more step)
- **Modify (tests, outside the budget)**:
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/LoaderProbe.java`
- **Reference**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`,
    `crates/iscc-jni/java/pom.xml` (`<argLine>` at :75), `.github/workflows/release.yml` →
    `test-jni` (:598-623), `docs/howto/java.md` (both-fail error description)

## Not In Scope

- `NativeLoader.java` and the docs: review found them correct; no rework
- The `low` temp-directory leak (`deleteOnExit` order) — human-directed only
- Editing `release.yml`; the `(dev/CI)` label in `specs/java-bindings.md`
- An `assumeTrue` skip when no fallback library is found — the release job would then test nothing

## Implementation Notes

**Fallback directory helper (`IsccLibTest`):** the child JVM's `-Djava.library.path` must hold a
real library in both layouts. Write one helper that returns that directory:

1. If some entry of the parent's `java.library.path` (split on `File.pathSeparator`) contains
    `System.mapLibraryName("iscc_jni")`, use that entry.
2. Otherwise read the bundled resource
    `"/META-INF/native/" + NativeLoader.detectOs() + "-" + NativeLoader.detectArch() + "/" + NativeLoader.libraryFileName(os)`
    through `NativeLoader.class.getResourceAsStream` in the parent (the parent classpath never
    holds the broken-library temp root, so this is the real library). Copy it into a fresh temp dir
    under the name `System.mapLibraryName("iscc_jni")` and return that dir. Delete the file and dir
    in a `finally` after the child exits.
3. Neither found → fail the test with a message naming both places. Never skip.

`assertLoaderProbePasses` uses that directory in place of the parent's `java.library.path`. Every
other test keeps its current child setup (`allocationFailureSurfacesAsOutOfMemoryError` passes in
the release layout today, because its child loads the bundled resource).

**Both-fail test (new):** garbage library on the classpath first (as in
`loaderFallsBackOnBrokenBundledLibrary`), `-Djava.library.path=` an empty temp dir. Extend
`LoaderProbe` so that on a throwable it also prints the cause class and each suppressed exception's
class, e.g. one extra line per item. The probe still prints `true` on success, so the three existing
tests stay unchanged. Assert: the exception is `UnsatisfiedLinkError`, the message names the
resource path (`META-INF/native/<os>-<arch>/…`), the cause is an `UnsatisfiedLinkError`, and there
is exactly one suppressed `UnsatisfiedLinkError`. Check this against what `docs/howto/java.md` says.
Factor the child-JVM launch so both helpers share it; don't copy the ProcessBuilder block.

**CI step (`ci.yml` → `java`, after "Run Maven tests"):** name it e.g. "Run Maven tests (release
layout: bundled library only)". Body: `mkdir -p`
`crates/iscc-jni/java/src/main/resources/META-INF/native/linux-x86_64`, copy
`target/debug/libiscc_jni.so` there, then `mv` the original out of `target/debug` (e.g. to
`$RUNNER_TEMP`), then `mvn test -f crates/iscc-jni/java/pom.xml`. `-DargLine` cannot remove the
pom's library path (learnings), so moving the file is the mechanism. The `.so` under `resources/` is
gitignored (`*.so`).

**Local proof of both layouts** (handoff recipe, about 40 s): `git archive HEAD` doesn't include
uncommitted edits, so commit first or copy the working-tree `crates/iscc-jni/java` into `$T`. Run
the CI step body as CI does (extract `run` with PyYAML, `bash --noprofile --norc -eo pipefail`) in a
throwaway copy, never in the dev tree, because it moves `target/debug/libiscc_jni.so`. After
probing, `rmdir /tmp/iscc-jni-[0-9]*` (known leak).

Binding-only Java + CI change: no CRAP, iai or core-API impact.

## Verification

- CI layout: `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` exits 0;
    surefire reports the three `loaderFallsBack*` tests and the both-fail test run, none skipped
- Release layout (handoff recipe on a tree with this step's test files, library only under
    `src/main/resources/META-INF/native/linux-x86_64/`, no `target/`) — `mvn test` exits 0 with the
    same four tests run, none skipped
- The both-fail test fails against a `LoaderProbe`/`NativeLoader` that drops `addSuppressed` (a
    throwaway-tree mutation, reported in the handoff)
- `ci.yml` job `java` has a step whose `run` copies the library into
    `src/main/resources/META-INF/native/linux-x86_64/`, moves `target/debug/libiscc_jni.so` away,
    and runs `mvn test`; that body exits 0 when run as CI runs it, in a throwaway copy
- `mise run check` exits 0

## Done When

All verification criteria pass: the loader tests run green in both layouts, and CI runs the release
layout on every push.
