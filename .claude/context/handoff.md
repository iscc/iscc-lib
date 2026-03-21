## 2026-03-21 — Review of: Generate Swift bindings and create SPM package structure

**Verdict:** PASS

**Summary:** The advance agent created the complete Swift package at `packages/swift/` with
UniFFI-generated bindings, SPM manifest, XCTest conformance tests (50 vectors across 9 gen
functions), and vendored data.json. Also added the `bindgen` feature and `[[bin]]` section to
`iscc-uniffi` for the `uniffi-bindgen` CLI. All verification criteria pass, scope is clean, and no
quality gate circumvention detected.

**Verification:**

- [x] `cargo build -p iscc-uniffi` compiles successfully
- [x] `cargo test -p iscc-uniffi` — 21 tests pass
- [x] `cargo clippy -p iscc-uniffi -- -D warnings` — clean
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- --help` — shows usage
- [x] `packages/swift/Package.swift` exists
- [x] `packages/swift/Sources/IsccLib/iscc_uniffi.swift` exists and contains `genMetaCodeV0`
- [x] `packages/swift/Sources/IsccLibFFI/iscc_uniffiFFI.h` exists
- [x] `packages/swift/Sources/IsccLibFFI/module.modulemap` exists
- [x] `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` exists — 9 test methods, 50
    vectors, correct vector counts (20+5+3+5+3+2+4+3+5)
- [x] `packages/swift/Tests/IsccLibTests/data.json` identical to `crates/iscc-lib/tests/data.json`
- [x] `mise run check` — 14/15 hooks pass (Ruby `standardrb-fix` is pre-existing PATH issue)

**Issues found:**

- (none)

**Codex review:** No regressions found. Codex briefly mentions `gen_sum_code_v0` (likely confused by
the large auto-generated UniFFI code in the diff), but the clean verdict is consistent with this
review's findings.

**Next:** Add Swift CI job on macOS runner (`swift build` + `swift test -Xlinker -L<path>`). The
conformance tests are structurally sound but cannot be executed in the Linux devcontainer — they
need a macOS runner with Swift toolchain. After CI, add `docs/howto/swift.md`, README Swift
sections, and version sync integration. The Swift bindings issue in issues.md still has CI, docs,
and release items remaining.

**Notes:**

- Generated Swift files are committed to git (2400 lines / 72KB for `iscc_uniffi.swift`, 935 lines /
    38KB for `iscc_uniffiFFI.h`). This is reasonable since `uniffi-bindgen` requires the compiled
    `libiscc_uniffi` to regenerate, which needs the full Rust toolchain
- `module.modulemap` was correctly simplified from generated version — removed Darwin-specific `use`
    directives that would break on other platforms
- README `build-from-source` section shows `.so` extension (Linux) — on macOS it's `.dylib`. Minor
    doc nit for future cleanup but not blocking
- `data.json` is now vendored in 4 places (Rust tests, Go, .NET, Swift) — consistent with
    established pattern
- The `bindgen` feature adds `clap_derive` and `strsim` to Cargo.lock but only activates with
    `--features bindgen`
- UniFFI `Data` type for `Vec<u8>` is idiomatic Swift, `camelCase` is auto-generated, `throws` for
    fallible functions — no manual wrapper needed (spec's `IsccLib.swift` wrapper deferred)
