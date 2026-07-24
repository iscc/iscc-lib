---
name: uniffi-swift-kotlin
description: UniFFI scaffolding crate details plus Swift package and Kotlin/JVM binding structure, generate commands, and platform quirks
metadata:
  type: project
---

# UniFFI Bindings (Swift/Kotlin) — Full Detail

Detail moved out of MEMORY.md index. See also [[ci-gates]] for the swift/kotlin CI jobs.

## `crates/iscc-uniffi/` — shared scaffolding crate

- `uniffi = "0.31"` (workspace dep). Proc macro approach only: `#[uniffi::export]`,
    `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`, `#[uniffi::constructor]`. No UDL
    files, no build.rs. Uses `uniffi::setup_scaffolding!()`. `publish = false`.
- 32 Tier 1 symbols, 11 result Records, `IsccUniError` enum (`#[derive(uniffi::Error)]` +
    `From<iscc_lib::IsccError>`), DataHasher/InstanceHasher Objects.
- `crate-type = ["cdylib", "staticlib", "lib"]` — cdylib for dynamic, staticlib for XCFramework.
- Streaming: `Mutex<Option<Inner>>` (like Ruby's `RefCell<Option<Inner>>` but thread-safe).
- UniFFI doesn't support: `const` exports (use getter fns), `usize` (use u64), borrowed refs (owned
    only).
- Result records need `Debug` derive for test `unwrap_err()`. Hashers need `Default` impl (clippy).
- 21 unit tests in-crate. Conformance testing happens in the Swift/Kotlin test suites.
- Binding generation: `uniffi-bindgen.rs` (3-line main), `[features] bindgen = ["uniffi/cli"]`,
    `[[bin]] required-features = ["bindgen"]`. Generate Swift via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate --library   target/debug/libiscc_uniffi.so --language swift --out-dir <dir>`
    → emits `iscc_uniffi.swift`, `iscc_uniffiFFI.h`, `iscc_uniffiFFI.modulemap` (rename to
    `module.modulemap` for SPM).

## Swift package

- Two `Package.swift` coexist: root (SPM consumers, reads for dep resolution) +
    `packages/swift/Package.swift` (CI/local dev). Root uses Ferrostar toggle `useLocalFramework` +
    `releaseTag`/`releaseChecksum`, `binaryTarget` for distribution; omits testTarget.
- `scripts/build_xcframework.sh`: 5 Rust targets → `lipo` → `xcodebuild -create-xcframework` →
    `ditto` zip → checksum. Output `target/ios/IsccLib.xcframework.zip` (`--release`/`--debug`).
- Version constant: `packages/swift/Sources/IsccLib/Constants.swift` (`isccLibVersion`). CI job
    (`swift:`, `macos-14`): `cargo build -p iscc-uniffi` → `swift build` → `swift test` with
    `-Xlinker -L`/`-rpath` → `target/debug`.

## Kotlin bindings (UniFFI/JVM)

- `packages/kotlin/` — Gradle JVM project, UniFFI-generated Kotlin via JNA (mature/complete).
- Generated `src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` — do NOT hand-edit; regenerate via
    uniffi-bindgen.
- JNA native loading needs `jna.library.path` + `LD_LIBRARY_PATH`. `ConformanceTest.kt` = 9
    methods/50 vectors.
- Full generate command, Gradle/JNA versions, gitignore quirks, Maven Central publishing →
    MEMORY-archive.md.
