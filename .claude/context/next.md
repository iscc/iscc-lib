# Next Work Package

## Step: Propagation slice 5 — Unicode boundary vectors in the C FFI test

## Goal

Gate the `iscc-ffi` C surface on the 12 Unicode 16.0.0 boundary vectors, taking criterion 3 of the
Unicode issue from **8 of 11** binding surfaces to **9 of 11**. This is the last propagation surface
that can be completed *and executed* autonomously in this container (C++ needs the absent `cmake`,
Swift the absent `swift`), so it closes out the locally-verifiable half of the remaining Unicode
work.

Picks up the `normal` `[human]` issue **"Declare and gate a Unicode data version (DECIDED)"**,
remainder (b). Not a bounce — iteration 158 passed review; this is the "Next" both the handoff and
state.md name.

## Scope

- **Create** (1 non-test/non-doc file — well inside budget):
    - `scripts/gen_ffi_boundary_vectors.py` — PEP 723 generator that renders the canonical fixture
        into a C header
- **Create** (generated / test files, outside the budget):
    - `crates/iscc-ffi/tests/unicode_boundary_vectors.h` — the generated header (tracked, ASCII)
    - `tests/test_gen_ffi_boundary_vectors.py` — the drift gate (regenerating must be a no-op)
- **Modify** (test/doc files, outside the budget):
    - `crates/iscc-ffi/tests/test_iscc.c` — include the header, add the metadata guard + 12 vector
        assertions
    - `docs/unicode.md` — add C FFI to the list of suites exercising the vectors (line ~138-143),
        naming the generated-header mechanism
    - `crates/iscc-ffi/CLAUDE.md` — one line in "Module Layout" + a sentence under "C tests" pointing
        at the generator (the file already documents `tests/test_iscc.c` conventions)
- **Reference**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — the canonical fixture (source of truth)
    - `scripts/gen_unicode16_unassigned.py` — the established PEP 723 generator shape (docstring,
        `OUTPUT_PATH`, invariant guards, "rerunning leaves the tree unchanged")
    - `tests/test_check_docs_nav.py` — the `importlib.util.spec_from_file_location` pattern for
        loading a `scripts/` module from a pytest test
    - `.github/workflows/ci.yml` job key `c-ffi` (lines 114-139) — the compile/run commands that must
        keep working **unchanged**
    - `.claude/context/issues.md` → "Declare and gate a Unicode data version (DECIDED)" (the
        authorization and the propagation ledger)

## Not In Scope

- **Do not touch `crates/iscc-lib/tests/unicode_boundary.json`.** Adding, renaming or reordering a
    vector reds nine other binding suites at once and is a separate deliberate slice.
- **Do not register the generated header in `VENDORED_COPIES`** (`tests/test_vendored_fixtures.py`).
    That table is for *byte-identical* copies of the fixture; a generated header is derived, and its
    equivalent guarantee is the regeneration-no-op gate this step adds. Leave that file untouched.
- **Do not change `.github/workflows/ci.yml`.** The header sits next to `test_iscc.c`, so a quoted
    `#include` resolves with no extra `-I` — verified while scoping (see Implementation Notes). If
    you find yourself editing the gcc line, the header is in the wrong directory.
- **Do not hand-roll a JSON parser in C** and do not hand-pin expected strings that are not
    generated from the fixture — both were considered and rejected (more code / silent drift).
- Do not start the C++ or Swift slices, and do not add the four sibling `data.json` copies.
- Do not add a prek hook for the new generator (the pytest gate already carries it into CI on both
    Python legs); do not add a `mise` task for it — neither existing `gen_unicode16_*.py` has one.
- No Rust source changes. `crates/iscc-ffi/src/lib.rs` and `include/iscc.h` must not move, so
    **neither** the CI-only CRAP `--fail-regression` baseline nor `.iai-baseline.json` may be
    refreshed — this step trips neither gate.
- Do not edit `.claude/context/specs/`, `decisions.md`, or delete the issue from `issues.md` — the
    review agent owns issue resolution.

## Implementation Notes

Everything below was **measured on the working tree while scoping** — do not re-derive, and do not
treat any of it as a hypothesis.

### Pre-work: the local FFI artifact was stale, and is now current

`target/debug/libiscc_ffi.so` predated the iteration-156 `Final_Sigma` freeze. I ran
`cargo build -p iscc-ffi` (14.8 s) — it is now current and the rebuild left **no tree diff** (the
`build.rs` regeneration of `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` is byte-stable). Rebuild
again before running the C test if any Rust source moves.

### All 12 vectors already pass through the FFI — zero skips

Probed with `ctypes` against the freshly built `.so`, calling `iscc_text_clean` /
`iscc_text_collapse` on every case in the canonical fixture: **12 OK, 0 FAIL**, including the
`Final_Sigma` sequence (`U+0391 U+03A3 U+0378 U+0392` → `U+03B1 U+03C2 U+03B2`). This slice lands
**green**, with no skip map — unlike Go, the C FFI wraps the Rust core.

Baseline for the existing C suite: `65 passed, 0 failed`.

### Generator design

`scripts/gen_ffi_boundary_vectors.py`, PEP 723 header with `requires-python = ">=3.10"` and
`dependencies = []` (stdlib `json` only — verified `uv run --script` resolves it offline). Shape:

- `render(fixture_path: Path) -> str` — **pure**, takes the path explicitly so the pytest gate can
    call it without touching the filesystem layout (the injected-`Path` convention from
    `check_docs_nav.py` / `check_release_workflow.py`).
- `main()` writes `render(FIXTURE_PATH)` to an `OUTPUT_PATH` constant pointing at
    `crates/iscc-ffi/tests/unicode_boundary_vectors.h`.
- Fail-closed guards, all `raise SystemExit(...)` (never `assert` — Ruff `S101` is only ignored
    under `tests/**`): the fixture's `_metadata.unicode_data_version` must be `"16.0.0"`; both
    sections must be non-empty; the rendered text must satisfy `.isascii()`.
- Docstring in the house style, ending with
    `Usage: uv run --script scripts/gen_ffi_boundary_vectors.py` and the sentence that rerunning on
    an up-to-date checkout leaves the tree unchanged.

**Escape every non-ASCII byte as a 3-digit octal escape** (Python: `"\\%03o" % byte`, so `0xF0`
becomes the four characters `\360`) — **never a `\x` hex escape.** C hex escapes are greedy and
unbounded, so `"a\xF0\x9F\xAB\xA9b"` mis-parses; octal escapes stop after 3 digits. Emit printable
ASCII literally, escaping only `"` and `\`. This was compiled and run during scoping —
`gcc -Wall -Wextra` is clean and all 12 vectors round-trip byte-exactly.

Header layout that was proven to work (reproduce it; the exact whitespace is yours to choose, it
just has to be stable):

```c
/* Generated by scripts/gen_ffi_boundary_vectors.py - DO NOT EDIT. */
#ifndef ISCC_UNICODE_BOUNDARY_VECTORS_H
#define ISCC_UNICODE_BOUNDARY_VECTORS_H

#define ISCC_UNICODE_DATA_VERSION "16.0.0"

struct iscc_unicode_boundary_vector {
    const char *name;
    const char *input;
    const char *expected;
};

#define ISCC_TEXT_CLEAN_VECTOR_COUNT 7
static const struct iscc_unicode_boundary_vector iscc_text_clean_vectors[ISCC_TEXT_CLEAN_VECTOR_COUNT] = {
    {"test_0000_u1fae9_assigned_so_retained",
     "a\360\237\253\251b",
     "a\360\237\253\251b"},
    /* ... */
};

#define ISCC_TEXT_COLLAPSE_VECTOR_COUNT 5
static const struct iscc_unicode_boundary_vector iscc_text_collapse_vectors[ISCC_TEXT_COLLAPSE_VECTOR_COUNT] = {
    /* ... */
};

#endif /* ISCC_UNICODE_BOUNDARY_VECTORS_H */
```

The counts in the two `#define`s come from the fixture (7 and 5 today), not from a constant in the
generator.

**The header must be byte-stable under the prek hygiene hooks** (`end-of-file-fixer`,
`trailing-whitespace`, `mixed-line-ending --fix=lf`), or `mise run check` will rewrite it and break
the no-op gate: emit LF only, exactly one trailing newline, no trailing spaces on any line. There is
no C formatter hook, so nothing else will touch it.

### C test wiring

In `crates/iscc-ffi/tests/test_iscc.c`, add `#include "unicode_boundary_vectors.h"` next to
`#include "iscc.h"`. A quoted include searches the includer's own directory first, so the CI command

```bash
gcc -o test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm
```

compiles it with **no new `-I`** — confirmed during scoping with the header in a sibling directory
and no `-I` for it.

Add one static helper above `main` taking
`(const char *section, char *(*fn)(const char *), const struct iscc_unicode_boundary_vector *v, size_t n)`
so both sections share a loop; use the existing `ASSERT_STR_EQ` macro with a test name of the form
`unicode_boundary/<section>/<case>` and free every returned string with `iscc_free_string`. Keep it
consistent with the file's existing style (numbered `/* 29. ... */` comment block, `tests_passed` /
`tests_failed` bookkeeping).

Add the **metadata guard** the other nine suites carry, as three assertions before the loops:
`ISCC_UNICODE_DATA_VERSION` equals `"16.0.0"`, `ISCC_TEXT_CLEAN_VECTOR_COUNT` equals `7`,
`ISCC_TEXT_COLLAPSE_VECTOR_COUNT` equals `5`. Hard-coding 7/5 here is deliberate and matches the
convention: adding a fixture vector must red every suite loudly.

Expected new total: `65 + 3 + 12` = **80 passed, 0 failed**.

### Drift gate

`tests/test_gen_ffi_boundary_vectors.py` loads the generator with
`importlib.util.spec_from_file_location` and asserts, at minimum:

1. `render(CANONICAL_FIXTURE) == HEADER_PATH.read_text(encoding="utf-8")` — regenerating is a no-op,
    i.e. the tracked header cannot drift from the fixture.
2. The tracked header is pure ASCII and contains no `\x` escape sequence (the greedy-escape trap).
3. `render()` on a fixture copy with a mutated expected value produces a *different* header — so the
    gate provably fires rather than merely being green. Write the mutated copy to `tmp_path`; never
    mutate the tracked fixture.

Keep it a plain module-level test file (no classes), Python 3.10-compatible (**no `tomllib`** — CI's
`python-test` matrix runs 3.10).

### Traps

- `ty check` and the Ruff `S` / `C901` selections are **pre-push-only**; `mise run check` cannot see
    them. Run `uv run ty check` and `uv run ruff check --select S,C901` explicitly after editing the
    Python files.
- Keep `scripts/gen_ffi_boundary_vectors.py` and the generated header **pure ASCII**; build any
    non-ASCII test data with `chr(0x...)` rather than backslash-u literals (a tool payload can
    decode escapes before they reach the file).
- Write the header by running the generator, not by hand — then verify with
    `git status --porcelain`.
- `uv run zensical build` wipes `site/`; run it before any `gen_llms_full.py` invocation.

## Verification

- `cargo build -p iscc-ffi` exits 0, then the **unmodified CI command**
    `gcc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
    exits 0 (proves no new `-I` is needed), and `LD_LIBRARY_PATH=target/debug /tmp/test_iscc`
    exits **0** with a summary line ending `0 failed`
- `LD_LIBRARY_PATH=target/debug /tmp/test_iscc | grep -c '^PASS: unicode_boundary/'` → **12**
- `gcc -Wall -Wextra -fsyntax-only crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/include`
    emits no warnings
- `uv run --script scripts/gen_ffi_boundary_vectors.py` exits 0 and
    `git status --porcelain -- crates/iscc-ffi/tests/unicode_boundary_vectors.h` is **empty**
    afterwards (regeneration is a no-op on the committed header)
- `git ls-files --error-unmatch crates/iscc-ffi/tests/unicode_boundary_vectors.h` exits 0 (the
    header is tracked, not gitignored)
- `uv run python -c "import pathlib,sys; t=pathlib.Path('crates/iscc-ffi/tests/unicode_boundary_vectors.h').read_text(); sys.exit(0 if t.isascii() and '\\x' not in t and t.endswith('\n') else 1)"`
    exits 0
- `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py` passes, including a case that asserts a
    **mutated** fixture renders a different header (the gate fires, not just passes)
- `uv run pytest -q` passes (393 pre-existing + the new cases, none removed)
- `uv run pytest -q tests/test_vendored_fixtures.py` passes and
    `git status --porcelain -- tests/test_vendored_fixtures.py` is empty
- `uv run ruff check`, `uv run ruff format --check`, `uv run ruff check --select S,C901` and
    `uv run ty check` all exit 0
- `mise run check` exits 0 and modifies no file (`git status --porcelain` shows only the intended
    additions/edits)
- `git status --porcelain -- crates/iscc-ffi/src crates/iscc-ffi/include crates/iscc-lib .crap-baseline.json .iai-baseline.json .claude/context/specs/ .github/workflows/`
    is **empty** (no Rust source, no cbindgen header, no fixture, no baseline, no spec, no workflow
    moved)
- `uv run zensical build` exits 0 with "No issues found" and `uv run scripts/check_docs_nav.py`
    prints `OK: 23 documentation pages consistent`; `grep -c 'C FFI' docs/unicode.md` is at least 2
    (the existing "11 native bindings" sentence plus the newly-gated-suites sentence)

## Done When

All verification criteria pass: the C test program compiles with the unchanged CI command, runs the
12 boundary vectors plus a 3-assertion metadata guard with `0 failed`, the generated header is
proven to be a deterministic no-op regeneration of the canonical fixture by a pytest gate that fires
on a mutation, and no Rust source, cbindgen header, fixture, baseline, spec or workflow file moved.
