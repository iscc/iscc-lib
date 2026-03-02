# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

## #16 — Add feature flags for embedded/minimal builds [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/16

Add Cargo feature flags (`meta-code`, `text-processing`) so embedded consumers can opt out of heavy
dependencies (~82K source lines for serde/unicode). Default behavior unchanged (all features on).
`conformance_selftest()` must adapt to available features (skip disabled code types). CI must test
`--all-features`, `--no-default-features`, and each feature individually.
