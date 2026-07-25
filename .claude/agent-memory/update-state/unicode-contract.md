---
name: unicode-contract
description: The declared Unicode 16.0.0 conformance contract + freeze rule — what the spec demands, what is implemented, and how to verify each part
metadata:
  type: project
---

# Unicode data version = declared 16.0.0 + freeze rule

Decided by Titusz 2026-07-25 (commits `8d267ff`, `8358eba`), written into
`.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance contract".
Supersedes the iteration-129 `[review]` issue "Rust core diverges on Unicode 16/17".

**Why:** `text_clean`/`text_collapse` classify code points by general category, so the table version
is part of the output contract. Left unpinned it silently changed ISCCs per runtime: Rust
`unicode-general-category` 1.1.0 = U16 (+`unicode-normalization` 0.1.25 = U17) KEPT characters that
Go (stdlib/x/text U15.0) and `iscc-core` (CPython 3.13 `unicodedata` U15.1) STRIPPED — 5,813
`text_clean` / 5,750 `text_collapse` disagreements; canonical repro `Ɤ` U+A7CB. Divergence from
`iscc-core` on non-16.0 runtimes is now **spec-blessed, not a conformance failure**.

**How to apply:** treat these as three separate, verifiable criteria — none was implemented as of
iteration 133 (re-verified: `utils.rs` still bare live-table lookups, `ls scripts/` still has no
generator, repo-wide grep for the three boundary code points still returns 0 hits).

1. **Freeze rule (unmet).** Code points unassigned in Unicode 16.0.0 must be removed **before any
    normalization or category lookup**, via a vendored range table (731 ranges / 819,533 code
    points) generated from `unicodedata2==16.0.0` by a checked-in generator script. Verify:
    `utils.rs` `is_c_category` (L26-33) and `is_cmp_category` (L38-59) currently do a bare
    live-table `get_general_category`; `ls scripts/` has no generator; `unicodedata2` is absent
    from `pyproject.toml`/`uv.lock`. Makes output invariant under all future table versions, so
    table deps stay freely upgradable — **no dependency change is needed** for the Rust core.
2. **Boundary vectors (unmet).** `U+1FAE9` must be retained, `U+113C5` decomposes, and the
    Unicode-17 `U+20C1` must be stripped — required in the Rust suite **and every binding's
    conformance test**, i.e. cross-cutting over all 12 bindings. Verify with a repo-wide grep for
    the three code points: 0 hits = untouched. The vendored `iscc-core/data.json` vectors all
    predate Unicode 16 and cannot catch this class of drift.
3. **Differential sweep (unmet).** A full-code-space sweep (all 1,112,032 code points, dump
    `text_clean`/`text_collapse` before/after, `diff`) must prove the freeze-rule implementation
    output-equivalent to uniform Unicode 16.0.0 tables. The spec also makes this sweep **mandatory
    on every future Unicode-table dependency bump** — a green vector suite is explicitly not
    sufficient evidence. Costs ~2 minutes; the technique generalizes to any locale/table
    dependency.

**Go is the exposed binding.** Its tables are U15.0 until go1.27 (~Aug 2026, `tables17.0.0.go` is
`//go:build go1.27`), so it cannot satisfy the boundary vectors unaided. The issue prescribes the
choice: vendor the 15.0→16.0 assigned delta (5,813 code points with their 16.0 categories), or skip
the boundary vectors for Go with a tracking note.

**Gates this work will trip** (both bite in CI, not locally — see [[MEMORY]] Quality Gates):

- **iai Ir gate, >10%**: a per-character range lookup added before normalization lands directly in
    the meta/text benches. Prefer binary search over sorted ranges or an ASCII/BMP short-circuit,
    and refresh the affected `.iai-baseline.json` entries in the same step.
- **CRAP `--fail-regression`** (CI-only, not in `mise run check`): the `utils.rs` functions are
    covered, so added branches/loops need a `.crap-baseline.json` refresh in the same step.

**Accepted cost:** inputs containing the 5,185 code points assigned between 15.1 and 16.0
(realistically the 7 Unicode-16 emoji) hash differently than `iscc-core` on CPython ≤ 3.13 produced
historically. Upstream counterpart — same architecture (`unicodedata2` for Python < 3.14 + the
freeze pre-filter + boundary vectors in `data.json`) — filed as
<https://github.com/iscc/iscc-core/issues/137>; per ISO 24138 Annex D the reference implementation
is normative, so the `iscc-core` release adopting it settles the standard's answer.
