## 2026-02-24 — Review of: Add tutorials getting-started guide

**Verdict:** PASS

**Summary:** Clean documentation-only change that adds a getting-started tutorial with all 7
required sections and a nav entry in zensical.toml. The tutorial is well-structured, uses
Python-only code blocks as specified, and differentiates itself from the landing page quick start by
going deeper with `iscc_decompose`, ISCC structure explanation, and streaming with `InstanceHasher`.
All quality gates pass.

**Verification:**

- [x] `test -f docs/tutorials/getting-started.md` exits 0 — file exists
- [x] `head -5` shows YAML front matter with `icon: lucide/rocket` — correct
- [x] `grep -c 'gen_meta_code_v0'` returns 5 (>= 1)
- [x] `grep -c 'iscc_decompose'` returns 3 (>= 1)
- [x] `grep 'Tutorials' zensical.toml` finds nav entry
- [x] `grep 'getting-started' zensical.toml` finds page reference
- [x] `uv run zensical build` exits 0 — `/tutorials/getting-started/` in output
- [x] `mise run check` exits 0 — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** The documentation spec still has two gaps: (1) nav icons on top-level sections and (2) the
Rust how-to guide. However, the tracked issues in issues.md (simhash panics, sliding_window panics,
performance improvements for sliding_window/codec/DataHasher) are normal-priority correctness and
performance items that deliver more value than remaining doc polish. The define-next agent should
prioritize correctness fixes over documentation gaps.

**Notes:** The tutorial correctly uses `InstanceHasher` (not `DataHasher`) for the streaming
Instance-Code example — this matches the API semantics. The "Other languages" tip mentions Rust
without linking to a Rust how-to page (which doesn't exist yet), using plain text instead of a
broken link. The Tutorials nav section is correctly positioned before How-to Guides per Diataxis
ordering.
