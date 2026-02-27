## 2026-02-27 — Review of: Update Go docs for 30/30 Tier 1 symbols

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, well-structured documentation update adding "Codec operations" and "Constants"
sections to `docs/howto/go.md` and replacing stale "planned" text in `packages/go/README.md` with
organized API tables covering all 30/30 Tier 1 symbols. One minor doc bug fixed during review
(incorrect prefix claim in decompose example).

**Verification:**

- [x] `grep -c 'EncodeComponent' docs/howto/go.md` → 1
- [x] `grep -c 'IsccDecode' docs/howto/go.md` → 2
- [x] `grep -c 'IsccDecompose' docs/howto/go.md` → 1
- [x] `grep -c 'JsonToDataUrl' docs/howto/go.md` → 1
- [x] `grep -c 'EncodeBase64' docs/howto/go.md` → 1
- [x] `grep -c 'SoftHashVideoV0' docs/howto/go.md` → 1
- [x] `grep -c 'MetaTrimName' docs/howto/go.md` → 1
- [x] `grep 'are planned' packages/go/README.md` → no matches (stale text removed)
- [x] `grep -c 'EncodeComponent' packages/go/README.md` → 1
- [x] `grep -c 'DataHasher' packages/go/README.md` → 2
- [x] `uv run zensical build` → exits 0
- [x] `mise run check` → all 14 hooks pass

**Issues found:**

- Minor: Decompose example comment said "Each unit code with 'ISCC:' prefix" but `iscc_decompose`
    returns units WITHOUT the prefix (verified via Rust docstring and Go test). Fixed in review.

**Codex review:** Codex identified the incorrect "ISCC:" prefix claim in the decompose example
comment — confirmed and fixed. No other actionable findings.

**Next:** All 6 language bindings have 30/30 Tier 1 symbols documented. The project can move on to
other target milestones — likely the remaining issues in issues.md: dict auto-serialization for
`gen_meta_code_v0` (issue #5), or adding codec/constants documentation sections to Java and Python
howto guides for cross-language parity.

**Notes:** The `SoftHashVideoV0` function is documented under "Codec operations" rather than
"Algorithm primitives" — this follows the next.md spec but is a slight taxonomic mismatch. Not worth
changing now; if howto guides get a structural refresh later, it could be moved. Java and Python
howto guides don't yet have codec/constants sections — Go is the first binding with full doc
coverage.
