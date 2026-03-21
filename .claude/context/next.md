# Next Work Package

## Step: Add maven-kotlin release workflow for Kotlin package publishing

## Goal

Add the `maven-kotlin` input and corresponding build/test/publish jobs to `release.yml`, plus
configure Gradle maven-publish and signing in `build.gradle.kts`. This is the last remaining
sub-task for the Kotlin bindings issue — completing it enables publishing `io.iscc:iscc-lib-kotlin`
to Maven Central.

## Scope

- **Create**: (none)
- **Modify**:
    - `.github/workflows/release.yml` — add `maven-kotlin` boolean input + 4 jobs (build native libs,
        assemble JAR, smoke test, publish)
    - `packages/kotlin/build.gradle.kts` — add `maven-publish` plugin, `signing` plugin, POM metadata,
        Sonatype Central repository config, native library resource bundling
- **Reference**:
    - `.github/workflows/release.yml` lines 414-580 (Java Maven Central pattern: build-jni →
        assemble-jar → test-jni → publish-maven)
    - `crates/iscc-jni/java/pom.xml` (POM metadata structure: groupId, artifactId, developers,
        licenses, SCM)
    - `packages/kotlin/build.gradle.kts` (current Gradle config)
    - `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` lines 350-360
        (`findLibraryName` — JNA loads library named `iscc_uniffi`)
    - `packages/kotlin/CLAUDE.md` (package context, JNA library path behavior)

## Not In Scope

- Publishing the actual package to Maven Central (this just adds the workflow — actual publish
    happens on next release tag or manual dispatch)
- Kotlin/Native or KMP targets — this is JVM-only, matching the current project setup
- Updating the Kotlin CI job in `ci.yml` (already working, separate concern)
- Changing `settings.gradle.kts` (current config is fine)
- Adding a 9th registry badge to the README (wait for successful first publish)

## Implementation Notes

### release.yml additions

Follow the existing Java Maven pattern (build-jni → assemble-jar → test-jni → publish-maven) but
adapted for UniFFI/JNA:

1. **Input**: Add `maven-kotlin` boolean input (description: "Publish iscc-lib-kotlin to Maven
    Central", default: false)

2. **`build-kotlin-native` job** — Build `iscc-uniffi` crate for 5 platforms (same matrix structure
    as `build-jni`):

    - Targets: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `aarch64-apple-darwin`,
        `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`
    - Library names: `libiscc_uniffi.so` (Linux), `libiscc_uniffi.dylib` (macOS), `iscc_uniffi.dll`
        (Windows)
    - Command: `cargo build -p iscc-uniffi --release --target ${{ matrix.target }}`
    - Upload artifacts as `kotlin-native-{native-dir}`
    - **Windows shell**: version extraction step needs `shell: bash` (Windows runners default to
        `pwsh`)

3. **`assemble-kotlin` job** — Download native libs, place in JNA resource paths inside the Gradle
    source tree, run `./gradlew build`:

    - JNA loads native libs from JAR resources at `{platform}/` paths following JNA's
        `Platform.RESOURCE_PREFIX` convention:
        - `linux-x86-64/libiscc_uniffi.so`
        - `linux-aarch64/libiscc_uniffi.so`
        - `darwin-aarch64/libiscc_uniffi.dylib`
        - `darwin-x86-64/libiscc_uniffi.dylib`
        - `win32-x86-64/iscc_uniffi.dll`
    - Copy native libs to `packages/kotlin/src/main/resources/{platform}/` before `./gradlew build`
    - Also needs `cargo build -p iscc-uniffi` for the test step (tests use debug build for
        `java.library.path`). Alternatively, copy the linux-x86-64 release lib to the right path and
        adjust `LD_LIBRARY_PATH`
    - Upload the built JAR artifact (from `packages/kotlin/build/libs/`)

4. **`test-kotlin-release` job** — Smoke test on ubuntu-latest:

    - Download assembled JAR + linux-x86-64 native lib
    - Set up JDK 17
    - Run a minimal Java/Kotlin program that calls `conformanceSelftest()` from the JAR
    - Or: re-run `./gradlew test` with the native lib available

5. **`publish-maven-kotlin` job** — Publish to Maven Central:

    - Condition: `startsWith(github.ref, 'refs/tags/v') || inputs.maven-kotlin`
    - needs: `[assemble-kotlin, test-kotlin-release]`
    - Check if version already published (Maven Central search API for
        `g:io.iscc+AND+a:iscc-lib-kotlin`)
    - Import GPG key (same `MAVEN_GPG_PRIVATE_KEY` secret as Java)
    - Run `./gradlew publishMavenPublicationToSonatypeRepository` with env vars
    - Use same `MAVEN_USERNAME` and `MAVEN_PASSWORD` secrets as the Java publish job

### build.gradle.kts additions

Add publishing infrastructure following standard Gradle Maven Central publishing:

**Plugins**:

```kotlin
plugins {
    kotlin("jvm") version "2.1.10"
    `maven-publish`
    signing
}
```

**Sources + Javadoc JARs** (Maven Central requires these):

```kotlin
java {
    withSourcesJar()
    withJavadocJar()
}
```

**Publishing block**:

```kotlin
publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "io.iscc"
            artifactId = "iscc-lib-kotlin"
            from(components["java"])
            pom {
                name.set("iscc-lib-kotlin")
                description.set("Kotlin bindings for iscc-lib - ISO 24138 ISCC")
                url.set("https://github.com/iscc/iscc-lib")
                licenses { license { name.set("Apache-2.0"); url.set("https://www.apache.org/licenses/LICENSE-2.0") } }
                developers { developer { id.set("titusz"); name.set("Titusz Pan") } }
                scm {
                    connection.set("scm:git:git://github.com/iscc/iscc-lib.git")
                    url.set("https://github.com/iscc/iscc-lib")
                }
            }
        }
    }
    repositories {
        maven {
            name = "sonatype"
            url = uri(System.getenv("MAVEN_REPO_URL") ?: "https://repo1.maven.org/maven2/")
            credentials {
                username = System.getenv("MAVEN_USERNAME") ?: ""
                password = System.getenv("MAVEN_PASSWORD") ?: ""
            }
        }
    }
}
```

**Signing** (only when publishing):

```kotlin
signing {
    useGpgCmd()
    sign(publishing.publications["maven"])
}
tasks.withType<Sign>().configureEach {
    onlyIf { gradle.taskGraph.hasTask("publish") }
}
```

**Important Sonatype Central Portal consideration**: The existing Java publish uses
`central-publishing-maven-plugin` which uploads a bundle to `https://central.sonatype.com`. For
Gradle, the recommended approach is either:

1. The `central-portal-plus` Gradle plugin (`tech.yanand.maven-central-publish`)
2. Manual bundle upload via the Central Portal REST API (curl-based)
3. The OSSRH staging approach with `io.github.gradle-nexus.publish-plugin`

The advance agent should research which approach works with the existing `MAVEN_USERNAME` and
`MAVEN_PASSWORD` secrets (these are Sonatype Central Portal token credentials). If the
`central-publishing-maven-plugin` uses the Central Portal API, the Gradle equivalent should too.
Option 1 or 2 is preferred. If using a Gradle plugin, add it to the plugins block. If using
curl-based upload, add a step in the publish job that creates a signed bundle zip and uploads it.

### Cross-compiler setup

The `aarch64-unknown-linux-gnu` target needs `gcc-aarch64-linux-gnu` (same as Java). Copy the
install step pattern from `build-jni`.

## Verification

- `grep -q 'maven-kotlin' .github/workflows/release.yml` exits 0 (input added)
- `grep -q 'build-kotlin-native' .github/workflows/release.yml` exits 0 (build job added)
- `grep -q 'publish-maven-kotlin' .github/workflows/release.yml` exits 0 (publish job added)
- `grep -q 'maven-publish' packages/kotlin/build.gradle.kts` exits 0 (plugin added)
- `grep -q 'signing' packages/kotlin/build.gradle.kts` exits 0 (signing plugin added)
- `grep -q 'io.iscc' packages/kotlin/build.gradle.kts` exits 0 (groupId in publishing block)
- `grep -q 'iscc-lib-kotlin' packages/kotlin/build.gradle.kts` exits 0 (artifactId)
- `cd packages/kotlin && ./gradlew build` passes (publishing config doesn't break local builds)
- `cargo clippy --workspace --all-targets -- -D warnings` clean

## Done When

The advance agent is done when `release.yml` contains a `maven-kotlin` input with corresponding
build/test/publish jobs, `build.gradle.kts` has maven-publish + signing configuration, and all
verification commands pass.
