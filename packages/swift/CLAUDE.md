# CLAUDE.md -- IsccLib (Swift)

Swift bindings for `iscc-lib` via UniFFI, providing an idiomatic Swift API for ISO 24138 ISCC code
generation.

## Package Role

- UniFFI-generated Swift bindings over the `iscc-uniffi` Rust crate
- All functions are free functions in the `IsccLib` module (not methods on a class)
- Does NOT implement any ISCC logic; all computation delegates through `iscc-uniffi` to `iscc-lib`
- Distributed as a Swift Package Manager (SPM) package via Git tags (not a registry upload)

## File Layout

```
packages/swift/
  Package.swift                              # SPM package manifest (swift-tools-version: 5.9)
  README.md                                  # Package readme with install and usage examples
  CLAUDE.md                                  # This file
  Sources/
    IsccLib/
      Constants.swift                        # Version constant (isccLibVersion)
      iscc_uniffi.swift                      # UniFFI-generated Swift bindings (DO NOT EDIT)
    iscc_uniffiFFI/
      iscc_uniffiFFI.h                       # UniFFI-generated C header (DO NOT EDIT)
      module.modulemap                       # Simplified modulemap for SPM (manual, not generated)
  Tests/
    IsccLibTests/
      ConformanceTests.swift                 # data.json conformance vectors for 9 gen_*_v0 functions
      data.json                              # Vendored ISCC conformance test vectors (SPM resource)
```

## Build Commands

```bash
# Prerequisites: cargo (for iscc-uniffi), Swift 5.9+ toolchain

# Build the Rust UniFFI library (required before swift build/test)
cargo build -p iscc-uniffi

# Build the Swift package
cd packages/swift
swift build \
    -Xlinker -L../../target/debug \
    -Xlinker -rpath -Xlinker ../../target/debug

# Run tests (requires libiscc_uniffi in linker/runtime path)
swift test \
    -Xlinker -L../../target/debug \
    -Xlinker -rpath -Xlinker ../../target/debug
```

## Test Patterns

### Conformance tests (`ConformanceTests.swift`)

- 9 test methods, 50 test vectors from `data.json`
- One test method per gen function (no `gen_sum_code_v0` vectors in data.json)
- `decodeStream` helper converts `"stream:<hex>"` format to `Data`
- Asserts exact ISCC string equality against expected outputs

## Binding Generation

The Swift bindings are generated from the `iscc-uniffi` crate using UniFFI's proc macro approach:

```bash
# Build the library first
cargo build -p iscc-uniffi

# Generate Swift bindings
cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- \
    generate --library target/debug/libiscc_uniffi.so \
    --language swift --out-dir packages/swift/Sources/IsccLib/
```

Generated files: `iscc_uniffi.swift` (Swift source), `iscc_uniffiFFI.h` (C header),
`iscc_uniffiFFI.modulemap` (module map — must be simplified for SPM by removing Darwin-specific
`use` directives).

## CI

- Swift CI job runs on `macos-14` (Apple Silicon) — Swift is not available on Linux CI runners
- Steps: `cargo build -p iscc-uniffi` -> `swift build` -> `swift test` with `-Xlinker` flags
- Swift tests also run locally in the Linux devcontainer via the swift.org Debian 12 toolchain (see
    Common Pitfalls for the recipe)

## Common Pitfalls

- **Forgetting to build `iscc-uniffi` first:** `swift build` may succeed (cached), but tests fail at
    runtime with dylib-not-found. Always `cargo build -p iscc-uniffi` before Swift tests.

- **Testing on Linux:** no `swift` on `$PATH` in the devcontainer, but swift.org ships a Debian 12
    toolchain that matches the container and runs the full suite (~784 MB download, no `sudo`):

    ```bash
    mkdir -p /tmp/swifttc
    curl -fsSL -o /tmp/swifttc/swift.tar.gz \
        https://download.swift.org/swift-6.1.2-release/debian12/swift-6.1.2-RELEASE/swift-6.1.2-RELEASE-debian12.tar.gz
    tar -xzf /tmp/swifttc/swift.tar.gz -C /tmp/swifttc
    export PATH=/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin:$PATH
    cargo build -p iscc-uniffi   # from the repo root
    cd packages/swift && swift test \
        --scratch-path /tmp/swiftbuild \
        -Xlinker -L../../target/debug \
        -Xlinker -rpath -Xlinker ../../target/debug
    ```

    Prefer `--scratch-path` outside the repo — a bare `swift build`/`swift test` writes
    `packages/swift/.build/` (gitignored, but keep the tree clean anyway).

- **Module name must match:** The SPM target `iscc_uniffiFFI` and the `module.modulemap` module name
    must both be `iscc_uniffiFFI` to match the `#if canImport(iscc_uniffiFFI)` guard in the
    generated Swift source.

- **Generated files are not manually edited:** `iscc_uniffi.swift` and `iscc_uniffiFFI.h` are
    UniFFI-generated. Re-generate them when the Rust API surface changes.

- **`module.modulemap` is manually maintained:** The generated `iscc_uniffiFFI.modulemap` includes
    Darwin-specific `use` directives that break SPM. The checked-in `module.modulemap` is a
    simplified version with just `header` + `export *`.

- **Constants are getter functions:** UniFFI cannot export `const` values. Constants like
    `metaTrimName()` are functions, not properties.

- **Version sync:** SPM uses Git tags for versioning (e.g., `from: "0.3.0"` in `Package.swift`
    dependencies). No separate version manifest to sync — version comes from the Git tag.
