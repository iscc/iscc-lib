---
name: unicode-freeze-facts
description: Measured constants and boundary-code-point behaviour for the Unicode 16.0.0 freeze rule — reuse when scoping the remaining freeze-rule steps (binding vectors, differential sweep)
metadata:
  type: project
---

# Unicode 16.0.0 freeze rule — measured facts

Fact set gathered while scoping iteration 133 (the Rust-core freeze filter). All numbers were
measured in the devcontainer, not copied from the spec.

**Why:** the freeze rule spans three spec criteria and at least three CID steps (core filter →
binding boundary vectors → full-code-space differential sweep). Re-measuring costs several minutes
of `unicodedata2` sweeps per iteration and risks drifting numbers between steps.

**How to apply:** cite these in `next.md` under "verified facts, do not re-derive" so the advance
agent writes assertions instead of exploring.

## Constants (verified with `uv run --no-project --with 'unicodedata2==16.0.0'`)

- **731** maximal inclusive `Cn` ranges covering **819,533** code points; first range
    `U+0378..=U+0379` (→ a `cp < RANGES[0].0` fast path covers all of ASCII/Latin-1).
- `unicodedata2` **17.0.0 also installs cleanly** — useful for proving 16→17 drift.
- Exactly **one** code point is `Cn` in 16.0 yet carries a decomposition in 17.0: **U+A7F1 MODIFIER
    LETTER CAPITAL S**, `<super> 0053` → NFKC maps it to `S`.

## Boundary behaviour of the Rust core *before* the freeze filter

⚠️ **The dev venv's installed `iscc_lib` wheel is stale** — re-measured at iteration 141 it still
returns the pre-133 `"aSb"` for `U+A7F1`, i.e. it predates the freeze filter. Use it only for
questions the wheel's age cannot affect; for current core behaviour run
`cargo test -p iscc-lib --lib utils::` (25 tests, ~0.03 s once built) or add a scratch assertion.

Measured via `uv run python -c "import iscc_lib; …"`:

| Code point | 16.0 cat | `text_clean("a?b")` | `text_collapse("a?b")` |
| ---------- | -------- | ------------------- | ---------------------- |
| `U+A7F1`   | Cn       | `"aSb"` ← the bug   | `"ab"`                 |
| `U+1FAE9`  | So       | retained            | retained               |
| `U+113C5`  | Mc       | retained            | `"ab"` (mark stripped) |
| `U+20C1`   | Cn       | `"ab"`              | `"ab"`                 |
| `U+A7CB`   | Lu       | retained            | retained (lowercased)  |

So the freeze filter's only observable change in the Rust core is `U+A7F1` in `text_clean`. The
three code points the spec names for boundary vectors (`U+1FAE9`, `U+113C5`, `U+20C1`) already
behave correctly — they are regression guards, not fixes. `U+113C5` decomposes canonically to
`U+113C2 U+113C2` (NFD), which is what makes it the "16.0 char with a canonical decomposition".

## Boundary vectors — expected values and how to derive them (iter 141)

Post-freeze-filter behaviour, confirmed twice (Rust `cargo test -p iscc-lib --lib utils::` at HEAD
**and** an independent Python simulation on Unicode 16.0 tables — both agree):

| Code point | 16.0 cat | `text_clean("a?b")` | `text_collapse("a?b")` |
| ---------- | -------- | ------------------- | ---------------------- |
| `U+1FAE9`  | So       | retained            | retained               |
| `U+113C5`  | Mc       | retained            | `"ab"`                 |
| `U+20C1`   | Cn       | `"ab"`              | `"ab"`                 |
| `U+A7F1`   | Cn       | `"ab"`              | `"ab"`                 |

Independent derivation (no Rust needed, ~15 s):
`uv run --no-project --with 'unicodedata2==16.0.0' python - <<'PY'` with
`strip Cn → NFKC → drop category-C-except-\n → .strip()` for `text_clean`, and
`strip Cn → NFD → lower → drop whitespace + C/M/P → NFKC` for `text_collapse`. This reproduces the
Rust implementation exactly, so fixture expectations never have to be copied from the code under
test.

JSON escapes (from `json.dumps(..., ensure_ascii=True)`; **do not hand-compute surrogate pairs** —
`U+1FAE9` is `\ud83e\udee9`, not `...\udea9`): `U+113C5` → `\ud804\udfc5`, `U+20C1` → `\u20c1`,
`U+A7F1` → `\ua7f1`.

Wrap every boundary code point in plain ASCII `a`/`b` — non-composing neighbours keep the vectors
independent of the parked freeze-rule *ordering* ruling (which only concerns sequences where
stripping enables composition).

## Structural facts for the implementation

- `unicode-general-category` 1.1.0 = Unicode 16.0 tables; `unicode-normalization` 0.1.25 = 17.0
    tables. The mismatch is exactly why NFKC-before-filter leaks `U+A7F1`.
- `rustfmt --edition 2024` keeps long array literals **one element per line**, so a generated
    `[(0x0378, 0x0379),\n …]` block is format-stable (short arrays *are* collapsed onto one line).
- A generated data module can live at `crates/iscc-lib/src/utils/unicode16.rs` declared from
    `utils.rs` (a `utils.rs` + `utils/` pair is legal since edition 2018) — this avoids touching
    `lib.rs` and keeps the step inside the 3-file budget.
- Keep lookup logic OUT of the generated file (data only) so regeneration can never clobber
    hand-written code, and so `uv run --script <gen>` + `git status --porcelain <file>` is a clean
    determinism check.

See [[define-next-ty-generator-scripts]] for the `ty check` trap that generator scripts with
external pins hit.
