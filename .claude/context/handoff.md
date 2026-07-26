# Handoff

## 2026-07-26 — Document the declared Unicode 16.0.0 data version for library consumers

**Done:** Created `docs/unicode.md` ("Text Processing and Unicode"), a user-facing Explanation page
covering the declared Unicode 16.0.0 data version, the freeze rule, the four boundary vectors, and
the two known divergences (iscc-core on CPython ≤ 3.13, pure-Go package on Go's 15.0 tables). Wired
it into the site nav, `ORDERED_PAGES`, `docs/llms.txt`, and added a Unicode-tables admonition to
`docs/howto/go.md` linking to the new page.

**Files changed:**

- `docs/unicode.md`: new page — intro, `## Why the Unicode version matters`,
    `## Declared version and freeze rule` (731 ranges / 819,533 code points / generator script),
    `## Boundary behaviour` (4-row table matching `unicode_boundary.json`, real glyphs for the two
    assigned code points in prose, `U+XXXX` labels in the table),
    `## Cross-implementation consistency` (`!!! warning` for iscc-core ≤ 3.13 with issue-137 link,
    `!!! note` for pure-Go), `## Practical guidance`
- `zensical.toml`: added `{ "Text and Unicode" = "unicode.md" }` to `Explanation` nav after
    Architecture
- `scripts/gen_llms_full.py`: added `"unicode.md"` to `ORDERED_PAGES` directly after
    `"architecture.md"`
- `docs/llms.txt`: one `## Reference` bullet for the new page, after Architecture
- `docs/howto/go.md`: `!!! note "Unicode tables"` in the Text utilities section — Go 15.0 tables, no
    freeze rule, link to `../unicode.md`

**Verification:** All next.md criteria pass, observed in this session:

- `uv run zensical build` → exit 0, "No issues found"
- `uv run python scripts/gen_llms_full.py` → exit 0, 23 pages, **no** `Auto-discovered` and **no**
    `Warning:` line
- `grep -q "16.0.0" site/llms-full.txt` → OK
- `grep -E -c 'U\+(1FAE9|113C5|20C1|A7F1)' docs/unicode.md` → **8** (≥ 4 required)
- `grep -q "iscc-core/issues/137" docs/unicode.md` → OK
- `grep -q "unicode.md" docs/llms.txt` → OK; `grep -q "Unicode 15.0" docs/howto/go.md` → OK
- Boundary table cross-checked against a dump of `crates/iscc-lib/tests/unicode_boundary.json`: all
    4 rows agree (U+1FAE9 retained/retained; U+113C5 retained/`"ab"`; U+20C1 and U+A7F1
    `"ab"`/`"ab"`)
- `git status --porcelain crates/ packages/go/*.go` → prints nothing (no source/fixture change)
- `uv run prek run --files` over the five touched files → every applicable hook `Passed` (mdformat
    reflowed once on first run; second run clean)
- `uv run ruff check` → "All checks passed!"
- `mise run check` → exit 0, all 16 hooks `Passed`; only runner-owned `iterations.jsonl` dirty
    afterwards (left unstaged)

**Next:** The natural follow-up from the previous review is still open: extend
`scripts/check_release_workflow.py` with an opt-in `--check-action-inputs` mode (network-fetching,
offline-skipping, out of the pytest suite) plus a `ci.yml` step. Alternatively, everything
Unicode-related beyond documentation remains parked on Titusz (freeze-rule ordering ruling, Go
15.0-tables decision).

**Notes:**

- Constraint compliance: the page makes **no** equivalence claim to uniform Unicode 16.0.0 tables
    (spec criterion 4 wording is under human review) and stays strictly on single code points — no
    composition/Hangul/`Final_Sigma` statements. No checkbox in `specs/rust-core.md` was ticked; no
    edit to `issues.md`.
- The two *assigned* boundary characters (🫩 U+1FAE9, 𑏅 U+113C5) appear as real glyphs in prose below
    the table; the table cells themselves use ASCII `U+XXXX` labels to keep mdformat table alignment
    stable, per next.md's gotcha. The two *unassigned* code points are labels only (they would
    render as tofu).
- `site/` is gitignored — the regenerated `llms-full.txt` is build output, not committed.
- Doc-only step: no Rust/Python runtime code changed, no baselines refreshed (CRAP, iai untouched by
    design).
