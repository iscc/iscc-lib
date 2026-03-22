# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## CI does not exercise root Package.swift `normal` [human]

CI tests only `packages/swift/Package.swift` (the dev manifest). The root `Package.swift` that real
SPM consumers resolve — with its binary target and checksum — is never exercised. Binary-target
regressions (wrong URL pattern, checksum format) can land unnoticed. Low priority because the
release workflow patches the checksum at publish time, but a manifest-resolution smoke check on
macOS CI would add defense in depth.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
