# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Implement Swift bindings via UniFFI `low` [human]

Add Swift bindings as a Swift Package in `packages/swift/` using UniFFI-generated code. Requires a
shared UniFFI scaffolding crate (`crates/iscc-uniffi/`) that also serves Kotlin bindings.

**Implementation scope:**

1. **UniFFI crate** (`crates/iscc-uniffi/`):

    - `Cargo.toml` (cdylib, depends on `iscc-lib` + `uniffi`)
    - `src/lib.rs` — UniFFI interface definition (proc macros or UDL) exposing all 32 Tier 1 symbols
    - `uniffi.toml` — binding generation config

2. **Swift package** (`packages/swift/`):

    - `Package.swift` — SPM manifest
    - Generated Swift bindings via `uniffi-bindgen`
    - Conformance tests via XCTest against `data.json`
    - `README.md` for the package

3. **CI** (`ci.yml`): Add `swift` job — `swift build` + `swift test` (may require macOS runner)

4. **Release**: Publish via Git tags (SPM resolves from Git repos, no upload registry)

5. **Documentation**: `docs/howto/swift.md` how-to guide, update README with Swift
    install/quickstart

## Implement Kotlin Multiplatform bindings via UniFFI `low` [human]

Add Kotlin Multiplatform (KMP) bindings in `packages/kotlin/` using UniFFI-generated code from the
shared `crates/iscc-uniffi/` crate. Published to Maven Central as `io.iscc:iscc-lib-kotlin`.

**Depends on:** Swift bindings (shares the UniFFI scaffolding crate)

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

## Add programming language logos to README and docs `normal` [human]

Add logos/icons for the supported programming languages (Rust, Python, etc.) to the README and
documentation pages where appropriate. Visual language indicators help users quickly identify
binding availability and make the project more approachable.
