## 2026-03-04 — Review of: Add missing registry badges to global README

**Verdict:** PASS

**Summary:** Clean, minimal change adding RubyGems, Maven Central, and npm @iscc/wasm badges to the
global README. All 7 published registries are now represented. Badge URLs verified against actual
package manifests (gem name `iscc-lib`, Maven `io.iscc:iscc-lib`, npm `@iscc/wasm`).

**Verification:**

- [x] `grep -c 'img.shields.io' README.md` — outputs 6 (not 7 as next.md predicted; Go Reference
    uses `pkg.go.dev/badge` instead of shields.io). All 7 registry badges are present
- [x] `grep -q 'img.shields.io/gem/v/iscc-lib' README.md` — exits 0
- [x] `grep -q 'img.shields.io/maven-central/v/io.iscc/iscc-lib' README.md` — exits 0
- [x] `grep -q 'img.shields.io/npm/v/@iscc/wasm' README.md` — exits 0
- [x] `mise run check` passes — all 15 hooks green

**Issues found:**

- (none)

**Codex review:** No issues found. Codex confirmed the commit is documentation-only, badge URLs are
consistent with package naming, and no correctness or maintainability concerns.

**Next:** The "Ensure all packages have registry badges in global README" issue is resolved and
deleted from issues.md. The remaining `normal`-priority issue is "Add release smoke tests for all
binding publish pipelines." All other issues are `low` priority (CID loop skips). Consider working
on release smoke tests next, or preparing a PR from develop → main if the project is ready for
release.

**Notes:** The next.md verification criterion for `grep -c 'img.shields.io'` predicted 7 (assuming 4
existing shields.io badges + 3 new), but the actual count is 6 because the Go Reference badge uses
`pkg.go.dev/badge`. This is a define-next prediction error, not an implementation problem — all 7
registries have badges. Future define-next steps should verify existing badge services before
predicting counts.
