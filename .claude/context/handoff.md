## 2026-03-04 — Add missing registry badges to global README

**Done:** Added RubyGems, Maven Central, and npm @iscc/wasm version badges to the global README.md
badge section, so all 7 published registries are represented.

**Files changed:**

- `README.md`: Added 3 shields.io badge lines after the Go Reference badge (lines 10-12)

**Verification:** All verification criteria pass:

- `grep -c 'img.shields.io' README.md` → 6 (3 existing shields.io + 3 new; Go Reference uses
    `pkg.go.dev/badge`, not shields.io, so total is 6 not the predicted 7)
- `grep -q 'img.shields.io/gem/v/iscc-lib' README.md` → exits 0 ✓
- `grep -q 'img.shields.io/maven-central/v/io.iscc/iscc-lib' README.md` → exits 0 ✓
- `grep -q 'img.shields.io/npm/v/@iscc/wasm' README.md` → exits 0 ✓
- `mise run check` → all 15 hooks pass ✓

**Next:** The "Ensure all packages have registry badges in global README" issue is resolved. The
remaining normal-priority issue is "Add release smoke tests for all binding publish pipelines." All
other issues are low priority. Consider working on release smoke tests or creating a PR from develop
→ main.

**Notes:** The next.md verification criterion predicted `grep -c 'img.shields.io'` would output 7,
but the actual count is 6 because the Go Reference badge uses `pkg.go.dev/badge` (not shields.io).
All 7 registry badges are present — the count discrepancy is just the badge service difference.
