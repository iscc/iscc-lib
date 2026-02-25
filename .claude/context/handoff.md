## 2026-02-25 — Add Java JNI cross-platform build to release workflow

**Done:** Added `build-jni` and `assemble-jar` jobs to `release.yml` with a 5-platform matrix
(linux-x86_64, linux-aarch64, macos-aarch64, macos-x86_64, windows-x86_64) matching the NativeLoader
path conventions. Also added a `maven` boolean input to workflow_dispatch for manual triggering.

**Files changed:**

- `.github/workflows/release.yml`: Added `maven` input, `build-jni` job (5-platform matrix with
    cross-compilation for aarch64-linux), and `assemble-jar` job (downloads artifacts, copies native
    libs to `META-INF/native/{os}-{arch}/`, runs `mvn package -DskipTests`, uploads JAR)

**Verification:**

- YAML validation: `python3 -c "import yaml; yaml.safe_load(...)"` exits 0
- `grep -c 'build-jni'` returns 2 (job name + needs reference)
- `grep -c 'assemble-jar'` returns 1 (job definition)
- `grep 'inputs.maven'` finds both condition lines (build-jni and assemble-jar)
- `grep 'META-INF/native'` finds the resource path in copy step
- `grep -c 'jni-'` returns 5 (artifact name template + download pattern + 3 in copy script)
- Matrix has exactly 5 entries (verified via Python YAML parsing)
- `mise run check` passes: all 14 pre-commit hooks clean

**Next:** Maven Central publishing job (requires GPG signing + Sonatype credentials — human-gated
setup). Alternatively, `version_sync.py` could be extended to handle Maven SNAPSHOT versioning. The
CID loop is otherwise in maintenance mode with all core functionality complete.

**Notes:** The `assemble-jar` job uses a shell loop to iterate over downloaded artifact directories
(`jni-staging/jni-*/`), extracting the `native-dir` name by stripping the `jni-` prefix. This relies
on `actions/download-artifact@v4` creating per-artifact subdirectories when `merge-multiple` is not
set (default `false`). The `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER` env var is set on all
matrix entries but only takes effect for the aarch64 target — same pattern as the existing
`build-napi` job.
