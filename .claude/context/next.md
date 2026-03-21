# Next Work Package

## Step: Create Kotlin JVM project scaffold with UniFFI-generated bindings

## Goal

Bootstrap the Kotlin bindings project in `packages/kotlin/` — a Gradle JVM project with the
UniFFI-generated Kotlin source and JNA dependency — so that the code compiles successfully. This is
the first step toward the "Implement Kotlin Multiplatform bindings via UniFFI" issue.

## Scope

- **Create**: `packages/kotlin/build.gradle.kts`, `packages/kotlin/settings.gradle.kts`,
    `packages/kotlin/gradle.properties`
- **Modify**: (none)
- **Reference**: `packages/swift/` (UniFFI binding package pattern), `crates/iscc-uniffi/src/lib.rs`
    (scaffolding crate), `.claude/context/specs/kotlin-bindings.md` (full spec)

## Not In Scope

- Conformance tests or any test code — tests come in a subsequent step
- CI job in `ci.yml` — CI integration is a separate step
- README.md, CLAUDE.md, or docs/howto/kotlin.md — documentation comes after core functionality
- Maven Central publishing setup — release workflow changes are a separate step
- Version sync in `version_sync.py` — separate step (after `gradle.properties` exists)
- Kotlin Multiplatform / Kotlin Native targets (iOS, macOS) — UniFFI generates JVM-only code; KMP is
    a future enhancement
- Wrapper classes or re-packaging (e.g., `io.iscc.lib.IsccLib` object) — use the generated
    `uniffi.iscc_uniffi` package as-is for now
- Updating README.md with Kotlin install/quickstart sections

## Implementation Notes

### 1. Generate Kotlin bindings

```bash
# Build the UniFFI crate first
cargo build -p iscc-uniffi

# Generate Kotlin bindings (use --no-format since ktlint is not installed)
cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- \
    generate --language kotlin --no-format \
    --out-dir packages/kotlin/src/main/kotlin/ \
    target/debug/libiscc_uniffi.so
```

This produces `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (3200+ lines). The
file has `package uniffi.iscc_uniffi` — the directory structure must match this package path.

### 2. Bootstrap Gradle wrapper

```bash
# Gradle 8.12.1 is available via mise (already installed)
cd packages/kotlin
mise exec gradle@8.12.1 -- gradle wrapper --gradle-version 8.12.1
```

This creates `gradlew`, `gradlew.bat`, and `gradle/wrapper/` files. These should be committed.

### 3. Create `build.gradle.kts`

Use the `kotlin("jvm")` plugin (NOT `kotlin("multiplatform")` — the UniFFI-generated code is
JVM-only due to JNA dependency). Key dependencies:

```kotlin
plugins {
    kotlin("jvm") version "2.1.10"  // Or latest stable Kotlin
}

group = "io.iscc"
version = providers.gradleProperty("version").get()

repositories {
    mavenCentral()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.16.0")
}

// For tests later: configure java.library.path to find libiscc_uniffi.so
tasks.withType<Test> {
    useJUnitPlatform()
    jvmArgs("-Djava.library.path=${rootProject.rootDir}/../../target/debug")
}
```

### 4. Create `settings.gradle.kts`

```kotlin
rootProject.name = "iscc-lib-kotlin"
```

### 5. Create `gradle.properties`

```properties
version=0.3.1
kotlin.code.style=official
```

### 6. Verify compilation

```bash
cd packages/kotlin && ./gradlew compileKotlin
```

This should resolve JNA from Maven Central and compile the generated Kotlin code successfully.

### Important notes

- The generated `iscc_uniffi.kt` uses JNA (`com.sun.jna.*`) for native library loading — this is
    JVM-only. The native lib name is `iscc_uniffi` (loads `libiscc_uniffi.so`/`.dylib`/`.dll`)
- Do NOT modify the generated Kotlin file — it's UniFFI output
- The `gradle/wrapper/gradle-wrapper.jar` (~65KB binary) must be committed
- Add `packages/kotlin/.gradle/` and `packages/kotlin/build/` to `.gitignore` (or verify they're
    covered by existing patterns)

## Verification

- `test -d packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi` exits 0 (generated code present)
- `test -f packages/kotlin/build.gradle.kts` exits 0 (build file exists)
- `test -f packages/kotlin/gradlew` exits 0 (Gradle wrapper present)
- `cd packages/kotlin && ./gradlew compileKotlin` exits 0 (compilation succeeds)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean

## Done When

The `packages/kotlin/` directory contains a Gradle JVM project with UniFFI-generated Kotlin bindings
that compiles successfully via `./gradlew compileKotlin`.
