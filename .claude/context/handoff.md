## 2026-02-28 — Review of: Convert getting-started tutorial to tabbed multi-language examples

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean docs-only change that converts the getting-started tutorial from Python-only to
tabbed multi-language format with 7 tabbed sections × 6 languages = 42 tabs. Code snippets are
accurate (cross-checked against howto guides), the narrative structure is preserved, and the JSON
serialization tip correctly remains Python-only. Minor fix applied: Java version corrected from 11+
to 17+ (pom.xml requires 17).

**Verification:**

- [x] `grep -c '=== "Python"' docs/tutorials/getting-started.md` returns 7 (≥ 5)
- [x] `grep -c '=== "Rust"' docs/tutorials/getting-started.md` returns 7 (≥ 5)
- [x] `grep -c '=== "Go"' docs/tutorials/getting-started.md` returns 7 (≥ 5)
- [x] `uv run zensical build` exits 0 — site builds cleanly
- [x] `mise run format` produces no changes
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- Java prerequisite version said "11+" but pom.xml requires Java 17+ — fixed directly
- Landing page Go code (`docs/index.md` lines 114-122) uses stale WASM-bridge API
    (`NewRuntime`/`ctx`) instead of pure Go API — filed as issue
- Tab order inconsistency: spec says "Python, Rust, Java, Node.js, WASM", landing page uses "Rust,
    Python, ...", tutorial uses "Python, Rust, Node.js, Java, Go, WASM" — filed as issue
- WASM examples after selftest omit `await init()` — minor inconsistency (selftest shows it, other 5
    WASM tabs don't). Not blocking since snippets are illustrative, but could confuse readers

**Codex review:** Two actionable findings: (1) Java version mismatch (11+ vs pom.xml 17+) — fixed.
(2) WASM `init()` inconsistency across examples — noted but not blocking (see issues above). Codex
also explored the full file structure and found no other issues.

**Next:** Fix the stale Go example on the landing page (`docs/index.md`) and standardize tab order
across landing page and tutorial. Both are small, well-scoped changes that could be done in one
iteration. The landing page Go fix is straightforward — replace the WASM-bridge pattern with the
pure Go API pattern from the howto guide.

**Notes:** The doc spec (`specs/documentation.md`) tab order doesn't include Go (was written before
Go bindings existed). If Go is included in the standard tab order, the spec should be updated. The
advance agent's observation about this inconsistency is valid. All 7 bindings remain at 30/30, CI
has 8 green jobs, PR #10 is open.
