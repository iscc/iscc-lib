## 2026-03-02 — Review of: Update docs for 4-parameter gen_sum_code_v0

**Verdict:** PASS

**Summary:** Clean, minimal documentation update across all 5 files specified in next.md. Every
`gen_sum_code_v0` reference now includes the `add_units` parameter and `units` result field. The
documented signatures exactly match the actual source code in `crates/iscc-lib/src/lib.rs` and
`crates/iscc-ffi/include/iscc.h`.

**Verification:**

- [x] `grep -n 'gen_sum_code_v0.*bits.*wide' docs/*.md docs/howto/*.md | grep -v add_units` — no
    matches (all signatures include `add_units`) ✅
- [x] `grep -c 'add_units' docs/rust-api.md` — 3 (≥2 required) ✅
- [x] `grep -c 'add_units' docs/c-ffi-api.md` — 2 (≥2 required) ✅
- [x] `grep 'units' docs/c-ffi-api.md | grep -c 'char \*\*'` — 1 (≥1 required) ✅
- [x] `uv run zensical build` — exits 0, docs site builds cleanly ✅
- [x] `mise run check` — 14/14 pre-commit hooks pass ✅
- [x] Scope discipline — only 5 docs files modified, nothing from "Not In Scope" touched ✅
- [x] No quality gate circumvention ✅

**Issues found:**

- (none)

**Codex review:** Codex reviewed the define-next commit (HEAD~1 targets define-next, not advance —
known protocol issue). Correctly identified it as context-only changes with no code impact. No
actionable findings.

**Next:** Documentation section should now be fully "met" — update-state should confirm. The only
remaining open issue is #16 (feature flags for embedded/minimal builds). The CID loop should verify
state and define the next work package for issue #16.

**Notes:** This was a docs-only iteration with no source code changes. The mdformat pre-commit hook
reformatted table alignment in rust-api.md and c-ffi-api.md (trailing whitespace normalization),
which is expected behavior. Issue #21 (add_units/units across all bindings + docs) is now fully
complete.
