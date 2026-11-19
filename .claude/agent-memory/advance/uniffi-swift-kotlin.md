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
- 33 Tier 1 symbols (33rd = `gen_iscc_id_v1`+`IsccIdResult`, iter 186), 12 result Records,
    `IsccUniError` enum (`#[derive(uniffi::Error)]` + `From<iscc_lib::IsccError>`),
    DataHasher/InstanceHasher Objects. `gen_iscc_id_v1(u64,u16,u8)` passes exact-width types
    straight to core — NO binding guard/narrowing (core validates ts→hub→realm). Adding it emitted a
    fresh `FfiConverterUInt16` (first u16 param) in the swift binding.
- **Swift bindgen dumps `.h`+`.modulemap` into the `--out-dir` (`Sources/IsccLib/`) but the
    checked-in tree keeps `iscc_uniffiFFI.h` in `Sources/iscc_uniffiFFI/` and a hand-simplified
    `module.modulemap` there.** After `generate`: `mv` the `.h` into `Sources/iscc_uniffiFFI/`, `rm`
    the generated `iscc_uniffiFFI.modulemap` (never overwrite `module.modulemap`).
- `crate-type = ["cdylib", "staticlib", "lib"]` — cdylib for dynamic, staticlib for XCFramework.
- Streaming: `Mutex<Option<Inner>>` (like Ruby's `RefCell<Option<Inner>>` but thread-safe).
- UniFFI doesn't support: `const` exports (use getter fns), `usize` (use u64), borrowed refs (owned
    only).
- Result records need `Debug` derive for test `unwrap_err()`. Hashers need `Default` impl (clippy).
- 28 unit tests in-crate (iter 210). Conformance testing happens in the Swift/Kotlin test suites.
- ISCC-SEQ (iter 210): `encode_seq(Vec<String>)->Vec<u8>` / `decode_seq(Vec<u8>)->Vec<String>`, no
    binding guards. UniFFI strings are length-delimited — embedded NUL reaches core (rejected).
    Harness helpers: Swift `decodeBytes`/`hexToData`/`assertIsccError`, Kotlin `decodeBytes`/
    `assertIsccError` (seq vectors use `bytes:`, outputs string/array, not object).
- **`mise run format` can fail mid-run with `os error 22` writing the 80 KB generated `.swift`**
    (WSL bind-mount flake) and leave trailing ws: `grep -c '[[:space:]]$'` the generated files,
    re-run the hook on the file + another format pass.
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
- Swift harness rule (218): convert record fields to `Int(...)` before boxing in `Any` and compare
    via typed casts, so the same code holds on macOS CI and Linux (unprobed cross-platform).
    `swift test --package-path   packages/swift` with absolute `$PWD/target/debug` linker paths
    works from the repo root.
- Tests: `DataEdgeTests.swift` (6, iter 218), `ConformanceTests.swift` (19 methods since iter 210;
    `UnicodeBoundaryTests.swift` and `unicode_boundary.json` are gone with the freeze removal).
    **Never compare Swift `String ==` on Unicode sequences** — it folds canonical equivalence;
    compare `unicodeScalars`. Vendored `data.json` is an SPM `.copy(...)` resource, registered in
    `tests/test_vendored_fixtures.py`.
- `scripts/build_xcframework.sh`: 5 Rust targets → `lipo` → `xcodebuild -create-xcframework` →
    `ditto` zip → checksum. Output `target/ios/IsccLib.xcframework.zip` (`--release`/`--debug`).
- Version constant: `packages/swift/Sources/IsccLib/Constants.swift` (`isccLibVersion`). CI job
    (`swift:`, `macos-14`): `cargo build -p iscc-uniffi` → `swift build` → `swift test` with
    `-Xlinker -L`/`-rpath` → `target/debug`.

## Kotlin bindings (UniFFI/JVM)

- `packages/kotlin/` — Gradle JVM project, UniFFI-generated Kotlin via JNA (mature/complete).
- Generated `src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` — do NOT hand-edit; regenerate via
    uniffi-bindgen.
- JNA native loading needs `jna.library.path` + `LD_LIBRARY_PATH`. `ConformanceTest.kt` = 19 methods
    (iter 210).
- Full generate command, Gradle/JNA versions, gitignore quirks, Maven Central publishing →
    MEMORY-archive.md.
