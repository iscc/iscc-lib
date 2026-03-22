# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## XCFramework release cache key incomplete `normal` [human]

`release.yml:1192` — the XCFramework build cache key only hashes `crates/iscc-*/src/**` and
`Cargo.lock`. Changes to `scripts/build_xcframework.sh`, the Swift headers/modulemap under
`packages/swift/Sources/iscc_uniffiFFI/`, or Cargo.toml feature flags will not invalidate the cache.
This can cause the release job to skip rebuilding and publish a stale XCFramework zip. Fix: expand
the cache key to include all packaging inputs, or remove caching from the release path entirely.

## Swift release job checks out `ref: main` instead of tag SHA `normal` [human]

`release.yml:1181` — the `build-xcframework` job always checks out `ref: main`, even on
tag-triggered releases. If `main` has moved since the tag (concurrent merge, hotfix), the
XCFramework is built from different source than what was tagged, breaking source/binary provenance.
The `ref: main` is needed because the job commits the computed checksum back to main and
force-updates the tag, but this creates a race window. Fix: on tag runs, check out
`${{ github.sha }}` and commit the checksum update to a temporary ref, or add a guard that fails if
`main` HEAD differs from the tag SHA.

## Kotlin release smoke test does not validate assembled JAR `normal` [human]

`release.yml:1079` — `test-kotlin-release` depends on `build-kotlin-native` (raw `.so`/`.dylib`
files) but not `assemble-kotlin`. It runs Gradle tests from source with raw native libs on the
library path, never consuming the assembled JAR. Resource-packaging mistakes (wrong JNA resource
paths, missing platform libs in JAR) can ship undetected. Fix: make the smoke test depend on
`assemble-kotlin`, download the built JAR, verify expected resource paths exist inside it (e.g.,
`linux-x86-64/libiscc_uniffi.so`), and run a consumer-style load test against it.

## CI does not exercise root Package.swift `normal` [human]

CI tests only `packages/swift/Package.swift` (the dev manifest). The root `Package.swift` that real
SPM consumers resolve — with its binary target and checksum — is never exercised. Binary-target
regressions (wrong URL pattern, checksum format) can land unnoticed. Low priority because the
release workflow patches the checksum at publish time, but a manifest-resolution smoke check on
macOS CI would add defense in depth.

## Kotlin bindings missing Android native libraries `critical` [human]

**Spec:** `.claude/context/specs/kotlin-bindings.md`

The Kotlin bindings were created to provide excellent mobile DX for Android developers, but the
release workflow only cross-compiles `iscc-uniffi` for 5 desktop/server targets. No Android ABIs are
built (`aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`,
`i686-linux-android`). The published JAR is unusable on Android — JNA cannot find a native library
for any Android ABI.

Required work:

1. Add Android NDK + Rust Android targets + `cargo-ndk` to devcontainer Dockerfile
2. Add Android NDK cross-compilation to the `build-kotlin-native` matrix in `release.yml` (use
    `cargo-ndk` or manual `CARGO_TARGET_*_LINKER` configuration)
3. Map Android Rust targets to JNA resource paths (`android-aarch64/`, `android-armv7/`,
    `android-x86-64/`, `android-x86/`) in the `assemble-kotlin` job
4. Add Android smoke test (emulator or resource-path verification)
5. Update `docs/howto/kotlin.md` with Android-specific install instructions (JNA AAR dependency)

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
