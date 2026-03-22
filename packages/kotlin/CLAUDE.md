# CLAUDE.md -- iscc-lib-kotlin

Kotlin/JVM bindings for `iscc-lib` via UniFFI, providing Kotlin-idiomatic access to ISO 24138 ISCC
code generation.

## Package Role

- UniFFI-generated Kotlin bindings over the `iscc-uniffi` Rust crate
- All functions are free functions in the `uniffi.iscc_uniffi` package
- Does NOT implement any ISCC logic; all computation delegates through `iscc-uniffi` to `iscc-lib`
- JVM-only (no Kotlin/Native or Kotlin Multiplatform support)
- Native library loaded at runtime via JNA (`net.java.dev.jna:jna:5.16.0`)

## File Layout

```
packages/kotlin/
  build.gradle.kts                          # Gradle build (Kotlin JVM plugin, JNA + test deps)
  gradle.properties                         # Version property (synced by version_sync.py)
  settings.gradle.kts                       # Root project name
  gradlew / gradlew.bat                     # Gradle wrapper scripts
  gradle/wrapper/                           # Gradle wrapper JAR + properties
  README.md                                 # Package readme with install and usage examples
  CLAUDE.md                                 # This file
  src/
    main/kotlin/uniffi/iscc_uniffi/
      iscc_uniffi.kt                        # UniFFI-generated Kotlin bindings (DO NOT EDIT)
    test/kotlin/uniffi/iscc_uniffi/
      ConformanceTest.kt                    # data.json conformance vectors for 9 gen_*_v0 functions
    test/resources/
      data.json                             # Vendored ISCC conformance test vectors
```

## Build Commands

```bash
# Prerequisites: cargo (for iscc-uniffi), JDK 17+, Gradle 8.x

# Build the Rust UniFFI library (required before Gradle build/test)
cargo build -p iscc-uniffi

# Build the Kotlin package
cd packages/kotlin
./gradlew build

# Run tests (requires libiscc_uniffi in library path)
./gradlew test
```

The Gradle build auto-configures `java.library.path`, `jna.library.path`, and `LD_LIBRARY_PATH` to
point to `../../target/debug/` relative to the Kotlin package root.

## Test Patterns

### Conformance tests (`ConformanceTest.kt`)

- 9 test methods, 50 test vectors from `data.json`
- One test method per gen function (no `gen_sum_code_v0` vectors in data.json)
- `decodeStream` helper converts `"stream:<hex>"` format to `ByteArray`
- Asserts exact ISCC string equality against expected outputs
- Uses JUnit 5 (`org.junit.jupiter`) and Gson (`com.google.code.gson:gson`) for JSON parsing

## Binding Generation

The Kotlin bindings are generated from the `iscc-uniffi` crate using UniFFI's proc macro approach:

```bash
# Build the library first
cargo build -p iscc-uniffi

# Generate Kotlin bindings
cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- \
    generate --language kotlin --no-format --out-dir packages/kotlin/src/main/kotlin/ \
    target/debug/libiscc_uniffi.so
```

Generated file: `iscc_uniffi.kt` (~3200 lines, `package uniffi.iscc_uniffi`). Do NOT manually edit —
regenerate when the Rust API surface changes.

## CI

- Kotlin CI job runs on `ubuntu-latest` with JDK 17
- Steps: `cargo build -p iscc-uniffi` -> `./gradlew test`
- Gradle wrapper is checked in (no separate Gradle install needed)

## Common Pitfalls

- **Forgetting to build `iscc-uniffi` first:** `./gradlew build` may succeed, but tests fail at
    runtime with `UnsatisfiedLinkError`. Always `cargo build -p iscc-uniffi` before Kotlin tests.
- **JNA library path:** JNA requires both `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env
    var for `Native.register()` — `java.library.path` alone is not sufficient.
- **Generated file is not manually edited:** `iscc_uniffi.kt` is UniFFI-generated. Re-generate it
    when the Rust API surface changes.
- **Constants are getter functions:** UniFFI cannot export `const` values. Constants like
    `metaTrimName()` are functions, not properties.
- **UInt/ULong parameters:** UniFFI maps Rust unsigned types to Kotlin unsigned types (`UInt`,
    `ULong`, `UByte`). Use unsigned literal suffixes (`64u`, `5u`) when calling functions.
- **Version sync:** `gradle.properties` contains the `version` property, synced by
    `scripts/version_sync.py`.
- **Gson groupId:** The Maven groupId for Gson is `com.google.code.gson` (not `com.google.gson` —
    that is the Java package name, not the Maven coordinate).
