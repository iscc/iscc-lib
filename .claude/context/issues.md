# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Swift package does not vend the native library `normal` [human]

The Swift package declares `.linkedLibrary("iscc_uniffi")` but does not include or build the native
dylib. Promoted to `normal` — prebuilt XCFramework distribution is now a target outcome. See
`.claude/context/specs/swift-bindings.md` for the full design (XCFramework with `.binaryTarget`,
Ferrostar-style force-update tag, caching, release workflow integration).

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
