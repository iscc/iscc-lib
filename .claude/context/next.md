# Next Work Package

## Step: Land the Unicode 16.0.0 differential sweep as a permanent, fail-closed gate

## Goal

Turn the throwaway full-code-space sweep — re-invented by hand at iterations 148, 155 and 156 — into
a committed, runnable check (`scripts/unicode_sweep.py` + a `mise run unicode:sweep` task + its own
CI job), closing criterion 4 of `specs/rust-core.md`. This is the *only* possible guard on the
residual recorded in `decisions.md` 2026-07-27: every **unconditional** lowercase mapping still
comes from rustc's tables, so a future toolchain could silently change hash output and nothing today
would notice.

Continuing the handoff's "Next" (not a bounce — iteration 156 passed review).

## Scope

- **Create**: `scripts/unicode_sweep.py`, `tests/test_unicode_sweep.py`
- **Modify**: `mise.toml`, `.github/workflows/ci.yml`, `docs/unicode.md`
- **Reference**:
    - `.claude/context/specs/rust-core.md` lines 103–116 (criterion 4 wording) and line 191 (the
        `Verified when` box)
    - `scripts/gen_unicode16_case.py` — the house style for a fail-closed Unicode script
        (`raise SystemExit(...)`, `EXPECTED_*` constants, module docstring)
    - `scripts/check_docs_nav.py` + `tests/test_check_docs_nav.py` — the house style for a gate script
        with injected `Path` arguments and its pytest suite
    - `.github/workflows/ci.yml` `python-test:` job (lines 47–73) — the exact
        checkout/toolchain/setup-python/setup-uv/`uv sync`/`maturin develop` step shape to copy
    - `mise.toml` `bench:iai:check` / `audit` tasks — task-comment style
    - `docs/unicode.md` — the page that documents the freeze rule

File budget: **3** non-test, non-doc files (`scripts/unicode_sweep.py`, `mise.toml`, `ci.yml`).

## Not In Scope

- **Do not edit `.claude/context/specs/ci-cd.md`.** Adding this job moves the CI check-name count;
    the human-authorized "Make the CI job table exhaustive" issue in `issues.md` owns that table and
    will pick the new row up. Specs are human-owned.
- **Do not touch any Rust source, `.crap-baseline.json` or `.iai-baseline.json`.** This step adds no
    Rust code, so neither the CI-only CRAP `--fail-regression` gate nor the 10% Ir gate moves. A
    baseline refresh here would be unexplained noise.
- **Do not extend `crates/iscc-lib/tests/unicode_boundary.json`** or touch any binding conformance
    suite — propagation to C FFI / C++ / Swift and the four sibling `data.json` copies is the next
    slice.
- **Do not wire the sweep into `mise run test`, `uv run pytest`'s default path, or the prek pre-push
    hooks.** It needs a release extension build plus ~60 s and it needs CPython 3.14; a skip-on-3.10
    pytest gate is exactly the fail-open shape to avoid.
- **Do not add CLI flags that shrink the sweep** (`--sample`, `--limit`, `--fast`). The pytest suite
    calls the sweep function directly with a small scalar iterable; a shrink flag is a
    gate-circumvention surface.
- Do not touch `packages/go` or anything on the go1.27 checklist.
- Do not add a new docs page (no `zensical.toml` / `ORDERED_PAGES` / `llms.txt` wiring needed) — add
    a section to the existing `docs/unicode.md`.

## Implementation Notes

### Verified facts — measured this iteration, do not re-derive

Probed in the devcontainer at HEAD (`b95f0ed`) against the freshly rebuilt release extension:

- Project interpreter is **CPython 3.14.6**, `unicodedata.unidata_version == "16.0.0"`; `iscc_core`
    is **1.3.0** and exports `iscc_core.text_clean` / `iscc_core.text_collapse` from the package
    root. On 3.14 `iscc_core` **is** the "uniform Unicode 16.0.0 tables" reference the spec asks for
    — it calls `unicodedata` directly with no freeze rule of its own, so post-16.0 code points are
    `Cn` to it exactly as the sentinel makes them for us. Use it as the oracle; do not reimplement
    `text_clean` / `text_collapse` in Python.
- Scalar denominator: **1,112,064** = `range(0x110000)` minus `0xD800..=0xDFFF`.
- With the 8 contexts below × 2 functions the sweep is **17,793,024** comparisons and takes **59 s**
    wall clock, **0 divergences**. (This is the same total the iteration-156 review reported, so the
    shape is reproducible.)
- `uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml` takes **~21 s**
    incremental here and **always** rewrites `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so`,
    refreshing its mtime even when cargo reports the crates fresh. That makes an mtime-based
    staleness guard safe (no false red after a rebuild).
- `.github/workflows/ci.yml` has **20** job keys today; `python-test` is a 2-leg matrix (3.10 /
    3.14).
- `uv run ty check` passes on `import iscc_core` already (four `tests/*.py` do it), so the new
    script needs **no** `[tool.ty.src] exclude` entry — it is stdlib + installed project deps only.
- Ruff `S` is selected repo-wide and `S101` is ignored **only** under `tests/**`. Use
    `raise SystemExit("...")` for every failure path in `scripts/unicode_sweep.py`; a bare `assert`
    will red `uv run ruff check`.

### The context set (this is the sequence half of criterion 4)

Eight `(name, prefix, suffix)` contexts; for each scalar `c` the case string is
`prefix + c + suffix` and both `text_clean` and `text_collapse` are compared. Build every non-ASCII
piece with `chr(0x...)` so the script file stays pure ASCII:

| name        | prefix                      | suffix        | why                                |
| ----------- | --------------------------- | ------------- | ---------------------------------- |
| `bare`      | `""`                        | `""`          | the single-code-point sweep proper |
| `ascii`     | `"a"`                       | `"b"`         | the boundary-fixture shape         |
| `base_mark` | `"e"`                       | `chr(0x0301)` | spec class **base+Cn+mark**        |
| `jamo`      | `chr(0x1100)`               | `chr(0x1161)` | spec class **jamo+Cn+jamo**        |
| `sigma`     | `chr(0x0391) + chr(0x03A3)` | `chr(0x0392)` | spec class **Sigma+Cn+cased**      |
| `marks`     | `chr(0x0301)`               | `chr(0x0301)` | spec class **Cn-between-marks**    |
| `space`     | `" "`                       | `" "`         | whitespace filter + `.strip()`     |
| `upper`     | `"A"`                       | `"Z"`         | cased neighbours for lowercasing   |

The four spec-mandated sequence classes are covered by rows 3–6, and covered for **every** scalar
rather than only the unassigned ones — a strict superset of what criterion 4 requires. Keep all
eight in one `CONTEXTS` tuple of `(name, prefix, suffix)`.

### Script shape (`scripts/unicode_sweep.py`)

Plain project script run as `uv run scripts/unicode_sweep.py` (**not** PEP 723 `--script`: it must
import the project's built `iscc_lib`). Module docstring first, explaining the oracle, the
denominator and why it is CI-only.

Constants, all fail-closed:

- `EXPECTED_UNIDATA_VERSION = "16.0.0"`
- `EXPECTED_SCALAR_COUNT = 1_112_064`
- `EXPECTED_COMPARISONS = 17_793_024`

Suggested functions (keep each short and pure; inject `Path`s rather than reading module constants,
so the pytest suite can point them at temp dirs):

- `scalar_values() -> Iterator[int]` — yields every code point except the surrogate block.
- `sweep(scalars: Iterable[int]) -> tuple[int, list[Divergence]]` — returns
    `(comparison_count, divergences)`. A `Divergence` can be a plain `NamedTuple`
    (`function, context, code_point, expected, actual`). No printing inside.
- `check_oracle(unidata_version: str) -> None` — `raise SystemExit` unless it equals
    `EXPECTED_UNIDATA_VERSION`, naming the required CPython 3.14.
- `check_extension_fresh(extension: Path, source_dirs: Sequence[Path]) -> None` — `raise SystemExit`
    if the extension's mtime is older than the newest `*.rs` under the given dirs, with a message
    naming `mise run unicode:sweep`. `main()` passes `Path(iscc_lib._lowlevel.__file__)` and
    `[crates/iscc-lib/src, crates/iscc-py/src]`.
- `main() -> int` — oracle check, freshness check, run the sweep, then **three** post-conditions:
    the scalar count equals `EXPECTED_SCALAR_COUNT`, the comparison count equals
    `EXPECTED_COMPARISONS`, and the divergence list is empty. Print at most 20 divergences as
    `U+XXXX` plus the section/context, then the summary line, then return 1.

Final stdout line, exactly (parsed by a verification criterion):

```text
TOTAL 17793024 comparisons, 0 divergences
```

Print the oracle provenance on a line above it (`iscc-core <version>`, `unidata <version>`,
`python <version>`) so a CI log records what was compared.

Order matters: assert the counts **before** reporting success, so a run that generated zero cases
cannot read green.

### `mise.toml`

New `# --- Unicode conformance ---` section with a single task, placed after the performance
section:

```toml
[tasks."unicode:sweep"]
description = "Differential sweep of text_clean/text_collapse against iscc-core on Unicode 16.0.0"
# Rebuilds the Python extension first: the sweep measures whatever `.so` is installed,
# and a stale one silently measures the previous commit.
run = """
uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml &&
uv run scripts/unicode_sweep.py
"""
```

### CI job

Add one job keyed `unicode-sweep` with `name: Unicode sweep (16.0.0 differential)`. Copy the
`python-test` step shape verbatim but pin `python-version: '3.14'` (keep `allow-prereleases: true`
and the `astral-sh/setup-uv@v9.0.0` exact-tag comment), then:

Two steps after `uv sync --group dev`, mirroring `python-test` but with `--release` added:

1. `Build Python bindings (release)` running
    `uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml`
2. `Run Unicode 16.0.0 differential sweep` running `uv run scripts/unicode_sweep.py`

Add a job-level comment explaining why it is a standalone job: it needs a **release** extension (a
debug build makes the 17.8M-comparison run several times slower) and CPython **3.14** specifically
(the `python-test` matrix's 3.10 leg carries Unicode 15.1.0 tables and could only skip). Place it
next to `audit:` / `release-workflow:` at the end of the file — the other standalone gates.

### `tests/test_unicode_sweep.py`

Load the script with `importlib.util.spec_from_file_location` (see `tests/test_check_docs_nav.py`).
Cover, all fast and all unconditional except where noted:

- `scalar_values()` yields `EXPECTED_SCALAR_COUNT` values and no surrogate.
- `sweep()` over a small explicit scalar list returns `len(scalars) * len(CONTEXTS) * 2` comparisons
    and — guarded by `pytest.mark.skipif(unicodedata.unidata_version != "16.0.0")` — zero
    divergences.
- `sweep()` **reports** a divergence when handed a deliberately wrong oracle (monkeypatch the
    module's oracle reference to a lambda returning a fixed string) — proves the comparison is
    load-bearing rather than always-equal.
- `check_oracle("15.1.0")` raises `SystemExit`; `check_oracle("16.0.0")` does not.
- `check_extension_fresh` raises `SystemExit` when a `tmp_path` `.rs` file is newer than a
    `tmp_path` stand-in extension, and returns cleanly when it is older.
- `EXPECTED_COMPARISONS == EXPECTED_SCALAR_COUNT * len(CONTEXTS) * 2` — pins the arithmetic so
    dropping a context cannot silently shrink the sweep.

### `docs/unicode.md`

Add one short section (heading level matching its neighbours), after the case-freeze section: name
the gate command `mise run unicode:sweep`, the CI job name, the oracle (`iscc-core` on CPython
3.14), the denominator (1,112,064 scalars x 8 contexts x 2 functions = 17,793,024 comparisons), and
state that the gate's purpose is the *unguarded* residual — unconditional lowercase mappings and
normalization tables still come from rustc — not the `Final_Sigma` condition it re-proves. Do not
restate the whole freeze rule.

## Verification

- `uv run scripts/unicode_sweep.py` exits 0 and its last stdout line is exactly
    `TOTAL 17793024 comparisons, 0 divergences` (~60 s)
- `mise run unicode:sweep` exits 0 (rebuild then sweep)
- Stale-extension guard fires: `touch crates/iscc-lib/src/utils.rs` then
    `uv run scripts/unicode_sweep.py` exits non-zero with a message naming the rebuild; afterwards
    `mise run unicode:sweep` exits 0 again and `git status --porcelain` is empty
- Comparison-count guard fires: with one entry temporarily removed from `CONTEXTS`,
    `uv run scripts/unicode_sweep.py` exits non-zero on the **count** assertion (not on a
    divergence); after restoring, `git status --porcelain scripts/unicode_sweep.py` is empty
- `uv run pytest -q` passes (379 existing + the new tests, zero errors) and
    `uv run pytest -q tests/test_unicode_sweep.py` collects a non-zero number of tests
- `uv run ruff check`, `uv run ruff format --check` and `uv run ty check` are all clean
- `uv run python -c "import yaml; j=yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']['unicode-sweep']; assert j['name']=='Unicode sweep (16.0.0 differential)'; s=[str(x.get('run','')) for x in j['steps']]; assert any('unicode_sweep.py' in x for x in s); assert any('--release' in x for x in s)"`
    exits 0
- `uv run zensical build` exits 0 and reports "No issues found"; `uv run scripts/check_docs_nav.py`
    still reports 23 consistent pages
- `git status --porcelain -- crates/ .crap-baseline.json .iai-baseline.json .claude/context/specs/`
    prints nothing (no Rust source, baseline or spec moved)
- `mise run check` exits 0 (all prek hooks) and `git status --porcelain` is empty afterwards

## Done When

`scripts/unicode_sweep.py` is committed, runs green as `mise run unicode:sweep` and as a dedicated
CPython 3.14 CI job, fails closed on a stale extension / wrong oracle version / shrunken case set,
and every verification command above passes on the working tree.
