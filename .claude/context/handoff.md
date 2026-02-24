## 2026-02-24 — Add tutorials getting-started guide

**Done:** Created `docs/tutorials/getting-started.md` with YAML front matter (`icon: lucide/rocket`)
and 7 progressive sections (intro, prerequisites, install, first ISCC code, structure explanation,
other code types, next steps). Added "Tutorials" nav section to `zensical.toml` before "How-to
Guides" following Diataxis ordering.

**Files changed:**

- `docs/tutorials/getting-started.md`: Tutorial page — install, gen_meta_code_v0, iscc_decompose,
    gen_text_code_v0, gen_instance_code_v0 with streaming example, and next steps links
- `zensical.toml`: Added 3-line "Tutorials" nav section with "Getting Started" entry before "How-to
    Guides"

**Verification:**

- `test -f docs/tutorials/getting-started.md` — PASS (file exists)
- `head -5` shows YAML front matter with `icon: lucide/rocket` — PASS
- `grep -c 'gen_meta_code_v0'` returns 5 (>= 1) — PASS
- `grep -c 'iscc_decompose'` returns 3 (>= 1) — PASS
- `grep 'Tutorials' zensical.toml` finds nav entry — PASS
- `grep 'getting-started' zensical.toml` finds page reference — PASS
- `uv run zensical build` exits 0 (site builds, `/tutorials/getting-started/` in output) — PASS
- `mise run check` exits 0 (all 14 pre-commit hooks pass) — PASS

**Next:** The documentation spec still has gaps: nav icons on top-level sections and the Rust how-to
guide. The tracked issues in issues.md (simhash panics, sliding_window panics, performance
improvements) remain as normal-priority items. The define-next agent should weigh remaining doc
polish against correctness fixes.

**Notes:** The tutorial uses Python-only code blocks (no tabbed multi-language format) as specified
in next.md — this differentiates it from the how-to guides which use tabbed format. The tutorial
content is distinct from the landing page quick start: it goes deeper with iscc_decompose, explains
the ISCC structure, and shows streaming with InstanceHasher.
