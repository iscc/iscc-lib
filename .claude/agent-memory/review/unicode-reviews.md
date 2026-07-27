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

## The differential sweep gate (landed iter 157)

`scripts/unicode_sweep.py` — 1,112,064 scalars × 8 `CONTEXTS` × 2 `FUNCTION_PAIRS` = **17,793,024
comparisons** against the installed `iscc-core` on CPython 3.14 (uniform 16.0.0 tables, no freeze
rule of its own). Success line: `TOTAL 17793024 comparisons, 0 divergences`.

- Run it as **`mise run unicode:sweep`** (rebuilds `--release` first, ~60 s) or via the standalone
    CI job `unicode-sweep`. **Never verify a dependency or toolchain bump with a bare
    `uv run scripts/unicode_sweep.py`** — `check_extension_fresh` watches only `*.rs` mtimes, so a
    `cargo update` / rustc bump reads a stale `.so` GREEN. Accepted residual (`decisions.md`
    2026-07-27); hardening filed in `issues.md`.
- Guards, all fail-closed: oracle `unidata_version`, extension freshness, scalar count, comparison
    count — the counts assert **before** the success line, so a zero-case run cannot read green.
- `tests/test_unicode_sweep.py` (10 cases) pins the machinery, incl.
    `test_contexts_discriminate_the_delete_filter_design`.

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
4. **Drive `main()` in-process**, never by editing the committed script: load it with
    `importlib.util.spec_from_file_location`, monkeypatch `scalar_values` / `CONTEXTS` /
    `FUNCTION_PAIRS` / the `EXPECTED_*` constants, wrap in `contextlib.redirect_stdout` and catch
    `SystemExit`. Keeps the working tree clean and each probe under a second.
5. **Assert stdout is empty when a count guard fires** — proof the guard runs before any success
    reporting, not just that the exit code is non-zero.

## Boundary vectors

`crates/iscc-lib/tests/unicode_boundary.json` (12 vectors: 4 single code points × 2 functions + 4
sequences). Gated in Rust (141/149), Python + pure-Go (150), WASM + Ruby (151), napi + Java (153),
C# + Kotlin (154) = **8 of 11**; C FFI, C++, Swift left. `git ls-files | grep unicode_boundary`
under-counts — the Java/C#/Kotlin suites are `UnicodeBoundaryTest*.{java,cs,kt}`.

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

## Vendored DERIVED-property tables (iter 156)

Never accept a behaviourally-derived generator as its own oracle. Audit the table against bounds
computable from `unicodedata.category` alone — `Lu∪Ll∪Lt ⊆ Cased`,
`Mn∪Me∪Cf∪Lm∪Sk ⊆ Case_Ignorable`, residuals exactly `Other_Uppercase`/`Other_Lowercase` plus the 17
UAX #29 MidLetter/MidNumLet/Single_Quote code points. Mutate **semantically** and keep the range
count constant, so the *behavioural* tests are proven rather than only the shape test. Probe the
pre-fix defect with a small throwaway crate instead of reverting the core.
