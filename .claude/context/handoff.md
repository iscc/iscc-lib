## 2026-03-22 — Add Android NDK cross-compilation to Kotlin release workflow

**Done:** Added 4 Android ABI targets (aarch64, armv7, x86_64, i686) to the `build-kotlin-native`
matrix in `release.yml`. Added conditional NDK setup (`nttld/setup-ndk@v1` with r27c) and
`cargo-ndk` install steps, plus split the build step into desktop (existing) and Android
(`cargo ndk`) variants.

**Files changed:**

- `.github/workflows/release.yml`: Added 4 Android matrix entries with `android-abi` field, 3 new
    conditional steps (Setup Android NDK, Install cargo-ndk, Build UniFFI library (Android)), and
    added `if` condition to existing Build step to skip Android targets.

**Verification:**

- YAML valid: `python3 -c "import yaml; yaml.safe_load(...)"` exits 0
- `grep -c 'android'` returns 17 (>= 8 required)
- All 4 Rust targets found: `aarch64-linux-android`, `armv7-linux-androideabi`,
    `x86_64-linux-android`, `i686-linux-android`
- `android-aarch64` JNA resource dir found
- `cargo-ndk` / `cargo ndk` references found
- `setup-ndk` references found
- `mise run format` — no changes
- `mise run check` — all 15 pre-commit hooks pass

**Next:** The `assemble-kotlin` and `publish-maven-kotlin` jobs already handle new artifacts via
wildcard patterns — no changes needed there. Next logical step would be DevContainer Dockerfile
changes (Android NDK for local dev) or docs updates for Android installation instructions.

**Notes:**

- The `android-abi` matrix field is only used by Android entries — desktop entries don't set it,
    which is fine since `cargo ndk` step is conditional on `contains(matrix.target, 'android')`.
- `cargo ndk` outputs to `target/<rust-triple>/release/` (same path as desktop builds), so the
    artifact upload step works unchanged.
- The existing `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER` env var on the desktop build step is
    harmless for non-aarch64 desktop targets (it's only used when the env var name matches the
    target).
