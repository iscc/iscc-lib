## 2026-02-24 — Rewrite README as public-facing polyglot introduction

**Done:** Replaced the development-workflow README with a public-facing polyglot developer
introduction following the iscc-core README structure. The new README includes CI badge, tagline,
key features, ISCC explanation, architecture diagram, MainTypes table, per-language installation
instructions, per-language quick start examples, implementors guide with all 9 `gen_*_v0` entry
points, documentation link, contributing section, Apache-2.0 license, and maintainer credit.

**Files changed:**

- `README.md`: Complete rewrite — replaced dev container/CID workflow content with public-facing
    polyglot introduction modeled after iscc-core README

**Verification:**

- `wc -l README.md` = 183 lines (>= 80 threshold) — PASS
- All 9 `gen_*_v0` entry points listed (grep count: 15, >= 9) — PASS
- `lib.iscc.codes` documentation link present (count: 2) — PASS
- Multi-language install instructions present (cargo add, pip install, npm install; count: 4) — PASS
- Apache-2.0 license declared (count: 1) — PASS
- @titusz maintainer credited (count: 1) — PASS
- Zero dev workflow terms (Dev Container, CID Loop, pre-commit, Quality Gates, mise run; count: 0) —
    PASS
- ISCC Architecture section present (count: 2) — PASS
- MainTypes table present (count: 1) — PASS
- `mise run check` passes — all 14 pre-commit hooks green

**Next:** All documentation-related target items (docs site, development page, README) are now
complete. The next step should address one of the tracked issues in issues.md (e.g., the
`alg_simhash` panic on mismatched digest sizes, or the `sliding_window` panic on `width < 2`), or
continue with any remaining target gaps identified by update-state.

**Notes:** mdformat reformatted the WASM import example to multi-line
(`import { gen_meta_code_v0 } from "@iscc/wasm"` became multi-line with the import on its own line).
This is valid JavaScript and renders correctly on GitHub. The HTML comments for future version
badges are preserved by mdformat with blank lines between them — slightly verbose but harmless.
