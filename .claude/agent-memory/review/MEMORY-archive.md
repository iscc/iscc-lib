# Review Agent Memory — Archive

Stale or niche entries moved out of `MEMORY.md` to keep it under the 200-line budget. Not loaded by
the agent automatically — reference for humans and occasional lookup only.

## Python Benchmark Review (archived iter 93)

- pytest-benchmark tests in `tests/test_benchmarks.py`: 18 benchmarks (9 fn x 2 impls), ~11s
    collection overhead (`--benchmark-disable` optimization noted in learnings-archive.md)
- Review shortcut: `mise run check` + ruff check/format + clippy (Python-only shortcut)

## Swift Package Review (archived iter 94 — Swift bindings fully complete)

- `packages/swift/` — SPM package: iscc_uniffiFFI (C header + modulemap) + IsccLib (generated Swift)
- Two `Package.swift` files: root (SPM consumers, binaryTarget) + `packages/swift/Package.swift`
    (CI/local dev, linkedLibrary). Root omits testTarget. Both coexist without conflict
- Root `Package.swift` uses Ferrostar-style variable toggle: `useLocalFramework` (bool),
    `releaseTag`, `releaseChecksum`. Default `false` → remote binaryTarget from GitHub Releases
- `scripts/build_xcframework.sh`: 5 Rust targets → lipo fat binaries → xcodebuild → ditto zip →
    swift package compute-checksum. Cannot test on Linux — `bash -n` syntax check only
- Review shortcut: `cargo build/test/clippy -p iscc-uniffi` + `mise run check` (no `swift test` on
    Linux). Swift tests structurally validated only — execution needs macOS CI
- Swift CI job (`swift:`) on `macos-14`: `dump-package` (root) → `cargo build -p iscc-uniffi` →
    `swift build` → `swift test`

## JNA Android ARM32 (archived iter 97)

- **JNA Android ARM32 resource path**: JNA canonicalizes ARM32 arch to `arm` (not `armv7`). Correct
    prefix is `android-arm/`, not `android-armv7/`. Verified via bytecode decompilation. Filed as
    spec issue with HUMAN REVIEW REQUESTED.
