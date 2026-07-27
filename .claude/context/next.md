# Next Work Package

## Step: Freeze `Final_Sigma` case classification at Unicode 16.0.0

## Goal

Close the **single** divergence between the Rust core and the normative reference, so that criterion
4 of the `normal` issue "Declare and gate a Unicode data version (DECIDED)" can land **green** in a
following step. `text_collapse` lowercases with `str::to_lowercase()`, whose `Final_Sigma` decision
reads the **compiler's** Unicode tables (rustc 1.97 ships 17.0). Unicode 17.0 reclassified U+0295
from `Ll` to `Lo`, so the Rust core emits final sigma where `iscc-core` on uniform 16.0.0 tables
emits medial sigma — the core's hash output is currently a function of the rustc version. This step
vendors the Unicode 16.0.0 `Cased` / `Case_Ignorable` classification and drives `Final_Sigma` from
it.

**Reframe of the handoff's "Next" item 1 — not a backtrack, and not a repeat.** The handoff asked
for the criterion-4 sweep harness. `specs/rust-core.md:103-118` requires the divergence set to be
**empty** ("any nonzero result is a defect, not a residual to accept") and its later paragraph is
explicit that "divergence caused by the runtime's tables being *newer* than 16.0 is **not** accepted
and must be zero". Today's set is `{U+0295}`, so landing the gate first would land it red. The fix
goes first; the permanent sweep gate is the step after. **No bounce signal:** iteration 155's
`define-next` hit the runner wall with 0 turns — `advance` and `review` never ran, so this step has
never been attempted, let alone rejected.

## Scope

- **Create**:
    - `scripts/gen_unicode16_case.py` — PEP 723 generator for the two vendored case tables (counted
        file 1 of 2)
    - `crates/iscc-lib/src/utils/unicode16_case.rs` — its data-only output (generated tool output, not
        counted against the budget; never hand-edit)
- **Modify**:
    - `crates/iscc-lib/src/utils.rs` — `mod unicode16_case;`, two predicates, a shared range-lookup
        helper, `to_lowercase_unicode16`, the one `text_collapse` call site, plus new `#[cfg(test)]`
        cases (counted file 2 of 2; its test module is not counted)
    - `.crap-baseline.json` — regenerated (tool output, not counted; see Verification)
- **Modify (docs, excluded from the budget)**:
    - `docs/unicode.md` — lines 57-60 overclaim; add a case-property-freeze subsection
    - `crates/iscc-lib/CLAUDE.md` — line ~165 carries the same gap
- **Reference**:
    - `crates/iscc-lib/src/utils.rs` lines 26-60 (`UNASSIGNED_SENTINEL`, `is_unassigned_in_unicode16`
        with its binary search + fast path) and lines 192-229 (`text_collapse`)
    - `scripts/gen_unicode16_unassigned.py` — the generator template to mirror (PEP 723 header,
        `EXPECTED_*` constants, `check_invariants` raising `SystemExit`, `render_module`,
        `write_text(..., encoding="utf-8", newline="\n")`, closing `print`)
    - `crates/iscc-lib/src/utils/unicode16.rs` — the generated-module shape to mirror
    - `reference/iscc-core/iscc_core/code_content_text.py` -> `text_collapse` (NFD, `.lower()`, filter
        C/M/P + whitespace, NFKC)
    - `.claude/context/specs/rust-core.md` lines 103-118 (criterion 4) and the "Case mapping must be
        context-sensitive in every implementation" paragraph
    - `.claude/context/issues.md` -> "Declare and gate a Unicode data version (DECIDED)"

## Not In Scope

- **Do not build the permanent sweep gate this step** — no `scripts/unicode_sweep.py`, no mise task,
    no CI job. The probe below runs from `/tmp` and must leave **no** tree diff. Wiring the sweep in
    as a runnable check (zero-case guard, `unidata_version` assert, CI placement) is the next step.
- **Do not add cases to `crates/iscc-lib/tests/unicode_boundary.json`.** Nine binding suites assert
    exactly 7 `text_clean` + 5 `text_collapse` cases as a metadata guard; adding a vector reds all
    nine at once. A sigma vector belongs in a later, deliberate "extend the fixture + bump all nine
    count guards" slice. Pin the new behaviour with Rust unit tests instead.
- **Do not vendor the full lowercase *mapping* table.** Only the `Final_Sigma` *classification*
    diverges. Mapping drift is a hypothetical the future sweep gate will catch.
- Do not touch `packages/go` — `x/text`'s `cases.Lower` is on 15.0 tables and will inherit the same
    reclassification only at go1.27, which is separately tracked.
- Do not change `is_c_category`, `is_cmp_category`, the `is_whitespace()` filter, the sentinel map,
    or the NFD/NFKC ordering. Do not rename, merge or re-run `scripts/gen_unicode16_unassigned.py`.
- Do not edit anything under `.claude/context/specs/` (human-owned) and do not delete the issue from
    `issues.md` — the review agent handles issue resolution.
- No propagation work (C FFI / C++ / Swift boundary suites), no dependency bumps, no unrelated
    refactors of `utils.rs`.

## Implementation Notes

### Verified facts — measured this iteration, do not re-derive

1. **The defect reproduces right now.** On the project venv (CPython **3.14.6**,
    `unicodedata.unidata_version == "16.0.0"`, `iscc-core` 1.3.0):

    | input (code points)         | `iscc-core` (16.0)          | Rust core today             |
    | --------------------------- | --------------------------- | --------------------------- |
    | U+0391 U+03A3 U+0295 U+0392 | U+03B1 U+03C3 U+0295 U+03B2 | U+03B1 U+03C2 U+0295 U+03B2 |
    | U+0295 U+03A3               | U+0295 U+03C2               | U+0295 U+03C3               |

2. **The cause.** U+0295 is `Ll` (hence `Cased`) in Unicode 16.0.0 and `Lo` (not `Cased`) in 17.0.
    Rust and CPython implement the *same* `Final_Sigma` algorithm (skip `Case_Ignorable` backwards,
    require `Cased`; skip `Case_Ignorable` forwards, require **not** `Cased`) over *different*
    tables.

3. **`text_clean` is unaffected** — no case mapping; its category filter uses the 16.0-pinned
    `unicode-general-category` crate.

4. **Table shapes — re-derived from scratch this iteration and confirmed exact** (the previous draft
    asserted these unverified; they now check out):

    - `CASED_RANGES`: **4,311** code points in **152** maximal inclusive ranges
    - `CASE_IGNORABLE_RANGES`: **2,749** code points in **452** maximal inclusive ranges
    - U+0295 is in `CASED_RANGES`; the sentinel U+FFFF is in **neither** table, so the existing
        fixture row U+0391 U+03A3 U+0378 U+0392 keeps its final sigma.

5. **`uv run --script` resolves a `requires-python = "==3.14.*"` script to 3.14.6 / 16.0.0** —
    probed. No dependency pin is needed or possible: `unicodedata2` exposes categories, not the
    derived `Cased` / `Case_Ignorable` properties, so the reference interpreter itself is the
    source.

### The generator: `scripts/gen_unicode16_case.py`

Mirror `scripts/gen_unicode16_unassigned.py` structurally. Two differences:

- PEP 723 header: `requires-python = "==3.14.*"`, `dependencies = []`. Assert
    `unicodedata.unidata_version == "16.0.0"` at the top and `raise SystemExit` otherwise — the
    script must never write a table from the wrong tables.

- Derivation, two O(1) behavioural probes per code point (whole sweep about 5 s). Build the two
    probe strings with `chr(0x03A3)` / `chr(0x03C2)` rather than literals so the file stays ASCII:

    ```python
    SIGMA = chr(0x03A3)
    FINAL = chr(0x03C2)
    ch = chr(cp)
    a = (ch + SIGMA).lower().endswith(FINAL)  # Cased and not Case_Ignorable
    b = ("A" + ch + SIGMA).lower().endswith(FINAL)  # Cased or Case_Ignorable
    cased = a
    case_ignorable = b and not a
    ```

    This reads the exact two predicates CPython's `handle_capital_sigma` consults. `CASED_RANGES` is
    therefore `Cased` **restricted to code points that are not `Case_Ignorable`** — say so in the
    module doc comment. The restriction is unobservable: the scan skips case-ignorables before
    testing `Cased`, so a code point that is both is always skipped first, by CPython and by the
    port alike. Skip surrogates U+D800..U+DFFF.

Emit both tables into one module in the `unicode16.rs` style — a doc header naming the generator and
the Unicode version, then `pub(crate) const CASED_RANGES: [(u32, u32); 152] = [ ... ];` and
`pub(crate) const CASE_IGNORABLE_RANGES: [(u32, u32); 452] = [ ... ];`, one `(0xLO, 0xHI),` entry
per line so rustfmt is stable. Carry over the sorted / non-adjacent / non-inverted invariant checks.

### The core change: `crates/iscc-lib/src/utils.rs`

All new items get `#[cfg(feature = "text-processing")]`, matching the neighbours.

- Factor the range lookup out of `is_unassigned_in_unicode16` into
    `fn in_ranges(cp: u32, ranges: &[(u32, u32)]) -> bool` (the existing `binary_search_by`
    comparator, unchanged) and call it from all three predicates. Keep the existing
    `cp < UNASSIGNED_RANGES[0].0` fast path where it is today.

- `fn is_cased_in_unicode16(c: char) -> bool` and
    `fn is_case_ignorable_in_unicode16(c: char) -> bool` over the two new tables — naming mirrors
    `is_unassigned_in_unicode16`.

- `fn to_lowercase_unicode16(text: &str) -> String`:

    ```text
    if !text.contains(CAPITAL_SIGMA) { return text.to_lowercase(); }   // hot path unchanged
    walk chars once, tracking `last_non_ignorable: Option<char>`;
    on U+03A3 push U+03C2 iff last_non_ignorable is Some(cased)
                          and the first non-ignorable char after it is not cased (or absent);
    otherwise push U+03C3; push every other char unchanged; then `.to_lowercase()` the result.
    ```

    Pre-substituting means std never sees a U+03A3, so its own 17.0-table `Final_Sigma` branch can
    never fire; U+03C3 / U+03C2 are already lowercase, so the delegated `to_lowercase()` leaves them
    alone while still handling every other full-case mapping (e.g. U+0130). Track the previous
    non-ignorable char in the forward walk instead of re-scanning backwards, so a sigma-dense string
    stays linear.

- **Placement matters.** In `text_collapse` the lowercasing happens *after* the sentinel map and NFD
    (`crates/iscc-lib/src/utils.rs:209-220`). Replace only that trailing `.to_lowercase()` with
    `to_lowercase_unicode16(...)`, so the function receives the sentinel-mapped, NFD-normalized
    string — the same input CPython's `.lower()` sees, since the reference NFDs first too. "Scan the
    original string for context" means the pre-lowercase string handed to `to_lowercase_unicode16`,
    **not** the raw `text` argument: casedness of a neighbour is the same before and after
    lowercasing, but NFD decomposition is not. Nothing else in the pipeline moves.

### Tests to add (existing `#[cfg(test)] mod tests` in `utils.rs`)

Expected values come from `iscc-core` 1.3.0 on CPython 3.14 — they are the oracle; do not recompute
them from the Rust code. Write inputs with `\u{...}` escapes in the Rust source.

| input                              | `text_collapse` expected           | pins                             |
| ---------------------------------- | ---------------------------------- | -------------------------------- |
| U+0391 U+03A3 U+0295 U+0392        | U+03B1 U+03C3 U+0295 U+03B2        | the fix (following context)      |
| U+0295 U+03A3                      | U+0295 U+03C2                      | the fix (preceding context)      |
| U+0391 U+03A3 U+2170               | U+03B1 U+03C3 U+0069               | `Other_Lowercase` cased extra    |
| U+0391 U+03A3 U+02B0 U+0392        | U+03B1 U+03C3 U+0068 U+03B2        | `Lm` is case-ignorable           |
| U+0391 U+03A3 U+0027 U+03B2        | U+03B1 U+03C3 U+03B2               | `Word_Break` ignorable extra     |
| U+039B U+039F U+0393 U+039F U+03A3 | U+03BB U+03BF U+03B3 U+03BF U+03C2 | plain final sigma (passes today) |
| U+0391 U+03A3 U+0378 U+0392        | U+03B1 U+03C2 U+03B2               | sentinel keeps final sigma       |

If any expected value above disagrees with a fresh
`uv run python -c "import iscc_core; print(repr(iscc_core.text_collapse(...)))"`, **the oracle
wins** — fix the table and say so in the handoff.

Add one shape test asserting `CASED_RANGES.len() == 152` and `CASE_IGNORABLE_RANGES.len() == 452` so
a truncated regeneration reds immediately. Keep non-test helpers at low cyclomatic complexity —
`cargo crap` scores them pessimistically.

### The throwaway differential probe (write to `/tmp`, never commit)

Rebuild the Python extension first, otherwise the probe measures a stale `.so`:
`uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml`.
`crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` is gitignored, so this leaves no diff.

```python
# /tmp/unicode_sweep_probe.py  -- throwaway; the next step turns this into a real gate
import unicodedata, iscc_core, iscc_lib

assert unicodedata.unidata_version == "16.0.0", unicodedata.unidata_version
MARK = chr(0x0301)  # combining acute
J_L, J_V = chr(0x1100), chr(0x1161)
CAP_A, CAP_S, CAP_B = chr(0x0391), chr(0x03A3), chr(0x0392)
CONTEXTS = [
    "{}",  # bare scalar
    "a{}b",  # ASCII neighbours
    "e{}" + MARK,  # base + X + mark   (canonical composition)
    J_L + "{}" + J_V,  # jamo + X + jamo   (Hangul composition)
    CAP_A + CAP_S + "{}" + CAP_B,  # cased + sigma + X + cased (Final_Sigma, after)
    "{}" + CAP_S,  # X + sigma         (Final_Sigma, before)
    CAP_A + "{}" + CAP_S,  # cased + X + sigma (Case_Ignorable, before)
    "a" + MARK + "{}" + MARK + "b",  # X between marks
]
scalars = [c for c in range(0x110000) if not (0xD800 <= c <= 0xDFFF)]
assert len(scalars) == 1_112_064
total = bad = 0
for name, rust, ref in (
    ("text_clean", iscc_lib.text_clean, iscc_core.text_clean),
    ("text_collapse", iscc_lib.text_collapse, iscc_core.text_collapse),
):
    for ctx in CONTEXTS:
        for c in scalars:
            s = ctx.format(chr(c))
            x, y = rust(s), ref(s)
            total += 1
            if x != y:
                bad += 1
                if bad <= 10:
                    print(
                        f"DIVERGE {name} ctx={ctx!a} cp=U+{c:04X} rust={x!a} ref={y!a}"
                    )
print(f"TOTAL {total} comparisons, {bad} divergences")
raise SystemExit(1 if bad or total != 17_793_024 else 0)
```

Run with `uv run python /tmp/unicode_sweep_probe.py` (about 60-70 s; 8 contexts x 2 functions x
1,112,064 scalars = 17,793,024 comparisons). The contexts cover the four sequence classes
`specs/rust-core.md` requires (base+Cn+mark, jamo+Cn+jamo, sigma+Cn+cased, Cn-between-marks) plus
the two that isolate `Cased` from `Case_Ignorable`. Before the fix it prints exactly 1 divergence
(U+0295); after the fix it must print **0**. Run it **before** the fix too, so the handoff can
report the 1 -> 0 transition as measured rather than assumed.

### Gates this step *will* trip (unlike the last five iterations)

- **CRAP** — new functions and branches in a covered file. `--fail-regression` is CI-only, so run
    `mise run coverage && mise run crap:baseline` and commit the refreshed `.crap-baseline.json`
    **in the same commit**. Also confirm `mise run crap` exits 0 (nothing above the 30 threshold).
- **iai-callgrind (10% Ir)** — `text_collapse` is benched. The `contains(CAPITAL_SIGMA)` fast path
    should keep non-Greek inputs in the noise band; run `mise run bench:iai:check`. If a bench
    genuinely exceeds 10%, do **not** quietly refresh `.iai-baseline.json`: refresh it *and* state
    the measured per-bench delta plus the correctness justification prominently in the handoff so
    review can rule on it.

## Verification

- `uv run --script scripts/gen_unicode16_case.py` exits 0; running it a second time leaves
    `git status --porcelain crates/iscc-lib/src/utils/unicode16_case.rs` **empty**
- `grep -c 'CASED_RANGES: \[(u32, u32); 152\]' crates/iscc-lib/src/utils/unicode16_case.rs` is 1 and
    `grep -c 'CASE_IGNORABLE_RANGES: \[(u32, u32); 452\]' crates/iscc-lib/src/utils/unicode16_case.rs`
    is 1
- The generator fails closed: temporarily edit one `EXPECTED_*` constant, observe a **non-zero**
    exit and an unchanged output file, then restore the constant (working-tree edit only, restored
    before commit)
- `cargo test -p iscc-lib` passes with **0 failed**, and the lib unit-test binary reports **at least
    288 passed** (`grep -c '#\[test\]' crates/iscc-lib/src` is 281 in the tree today)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml` exits 0, then
    `uv run python /tmp/unicode_sweep_probe.py` prints `TOTAL 17793024 comparisons, 0 divergences`
    and **exits 0**
- Mutation probe (working-tree edit, restored afterwards): truncating `CASED_RANGES` to its first 10
    entries makes `cargo test -p iscc-lib` **fail**; restoring makes it pass, and
    `git status --porcelain crates/iscc-lib/src/utils/unicode16_case.rs` is empty at the end
- `mise run coverage && mise run crap:baseline` run, `.crap-baseline.json` staged in the same
    commit, and `mise run crap` exits 0
- `mise run bench:iai:check` exits 0 (or the handoff carries the measured per-bench delta and the
    justification for a baseline refresh)
- `uv run pytest -q` passes with the rebuilt extension
- `grep -c 'U+0295' docs/unicode.md` is at least 1, the file states that `Cased` / `Case_Ignorable`
    are pinned to Unicode 16.0.0 by a vendored table and that `str::to_lowercase()` alone would make
    output depend on the rustc version, and `uv run scripts/check_docs_nav.py` exits 0 reporting
    **23** pages
- `mise run check` exits 0 (all prek hooks) and `git status --porcelain` is clean afterwards
- `git status --porcelain -- scripts/unicode_sweep.py crates/iscc-lib/tests/unicode_boundary.json .github .claude/context/specs`
    is **empty**, and `git ls-files -- '*unicode_boundary.json' | wc -l` is **2** — no sweep gate,
    no fixture change, no new vendored copy, no CI or spec edit slipped in

## Done When

`text_collapse` derives `Final_Sigma` from the vendored Unicode 16.0.0 `Cased` / `Case_Ignorable`
tables, the full-code-space plus sequence-class probe reports 0 divergences from `iscc-core` on
uniform 16.0.0 tables, the CRAP baseline is refreshed in the same commit, and every verification
command above passes.
