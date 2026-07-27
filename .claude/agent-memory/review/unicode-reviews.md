---
name: unicode-reviews
description: Reviewing anything that touches the Unicode 16.0.0 freeze rule — the sentinel map, the Final_Sigma case freeze, the differential sweep gate, and boundary-vector propagation.
metadata:
  type: project
---

# Reviewing Unicode freeze-rule work

Everything under the human-authorized "Declare and gate a Unicode data version" issue. Live
propagation tally and the go1.27 checklist stay in `issues.md`; design rationale in `decisions.md`.

## The contract (what a diff must not break)

- **The freeze rule is a `U+FFFF` SENTINEL MAP**, adopted iter 148. Code points unassigned in
    Unicode 16.0.0 are *replaced* by `UNASSIGNED_SENTINEL` before normalization; the unchanged
    category-`C` filter then removes the sentinel. The **delete filter** (iter-133 design) and the
    **category override** are both RULED OUT — deletion changes adjacency and unblocks
    canonical/jamo composition, `Final_Sigma` and diaeresis.
- **`str::to_lowercase()` reads the COMPILER's Unicode tables** (found 156). rustc 1.97 ships 17.0,
    which moved `U+0295` `Ll`→`Lo`, so a bare `.to_lowercase()` made hash output a function of the
    rustc version. `text_collapse` must go through `to_lowercase_unicode16` + the vendored
    `crates/iscc-lib/src/utils/unicode16_case.rs` tables. **Reject any diff that reintroduces a bare
    `.to_lowercase()` there.** Only the *conditional* `Final_Sigma` half is frozen; every
    unconditional mapping still comes from rustc — an accepted, currently-zero residual whose only
    guard is the sweep gate (`decisions.md` 2026-07-27).
- **Never "fix" one binding to match another.** They all wrap the same Rust core except
    `packages/go`, which is an independent port.

## The differential sweep gate (landed iter 157, hardened 158)

`scripts/unicode_sweep.py` — 1,112,064 scalars × 8 `CONTEXTS` × 2 `FUNCTION_PAIRS` = **17,793,024
comparisons** against the installed `iscc-core` on CPython 3.14 (uniform 16.0.0 tables, no freeze
rule of its own). Success line: `TOTAL 17793024 comparisons, 0 divergences` — byte-frozen, three
context files assert it.

- Run it as **`mise run unicode:sweep`** (rebuilds `--release` first, ~90 s total) or via the
    standalone CI job `unicode-sweep`. A bare `uv run scripts/unicode_sweep.py` **refuses** since
    158: `check_rebuilt` demands the `--rebuilt` caller assertion, passed only by the mise task and
    the CI step, each immediately after an unconditional release build. The flag is *trusted*, not
    observed — `--rebuilt` without a rebuild still sweeps a stale `.so` (documented residual).
- Guards, all fail-closed and in this order: `check_rebuilt`, oracle `unidata_version`, extension
    freshness (`.so` mtime vs newest `*.rs`; an **empty** source set fails since 158), scalar count,
    comparison count — the counts assert **before** the success line, so a zero-case run cannot read
    green. Residual: freshness is checked over the *union* of `RUST_SOURCE_DIRS`, so one renamed or
    emptied dir still passes as long as the other has sources.
- `sweep()` returns `SweepResult(comparisons, divergences, samples)`; `samples` is capped at
    `MAX_REPORTED_DIVERGENCES` (20). Any test asserting over `samples` inherits the cap —
    `test_contexts_discriminate_the_delete_filter_design` is safe only because 8 contexts × 2 fns =
    16 < 20.
- `tests/test_unicode_sweep.py` (14 cases) pins the machinery.

## Differential-gate recipe (iter 157, ~25 min on top of the new-gate-script recipe)

1. **Probe the case SET, not just the case COUNT.** An arithmetic pin
    (`EXPECTED == scalars × len(CONTEXTS) × 2`) catches a *shrunken* set and never a *swapped* one.
    Re-run the gate with a **superseded real design** as the subject and record which rows light
    up. Measured here: an `iscc-core`-based delete filter over the 831 unassigned scalars below
    `U+2000` lights up exactly `text_clean/base_mark`, `text_clean/jamo`, `text_collapse/sigma`
    (831 each); `bare`, `ascii`, `marks`, `space`, `upper` all score **0**. Swap the three sequence
    shapes for ASCII ones and the probe exposes *nothing* — that is the hole to close with a
    committed test.
2. **Ask what the freshness/staleness guard cannot see.** mtime guards miss dependency bumps,
    toolchain bumps and anything outside the watched dirs; `max(..., default=0.0)` over a missing
    directory passes vacuously. Reproduce both before deciding whether they are acceptable.
3. **Check the failure path's memory growth.** Retaining one record per divergence is fine at 0
    divergences and fatal at 17.8M — the most informative failure becomes an OOM kill with no log.
    A unit test on the sweep function proves the cap; only an end-to-end `main()` run proves the
    *reporting* (sample lines + "showing first N of M" + the frozen TOTAL) survives it.
4. **Drive `main()` in-process**, never by editing the committed script: load it with
    `importlib.util.spec_from_file_location`, then `setattr(mod, …)` (never `mod.X = …`, `ty`
    rejects it) on `scalar_values` / `CONTEXTS` / `FUNCTION_PAIRS` / `check_extension_fresh` / the
    `EXPECTED_*` constants, wrap in `contextlib.redirect_stdout` and catch `SystemExit`. A
    30-scalar wrong-oracle run of `main(["--rebuilt"])` returns 1 and prints exactly 20
    `DIVERGENCE` lines + `showing first 20 of 480 divergences` +
    `TOTAL 480 comparisons, 480 divergences`.
5. **Probe near-miss forms of any new caller-assertion flag** — `--rebuild`, `--rebuilt=true` must
    both be refused (membership tests in `argv` are exact; there is no argparse here).
6. **Assert stdout is empty when a count or precondition guard fires** — proof the guard runs before
    any success reporting, not just that the exit code is non-zero.

## Boundary vectors

`crates/iscc-lib/tests/unicode_boundary.json` (12 vectors: 4 single code points × 2 functions + 4
sequences). Gated in Rust (141/149), Python + pure-Go (150), WASM + Ruby (151), napi + Java (153),
C# + Kotlin (154), C FFI (159), C++ (160, reusing the C FFI header) = **10 of 11**; only **Swift**
left — its toolchain genuinely is not in this container and there is no PyPI substitute, unlike
`cmake` (`uv run --with cmake cmake …` works). `git ls-files | grep unicode_boundary` under-counts —
the Java/C#/Kotlin suites are `UnicodeBoundaryTest*.{java,cs,kt}` and C/C++ consume a generated
header, not a JSON file.

- Single-code-point vectors are **deletion-agnostic**; only the 4 sequence vectors discriminate
    sentinel-vs-delete, and their expected values already differ from the delete-filter results, so
    a binding suite needs no oracle column.
- **A binding can pass for the WRONG reason — check the MECHANISM** (150). `packages/go` has no
    freeze rule, but its Unicode 15.0 tables call `U+20C1`/`U+A7F1` `Cn`, so its category-`C` filter
    coincidentally matches. Under go1.27 five green cases flip red; that red is the *designed
    signal* — REJECT any proposal to version-gate the skip list (`decisions.md` 2026-07-26).
- Fixtures are ASCII-escaped: writing `\uXXXX` through Edit/Write decodes it to literal UTF-8. Edit
    via Python and assert `isascii()` + numeric `ord()`. `tests/test_vendored_fixtures.py` gates
    byte-identity of the canonical file and every registered copy.
- **A JSON-less surface gets a GENERATED tracked artifact, not a vendored copy** (159, C FFI; reuse
    for C++). Do not look for it in `VENDORED_COPIES` — that table is byte-identical copies only,
    and a derived file cannot satisfy it. Review recipe → `binding-reviews.md` "C FFI".

## Vendored DERIVED-property tables (iter 156)

Never accept a behaviourally-derived generator as its own oracle. Audit the table against bounds
computable from `unicodedata.category` alone — `Lu∪Ll∪Lt ⊆ Cased`,
`Mn∪Me∪Cf∪Lm∪Sk ⊆ Case_Ignorable`, residuals exactly `Other_Uppercase`/`Other_Lowercase` plus the 17
UAX #29 MidLetter/MidNumLet/Single_Quote code points. Mutate **semantically** and keep the range
count constant, so the *behavioural* tests are proven rather than only the shape test. Probe the
pre-fix defect with a small throwaway crate instead of reverting the core.
