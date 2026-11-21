# Next Work Package

## Step: Make the published-package docs match what 0.7.0 ships

## Goal

Close the `normal` `[audit]` issue "Published-package docs contradict what ships", the last open
v0.8.0 blocker. Every install, distribution, return-shape and gate claim in the files below must
describe the artifact as it is published and the code as it runs.

## Alternatives Considered

- **Chosen:** the whole issue in one step. Every file is documentation, and the default rule does
    not count docs against the 6-file cap. The edits are small text corrections, so splitting the
    issue across two iterations would only double its cost
- **Rejected:** a `low` doc issue such as the Sum-Code `KAA` comments or the pkg-config `0.2.0` line
    — `low` items are for the human to direct. No other unmet target criterion is reachable by CID:
    v1.0.0, MSRV and the EOL floors are human-gated

## Scope

- **Modify** (all docs, 9 files): `README.md`, `docs/howto/java.md`, `docs/howto/kotlin.md`,
    `docs/architecture.md`, `docs/development.md`, `crates/iscc-jni/README.md`,
    `packages/kotlin/README.md`, `packages/swift/README.md`, `CLAUDE.md` (root)
- **Reference**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`,
    `Package.swift` (root), `crates/iscc-napi/src/lib.rs`, `crates/iscc-wasm/src/lib.rs`,
    `crates/iscc-wasm/README.md` + `docs/howto/wasm.md` (the `await init()` pattern that is already
    correct), `.github/workflows/release.yml:427`, `.pre-commit-config.yaml` (pytest hook)

## Not In Scope

- The `low` doc issues: Sum-Code `KAA` comments, pkg-config `Version: 0.2.0`, the `ci-cd.md` tag
    trigger, the `target.md` count. Leave them for the human
- `notes/` (historical, never swept) and the per-crate `CLAUDE.md` files. Their `java.library.path`
    lines describe the dev/CI fallback and are correct
- Adding pytest coverage enforcement. Fix the claim, not the gate
- Any code, workflow or version change

## Implementation Notes

Facts measured at define time on the published 0.7.0 artifacts:

- `repo1.maven.org/.../io/iscc/iscc-lib/0.7.0/iscc-lib-0.7.0.jar` contains
    `META-INF/native/{linux-x86_64,linux-aarch64,macos-aarch64,macos-x86_64,windows-x86_64}/` libs.
    `IsccLib` calls `NativeLoader.load()` in its static initializer. It extracts the bundled lib and
    falls back to `System.loadLibrary`. So Java needs only the Maven dependency. `java.library.path`
    is an optional override or dev fallback, not a requirement
- `iscc-lib-kotlin-0.7.0.jar` bundles `libiscc_uniffi` under JNA resource prefixes (`android-*`,
    `darwin-*`, `linux-*`, `win32-x86-64`). Its POM declares `net.java.dev.jna:jna:5.19.1`. Read
    that dependency's `<scope>` in the POM before you decide whether docs still tell users to add
    JNA explicitly. Kotlin is on Maven Central, so drop the "Not yet published" note. Keep "Build
    from source" as an optional collapsed tip
- The GitHub release v0.7.0 has `IsccLib.xcframework.zip`, and root `Package.swift` fetches it by
    default (`useLocalFramework = false`). So the Swift notes in `README.md:143` and
    `packages/swift/README.md:19-21` are wrong. SPM needs no Rust toolchain, and building from
    source is optional
- napi and WASM `gen_meta_code_v0` return a **string** (`.map(|r| r.iscc)`), so `README.md:190` and
    `:285` must print `result` itself (or rename it `iscc`). Check every other JS snippet in
    `README.md` for the same pattern. `@iscc/wasm` is built `--target web`, so the README WASM
    snippet needs `import init, {...}` + `await init();`. Mirror `crates/iscc-wasm/README.md`
- `docs/architecture.md`: the mermaid graph and the "five binding crates" sentence (line 30) must
    list every binding crate (`iscc-rb`, `iscc-uniffi` too). Mention that the .NET and C++ packages
    wrap the C FFI, and that Swift and Kotlin wrap `iscc-uniffi`. Do not write a count you would
    have to keep current. Line 178: napi has **no** `AsyncTask` (0 matches in `lib.rs`), and its
    exports are sync. Describe the real strategy, and add rows for the missing bindings only if the
    table claims to be complete
- `docs/development.md:181` and `CLAUDE.md:207`: the pytest pre-push hook is
    `uv run pytest --timeout=120` with no `--cov`, so drop "coverage enforcement"
- Do not hand-type a version anywhere. `version_sync.py` only keeps the Maven, Gradle and SPM
    snippets current (learnings, 244)
- Use the doc-example casing rules: JNI `genMetaCodeV0` returns a `String`, and Kotlin and Swift
    take named args. Run `uv run zensical build`, then `gen_llms_full.py` (zensical wipes `site/`)

## Verification

- `grep -rnE "not yet (available|included|published)|planned for a future release|must be built from source|AsyncTask|five binding crates|coverage enforcement" README.md docs/howto/java.md docs/howto/kotlin.md docs/architecture.md docs/development.md crates/iscc-jni/README.md packages/kotlin/README.md packages/swift/README.md CLAUDE.md`
    prints nothing
- `grep -n "result.iscc" README.md` prints no line from a JavaScript (Node.js or WASM) code block,
    and the README WASM block contains `await init()`
- `uv run zensical build` exits 0
- `uv run pytest tests/` passes, including the docs gates (`test_check_docs_nav`, Tier 1 docs,
    `test_rust_api_signatures`)
- `mise run check` exits 0
- `git diff --name-only HEAD` (plus any untracked files) lists only `.md` files

## Done When

All six verification criteria pass and each claim named in the issue matches the published artifact
or the code.
