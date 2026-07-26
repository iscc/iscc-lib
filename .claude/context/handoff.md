# Handoff

## 2026-07-26 — Gate `release.yml` action-input compatibility in CI (`--check-action-inputs`)

**Done:** Extended `scripts/check_release_workflow.py` with an opt-in `--check-action-inputs` mode
(check 4): for every repo-action `uses:` ref in `release.yml` it fetches the published
`action.yml`/`action.yaml` from raw GitHub (stdlib `urllib`, 20s timeout, one cached fetch per
distinct ref) and validates all `with:` keys against declared `inputs` and all
`steps.<id>.outputs.<x>` reads from action steps against declared `outputs`. A 404 on both metadata
URLs is an error; any transport failure degrades to a `warning: skipped <ref>: <reason>` stderr line
and exit 0. Wired a new `release-workflow` CI job that runs the mode, extended the docs bullet, and
added 8 network-free tests via an injected fake fetcher.

**Files changed:**

- `scripts/check_release_workflow.py`: new `ActionNotFoundError`, `is_repo_action`,
    `action_yml_urls`, `fetch_action`, `cached_fetch`, `iter_strings`, `check_with_keys`,
    `check_step_outputs`, `check_action_compat`; `--check-action-inputs` flag in `main()`; module
    docstring documents check 4. The fetcher is injected into `check_action_compat` (via
    `functools.partial`-based caching), so tests never touch the network. One `# noqa: S310` on
    `urlopen` with an inline justification (URL built from the `https://raw.githubusercontent.com`
    literal — scheme not attacker-controllable).
- `.github/workflows/ci.yml`: new `release-workflow` job ("Release workflow (action inputs)") —
    checkout + `astral-sh/setup-uv@v9.0.0` (with the mandated `# exact tag:` comment) +
    `uv run --no-project --with pyyaml python scripts/check_release_workflow.py   --check-action-inputs`.
    **CI job count goes 19 → 20 — expected, not drift.**
- `tests/test_check_release_workflow.py`: 8 new tests — `action_yml_urls` covering all four ref
    shapes (plain, sub-path `oxidize-rb/actions/cross-gem@v1`, slash ref `@release/v1`, branch ref
    `@stable`), `is_repo_action` (docker/local/no-`@` skipped), and `check_action_compat` with an
    in-memory fake fetcher: clean, undeclared `with:` key, undeclared step output, 404 → single
    error despite two uses (cache), `None`-returning fetcher → skip, local `run:`-step / unknown-id
    output references ignored.
- `docs/development.md`: extended the `check_release_workflow.py` bullet with the CI-only
    `--check-action-inputs` mode and its offline-skip semantics.

**Verification:** (all observed in this session)

- `uv run scripts/check_release_workflow.py` → exit 0 (default mode unchanged, no network)
- `uv run scripts/check_release_workflow.py --check-action-inputs` → exit 0, **zero**
    `warning: skipped` lines with live network (all 18 distinct refs compatible at HEAD, matching
    next.md's scoping probe)
- Offline (`https_proxy=http_proxy=http://127.0.0.1:9`): exit 0 with **18**
    `warning: skipped …   Connection refused` lines — transport failure degrades to skip, never a
    red gate
- End-to-end negative check: mutating `fetch-depth` → `fetch-depthz` in a temp copy of `release.yml`
    → exit 1 with
    `action: job 'prepare-release' passes undeclared input   'fetch-depthz' to 'actions/checkout@v7'`
    (real fetcher, real 18-ref run)
- `uv run pytest tests/test_check_release_workflow.py` under the dead-proxy env → 19 passed (11
    existing + 8 new), proving the suite stays network-free
- `grep -q -- "--check-action-inputs"` on both `ci.yml` and `docs/development.md` → OK; `ci.yml`
    parses via `yaml.safe_load`; `actionlint@v1.7.7` → clean
- `uv run ruff check` → "All checks passed!"; `uv run ruff format --check` → 160 files already
    formatted; `uv run ty check` → all passed
- `mise run check` → all 16 hooks Passed; full `uv run pytest` → 333 passed
- `git status --porcelain .github/workflows/release.yml` → empty (workflow under test untouched)

**Next:** The issue's remaining checklist item is closed by this step (review updates issues.md and
the ci-cd spec job table — 19 → 20 jobs). Remaining unblocked candidates are thin: the
`zensical.toml` nav ↔ `ORDERED_PAGES` ↔ `docs/llms.txt` parity gate still needs to be filed as an
issue (next.md asked review to do this — see below), and everything Unicode/rubygems-pin remains
human-gated.

**Notes:**

- **Request relayed from next.md's Not-In-Scope:** review should file the docs-parity gate
    (`zensical.toml` nav ↔ `gen_llms_full.py` `ORDERED_PAGES` ↔ `docs/llms.txt`) as an issues.md
    entry so it stops being carried in prose only.
- The `# noqa: S310` is the only suppression added; RUF100 keeps it honest (it would flag if
    unused). Ruff `C901` needed no suppressions — the check is split across 6 small functions.
- Error-vs-skip semantics follow next.md exactly: 404 on both URLs = error (wrong ref/sub-path);
    HTTP 403/429/5xx and all `OSError`/`URLError`/timeout = warning + skip. `HTTPError` is caught
    before the general `OSError` (it is a subclass).
- Step-output resolution is per job: references to ids that are local `run:` steps or absent from
    the job are ignored (verified against the five real references in `release.yml`: two local, two
    action-declared, one action-declared cache-hit).
- yamlfix folded the CI job's `run:` line onto a continuation line — still a single plain scalar,
    executes as one command; `check yaml` + actionlint both pass.
- The new CI job runs on every push. If GitHub raw ever rate-limits CI runners (HTTP 429), the job
    prints warnings and stays green by design — a fully-skipped run is visible in the log but not
    distinguishable from a pass by status alone. Accepted trade-off per the issue's "gated to skip
    offline" requirement.
