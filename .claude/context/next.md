# Next Work Package

## Step: Validate assembled JAR in Kotlin release smoke test

## Goal

Make the `test-kotlin-release` job in `release.yml` depend on `assemble-kotlin` and validate that
the assembled JAR contains native libraries for all 9 expected JNA resource paths. This catches
resource-packaging mistakes (wrong directory names, missing platform libs) before publishing — the
kind of bug the ARM32 path mismatch (`android-armv7` → `android-arm`) exemplified.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml` (the `test-kotlin-release` job)
- **Reference**:
    - `.github/workflows/release.yml` lines 991–1135 (build-kotlin-native matrix, assemble-kotlin,
        test-kotlin-release)
    - `.claude/context/issues.md` (the "Kotlin release smoke test does not validate assembled JAR"
        issue)

## Not In Scope

- Changing the `assemble-kotlin` job itself — it works correctly
- Adding a consumer-project integration test (running Java/Kotlin against the JAR classpath) — too
    complex for this step
- Fixing the other 3 normal release workflow issues (XCFramework cache key, Swift ref:main, root
    Package.swift CI)
- Removing the existing Gradle source-level test — keep it for functional validation alongside the
    new structural check

## Implementation Notes

The `test-kotlin-release` job currently depends only on `build-kotlin-native` and tests with a
manually-placed raw native library for linux-x86-64 only. It never validates the actual assembled
JAR that gets published.

**Changes to `test-kotlin-release`:**

1. **Change `needs`** from `[build-kotlin-native]` to `[build-kotlin-native, assemble-kotlin]`
2. **Add JAR content validation steps** after the existing Gradle test:
    - Download the `kotlin-jar` artifact (uploaded by `assemble-kotlin`)
    - Use `jar tf` or `unzip -l` to list JAR contents
    - Verify all 9 expected native library resource paths are present:
        - `linux-x86-64/libiscc_uniffi.so`
        - `linux-aarch64/libiscc_uniffi.so`
        - `darwin-aarch64/libiscc_uniffi.dylib`
        - `darwin-x86-64/libiscc_uniffi.dylib`
        - `win32-x86-64/iscc_uniffi.dll`
        - `android-aarch64/libiscc_uniffi.so`
        - `android-arm/libiscc_uniffi.so`
        - `android-x86-64/libiscc_uniffi.so`
        - `android-x86/libiscc_uniffi.so`
    - Fail the job if any expected path is missing

The validation script should be a straightforward shell loop that checks
`jar tf $JAR | grep "$path"` for each expected path, collecting any missing paths and failing with a
clear error message listing what's absent.

**Note:** The publish job (`publish-maven-kotlin`) already depends on both `assemble-kotlin` and
`test-kotlin-release`, so adding `assemble-kotlin` to the test's `needs` doesn't change the overall
dependency graph for publishing — it just ensures the test runs after assembly completes.

## Verification

- `python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -A2 'test-kotlin-release' .github/workflows/release.yml | grep 'assemble-kotlin'` finds a
    match (dependency added)
- `grep 'kotlin-jar' .github/workflows/release.yml` returns at least 2 matches (upload in
    assemble-kotlin + download in test-kotlin-release)
- `grep -c 'android-arm/' .github/workflows/release.yml` returns at least 2 (build matrix +
    validation)
- `grep 'jar tf\|unzip -l' .github/workflows/release.yml` finds the JAR inspection command
- `mise run format` produces no changes
- `mise run check` passes all pre-commit hooks

## Done When

All verification criteria pass, confirming that `test-kotlin-release` structurally validates the
assembled JAR's native library resource paths for all 9 platforms before the publish job can
proceed.
