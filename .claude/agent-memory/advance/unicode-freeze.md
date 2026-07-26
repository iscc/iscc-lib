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
- Boundary fixture (iter 141, sequences iter 149): `crates/iscc-lib/tests/unicode_boundary.json` (7
    `text_clean` + 5 `text_collapse` cases: 4 single code points per section + 4 sequence vectors) +
    `tests/test_unicode_boundary.rs` (`SEQUENCE_VECTORS` const pins
    input/expected/delete-filter-output in source; 2 ungated guards + 2 gated vector tests with
    counts derived `4 + rows`) — propagation source for bindings.
- Six sentinel regression tests live in `utils.rs` `mod tests` (composition-block, jamo, final
    sigma, decomposition leak, normalizer pass-through, literal U+FFFF).
- User-facing page: `docs/unicode.md` (sentinel mechanism + "How much does this matter?" + both
    boundary-vector tables).
- Pending: binding propagation (11 bindings + 4 sibling data.json copies; Go skips table-dependent
    vectors per ruling but takes `Final_Sigma`), full-code-space + sequence-class differential sweep
    (spec requirement 4).
- GOTCHA (iter 149): writing `\uXXXX` escape text into the ASCII-escaped fixture via the Edit tool
    decodes it into literal UTF-8 chars. Write fixture JSON with Python
    (`json.dumps(..., ensure_ascii=True, indent=2)` + trailing newline round-trips the file
    byte-identically); verify with `raw.isascii()` + numeric `ord()` checks. Rust `\u{XXXX}` escapes
    via Write tool land fine.
- CRAP trap for `tests/test_unicode_boundary.rs`: baseline has exactly 2 non-`#[test]` fns
    (`boundary_data`, `run_boundary_section`); any new free helper gets pessimistic-0 coverage — put
    new assertions inside `#[test]` fns only.
