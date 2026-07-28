---
name: uniffi-swift-kotlin
description: UniFFI scaffolding crate details plus Swift package and Kotlin/JVM binding structure, generate commands, and platform quirks
metadata:
  type: project
---

# UniFFI Bindings (Swift/Kotlin) — Full Detail

Detail moved out of MEMORY.md index. See also [[ci-gates]] for the swift/kotlin CI jobs.

## `crates/iscc-uniffi/` — shared scaffolding crate

- `uniffi = "0.32"` (workspace dep, since iter 172). Proc macro approach only: `#[uniffi::export]`,
    `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`, `#[uniffi::constructor]`. No UDL
    files, no build.rs. Uses `uniffi::setup_scaffolding!()`. `publish = false`.
- **Declares its own `rust-version = "1.91"`** (iter 173, deliberate — do NOT revert to
    `rust-version.workspace = true`): uniffi → cargo_metadata 0.23.1 → cargo-platform 0.3.3 imposes
    rustc 1.91 on the non-dev graph. Published `iscc-lib` MSRV stays 1.85 (decisions.md 2026-07-28).
- 0.32 CLI: `--library` is a deprecated-but-accepted boolean flag; the cdylib is the positional
    `source` arg (library mode auto-detected). Both CLAUDE.md recipes still parse verbatim.
- **Raw bindgen output is NOT hook-clean**: trailing whitespace on blank lines (all 3 files) +
    EOF-newline issues (`.swift` lacks one, `.kt`/`.h` carry extra blank lines). prek hygiene hooks
    normalize on `mise run format`/`check` — the regeneration-no-op check therefore holds only
    *modulo* trailing-ws/EOF normalization (compare via `sed 's/[[:space:]]*$//'`). May need TWO
    hook passes to reach a fixed point (ws-strip exposes a fresh EOF blank line). Never hand-edit
    generated files to satisfy hooks.
- Generated `.kt` triggers one benign Kotlin compile warning (`Expression is unused` at the
    `IntegrityCheckingUniffiLib` reference in `uniffiEnsureInitialized`) — uniffi-intended, ignore.
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
- **Swift IS locally testable on Linux** (proven iters 160-161): swift.org ships a Debian 12 x86_64
    tarball matching this container — recipe in `packages/swift/CLAUDE.md` Common Pitfalls (unpack
    to `/tmp/swifttc`, `cargo build -p iscc-uniffi`,
    `swift test --scratch-path   /tmp/swiftbuild -Xlinker -L<repo>/target/debug -Xlinker -rpath -Xlinker …`).
    Use `--scratch-path` outside the repo; `.build/` is gitignored since iter 161.
- Tests: `ConformanceTests.swift` (9 methods/50 vectors) + `UnicodeBoundaryTests.swift` (12 boundary
    vectors + metadata guard). **Never compare Swift `String ==` in Unicode tests** — it folds
    canonical equivalence ("e"+U+0301 == U+00E9), making sequence vectors vacuous; compare
    `unicodeScalars.map { $0.value }` arrays. Vendored `unicode_boundary.json` + `data.json` are SPM
    `.copy(...)` resources, registered in `tests/test_vendored_fixtures.py` (gate reads
    `git   ls-files`, so it stays red until the copy is `git add`ed).
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
