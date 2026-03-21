# Next Work Package

## Step: Fix Kotlin CI — wrong Gson Maven groupId

## Goal

Fix the Kotlin CI failure (`Could not find com.google.gson:gson:2.8.9`) by correcting the Maven
groupId from `com.google.gson` to `com.google.code.gson` and upgrading to 2.11.0 to match the Java
binding.

## Scope

- **Create**: (none)
- **Modify**: `packages/kotlin/build.gradle.kts` (fix Gson coordinates)
- **Reference**: `crates/iscc-jni/java/pom.xml` (correct Gson coordinates:
    `com.google.code.gson:gson:2.11.0`)

## Not In Scope

- Switching from Gson to kotlinx-serialization — Gson works fine with the correct groupId and
    matches the Java binding's choice
- Adding `gradle/actions/setup-gradle@v4` to CI — not needed; the dependency resolution failure is
    purely a wrong coordinate, not a Gradle setup issue
- Kotlin documentation, README, release workflow — those come after CI is green
- Modifying the conformance test code — the Java import `com.google.gson.*` is the Java *package*
    name (correct); only the Maven *artifact* coordinate in build.gradle.kts is wrong

## Implementation Notes

**Root cause**: The Kotlin `build.gradle.kts` declares
`testImplementation("com.google.gson:gson:2.8.9")` but Gson's Maven groupId is
`com.google.code.gson`, not `com.google.gson`. The Java import statements use `com.google.gson.*`
(Java package name) which is different from Maven coordinates. The Java binding's `pom.xml`
correctly uses `com.google.code.gson:gson:2.11.0`.

**Fix**: In `packages/kotlin/build.gradle.kts`, change line 16 from:

```kotlin
testImplementation("com.google.gson:gson:2.8.9")
```

to:

```kotlin
testImplementation("com.google.code.gson:gson:2.11.0")
```

This is a 1-line fix. No other files need changes. The Kotlin test source code imports
`com.google.gson.JsonParser` etc. — those are Java package names and remain correct.

## Verification

- `cd packages/kotlin && ./gradlew compileTestKotlin` succeeds (dependency resolves from Maven
    Central)
- `cd packages/kotlin && ./gradlew test` passes all 9 conformance tests (requires
    `cargo build -p iscc-uniffi` first)
- `grep 'com.google.code.gson:gson:2.11.0' packages/kotlin/build.gradle.kts` finds the corrected
    dependency

## Done When

All verification criteria pass — `./gradlew test` succeeds with the corrected Gson dependency,
confirming the CI failure was caused by the wrong Maven groupId.
