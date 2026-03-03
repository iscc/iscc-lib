# Spec: Kotlin Multiplatform Bindings — UniFFI-Generated KMP Library

A Kotlin Multiplatform (KMP) library published to Maven Central as `io.iscc:iscc-lib-kotlin`,
extending ISCC support to Kotlin/Native targets (iOS, desktop) beyond the existing Java/JNI
coverage. Shares the UniFFI scaffolding crate (`crates/iscc-uniffi/`) with Swift bindings.

## Architecture

**Shared infrastructure with Swift:**

1. **UniFFI scaffolding crate** (`crates/iscc-uniffi/`): Same crate used by Swift bindings. Single
    Rust interface definition generates both Swift and Kotlin code via `uniffi-bindgen`.
2. **Generated Kotlin code** (`packages/kotlin/src/commonMain/kotlin/.../generated/`):
    `uniffi-bindgen` generates Kotlin bindings from the shared scaffolding.
3. **KMP project** (`packages/kotlin/`): Gradle-based multiplatform project targeting JVM, iOS
    (Kotlin/Native), and macOS (Kotlin/Native).

**Relationship to existing Java/JNI:**

- `crates/iscc-jni` (existing) — JNI bridge for JVM only, `static native` methods, manual memory
    management via opaque pointers
- `packages/kotlin` (new) — UniFFI-generated, targets JVM + Native, idiomatic Kotlin with null
    safety, sealed classes, and automatic resource cleanup

Both coexist: Java developers continue using `io.iscc:iscc-lib` (JNI), Kotlin developers choose
`io.iscc:iscc-lib-kotlin` (UniFFI/KMP) for native platform support and idiomatic API.

## Package Structure

```
packages/kotlin/
├── build.gradle.kts                    # KMP project config
├── settings.gradle.kts
├── gradle.properties
├── src/
│   ├── commonMain/
│   │   └── kotlin/io/iscc/lib/
│   │       ├── generated/              # uniffi-bindgen output
│   │       │   └── iscc_uniffi.kt
│   │       ├── IsccLib.kt              # Idiomatic Kotlin wrapper
│   │       └── Results.kt             # Sealed/data class result types
│   ├── jvmMain/                        # JVM-specific (loads native lib from resources)
│   ├── iosMain/                        # iOS-specific (links static lib)
│   └── commonTest/
│       └── kotlin/io/iscc/lib/
│           ├── ConformanceTest.kt      # kotlin.test against data.json
│           └── data.json               # Vendored conformance vectors
├── libs/                               # Platform-specific native libraries
│   ├── jvm/
│   │   └── libiscc_uniffi.so           # (or .dylib, .dll)
│   └── ios/
│       └── libiscc_uniffi.a
└── README.md                           # Per-package README
```

## Idiomatic Kotlin API

### Result Types

```kotlin
data class MetaCodeResult(
    val iscc: String,
    val name: String,
    val metahash: String,
    val description: String? = null,
    val meta: String? = null,
)

data class SumCodeResult(
    val iscc: String,
    val datahash: String,
    val filesize: ULong,
)

// ... one data class per gen_*_v0 return type
```

### Public API

```kotlin
object IsccLib {
    fun genMetaCodeV0(
        name: String,
        description: String? = null,
        meta: String? = null,
        bits: UInt = 64u,
    ): MetaCodeResult

    fun genDataCodeV0(data: ByteArray, bits: UInt = 64u): DataCodeResult
    fun genSumCodeV0(path: String, bits: UInt = 64u, wide: Boolean = false): SumCodeResult
    fun conformanceSelftest(): Boolean
    // ... all 32 Tier 1 symbols
}
```

### Streaming Types

```kotlin
class DataHasher : AutoCloseable {
    constructor()
    fun update(data: ByteArray)
    fun finalize(bits: UInt = 64u): DataCodeResult
    override fun close()
}
```

### Kotlin Conventions

- `camelCase` method names (auto-converted by UniFFI)
- Nullable types (`String?`) for optional parameters
- `data class` result types with named properties
- `AutoCloseable` for streaming hashers (supports `use { }` blocks)
- Default parameter values: `genMetaCodeV0(name = "Title")`

### Target DX

```kotlin
import io.iscc.lib.IsccLib

// Code generation
val result = IsccLib.genMetaCodeV0(name = "Title", description = "A description")
println(result.iscc)       // "ISCC:..."
println(result.name)       // "Title"

// Streaming with automatic cleanup
DataHasher().use { hasher ->
    hasher.update(chunk1)
    hasher.update(chunk2)
    val code = hasher.finalize(bits = 64u)
}

// ISCC-SUM from file
val sum = IsccLib.genSumCodeV0(path = "/path/to/file.bin")

// Conformance selftest
check(IsccLib.conformanceSelftest())
```

## Distribution / Publishing (Maven Central)

**Coordinates:** `io.iscc:iscc-lib-kotlin`

Published alongside the existing `io.iscc:iscc-lib` (JNI). Uses the same Sonatype credentials and
GPG signing infrastructure.

**KMP artifact naming:**

| Artifact                            | Target       |
| ----------------------------------- | ------------ |
| `iscc-lib-kotlin-jvm`               | JVM          |
| `iscc-lib-kotlin-iosarm64`          | iOS (device) |
| `iscc-lib-kotlin-iossimulatorarm64` | iOS (sim)    |
| `iscc-lib-kotlin-macosarm64`        | macOS        |

## CI Integration

```yaml
kotlin:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: actions/setup-java@v4
      with:
        distribution: temurin
        java-version: '17'
    - run: cargo build -p iscc-uniffi --release
    - run: >-
        cargo run -p iscc-uniffi -- generate --language kotlin
        --out-dir packages/kotlin/src/commonMain/kotlin/io/iscc/lib/generated/
    - run: ./gradlew check
      working-directory: packages/kotlin
```

Note: iOS/macOS native tests require macOS runner. JVM tests run on Ubuntu.

## DevContainer Setup

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-17-jdk-headless \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
# Gradle wrapper downloads Kotlin compiler automatically
```

## Version Sync

| Target                              | What is synced  |
| ----------------------------------- | --------------- |
| `packages/kotlin/gradle.properties` | `version=X.Y.Z` |

## Code Quality

- **ktlint**: Kotlin linting/formatting (matches project pattern of opinionated formatters)
- Pre-commit: `ktlint --format` auto-fix; pre-push: `ktlint` check + `gradlew check`

## Documentation

- **How-to guide**: `docs/howto/kotlin.md`
- **Per-package README**: `packages/kotlin/README.md`

## Account Setup Required

Uses the existing Sonatype/Maven Central credentials from the Java/JNI publishing setup.

## Verification Criteria

- [ ] Kotlin tests pass conformance vectors on JVM target
- [ ] Published to Maven Central as `io.iscc:iscc-lib-kotlin`
- [ ] All 10 `gen_*_v0` functions return correct data classes
- [ ] All 32 Tier 1 symbols accessible from Kotlin
- [ ] Works from Kotlin/JVM and Kotlin/Native (iOS, macOS)
- [ ] `DataHasher` / `InstanceHasher` work with `use { }` blocks (AutoCloseable)
- [ ] Nullable types used for optional parameters
- [ ] Default parameter values work correctly
- [ ] API uses Kotlin conventions: `camelCase`, data classes, null safety
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] `conformanceSelftest()` returns `true`
- [ ] `ktlint` passes with zero violations
- [ ] UniFFI scaffolding crate shared with Swift (no duplication)
