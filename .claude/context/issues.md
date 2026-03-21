# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Swift release job `--ref main` re-trigger incompatible `normal` [review]

The `build-xcframework` job uses `GITHUB_REF_NAME` for version extraction and tag operations, but
all other release jobs derive version from `Cargo.toml`. When re-triggered with
`gh workflow run release.yml --ref main -f swift=true`, `GITHUB_REF_NAME` is `main` (not a version
tag), causing `sed` to set `releaseTag = "main"` and `git tag -fa main` to corrupt the repo. Fix:
derive version from `Cargo.toml` (like other jobs) and construct tag name as `v$VERSION`.

**Spec:** `.claude/context/specs/swift-bindings.md` section "Release Workflow Integration"

`HUMAN REVIEW REQUESTED` — spec explicitly uses `GITHUB_REF_NAME`; fix requires spec update.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
