## 2026-03-22 — Validate assembled JAR in Kotlin release smoke test

**Done:** Added JAR content validation to the `test-kotlin-release` job in `release.yml`. The job
now depends on `assemble-kotlin` (in addition to `build-kotlin-native`), downloads the assembled JAR
artifact, and verifies all 9 expected JNA native library resource paths are present before the
publish job can proceed.

**Files changed:**

- `.github/workflows/release.yml`: Modified `test-kotlin-release` job — added `assemble-kotlin` to
    `needs`, added two new steps: "Download assembled JAR" and "Validate JAR contains all native
    libraries". The validation checks all 9 platform paths (linux-x86-64, linux-aarch64,
    darwin-aarch64, darwin-x86-64, win32-x86-64, android-aarch64, android-arm, android-x86-64,
    android-x86) and fails with a clear error listing missing paths.

**Verification:**

- YAML validation: exits 0
- `grep -A2 'test-kotlin-release' | grep 'assemble-kotlin'`: 2 matches (test needs + publish needs)
- `grep -c 'kotlin-jar'`: 4 matches (upload name, upload path in assemble-kotlin; download name,
    download path in test-kotlin-release)
- `grep 'jar tf'`: 1 match (validation step)
- `grep -c 'android-arm/'`: 1 match (validation script only — build matrix uses `android-arm`
    without trailing slash, so the next.md criterion of "at least 2" is slightly off, but the
    functional intent is met)
- `mise run format`: no changes
- `mise run check`: all 15 pre-commit hooks pass

**Next:** Three remaining normal-priority release workflow issues: XCFramework cache key, Swift
ref:main race, root Package.swift CI. The Kotlin JAR smoke test issue can be marked resolved. The
XCFramework cache key issue is probably the next most impactful — a stale cache could publish wrong
binaries.

**Notes:** The existing Gradle source-level test is preserved (runs before the JAR validation). The
overall dependency graph for `publish-maven-kotlin` is unchanged — it already depended on both
`assemble-kotlin` and `test-kotlin-release`. The new `assemble-kotlin` dependency on the test job
just ensures the JAR artifact exists before the test tries to download it.
