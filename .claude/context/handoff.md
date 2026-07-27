# Handoff

## 2026-07-27 — Land the Unicode 16.0.0 differential sweep as a permanent, fail-closed gate

**Done:** Committed the throwaway full-code-space sweep as a permanent gate:
`scripts/unicode_sweep.py` compares `iscc_lib.text_clean` / `text_collapse` against the installed
`iscc_core` (1.3.0) oracle on CPython 3.14 over all 1,112,064 scalar values × 8 contexts × 2
functions = 17,793,024 comparisons, fail-closed on oracle version, stale extension, scalar count,
comparison count, and any divergence. Wired as `mise run unicode:sweep` (rebuild-then-sweep) and a
new standalone `unicode-sweep` CI job (21st job), with a pytest suite and a `docs/unicode.md`
section. Closes criterion 4 of `specs/rust-core.md`.

**Files changed:**

- `scripts/unicode_sweep.py` (new): the gate — `CONTEXTS` (8 rows incl. the four spec-mandated
    sequence classes), `FUNCTION_PAIRS` (read at call time so tests can monkeypatch the oracle),
    `Divergence` NamedTuple, `scalar_values()`, `sweep()` (pure, no printing), `check_oracle()`,
    `check_extension_fresh()` (mtime vs newest `*.rs` under `crates/iscc-lib/src` +
    `crates/iscc-py/src`), `main()` asserting both counts before printing the success line
- `tests/test_unicode_sweep.py` (new): 9 tests — scalar denominator + no surrogates, comparisons
    arithmetic pin (`EXPECTED_COMPARISONS == scalars × 8 × 2`), per-scalar comparison count,
    zero-divergence boundary sample (skipif unidata ≠ 16.0.0), wrong-oracle monkeypatch proves the
    comparison is load-bearing, both `check_oracle` directions, both `check_extension_fresh`
    directions (explicit `os.utime`, no timing races)
- `mise.toml`: new `# --- Unicode conformance ---` section with `unicode:sweep` task (maturin
    `--release` rebuild && sweep), placed after the performance section
- `.github/workflows/ci.yml`: new `unicode-sweep` job at end of file (after `release-workflow`),
    name `Unicode sweep (16.0.0 differential)`, python-test step shape with `python-version: '3.14'`
    pinned, `--release` build step, job-level comment explaining why it is standalone
- `docs/unicode.md`: new "Differential sweep gate" section between the case-freeze and boundary
    sections — names the mise task, CI job, oracle, denominator, and the unguarded-residual purpose

**Verification:** (all run this session)

- `uv run scripts/unicode_sweep.py` exit 0; last stdout line exactly
    `TOTAL 17793024 comparisons, 0 divergences`; provenance line above it
    (`oracle: iscc-core 1.3.0, unidata 16.0.0, python 3.14.6`)
- `mise run unicode:sweep` exit 0 (run twice — initial and after the stale-guard probe)
- Stale guard fires: `touch crates/iscc-lib/src/utils.rs` → script exit 1 with the message
    "\_lowlevel.abi3.so is older than the Rust sources; rebuild and rerun via 'mise run
    unicode:sweep'" (actual message uses backticks); `mise run unicode:sweep` afterwards exit 0
- Count guard fires: with the `space` context temporarily removed, exit 1 on
    `expected 17793024 comparisons, got 15568896` — no divergence/TOTAL output, so the failure is
    the count assertion; context restored (current file re-verified by the later green runs and
    `mise run check`)
- `uv run pytest -q`: **388 passed** (379 existing + 9 new); `tests/test_unicode_sweep.py` alone: 9
    passed in 0.61 s
- `uv run ruff check`, `uv run ruff format --check`, `uv run ty check` all clean (no new ty
    exclusion needed, as next.md predicted)
- next.md's PyYAML job-shape one-liner exit 0 (job name, `unicode_sweep.py` step, `--release` step
    all present)
- `uv run zensical build` exit 0, "No issues found"; `uv run scripts/check_docs_nav.py` → OK, 23
    pages (no nav wiring needed — existing page)
- `git status --porcelain -- crates/ .crap-baseline.json .iai-baseline.json   .claude/context/specs/`
    empty — no Rust source, baseline, or spec moved
- `mise run check` exit 0 (all prek hooks); porcelain after format+check shows only the intended 6
    files plus the runner-owned `iterations.jsonl`

**Next:** Propagation slice 5 — the remaining 3 binding surfaces for the boundary fixture (C FFI /
C++ / Swift). All three were flagged blocked on missing toolchains in this container (no `cmake`, no
`swift`), so define-next should either scope a C-FFI-only slice (gcc is present — the
`tests/test_iscc.c` harness could gain a boundary-vector section with a generated header or
hand-pinned expected strings) or hand the slice to a human/different runner. The four sibling
`data.json` copies remain untouched afterwards.

**Notes:**

- The CI job's Unicode-version comment says the 3.10 leg carries "pre-16.0 Unicode tables" rather
    than next.md's "15.1.0" — CPython 3.10 actually ships Unicode 13.0.0 (15.1.0 is 3.13); the
    vaguer phrasing is accurate for the whole matrix without contradicting the spec's version map.
- The sweep ran full three times this session (initial task run, direct run, post-stale-guard task
    run) plus one deliberately shrunken run — all four consistent with the iteration-156
    measurements (~60 s, 0 divergences).
- `check_extension_fresh` compares against the newest `*.rs` under both `crates/iscc-lib/src` and
    `crates/iscc-py/src`; it does not watch `Cargo.toml`/`Cargo.lock` or transitive deps. That is
    the same blind spot any mtime guard has — the mise task and CI job both rebuild unconditionally,
    so the guard only has to catch the "ran the script directly after editing Rust" case, which it
    does.
- `iterations.jsonl` is modified in the working tree (runner-owned, per ledger rule) — left
    unstaged.
- No API surface, benchmarked hot path, or Rust code touched; no baseline refresh needed.
