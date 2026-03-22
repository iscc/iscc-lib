## 2026-03-22 — Fix JNA ARM32 resource path mismatch (`android-armv7` → `android-arm`)

**Done:** Changed the JNA ARM32 native library resource directory from `android-armv7` to
`android-arm` in the release workflow matrix and the Kotlin bindings spec. JNA 5.16.0 canonicalizes
ARM32 arch to `arm`, so the previous `android-armv7` directory would never be found by JNA on ARMv7
Android devices.

**Files changed:**

- `.github/workflows/release.yml`: Changed `native-dir: android-armv7` to `native-dir: android-arm`
    (line 1026)
- `.claude/context/specs/kotlin-bindings.md`: Updated package structure tree and Android target
    table to use `android-arm/` instead of `android-armv7/` (lines 59, 90)

**Verification:**

- `grep 'native-dir: android-arm$' .github/workflows/release.yml` — 1 match (PASS)
- `grep -c 'android-armv7' .github/workflows/release.yml` — 0 (PASS)
- `grep 'android-arm/' .claude/context/specs/kotlin-bindings.md` — 2 matches in tree and table
    (PASS)
- `grep -c 'android-armv7' .claude/context/specs/kotlin-bindings.md` — 0 (PASS)
- YAML validation via `yaml.safe_load()` — exits 0 (PASS)
- `mise run format` — no changes (PASS)
- `mise run check` — all 15 pre-commit hooks pass (PASS)

**Next:** The JNA ARM32 issue in `issues.md` can be deleted (resolved). Next work could be adding
Android-specific install instructions to `docs/howto/kotlin.md`, or tackling the Kotlin release
smoke test issue (validating the assembled JAR's resource paths).

**Notes:** No surprises. This was a clean 1-line fix in release.yml plus 2-line spec update. The
issue was a spec error (JNA's ARM32 canonicalization behavior was not accounted for when the spec
was written), not an implementation error — the advance agent correctly followed the spec in the
prior iteration.
