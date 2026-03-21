# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Implement Kotlin Multiplatform bindings via UniFFI `normal` [human]

Add Kotlin Multiplatform (KMP) bindings in `packages/kotlin/` using UniFFI-generated code from the
shared `crates/iscc-uniffi/` crate. Published to Maven Central as `io.iscc:iscc-lib-kotlin`.

**Depends on:** ~~Swift bindings (shares the UniFFI scaffolding crate)~~ Resolved — UniFFI crate
complete.

**Implementation scope:**

1. **Kotlin package** (`packages/kotlin/`):

    - `build.gradle.kts` — KMP project targeting JVM + iOS + macOS
    - Generated Kotlin bindings via `uniffi-bindgen`
    - Platform-specific native libraries per target
    - Conformance tests via kotlin.test against `data.json`
    - `README.md` for the package

2. **CI** (`ci.yml`): Add `kotlin` job — Gradle build + test

3. **Release** (`release.yml`):

    - Add `maven-kotlin` boolean input to `workflow_dispatch`
    - Publish to Maven Central as `io.iscc:iscc-lib-kotlin`
    - GPG signing + Sonatype credentials (same as Java/JNI)

4. **Version sync**: Add Kotlin project version to sync targets

5. **Documentation**: `docs/howto/kotlin.md` how-to guide, update README with Kotlin
    install/quickstart

## Swift SPM install instructions are incorrect `normal` [human]

README.md (line 135) and packages/swift/README.md (line 9) advertise a SwiftPM dependency URL
`.package(url: "https://github.com/iscc/iscc-lib", from: "0.3.0")`, but `Package.swift` lives in
`packages/swift/`, not the repo root. SwiftPM always resolves from the repo root, so this URL will
not resolve. The version `0.3.0` also predates the Swift package (added in 0.3.1).

**Fix options:**

1. Add a root-level `Package.swift` that re-exports the subdirectory package
2. Publish a separate `iscc/iscc-swift` repo with just the Swift package
3. Rewrite the install docs to document a build-from-source workflow (clone, cargo build, swift
    build with `-Xlinker` flags)

Option 3 is honest but has worse DX. Option 1 or 2 would give users a proper SPM one-liner.

## Swift package does not vend the native library `normal` [human]

The Swift package declares `.linkedLibrary("iscc_uniffi")` but does not include or build the native
dylib. CI works around this with manual `cargo build -p iscc-uniffi` + `-Xlinker -L` flags.
Downstream users following the README will get link failures.

**Fix options:**

1. Add a `.binaryTarget` with prebuilt XCFrameworks (uploaded as release artifacts)
2. Add a SwiftPM build plugin that invokes `cargo build` automatically
3. Document the build-from-source requirement clearly in the install instructions

Option 1 is the standard approach for native Swift packages. Requires CI to build and upload
XCFrameworks for macOS (arm64, x86_64) and optionally iOS.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
