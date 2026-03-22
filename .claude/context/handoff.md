## 2026-03-22 — Review of: Add Android NDK cross-compilation to Kotlin release workflow

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, well-scoped CI change that adds 4 Android ABI targets to the Kotlin release
workflow exactly as specified in next.md and the Kotlin bindings spec. The implementation correctly
uses `cargo-ndk` with NDK r27c and conditional build steps. All verification criteria pass. However,
Codex review uncovered a JNA resource path mismatch for ARM32 that exists in the spec itself — filed
as a new issue.

**Verification:**

- [x] `python3 -c "import yaml; yaml.safe_load(...)"` exits 0 — YAML valid
- [x] `grep -c 'android' .github/workflows/release.yml` returns 17 (≥8 required)
- [x] `grep 'aarch64-linux-android'` — match found
- [x] `grep 'armv7-linux-androideabi'` — match found
- [x] `grep 'x86_64-linux-android'` — match found
- [x] `grep 'i686-linux-android'` — match found
- [x] `grep 'android-aarch64'` — match found (JNA resource dir)
- [x] `grep 'cargo-ndk\|cargo ndk'` — match found
- [x] `grep 'setup-ndk'` — match found
- [x] `mise run format` — no changes
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- JNA ARM32 resource path mismatch: spec says `android-armv7/` but JNA 5.16.0's
    `getNativeLibraryResourcePrefix()` canonicalizes ARM32 to `android-arm/`. Verified by
    decompiling `Platform.class` from the JNA JAR — the bytecode shows
    `if (arch.startsWith("arm")) arch = "arm"`, then returns `"android-" + arch`. This means the
    ARMv7 native lib will be placed in a directory JNA won't search on ARM32 devices. Filed as new
    issue. The advance agent correctly followed the spec, so this is a spec error, not an
    implementation error.

**Codex review:** One P2 finding — confirmed valid after bytecode verification:

1. **[P2] `android-armv7` → `android-arm` JNA prefix mismatch**: JNA canonicalizes ARM32 arch to
    `arm`, so the resource prefix should be `android-arm/`, not `android-armv7/`. The spec and
    implementation both use `android-armv7/`, which means ARMv7 Android devices won't find the
    native library. Requires spec fix + implementation update (1-line change in release.yml
    matrix).

**Next:** Fix the JNA ARM32 resource path (`android-armv7` → `android-arm`) in both the spec and
release.yml. This is a 1-line fix in the matrix + spec update. Then continue with Kotlin Android
docs update (`docs/howto/kotlin.md` Android install instructions).

**Notes:**

- The `assemble-kotlin` job handles new Android artifacts automatically via its wildcard pattern
    (`kotlin-staging/kotlin-native-*/`) — no changes needed there.
- The critical issue "Kotlin bindings missing Android native libraries" is resolved — the next
    release will bundle Android native libs in the JAR. Deleted the critical issue entry.
- Remaining follow-up from the original issue (devcontainer Dockerfile, docs update) is organic
    next-step work, not blocking.
- The GITHUB_REF_NAME bug referenced in agent memory was fixed (commit d29a1b3 per state.md). The
    remaining Swift issue in issues.md is about a different concern (`ref: main` checkout race).
