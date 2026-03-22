# Next Work Package

## Step: Add Android NDK cross-compilation to Kotlin release workflow

## Goal

Add 4 Android ABI targets to the `build-kotlin-native` matrix in `release.yml` so the published
Kotlin JAR bundles native libraries for Android devices. This directly resolves the critical issue
"Kotlin bindings missing Android native libraries" — without this, the JAR is unusable on Android.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**: `.claude/context/specs/kotlin-bindings.md` (Android target table, cargo-ndk usage),
    `.github/workflows/release.yml` lines 992-1038 (existing `build-kotlin-native` job)

## Not In Scope

- DevContainer Dockerfile changes (Android NDK for local dev) — separate step
- `docs/howto/kotlin.md` Android install instructions — follow-up step after CI is verified
- Android emulator integration tests — resource-path verification is sufficient for now
- Fixing the "Kotlin release smoke test doesn't validate assembled JAR" normal issue — separate
- Dropping `x86` (i686) Android ABI — include all 4 per spec even though x86 is rare
- Changes to `assemble-kotlin` or `publish-maven-kotlin` jobs — they already handle
    `kotlin-native-*` artifacts generically via wildcard pattern

## Implementation Notes

### Matrix entries to add

Add 4 Android entries to the `build-kotlin-native` strategy matrix. All run on `ubuntu-latest`:

| Rust Target               | `native-dir`      | `lib-name`          | `android-abi` |
| ------------------------- | ----------------- | ------------------- | ------------- |
| `aarch64-linux-android`   | `android-aarch64` | `libiscc_uniffi.so` | `arm64-v8a`   |
| `armv7-linux-androideabi` | `android-armv7`   | `libiscc_uniffi.so` | `armeabi-v7a` |
| `x86_64-linux-android`    | `android-x86-64`  | `libiscc_uniffi.so` | `x86_64`      |
| `i686-linux-android`      | `android-x86`     | `libiscc_uniffi.so` | `x86`         |

### NDK setup and build approach

Use `cargo-ndk` for ergonomic cross-compilation. Add conditional steps that only run for Android
matrix entries (detected via `contains(matrix.target, 'android')`):

1. **Setup Android NDK** — use `nttld/setup-ndk@v1` action:

    ```yaml
      - name: Setup Android NDK
        if: contains(matrix.target, 'android')
        uses: nttld/setup-ndk@v1
        id: setup-ndk
        with:
          ndk-version: r27c
          add-to-path: false
    ```

2. **Install cargo-ndk** (binary cached by `Swatinem/rust-cache` in `~/.cargo/bin`):

    ```yaml
      - name: Install cargo-ndk
        if: contains(matrix.target, 'android')
        run: cargo install cargo-ndk
    ```

3. **Split build step** — desktop keeps existing logic, Android uses cargo-ndk:

    ```yaml
      - name: Build UniFFI library
        if: "!contains(matrix.target, 'android')"
        run: cargo build -p iscc-uniffi --release --target ${{ matrix.target }}
        env:
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER: aarch64-linux-gnu-gcc

      - name: Build UniFFI library (Android)
        if: contains(matrix.target, 'android')
        run: cargo ndk --target ${{ matrix.android-abi }} build -p iscc-uniffi
          --release
        env:
          ANDROID_NDK_HOME: ${{ steps.setup-ndk.outputs.ndk-path }}
    ```

    `cargo ndk` outputs to `target/<rust-triple>/release/` (same path convention as desktop), so the
    artifact upload step (`target/${{ matrix.target }}/release/${{ matrix.lib-name }}`) works
    unchanged.

### How existing jobs handle the new artifacts

- **`assemble-kotlin`**: Already copies all `kotlin-native-*` artifacts to JNA resource paths via
    `for dir in kotlin-staging/kotlin-native-*/; do ...`. New `kotlin-native-android-aarch64/` etc.
    artifacts are picked up automatically — no changes needed.
- **`publish-maven-kotlin`**: Same wildcard copy pattern — no changes needed.
- **`test-kotlin-release`**: Only tests `linux-x86-64` — acceptable for now (Android testing is out
    of scope).

### Key constraints

- `android-abi` is a new matrix field needed only by Android entries (maps Rust triple to cargo-ndk
    ABI name). Desktop entries don't need it.
- NDK version `r27c` matches the spec's NDK 27.x recommendation.
- `Swatinem/rust-cache@v2` caches `~/.cargo/bin`, so `cargo-ndk` binary is cached after first run.
- Existing `Install cross-compiler` step for `aarch64-unknown-linux-gnu` stays unchanged (it's
    conditional on that specific target).

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0
- `grep -c 'android' .github/workflows/release.yml` returns 8 or more
- `grep 'aarch64-linux-android' .github/workflows/release.yml` finds a match
- `grep 'armv7-linux-androideabi' .github/workflows/release.yml` finds a match
- `grep 'x86_64-linux-android' .github/workflows/release.yml` finds a match
- `grep 'i686-linux-android' .github/workflows/release.yml` finds a match
- `grep 'android-aarch64' .github/workflows/release.yml` finds a match (JNA resource dir)
- `grep 'cargo-ndk\|cargo ndk' .github/workflows/release.yml` finds a match
- `grep 'setup-ndk' .github/workflows/release.yml` finds a match
- `mise run format` produces no changes

## Done When

All verification criteria pass and the release workflow matrix includes 4 Android ABI targets with
NDK setup and cargo-ndk build steps, with clean formatting.
