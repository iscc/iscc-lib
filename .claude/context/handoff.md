## 2026-03-21 — Generate Swift bindings and create SPM package structure

**Done:** Created the Swift package at `packages/swift/` with UniFFI-generated bindings, SPM
manifest, XCTest conformance tests (50 vectors across 9 gen functions), and vendored data.json. Also
added the `bindgen` feature and `[[bin]]` section to `iscc-uniffi` for the `uniffi-bindgen` CLI.

**Files changed:**

- `crates/iscc-uniffi/Cargo.toml`: Added `[features] bindgen = ["uniffi/cli"]` and `[[bin]]` section
    for `uniffi-bindgen` binary
- `crates/iscc-uniffi/uniffi-bindgen.rs`: 3-line entry point for UniFFI binding generation CLI
- `packages/swift/Package.swift`: SPM manifest with IsccLibFFI + IsccLib + IsccLibTests targets
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift`: Generated Swift bindings (~2260 lines) with
    all 32 Tier 1 symbols in camelCase
- `packages/swift/Sources/IsccLibFFI/iscc_uniffiFFI.h`: Generated C header for FFI layer
- `packages/swift/Sources/IsccLibFFI/module.modulemap`: Module map for IsccLibFFI target (simplified
    from generated `iscc_uniffiFFI.modulemap`)
- `packages/swift/Tests/IsccLibTests/ConformanceTests.swift`: XCTest class with 9 test methods
    covering all 50 conformance vectors
- `packages/swift/Tests/IsccLibTests/data.json`: Vendored conformance vectors (identical copy from
    `crates/iscc-lib/tests/data.json`)
- `packages/swift/README.md`: Per-package README with install, usage, and build-from-source
    instructions

**Verification:**

- `cargo build -p iscc-uniffi` — compiles successfully
- `cargo test -p iscc-uniffi` — 21 tests pass (existing tests unaffected)
- `cargo clippy -p iscc-uniffi -- -D warnings` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- --help` — shows usage
- All 8 file existence checks pass
- `data.json` is identical to source
- `mise run check` — 14/15 hooks pass (Ruby `standardrb-fix` fails due to pre-existing `bundle` not
    on PATH, unrelated to this change)

**Next:** Add CI job for Swift on macOS runner (`swift build` + `swift test`). The Swift tests
cannot be run in this Linux devcontainer — they need a macOS runner with Swift toolchain and the
compiled `libiscc_uniffi` native library. The CI job would: (1) build `iscc-uniffi` with Rust, (2)
run `swift test -Xlinker -L<path>` in `packages/swift/`.

**Notes:**

- The generated `iscc_uniffiFFI.modulemap` references `"Darwin"` and `"_Builtin_stdbool"` modules
    which are macOS-specific. The `module.modulemap` placed in the package was simplified to just
    reference the header and export all — this is the standard pattern for SPM C targets
- UniFFI generates `Data` (not `[UInt8]`) for Rust `Vec<u8>` — this is idiomatic Swift
- UniFFI auto-generates `Sendable` conformance for result structs (compiler >= 6)
- The generated Swift file is ~72KB / ~2260 lines — includes all internal FFI converters, not just
    the public API surface
- Swift conformance tests use `JSONSerialization` for flexible JSON parsing, matching the .NET
    `JsonElement` pattern. The `prepareMeta` helper handles null/string/dict meta variants
- The `Cargo.lock` gains 2 new transitive deps (`clap_derive`, `strsim`) from the `uniffi/cli`
    feature, but only when `bindgen` feature is active
- Cannot run `swift test` in this environment (Linux devcontainer without Swift toolchain) — tests
    are structurally validated but not executed. Execution requires the macOS CI job
