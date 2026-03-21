## 2026-03-21 — Add maven-kotlin release workflow for Kotlin package publishing

**Done:** Added `maven-kotlin` boolean input to `release.yml` with 4 jobs (build-kotlin-native,
assemble-kotlin, test-kotlin-release, publish-maven-kotlin) following the existing Java Maven
Central pattern. Updated `build.gradle.kts` with `maven-publish` and `signing` plugins, POM
metadata, sources/javadoc JARs, and Sonatype Central Portal publishing via bundle upload.

**Files changed:**

- `.github/workflows/release.yml`: Added `maven-kotlin` input + 4 new jobs (~180 lines). Build job
    compiles `iscc-uniffi` for 5 platforms. Assemble job bundles native libs as JNA resources in
    JAR. Test job smoke-tests linux-x86-64. Publish job stages signed artifacts then uploads bundle
    to Central Portal REST API via curl.
- `packages/kotlin/build.gradle.kts`: Added `maven-publish` and `signing` plugins,
    `java { withSourcesJar(); withJavadocJar() }`, publishing block with Maven POM metadata (groupId
    `io.iscc`, artifactId `iscc-lib-kotlin`), local staging repository, and in-memory PGP key
    signing (env-var driven, skipped for local builds).

**Verification:**

- All 7 grep checks pass (maven-kotlin, build-kotlin-native, publish-maven-kotlin, maven-publish,
    signing, io.iscc, iscc-lib-kotlin)
- `cd packages/kotlin && ./gradlew build` passes (tests + JAR + sources + javadoc)
- `./gradlew publishMavenPublicationToStagingRepository` produces correct Maven repo layout (POM +
    JAR + sources + javadoc + checksums)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `mise run check` — all 15 pre-commit hooks pass
- YAML syntax validated via `yaml.safe_load()`

**Next:** The Kotlin bindings issue is now complete (CI + tests + conformance + version sync + docs
\+ release workflow all done). Consider closing issue #18 (Kotlin bindings) and starting on the next
target objective, or performing a release to test the maven-kotlin workflow end-to-end.

**Notes:**

- **Publishing approach:** Chose `useInMemoryPgpKeys` (env var `SIGNING_KEY`/`SIGNING_PASSWORD`)
    over `useGpgCmd()` to avoid GPG system keyring setup in CI. Signing is skipped entirely when env
    vars are absent (local development). This differs slightly from Java (which imports GPG key into
    system keyring) but is functionally equivalent and simpler for Gradle.
- **Central Portal upload:** Uses direct curl-based REST API upload to
    `https://central.sonatype.com/api/v1/publisher/upload` with basic auth from existing
    `MAVEN_USERNAME`/`MAVEN_PASSWORD` secrets. This avoids adding a third-party Gradle plugin and
    matches the authentication mechanism used by the Java `central-publishing-maven-plugin`.
- **JNA resource loading in CI:** Native libs are placed at JNA `Platform.RESOURCE_PREFIX` paths
    (e.g., `linux-x86-64/libiscc_uniffi.so`) inside `src/main/resources/`. JNA discovers these from
    the classpath even though `jna.library.path` points to the non-existent `../../target/debug`
    (JNA falls through path-based search to classpath resource search).
- **Matrix native-dir naming:** Uses JNA conventions (`linux-x86-64`, `darwin-aarch64`,
    `win32-x86-64`) which differ from JNI conventions (`linux-x86_64`, `macos-aarch64`,
    `windows-x86_64`). This is intentional — JNA loads from `Platform.RESOURCE_PREFIX` paths.
