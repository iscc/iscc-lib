# Spec: Kotlin Bindings — Android and JVM via UniFFI

A Kotlin/JVM library published to Maven Central as `io.iscc:iscc-lib-kotlin`, providing idiomatic
Kotlin access to all ISCC functions. **Primary audience: Android developers.** Also works on
server/desktop JVM. Shares the UniFFI scaffolding crate (`crates/iscc-uniffi/`) with Swift bindings.

**Mobile DX story:** iOS developers use the Swift bindings (XCFramework via SPM). Android developers
use the Kotlin bindings (JAR with bundled native libraries via Maven Central). Together they provide
zero-friction ISCC integration for both mobile platforms.

## Architecture

**Why Kotlin/JVM (not Kotlin Multiplatform):**

UniFFI 0.31's Kotlin backend generates JVM-only code using `com.sun.jna.*` for native library
loading. Kotlin/Native would require a separate `cinterop`-based approach that UniFFI does not
support. Since iOS is already covered by the Swift bindings (which share the same UniFFI scaffolding
crate), Kotlin/Native targets (iOS, macOS) would be redundant. The Kotlin bindings focus on Android
and desktop/server JVM where JNA is the standard native loading mechanism.

**Shared infrastructure with Swift:**

1. **UniFFI scaffolding crate** (`crates/iscc-uniffi/`): Same crate used by Swift bindings. Single
    Rust interface definition generates both Swift and Kotlin code via `uniffi-bindgen`.
2. **Generated Kotlin code** (`packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/`):
    `uniffi-bindgen` generates Kotlin bindings from the shared scaffolding.
3. **Gradle JVM project** (`packages/kotlin/`): Kotlin/JVM project with JNA dependency for native
    library loading.

**Relationship to existing Java/JNI:**

- `crates/iscc-jni` (existing) — JNI bridge for JVM only, `static native` methods, manual memory
    management via opaque pointers
- `packages/kotlin` — UniFFI-generated, idiomatic Kotlin with null safety, data classes, and
    automatic resource cleanup

Both coexist: Java developers continue using `io.iscc:iscc-lib` (JNI), Kotlin developers choose
`io.iscc:iscc-lib-kotlin` (UniFFI) for idiomatic API and Android support.

## Package Structure

```
packages/kotlin/
├── build.gradle.kts                    # Kotlin/JVM project config
├── settings.gradle.kts
├── gradle.properties                   # Version property (synced by version_sync.py)
├── gradlew / gradlew.bat              # Gradle wrapper scripts
├── gradle/wrapper/                    # Gradle wrapper JAR + properties
├── src/
│   ├── main/kotlin/uniffi/iscc_uniffi/
│   │   └── iscc_uniffi.kt            # UniFFI-generated Kotlin bindings (DO NOT EDIT)
│   ├── main/resources/               # Bundled native libraries (release JAR only)
│   │   ├── linux-x86-64/             # JNA resource path for Linux x86_64
│   │   ├── linux-aarch64/            # JNA resource path for Linux ARM64
│   │   ├── darwin-aarch64/           # JNA resource path for macOS ARM64
│   │   ├── darwin-x86-64/            # JNA resource path for macOS x86_64
│   │   ├── win32-x86-64/            # JNA resource path for Windows x86_64
│   │   ├── android-aarch64/          # JNA resource path for Android ARM64
│   │   ├── android-arm/              # JNA resource path for Android ARM32
│   │   └── android-x86-64/           # JNA resource path for Android x86_64
│   └── test/
│       ├── kotlin/uniffi/iscc_uniffi/
│       │   └── ConformanceTest.kt    # data.json conformance vectors
│       └── resources/
│           └── data.json             # Vendored conformance test vectors
├── README.md
└── CLAUDE.md
```

## Native Library Targets

The release JAR must bundle native libraries for all supported platforms. JNA discovers them
automatically from classpath resources using the `<os>-<arch>/` directory convention.

### Desktop/Server targets (existing)

| Platform       | Rust Target                 | JNA Resource Path |
| -------------- | --------------------------- | ----------------- |
| Linux x86_64   | `x86_64-unknown-linux-gnu`  | `linux-x86-64/`   |
| Linux ARM64    | `aarch64-unknown-linux-gnu` | `linux-aarch64/`  |
| macOS ARM64    | `aarch64-apple-darwin`      | `darwin-aarch64/` |
| macOS x86_64   | `x86_64-apple-darwin`       | `darwin-x86-64/`  |
| Windows x86_64 | `x86_64-pc-windows-msvc`    | `win32-x86-64/`   |

### Android targets (required for mobile DX)

| Platform       | Rust Target               | JNA Resource Path  | Android ABI   |
| -------------- | ------------------------- | ------------------ | ------------- |
| Android ARM64  | `aarch64-linux-android`   | `android-aarch64/` | `arm64-v8a`   |
| Android ARM32  | `armv7-linux-androideabi` | `android-arm/`     | `armeabi-v7a` |
| Android x86_64 | `x86_64-linux-android`    | `android-x86-64/`  | `x86_64`      |
| Android x86    | `i686-linux-android`      | `android-x86/`     | `x86`         |

Android cross-compilation requires the Android NDK. The release workflow should use `cargo-ndk` or
set `CARGO_TARGET_*_LINKER` to the NDK's clang for each target.

## Idiomatic Kotlin API

UniFFI generates free functions in the `uniffi.iscc_uniffi` package with `camelCase` naming:

```kotlin
import uniffi.iscc_uniffi.*

// Code generation
val result = genMetaCodeV0(name = "Title", description = null, meta = null, bits = 64u)
println(result.iscc)       // "ISCC:..."
println(result.name)       // "Title"

// Streaming with automatic cleanup
DataHasher().use { hasher ->
    hasher.update(chunk1)
    hasher.update(chunk2)
    val code = hasher.finalize(bits = 64u)
}

// ISCC-SUM from file
val sum = genSumCodeV0(path = "/path/to/file.bin", bits = 64u, wide = false, addUnits = false)

// Conformance selftest
check(conformanceSelftest())
```

### Kotlin Conventions

- `camelCase` method names (auto-converted by UniFFI)
- Nullable types (`String?`) for optional parameters
- `data class` result types with named properties (UniFFI Records)
- `AutoCloseable` for streaming hashers (supports `use { }` blocks)
- `UInt`/`ULong` for unsigned parameters

## Distribution / Publishing (Maven Central)

**Coordinates:** `io.iscc:iscc-lib-kotlin`

Published alongside the existing `io.iscc:iscc-lib` (JNI). Uses the same Sonatype credentials and
GPG signing infrastructure.

The release JAR bundles native libraries for all 8-9 platforms as JNA classpath resources. Android
consumers add the dependency to their `build.gradle.kts` and JNA handles native library extraction
and loading automatically.

## Android Developer Experience

Target DX for an Android developer:

```kotlin
// build.gradle.kts (app module)
dependencies {
    implementation("io.iscc:iscc-lib-kotlin:0.4.0")
    implementation("net.java.dev.jna:jna:5.16.0@aar")
}
```

```kotlin
// MainActivity.kt
import uniffi.iscc_uniffi.*

val result = genMetaCodeV0(name = "My Photo", description = null, meta = null, bits = 64u)
textView.text = result.iscc
```

No NDK installation, no `jniLibs` directory management, no `System.loadLibrary()` calls. JNA loads
the correct `.so` for the device's ABI from the JAR resources automatically.

**JNA on Android note:** JNA requires `net.java.dev.jna:jna:5.16.0@aar` (the AAR variant, not the
plain JAR) for Android. This extracts the JNA native libraries for Android ABIs. The iscc-lib-kotlin
native libraries are loaded from classpath resources by JNA's `Native.register()` mechanism.

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
    - run: cargo build -p iscc-uniffi
    - run: ./gradlew test
      working-directory: packages/kotlin
```

JVM tests run on Ubuntu. Android integration testing (loading `.so` on Android ABI) would require an
Android emulator CI step or be validated by the release smoke test.

## DevContainer Setup

The devcontainer needs Android NDK, Rust Android targets, and `cargo-ndk` for local
cross-compilation:

```dockerfile
# Android NDK (for Kotlin/Android cross-compilation)
ARG NDK_VERSION=27.2.12479018
RUN curl -fsSL https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip \
    -o /tmp/cmdtools.zip && unzip -q /tmp/cmdtools.zip -d /tmp/cmdtools && rm /tmp/cmdtools.zip \
    && yes | /tmp/cmdtools/cmdline-tools/bin/sdkmanager --sdk_root=$HOME/android \
        "ndk;${NDK_VERSION}" && rm -rf /tmp/cmdtools
ENV ANDROID_NDK_HOME=$HOME/android/ndk/$NDK_VERSION
```

```dockerfile
# Rust Android targets + cargo-ndk
RUN rustup target add aarch64-linux-android armv7-linux-androideabi \
        x86_64-linux-android i686-linux-android \
    && cargo install cargo-ndk
```

## Release Workflow

The release workflow cross-compiles `iscc-uniffi` for all target platforms (5 desktop + 3-4 Android
ABIs), bundles the native libraries into the JAR's resource paths, and publishes to Maven Central
via Sonatype Central Portal.

Android targets require the Android NDK in the build environment. Use `cargo-ndk` for ergonomic
cross-compilation:

```bash
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o packages/kotlin/src/main/resources/ \
    build -p iscc-uniffi --release
```

## Version Sync

| Target                              | What is synced  |
| ----------------------------------- | --------------- |
| `packages/kotlin/gradle.properties` | `version=X.Y.Z` |

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
- [ ] Native libraries bundled for 5 desktop platforms (linux-x86-64, linux-aarch64, darwin-aarch64,
    darwin-x86-64, win32-x86-64)
- [ ] Native libraries bundled for Android ABIs (arm64-v8a, armeabi-v7a, x86_64)
- [ ] Android app can add Maven dependency and call ISCC functions without manual native lib setup
- [ ] `DataHasher` / `InstanceHasher` work with `use { }` blocks (AutoCloseable)
- [ ] Nullable types used for optional parameters
- [ ] API uses Kotlin conventions: `camelCase`, data classes, null safety
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] `conformanceSelftest()` returns `true`
- [ ] UniFFI scaffolding crate shared with Swift (no duplication)
