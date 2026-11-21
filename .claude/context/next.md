# Next Work Package

## Step: JNI `NativeLoader` reaches the `java.library.path` fallback on any platform

## Goal

Close the `normal` issue "JNI `NativeLoader` rejects an unknown OS or architecture before the
`java.library.path` fallback": an unrecognized OS or architecture, and a bundled library that fails
to load (musl Linux, broken binary), must fall through to `System.loadLibrary("iscc_jni")` instead
of throwing first.

## Alternatives Considered

- **Chosen:** JNI loader fallback — a user-visible code defect on a published artifact (FreeBSD,
    riscv64 and likely musl users cannot supply their own build); the handoff's first pick, and the
    last iteration was docs-only.
- **Rejected:** docs-only batch (`architecture.md` sections + `ecosystem.md` / `nodejs.md` counts) —
    cheap and independent, good next step; no code risk to defer it by one iteration.
- Also open: `@iscc/wasm` Node docs (needs a release-build probe), one-shot dedup (iai gate),
    `alg_minhash_256([])` (needs a decision first).

## Scope

- **Modify**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`
- **Modify** (tests): `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Create** (tests): `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/LoaderProbe.java`
- **Modify** (docs): `docs/howto/java.md` (→ "Setup", the sentence on unsupported OS/arch),
    `crates/iscc-jni/README.md` (:26-30), `crates/iscc-jni/CLAUDE.md` (→ "Native Library Loading")
- **Reference**: `OomProbe.java` + `IsccLibTest.allocationFailureSurfacesAsOutOfMemoryError`
    (child-JVM pattern), `.claude/context/specs/java-bindings.md` → "Native Library Loading"

## Not In Scope

- Adding bundled libraries for more platforms (musl, riscv64, Windows aarch64) or changing
    `release.yml`
- Renaming resource directories or changing the resource path layout (the spec's table stays)
- Any change to `IsccLib.java`, the Rust JNI crate, or the Kotlin/JNA loader
- A `-Xcheck:jni` sweep or other unrelated JNI test cleanup

## Implementation Notes

- `detectOs()` / `detectArch()` must no longer throw: return `null` (or `Optional`) for an unknown
    value. `load()` skips strategy 1 when either is unknown and goes straight to
    `System.loadLibrary`. `libraryFileName`'s `default` throw becomes unreachable — remove or keep
    it consistent; no throw may precede strategy 2.
- Strategy 1 must also fall through when `System.load` of the extracted file throws
    `UnsatisfiedLinkError` (the musl/glibc case — today only `IOException` falls through). Keep that
    first error: attach it via `addSuppressed` to the final `UnsatisfiedLinkError` if strategy 2
    also fails, so a user sees why the bundled binary was rejected.
- Final error message: when OS/arch is unknown, print the raw `os.name` / `os.arch` property values
    and say no bundled library exists for them, instead of a resource path that was never tried.
- Update the class/method Javadoc so it describes the new behaviour (evergreen wording).
- `LoaderProbe` (test source, modelled on `OomProbe`): `main` calls `IsccLib.conformanceSelftest()`
    and prints `true`/`false` or the thrown class name + message. Tests in `IsccLibTest` run it in a
    child JVM with `-Djava.library.path=` + `System.getProperty("java.library.path")` and
    `-cp System.getProperty("java.class.path")`:
    1. `-Dos.arch=riscv64` → stdout `true`
    2. `-Dos.name=FreeBSD` → stdout `true` (the issue measured that both overrides reach
        `NativeLoader` on HotSpot 17)
    3. broken bundled library: write garbage bytes to a temp dir at
        `META-INF/native/<detectOs()>-<detectArch()>/<libraryFileName(os)>` (call the package-private
        helpers from the test; skip with `assumeTrue` only if the host is unknown), PREPEND that dir
        to the child classpath → stdout `true`. Against today's code this child throws. One child per
        case; capture stderr into the assertion message like the OOM test.
- Docs: state that an unrecognized platform, or a bundled library that fails to load, falls back to
    `java.library.path`; drop "Any other operating system or architecture fails". Keep the README's
    list of bundled platforms.
- Binding-only change: trips neither the CRAP nor the iai gate.
- Rebuild the debug JNI library before `mvn test` (`cargo build -p iscc-jni`): gitignored artifacts
    go stale silently.

## Verification

- `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` exits 0, including the
    three child-JVM loader tests
- `grep -n 'throw new UnsatisfiedLinkError("Unsupported' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`
    prints nothing
- Each of the three loader tests fails against `git show HEAD:<NativeLoader.java path>` (advance
    reports the mechanism, e.g. a throwaway `git archive` tree, and the failing output in the
    handoff) — never via `git stash`/`reset` in the working tree
- `git grep -nE 'other operating system or architecture fails|before it tries either strategy' -- docs crates/iscc-jni`
    prints nothing
- `mise run check` exits 0

## Done When

All verification criteria pass and the docs describe the fallback for unrecognized platforms and
unloadable bundled libraries.
