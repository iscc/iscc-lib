## 2026-03-02 — Review of: Add gen_sum_code_v0 examples to all 6 howto guides

**Verdict:** PASS

**Summary:** Clean documentation-only change adding `### Sum-Code` subsections with
language-appropriate code examples to all 6 per-language howto guides. All examples use correct API
signatures verified against actual binding source code. Style is consistent with adjacent
subsections in each file.

**Verification:**

- [x] `grep -c "### Sum-Code"` across all 6 howto files — each returns 1
- [x] `grep -l "gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0" docs/howto/*.md | wc -l` — returns 6
- [x] `uv run zensical build` — exits 0, docs site builds successfully
- [x] `mise run format` — exits 0, formatting clean
- [x] `mise run check` — 14/14 hooks pass

**Issues found:**

- (none)

**Codex review:** One advisory finding: next.md's verification command
`uv run zensical build 2>&1 | tail -1` could mask build failures due to pipeline exit status.
However, the advance agent ran the build directly and confirmed success — the actual verification
was sound. The `| tail -1` pattern is a define-next convention for concise output, not a quality
gate weakness.

**Next:** Documentation for gen_sum_code_v0 is now complete across all READMEs, docs pages, and
howto guides. The only tracked issue remaining is #16 (feature flags for minimal builds, low
priority). The project may be approaching target completion — the next update-state should evaluate
whether all target.md criteria are met.

**Notes:** This was a straightforward doc-only iteration. All 6 examples follow the existing pattern
(heading, description, code block) and use correct language-specific patterns (WASM takes
`Uint8Array`, Go uses exported fields, Java uses accessor methods, etc.). No code changes, no test
changes, no quality gate changes.
