## 2026-02-24 — Review of: Rewrite README as public-facing polyglot introduction

**Verdict:** PASS

**Summary:** Clean, well-structured README rewrite that closely follows the iscc-core README
pattern. All 14 required sections are present with accurate content. One minor fix applied: replaced
`---` with Unicode em dash `—` in the tagline for correct rendering on GitHub.

**Verification:**

- [x] `wc -l README.md` >= 80 — 183 lines
- [x] All 9 `gen_*_v0` entry points listed — grep count: 15
- [x] `lib.iscc.codes` documentation link present — count: 2
- [x] Multi-language install instructions (cargo add, pip install, npm install) — count: 4
- [x] Apache-2.0 license declared — count: 1
- [x] @titusz maintainer credited — count: 1
- [x] Zero dev workflow terms (Dev Container, CID Loop, pre-commit, Quality Gates, mise run) —
    count: 0
- [x] ISCC Architecture section present — count: 2
- [x] MainTypes table present — count: 1
- [x] `mise run check` passes — all 14 pre-commit hooks green

**Issues found:**

- (none — minor em dash rendering fix applied directly)

**Next:** All documentation-related target items (docs site, development page, README) are complete.
The next step should address one of the tracked issues in issues.md — the `alg_simhash` panic on
mismatched digest sizes and the `sliding_window` panic on `width < 2` are both normal-priority
correctness issues affecting Tier 1 public API. Alternatively, update-state may identify remaining
target gaps.

**Notes:** The WASM quick start example was reformatted by mdformat to multi-line `import { ... }`
syntax — valid JavaScript, renders correctly on GitHub. HTML comments for future version badges are
preserved by mdformat with blank lines between them. The "not yet published" note in the
Installation section is appropriate since no packages are on crates.io/PyPI/npm yet.
