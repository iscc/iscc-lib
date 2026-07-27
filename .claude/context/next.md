# Next Work Package

## Step: Harden the Unicode sweep gate — require a rebuild, bound the divergence list

## Goal

Close the two blind spots filed as the `normal` `[review]` issue **"Harden the Unicode differential
sweep gate"**: a bare `uv run scripts/unicode_sweep.py` can report a **false green** from an
extension that predates a `cargo update` or a rustc bump (exactly the upgrade the gate exists to
validate), and `sweep()` retains *every* divergence, so a broad regression can OOM-kill CI before it
prints a single diagnostic — turning the most informative failure into the least.

Not a bounce: iteration 157 passed review (PASS_WITH_NOTES) and this is the follow-up the review
agent filed, which both the handoff and state.md name as the better next step ahead of the C-FFI
propagation slice (that slice still needs an unmade design call about how a C test reads JSON).

## Scope

- **Modify** (3 non-test/non-doc files — the budget, exactly):
    - `scripts/unicode_sweep.py` — add the rebuild-assertion guard, bound divergence retention, fix
        the vacuous freshness pass
    - `mise.toml` — the `unicode:sweep` task passes the new flag after its `maturin develop` step
    - `.github/workflows/ci.yml` — the `unicode-sweep` job's sweep step passes the new flag
- **Modify** (tests/docs, outside the budget):
    - `tests/test_unicode_sweep.py` — update the four `sweep()` call sites to the new return shape;
        add the three new guard tests
    - `docs/unicode.md` — one sentence in "Differential sweep gate": the script refuses to run unless
        the caller just rebuilt, so `mise run unicode:sweep` (or the CI job) is the only way to run it
- **Reference**:
    - `.claude/context/issues.md` → "Harden the Unicode differential sweep gate" (the authorization)
    - `.claude/context/decisions.md` 2026-07-27 → "The Unicode sweep gate trusts an mtime freshness
        guard, not a build-input hash" (which alternatives are already rejected)
    - `.claude/context/handoff.md` (iteration-157 review: the two findings, both reproduced there)

## Not In Scope

- **Do not widen the mtime set** to `Cargo.toml` / `Cargo.lock` / `rust-toolchain*`. `decisions.md`
    2026-07-27 rejects it as buying a *false* sense of completeness (still blind to a rustc-only
    bump), and the rebuild assertion below makes it moot.
- **Do not make the script shell out to `maturin` itself** — same ruling: it removes the fast
    fail-fast signal and makes the script un-runnable without a build.
- **Do not embed build fingerprints in the Rust extension** (a `__build_info__`-style symbol would
    add an unbound public symbol to the Python surface and breaks the 32-symbol Tier 1 story).
- **Do not wire the full 17.8M-comparison sweep into `pytest`, `mise run test` or the pre-push
    hooks.** The pytest suite must keep exercising `sweep()` with tiny explicit scalar lists only.
- **Do not change the success line format.** `TOTAL 17793024 comparisons, 0 divergences` must stay
    byte-identical — state.md, the review handoff and agent memory all assert it.
- No Rust source, no `.crap-baseline.json`, no `.iai-baseline.json`, no `.claude/context/specs/`
    edits. No Rust code moves in this step, so **neither** the CI-only CRAP `--fail-regression`
    baseline nor the iai Ir baseline may be refreshed.
- Do not start propagation slice 5 (C FFI / C++ / Swift) and do not touch
    `crates/iscc-lib/tests/unicode_boundary.json` or any sibling `data.json`.
- Do not edit `.claude/context/decisions.md` or delete the issue from `issues.md` — the review agent
    owns both.

## Implementation Notes

Measured on the working tree while scoping (all facts below are current, not inherited):

- The tree is green *before* this step and must stay green — `mise run unicode:sweep` exits 0 with
    this as its last stdout line:

    ```text
    TOTAL 17793024 comparisons, 0 divergences
    ```

- `uv run pytest -q --collect-only` → **389** tests.

- Retention is 1:1 — `sweep()` with a wrong oracle over 30 scalars: **480 comparisons, 480 retained
    tuples**, `MAX_REPORTED_DIVERGENCES = 20`. At full scale that is up to 17.8M two-string tuples.

- `mise.toml` already uses `env = { … }` on a task (`bench:iai`), so either an env key or a CLI flag
    is available. **Prefer the CLI flag** — it is visible in the CI log line and in `mise.toml`, and
    it needs no cross-platform env-prefix shell syntax.

### 1. Rebuild assertion (issue item 1, the completeness half)

Add a module-level `REBUILD_FLAG = "--rebuilt"` and a small, directly testable guard:

```python
def check_rebuilt(argv: Sequence[str]) -> None:
    """Fail closed unless the caller rebuilt the extension in this invocation."""
```

Raise `SystemExit` naming `mise run unicode:sweep` when the flag is absent. Make `main` take
`argv: Sequence[str]` (call it as `sys.exit(main(sys.argv[1:]))`) and call `check_rebuilt(argv)`
**first**, before `check_oracle` — the failure must arrive before any sweeping, and stdout must
carry no `TOTAL` line.

Why this and not the mtime widening: the issue explicitly lists "make the direct invocation refuse
to run outside `mise run unicode:sweep`" as an option, and it *implements* the operating rule that
`decisions.md` 2026-07-27 already recorded as convention ("always re-run the sweep through
`mise run unicode:sweep`, never the script directly"). It therefore hardens that ruling rather than
reversing it — say so in the code comment, and mention the residual honestly: the flag is a caller
*assertion*, so a human who passes it without rebuilding is trusted.

Keep `check_extension_fresh` — the flag is an assertion, the mtime check is an observation, and the
cheap redundancy still catches "exported the flag, edited a `.rs`, forgot to rebuild".

Wire the flag through both authoritative paths:

- `mise.toml` task `unicode:sweep` — append the flag to the sweep line only; the build line above it
    stays byte-identical and still `&&`-chained, so a failed build never reaches the sweep:

    ```bash
    uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml &&
    uv run scripts/unicode_sweep.py --rebuilt
    ```

- `.github/workflows/ci.yml`, job key `unicode-sweep`, step "Run Unicode 16.0.0 differential sweep"
    → same flag. Nothing else in that job changes.

### 2. Vacuous freshness pass (issue item 1, the trivial half)

`check_extension_fresh` currently uses `max(…, default=0.0)`, so a missing/empty source directory
passes silently. Collect the `*.rs` paths first and `raise SystemExit` when the list is empty,
naming the directories it looked in. One check covers both "directory missing" and "no sources
found". `grep default=0.0` must come back empty afterwards.

### 3. Bounded divergence retention (issue item 2)

Give `sweep()` an explicit result shape rather than a bare tuple:

```python
class SweepResult(NamedTuple):
    comparisons: int
    divergences: int  # every divergence, counted
    samples: list[Divergence]  # at most MAX_REPORTED_DIVERGENCES, retained in order
```

`sweep()` increments `divergences` unconditionally and appends to `samples` only while
`len(samples) < MAX_REPORTED_DIVERGENCES`. `main()` prints `result.samples` and keeps the final line
exactly `f"TOTAL {result.comparisons} comparisons, {result.divergences} divergences"`; return
`1 if result.divergences else 0`. An extra "showing first N of M" line before the TOTAL line is
welcome (it only prints on failure), but the TOTAL line itself must not change.

Four existing tests unpack `count, divergences = us.sweep(...)` — update all four
(`test_sweep_comparison_count`, `test_sweep_zero_divergences_on_boundary_scalars`,
`test_sweep_reports_divergence_with_wrong_oracle`,
`test_contexts_discriminate_the_delete_filter_design`). The last one sweeps a single scalar (16
comparisons < 20), so its `exposed` set is unaffected by the cap — keep it asserting the same three
`(function, context)` pairs; do **not** weaken it.

### Traps

- `ty check` and the Ruff `S`/`C901` selections are **pre-push-only** — `mise run check` cannot see
    them. Run `uv run ty check` explicitly after editing the `.py` files (this bit the iteration-157
    review).
- The pytest module is loaded via `importlib`, so patch module attributes with
    `monkeypatch.setattr(us, …)`, never direct assignment (`ty` rejects the latter).
- Keep `scripts/unicode_sweep.py` pure ASCII — non-ASCII pieces are built with `chr(0x…)`.
- `uv run zensical build` wipes `site/`; run it before any `gen_llms_full.py` invocation.

## Verification

- `timeout 60 uv run scripts/unicode_sweep.py` exits **non-zero**, its message names
    `mise run unicode:sweep`, and its stdout contains **no** line starting with `TOTAL` (it fails
    before sweeping, so it also returns in a couple of seconds)
- `mise run unicode:sweep` exits 0 and its last stdout line is byte-exactly
    `TOTAL 17793024 comparisons, 0 divergences`
- `grep -c "default=0.0" scripts/unicode_sweep.py` → `0`
- `uv run pytest -q tests/test_unicode_sweep.py` passes (10 updated + at least 3 new cases) in under
    10 s, and includes tests asserting all three of:
    - `sweep()` with a wrong oracle over 30 scalars gives `comparisons == 480`, `divergences == 480`,
        `len(samples) == us.MAX_REPORTED_DIVERGENCES`
    - `check_rebuilt([])` raises `SystemExit` whose message contains `mise run unicode:sweep`, and
        `check_rebuilt([us.REBUILD_FLAG])` returns `None`
    - `check_extension_fresh` raises `SystemExit` when the source dirs yield no `*.rs` (empty
        directory and missing directory both)
- `uv run pytest -q` passes (389 pre-existing + the new cases, none removed)
- CI job shape holds:
    `uv run python -c "import yaml,sys; j=yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']['unicode-sweep']; s=[x for x in j['steps'] if 'unicode_sweep.py' in str(x.get('run',''))]; assert len(s)==1 and '--rebuilt' in s[0]['run']; print('OK')"`
    prints `OK`
- `uv run ruff check` , `uv run ruff format --check` and `uv run ty check` all exit 0
- `uv run zensical build` exits 0 with "No issues found"; `uv run scripts/check_docs_nav.py` prints
    `OK: 23 documentation pages consistent`; `docs/unicode.md` states the sweep is run via
    `mise run unicode:sweep` and that the script refuses a bare invocation
- `mise run check` exits 0 (all prek hooks)
- `git status --porcelain -- crates/ .crap-baseline.json .iai-baseline.json .claude/context/specs/`
    is empty (no Rust source, no baseline, no spec moved), and
    `git status --porcelain -- crates/iscc-lib/tests/unicode_boundary.json` is empty

## Done When

All verification criteria pass: a bare script invocation fails closed, the authoritative
`mise run unicode:sweep` and CI paths still report `TOTAL 17793024 comparisons, 0 divergences`, the
divergence list is provably capped at 20 retained samples while the count stays exact, and no Rust
source, baseline or spec moved.
