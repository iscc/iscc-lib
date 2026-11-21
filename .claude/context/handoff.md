## 2026-11-21 — Review of: JNI `NativeLoader` reaches the `java.library.path` fallback on any platform

**Verdict:** NEEDS_WORK

**Summary:** The loader change is correct and well built: an unknown OS or architecture and a
bundled library that fails to load now reach `System.loadLibrary`, and the final error carries both
failures. But the three new tests fail in the layout that `release.yml` → `test-jni` uses, and
`publish-maven` needs that job, so the next release could not publish to Maven Central. Nothing was
pushed. The repair is in the tests only; `NativeLoader.java` and the docs need no rework.

**Verification:**

- [x] `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` exits 0, including
    the three child-JVM loader tests — IsccLibTest 113, DataEdgeTest 8, no failures or skips
- [x] `grep -n 'throw new UnsatisfiedLinkError("Unsupported' …/NativeLoader.java` prints nothing
- [x] Each of the three loader tests fails against the previous `NativeLoader.java` — rerun in a
    `git archive` tree of the define-next commit with the new test files:
    `Tests run: 3, Failures: 3` ("Unsupported OS: FreeBSD", "Unsupported architecture: riscv64",
    "file too short")
- [x] `git grep -nE 'other operating system or architecture fails|before it tries either strategy' -- docs crates/iscc-jni`
    prints nothing
- [x] `mise run check` exits 0
- [ ] Beyond next.md: `mvn test` in the release layout (library only bundled as a resource, no
    `target/debug`) — parent commit exit 0, HEAD `Tests run: 121, Failures: 3`
- [x] Beyond next.md: the error when both strategies fail matches `docs/howto/java.md` (message
    names the resource or the raw `os.name`/`os.arch`; cause is the `loadLibrary` error; the
    bundled-library error is suppressed). Checked by hand, no test pins it

**Issues found:**

- **Release regression (filed `critical`, replaces the resolved loader issue):** the three
    `loaderFallsBack*` tests pass the parent's `java.library.path` (`target/debug` from the pom) to
    the child JVM. `test-jni` runs `mvn test` with the libraries copied into
    `src/main/resources/META-INF/native/` and no `cargo build`, so the fallback path is empty there
- **Temp directory leak (filed `low`, untriaged, pre-existing):** `loadFromResource` registers the
    file for `deleteOnExit` before its directory, the JVM deletes in reverse order, so every JVM
    that loads the bundled library leaves an empty `iscc-jni-*` directory in the temp dir
- Not filed: the docs and Javadoc give "musl Linux" as the example of a bundled library that fails
    to load. Nothing measured it (no container runtime or musl loader in the devcontainer). The
    fallthrough itself is proven by the garbage-library test, so the sentence stays
- Not filed: no test pins the both-fail message or the suppressed exception; `LoaderProbe` prints
    only class and message
- Not filed: `specs/java-bindings.md` → "Native Library Loading" still labels strategy 2 "(dev/CI)";
    true but incomplete, a mechanical realignment the loop may make

**Codex review:** one finding (P1), confirmed by my own run and the reason for the verdict: the new
tests fail under the `test-jni` configuration and block `publish-maven`, while the parent commit
passes there.

**Next:** repair the three loader tests so they pass in both layouts, run and not skipped:

- **CI layout:** library on `java.library.path` (`target/debug`), no bundled resource
- **Release layout:** library bundled at `META-INF/native/<os>-<arch>/`, no `target/debug`

Suggested shape (tests only, `IsccLibTest.java`): give the child a fallback directory that holds a
real library in either layout. Use the parent's `java.library.path` when it contains
`System.mapLibraryName("iscc_jni")`; otherwise extract the bundled resource for the host from the
classpath into a temp directory and pass that. An `assumeTrue` skip when no library is found is not
a fix: the release job would then test nothing. The alternative, building or copying a library to
`target/debug` in `test-jni`, edits `release.yml` and no run can prove it.

Add the permanent check in the same step: a second step in the CI `java` job (`ci.yml`) that runs
the suite in the release layout (copy the built `.so` into
`src/main/resources/META-INF/native/linux-x86_64/`, move `target/debug/libiscc_jni.so` away; a
`-DargLine` override does not work, the pom's `<argLine>` wins). Without it the next Java test that
needs the fallback library breaks the release again unseen. A cheap extra in the same file: one
child-JVM test for the both-fail path (`-Djava.library.path=` an empty directory) that asserts the
message and, with a small `LoaderProbe` extension, the suppressed exception.

Local reproduction of the release layout (about 40 s):

```
T=$(mktemp -d); git archive HEAD crates/iscc-jni/java crates/iscc-lib/tests/data.json \
  crates/iscc-lib/tests/data-edge.json | tar -x -C $T
D=$T/crates/iscc-jni/java/src/main/resources/META-INF/native/linux-x86_64
mkdir -p $D && cp target/debug/libiscc_jni.so $D/
mvn -q test -f $T/crates/iscc-jni/java/pom.xml
```

After the repair, the docs-only batch is next: the `docs/architecture.md` sections plus the
`ecosystem.md` / `howto/nodejs.md` "10 standard functions" leftovers.

**Notes:**

- The advance commit `8a99a3c` is local only; the repair goes on top and the batch is pushed
    together
- The resolved issue "JNI `NativeLoader` rejects an unknown OS or architecture…" is deleted from
    issues.md; its remaining problem is the new `critical` entry
- Each bundled load leaves an empty `/tmp/iscc-jni-*` directory (the `low` issue): clean up with
    `rmdir /tmp/iscc-jni-[0-9]*` after probing
- Loading the garbage `.so` makes HotSpot print a "stack guard" warning on the child's stderr;
    harmless
- Binding-only Java change: no CRAP or iai impact, no core API change
