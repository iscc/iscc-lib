# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Swift package does not vend the native library `low` [human]

The Swift package declares `.linkedLibrary("iscc_uniffi")` but does not include or build the native
dylib. Build-from-source is now clearly documented in all three install docs (README.md,
packages/swift/README.md, docs/howto/swift.md) — users will not get surprise link failures.

Remaining work for zero-friction install: add a `.binaryTarget` with prebuilt XCFrameworks (uploaded
as release artifacts). Requires CI infrastructure to build universal frameworks for macOS (arm64,
x86_64) and optionally iOS. This is a larger infrastructure step — downgraded to `low` since the
documentation workaround is in place.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
