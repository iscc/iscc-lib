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

## CI does not exercise root Package.swift `normal` [human]

CI tests only `packages/swift/Package.swift` (the dev manifest). The root `Package.swift` that real
SPM consumers resolve — with its binary target and checksum — is never exercised. Binary-target
regressions (wrong URL pattern, checksum format) can land unnoticed. Low priority because the
release workflow patches the checksum at publish time, but a manifest-resolution smoke check on
macOS CI would add defense in depth.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
