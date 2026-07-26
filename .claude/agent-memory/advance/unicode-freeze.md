---
name: unicode-freeze
description: Unicode 16.0.0 freeze rule — U+FFFF sentinel map design, table regen, boundary fixture, pending work
metadata:
  type: project
---

# Unicode 16.0.0 freeze rule (sentinel map, iter 148, RULED)

- `text_clean`/`text_collapse` MAP 16.0-unassigned code points to `UNASSIGNED_SENTINEL` (`U+FFFF`,
    permanent noncharacter: forever `Cn`, `ccc = 0`, no decomposition) inside the fused iterator
    BEFORE normalization. NOT a delete-filter (iter-133 design failed 42/140 sequence cases —
    changes adjacency, unblocks composition/`Final_Sigma`) and NOT a category override (`U+A7F1`
    decomposes to `S` under Unicode 17 tables before the filter sees it).
- The category filters (`is_c_category`/`is_cmp_category`) stay exactly as the reference defines
    them — `U+FFFF` is `Cn`, so they remove the sentinel where the reference removes unassigned code
    points. **Why:** IEP-0003 conformance = output-equivalence with the reference; this preserves
    composition-blocking + `Final_Sigma` context while making output table-version invariant.
    Authority: `specs/rust-core.md` requirement 1; `decisions.md` 2026-07-26.
- Table: `crates/iscc-lib/src/utils/unicode16.rs` (731 ranges, 819,533 cps; regen:
    `uv run --script scripts/gen_unicode16_unassigned.py` — PEP 723, pins `unicodedata2==16.0.0`;
    data output is docstring-independent).
- Boundary fixture (iter 141): `crates/iscc-lib/tests/unicode_boundary.json` (4 code points × 2
    sections) + `tests/test_unicode_boundary.rs` — propagation source for bindings.
- Six sentinel regression tests live in `utils.rs` `mod tests` (composition-block, jamo, final
    sigma, decomposition leak, normalizer pass-through, literal U+FFFF).
- User-facing page: `docs/unicode.md` (sentinel mechanism + "How much does this matter?").
- Pending: sequence vectors in the fixture, binding propagation, full-code-space + sequence-class
    differential sweep (spec requirement 4).
