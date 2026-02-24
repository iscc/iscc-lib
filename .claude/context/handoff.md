## 2026-02-24 — Add nav icons to all docs pages

**Done:** Added `icon: lucide/...` and `description:` YAML front matter to all 10 docs pages that
were missing it (index, architecture, benchmarks, development, rust-api, api, howto/rust,
howto/python, howto/nodejs, howto/wasm). Combined with the existing getting-started.md, all 11 docs
pages now have complete front matter matching the iscc-usearch pattern.

**Files changed:**

- `docs/index.md`: Added `icon: lucide/house` + description
- `docs/architecture.md`: Added `icon: lucide/blocks` + description
- `docs/benchmarks.md`: Added `icon: lucide/gauge` + description
- `docs/development.md`: Added `icon: lucide/git-pull-request` + description
- `docs/rust-api.md`: Added `icon: lucide/book-open` + description
- `docs/api.md`: Added `icon: lucide/book-open` + description
- `docs/howto/rust.md`: Added `icon: lucide/cog` + description
- `docs/howto/python.md`: Added `icon: lucide/terminal` + description
- `docs/howto/nodejs.md`: Added `icon: lucide/hexagon` + description
- `docs/howto/wasm.md`: Added `icon: lucide/globe` + description

**Verification:**

- `uv run zensical build` exits 0 — site builds with all pages
- `grep -c '^icon: lucide/' ...` shows 11 files with matches (all 11 pages)
- `grep -c '^description:' ...` shows 11 files with matches (all 11 pages)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no regressions)
- `mise run format` clean (no formatting changes needed)

**Next:** Documentation section is now complete — all pages have nav icons, descriptions, and
content. The highest-value remaining work items are the correctness issues from issues.md:
`alg_simhash` panics on mismatched digest sizes and `sliding_window` panics on width < 2 (both
normal priority). Alternatively, the `docs/rust-api.md` content is outdated (still shows
`IsccResult<String>` return types instead of dedicated result structs).

**Notes:** Used the exact icon assignments from next.md's recommendation table. No content was
modified in any page — only YAML front matter was inserted before the existing `#` heading.
