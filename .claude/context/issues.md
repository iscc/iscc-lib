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

## JNA ARM32 resource path mismatch: `android-armv7` should be `android-arm` `normal` [review]

**Spec:** `.claude/context/specs/kotlin-bindings.md`

`HUMAN REVIEW REQUESTED`

JNA 5.16.0's `Platform.getNativeLibraryResourcePrefix()` canonicalizes ARM32 architectures to `arm`
(not `armv7`). Bytecode verification of `Platform.class` confirms:
`if (arch.startsWith("arm")) arch = "arm"`, then returns `"android-" + arch` → `android-arm`. The
spec's Android target table says `android-armv7/` for the ARMv7 JNA resource path, and
`release.yml:1027` implements this faithfully. At runtime on ARM32 Android devices, JNA will look
for `android-arm/libiscc_uniffi.so` but the JAR will contain `android-armv7/libiscc_uniffi.so` —
load failure.

Fix: change `native-dir` from `android-armv7` to `android-arm` in (1) the spec table, (2) the spec's
Package Structure tree, (3) `release.yml` matrix, and (4) the advance agent memory's JNA resource
path list.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
