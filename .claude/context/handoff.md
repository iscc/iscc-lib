## 2026-11-21 — Make the published-package docs match what 0.7.0 ships

**Done:** Fixed every install, distribution, return-shape and gate claim that the issue
"Published-package docs contradict what ships" names, in the 9 scoped Markdown files. Java and
Kotlin now say the JARs bundle and load the native library. Swift says SwiftPM fetches the released
XCFramework. The JS quick starts print the returned string (WASM with `await init()`). The
architecture page lists every binding crate and package, and the pytest hook is no longer described
as enforcing coverage.

**Files changed:**

- `README.md`: Java note says the JAR bundles and loads the native lib. Swift note says the
    XCFramework comes from the GitHub Release and no Rust is needed. Kotlin snippet drops the
    explicit JNA line and the library-path sentence. The Node.js and WASM snippets print `iscc` (the
    string return), and WASM uses `import init, {…}` + `await init();`
- `docs/howto/java.md`: rewrote the intro and Setup to describe the two `NativeLoader` strategies
    (bundled `META-INF/native/{os}-{arch}/`, then `System.loadLibrary` on `java.library.path`).
    "Build from source" is now an optional collapsed `??? tip`
- `docs/howto/kotlin.md`: dropped the JNA line, the library-path sentence and the "Not yet published
    to Maven Central" admonition. Build from source stays as an optional collapsed tip
- `crates/iscc-jni/README.md`: replaced "automatic native loader is not yet included" with the
    bundled platforms, plus `java.library.path` as the fallback for other platforms
- `packages/kotlin/README.md`: dropped the JNA line and the library-path sentence. "Building from
    Source" is marked optional
- `packages/swift/README.md`: replaced the "must be built from source / planned for a future
    release" note. The Rust toolchain is now listed as needed only to build the library yourself
- `docs/architecture.md`: the mermaid graph adds `iscc-rb`, `iscc-uniffi`, .NET and C++ (→ FFI), and
    Swift and Kotlin (→ UniFFI). The prose names the crates with no count. The Node.js async row now
    reads as sync exports with `DataHasher`/`InstanceHasher` classes
- `docs/development.md`, `CLAUDE.md`: the pytest pre-push entry no longer claims coverage
    enforcement

**Verification:**

- Verification grep 1
    (`not yet …|planned for a future release|must be built from source|AsyncTask|five binding crates|coverage enforcement`
    over the 9 files) printed nothing (exit 1)
- `grep -n "result.iscc" README.md`: lines 172/229/249/260/274 only (Rust, Ruby, C++, Swift, Kotlin
    blocks), none from JS. `await init();` is at README:284
- `uv run zensical build`: exit 0. `gen_llms_full.py` re-run afterwards: 22 pages
- `uv run pytest tests/`: 769 passed
- `mise run check`: exit 0. `version_sync.py --check`: exit 0, so removing the JNA line kept the
    Gradle snippets' version targets intact
- `git status --short`: only `.md` files changed (plus the runner's `iterations.jsonl`)
- **Published-artifact probes:**
    - The Kotlin POM (`iscc-lib-kotlin-0.7.0.pom`) declares `jna 5.19.1` with `<scope>runtime</scope>`
    - A throwaway Gradle project in `/tmp/kprobe` uses only
        `implementation("io.iscc:iscc-lib-kotlin:0.7.0")` from Maven Central: no JNA declaration, no
        library path. It compiled and ran `genMetaCodeV0` and `DataHasher`, printing
        `ISCC:AAAT4EBWK27737D2` / `ISCC:GAAXSGGUNFQ4GKBE`
    - The JAR lists `android-*`, `darwin-*`, `linux-*` and `win32-x86-64` libs
    - `gh release view v0.7.0` lists `IsccLib.xcframework.zip`
    - napi `gen_meta_code_v0` returns `napi::Result<String>` via `.map(|r| r.iscc)`, and
        `AsyncTask|Promise|spawn` has 0 matches in napi `lib.rs`

**Next:** No `normal` issues remain. The open items are the `low` doc issues (Sum-Code `KAA`
comments, pkg-config `0.2.0`, the `ci-cd.md` tag trigger, the `target.md` count), which the human
directs, and the human-gated v1.0.0, MSRV and EOL work.

**Notes:**

- I removed the explicit JNA line from the Kotlin snippets only after the runtime-scope POM check
    and the compile+run probe. Kotlin's public API exposes no JNA supertypes: the hashers implement
    `Disposable`/`AutoCloseable`.
- The Kotlin docs say "Linux, macOS and Windows" even though the JAR also carries Android libs.
    Loading via JNA on Android (which needs the JNA AAR) is unverified, so I made no claim about it.
- Only Linux x86_64 was probed at runtime (Kotlin). The Java platform list comes from next.md's JAR
    listing and `crates/iscc-jni/CLAUDE.md`.
- `docs/architecture.md`'s "Per-Binding Adaptation" table does not claim to be complete, so I fixed
    the Node.js row only and added no rows for Ruby, .NET, C++, Swift or Kotlin, per next.md.
- No code was touched, so there is no CRAP, iai or semver impact.
