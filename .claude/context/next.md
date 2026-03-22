# Next Work Package

## Step: Fix JNA ARM32 resource path mismatch (`android-armv7` → `android-arm`)

## Goal

Fix the JNA ARM32 native library resource path so that ARMv7 Android devices can load the native
library at runtime. JNA 5.16.0 canonicalizes ARM32 to `arm` (not `armv7`), so the directory must be
`android-arm/` — verified by bytecode decompilation of `Platform.class`.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml` (line 1026: `android-armv7` → `android-arm`),
    `.claude/context/specs/kotlin-bindings.md` (lines 59, 90: fix table and package structure tree)
- **Reference**: `.claude/context/issues.md` (JNA ARM32 issue description with bytecode evidence),
    `.claude/context/learnings.md` (JNA ARM32 section confirming the correct prefix)

## Not In Scope

- Adding Android-specific install instructions to `docs/howto/kotlin.md` — follow-up step
- Changing the Kotlin release smoke test to validate the assembled JAR — separate issue
- Adding Android emulator CI tests to verify native loading — excessive for a path fix
- Updating any other matrix fields (the `android-abi`, `target`, `lib-name` fields are all correct)
- Fixing other normal issues (XCFramework cache key, Swift ref:main race, root Package.swift CI)

## Implementation Notes

### release.yml change

Line 1026 currently reads:

```yaml
native-dir: android-armv7
```

Change to:

```yaml
native-dir: android-arm
```

This is the only change needed in release.yml. The `target` field (`armv7-linux-androideabi`),
`lib-name` (`libiscc_uniffi.so`), and `android-abi` (`armeabi-v7a`) are all correct. The
`assemble-kotlin` job uses `native-dir` to set the JNA resource directory name inside the JAR.

### Spec changes (kotlin-bindings.md)

1. **Package structure tree** (line 59): Change `android-armv7/` to `android-arm/` and update
    comment to say "JNA resource path for Android ARM32"
2. **Android target table** (line 90): Change `android-armv7/` to `android-arm/` in the JNA Resource
    Dir column

### Why this is correct

JNA 5.16.0's `Platform.getNativeLibraryResourcePrefix()` bytecode shows:
`if (arch.startsWith("arm")) arch = "arm"`, then returns `"android-" + arch` → `android-arm`. Other
Android prefixes (`android-aarch64`, `android-x86-64`, `android-x86`) are unaffected.

## Verification

- `grep 'native-dir: android-arm$' .github/workflows/release.yml` finds exactly 1 match
- `grep -c 'android-armv7' .github/workflows/release.yml` returns 0
- `grep 'android-arm/' .claude/context/specs/kotlin-bindings.md` finds matches (table + tree)
- `grep -c 'android-armv7' .claude/context/specs/kotlin-bindings.md` returns 0
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0
- `mise run format` produces no changes

## Done When

All verification criteria pass — the ARM32 JNA resource path is `android-arm` in both the release
workflow and the Kotlin bindings spec, with no remaining `android-armv7` references.
