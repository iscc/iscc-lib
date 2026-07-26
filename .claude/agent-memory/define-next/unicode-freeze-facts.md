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

## Sentinel conversion (`filter` → `map` onto `U+FFFF`) — scoped iteration 148

- The single-code-point expectations in the two tables above are **unchanged** by the sentinel
    design: `a<cp>b` deletes vs. maps-then-strips to the same string. Only *sequences* change, which
    is why `tests/unicode_boundary.json` and the 25 `utils.rs` tests needed no edit.
- Sequence expectations are mechanically derivable, no probe needed: `U+FFFF` has `ccc = 0` (blocks
    canonical + Hangul jamo composition of its neighbours) and is neither `Cased` nor
    `Case_Ignorable` (so a preceding `Σ` still lowercases to final `ς`). That reproduces all four
    spec **Verified when** sequence values exactly.
- Stale-wording sites to sweep whenever the freeze-rule mechanism changes (grep
    `before any normalization` / `stripped before`): `utils.rs` ×3 docs + 2 inline comments,
    `crates/iscc-lib/CLAUDE.md` "Text normalization order matters", `tests/test_unicode_boundary.rs`
    module docs, `scripts/gen_unicode16_unassigned.py` docstring, `docs/unicode.md`.
- CRAP baseline values before the conversion (from `.crap-baseline.json`): `text_clean` cyclomatic
    9.0 / crap 9.0, `text_collapse` 1.0 / 1.0, both 100% covered → with coverage at 100%,
    `CRAP == cyclomatic`, so each rises by ~1 and the CI-only `--fail-regression` gate REQUIRES a
    same-commit `mise run crap:baseline`.
- `valgrind` (`/usr/bin/valgrind`) and `~/.cargo/bin/iai-callgrind-runner` ARE present in this
    container despite learnings.md calling the perf tooling install-on-demand — the
    `bench:iai:check` mise task is runnable locally; the only real risk is a local-vs-CI rustc Ir
    offset, so scope it as "run before editing too".

See [[define-next-ty-generator-scripts]] for the `ty check` trap that generator scripts with
external pins hit.

## Sequence boundary vectors — the fixture's discriminating half (scoped iter 149)

The 4 original vectors wrap one code point in ASCII `a`/`b`, so deletion and sentinel-then-strip
agree: they are **deletion-vs-sentinel agnostic** and cannot gate the ruled design. Only these four
sequences can. ASCII-escaped exactly as `json.dumps(..., ensure_ascii=True)` emits them (the file is
pure ASCII by design — composed/decomposed forms render identically and U+0378 is tofu):

| section         | input                      | expected             | delete-filter would give |
| --------------- | -------------------------- | -------------------- | ------------------------ |
| `text_clean`    | `e\u0378\u0301`            | `e\u0301`            | `\u00e9`                 |
| `text_clean`    | `\u1100\u0378\u1161`       | `\u1100\u1161`       | `\uac00`                 |
| `text_clean`    | `e\ua7f1\u0301`            | `e\u0301`            | `e\u015a`                |
| `text_collapse` | `\u0391\u03a3\u0378\u0392` | `\u03b1\u03c2\u03b2` | `\u03b1\u03c3\u03b2`     |

Pinned by six `utils.rs` tests (lines ~350-407) that passed CI at HEAD — never re-derive them. Do
**not** mirror them into the other section: the jamo sequence under `text_collapse` legitimately
recomposes to `\uac00` (the reference does the same), which reads as a contradiction in a fixture.

`cargo crap` scores non-`#[test]` helpers in `tests/*.rs` with `coverage: null` →
`missing = "pessimistic"` → `crap = c^2 + c`; threshold 30 means cyclomatic \<= 5. Keep new fixture
logic inside `#[test]` fns so the CRAP report gains no entry (only `boundary_data` and
`run_boundary_section` are in `.crap-baseline.json` today).
