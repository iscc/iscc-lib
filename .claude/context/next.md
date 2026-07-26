# Next Work Package

## Step: Gate `release.yml` action-input compatibility in CI (`--check-action-inputs`)

## Goal

Close check 3 of the tracked `normal` issue "Gate `release.yml` action-input compatibility in CI"
(issues.md, `[review]`): extend `scripts/check_release_workflow.py` with an opt-in, network-fetching
`--check-action-inputs` mode that validates every `with:` key and every `steps.<id>.outputs.<x>`
reference in `.github/workflows/release.yml` against each action's published `action.yml`, and wire
it into `ci.yml`. This is the last release-workflow invariant still performed by hand on every
action bump (iters 127, 140) and the one that catches an input silently dropped across a major.

**Cadence note:** this is workflow tooling, which state.md flags as over-weighted. Weighed and taken
anyway — the last four iterations were 140 tooling / 141 library tests / 142 tooling / 143 docs (2
of 4, below the "3 of the last 4" threshold in the scoping memory), and this is the only fully
unblocked tracked issue: the Unicode binding propagation is double-blocked on two human rulings, and
the `rubygems/configure-rubygems-credentials` pin needs a tag-vs-SHA ruling.

## Scope

- **Modify**:
    - `scripts/check_release_workflow.py` — add the `--check-action-inputs` mode (stdlib `urllib`
        only, no new dependency)
    - `.github/workflows/ci.yml` — one new job that runs the mode
    - `docs/development.md` (docs, outside the file budget) — extend the
        `scripts/check_release_workflow.py` bullet (~line 145) with the CI-only network check
    - `tests/test_check_release_workflow.py` (tests, outside the file budget) — network-free tests for
        the new pure functions via an injected fake fetcher
- **Reference**:
    - `scripts/check_release_workflow.py` (current structure: `run_checks`, `main`, per-check
        functions returning `list[str]` error strings)
    - `tests/test_check_release_workflow.py` (existing anchor + mutation test style)
    - `.github/workflows/release.yml` (the file under test)
    - `.github/workflows/ci.yml` lines 12–81 (`version-check` and `python-test` job patterns,
        including the `# exact tag:` comment on `astral-sh/setup-uv@v9.0.0`)
    - `.claude/context/issues.md` → "Gate `release.yml` action-input compatibility in CI" (the issue
        this step cites)

## Not In Scope

- **Do not change `.github/workflows/release.yml`.** All 18 distinct action refs are compatible at
    HEAD (verified during scoping, see Implementation Notes) — the mode must land green, not as a
    fix. If it reports a real error, report it in the handoff instead of editing `release.yml`.
- Do not touch the `rubygems/configure-rubygems-credentials@main` floating ref — that is a separate
    human-gated issue (tag-vs-SHA convention).
- Do not add the network check to prek / `mise run check` / the pytest suite. prek and pytest must
    stay network-free; this mode is CI-only and offline-skipping.
- Do not add a new Python dependency (no `requests`, no `httpx`, no `gh` CLI shell-out) — stdlib
    `urllib.request` plus the already-declared `pyyaml` is sufficient.
- Do not edit `.claude/context/issues.md` or `.claude/context/specs/ci-cd.md` — issue progress and
    the spec's CI job table are the review agent's to update.
- Do not build the separate `zensical.toml` nav ↔ `ORDERED_PAGES` ↔ `docs/llms.txt` parity gate
    flagged in the handoff. **Request for review:** please file that as an issue so it stops being
    carried in prose only.

## Implementation Notes

### Facts verified during scoping (probe scripts, real network)

- `release.yml` has **18 distinct** `uses:` refs. Every `with:` key on all 18 is a declared `inputs`
    key of the published `action.yml`, and every action-sourced step output resolves — so the mode
    must exit 0 at HEAD.
- Ref shapes that must all parse:
    - plain: `actions/checkout@v7`
    - **sub-path action**: `oxidize-rb/actions/cross-gem@v1` →
        `.../oxidize-rb/actions/v1/cross-gem/action.yml` (owner/repo = first two segments, the rest is
        the in-repo directory)
    - **ref containing a slash**: `pypa/gh-action-pypi-publish@release/v1`
    - **branch refs**: `dtolnay/rust-toolchain@stable`, `rubygems/configure-rubygems-credentials@main`
- Raw fetch works unauthenticated:
    `https://raw.githubusercontent.com/<owner>/<repo>/<ref>/<subpath>/action.yml` (split the ref on
    the **first** `@`; do not URL-encode the slashes in the ref). Fall back to `action.yaml`.
- The five `steps.<id>.outputs.<x>` references in `release.yml`: `steps.check.outputs.skip` and
    `steps.version.outputs.version` come from local `run:` steps (**must be ignored** — no `uses:`,
    nothing to validate); `steps.crates-auth.outputs.token`, `steps.setup-ndk.outputs.ndk-path` and
    `steps.xcf-cache.outputs.cache-hit` come from actions and are all declared.

### Design

Keep the existing shape: pure functions returning `list[str]` error strings, composed in `main()`.

```text
FETCH_TIMEOUT = 20  # seconds

action_yml_urls(ref) -> list[str]      # ["…/action.yml", "…/action.yaml"], pure & testable
fetch_action(ref) -> dict | None       # None == skipped (network unavailable); raises/returns
                                       #   an error marker on a real 404
check_action_compat(wf, fetch) -> list[str]   # pure: takes the fetcher as a parameter
```

- `check_action_compat` takes the fetcher **as an argument** so the pytest suite injects a fake
    dict-returning fetcher and never touches the network.
- Cache fetches per distinct ref (a plain `dict`) — 18 requests per run, not 90.
- Skip refs that are not repo actions (`docker://…`, `./local-action`); none exist today, but the
    parser must not crash if one is added.
- **Error vs skip is the crux of the design:**
    - *error* (collected, exit 1): a `with:` key absent from the action's `inputs`; a
        `steps.<id>.outputs.<x>` where `<id>` is a step **with** a `uses:` in the same job and the
        action declares no such `outputs` key; **404 on both `action.yml` and `action.yaml`** (the ref
        or the sub-path is wrong — a real defect).
    - *skip* (warning on stderr, does not fail): any transport failure — `urllib.error.URLError`,
        socket timeout, HTTP 403/429/5xx. Print one `warning: skipped <ref>: <reason>` line so a
        silently-degraded CI run is visible in the log.
- Resolve `steps.<id>.outputs.<x>` **per job**: walk the job's nested structure collecting every
    string, regex out the references, and match `<id>` against that job's step ids. A reference to
    an id that is not a step in the same job is ignored (do not guess).
- `--check-action-inputs` is additive: it runs the existing local checks **and** the network check,
    so the CI step is a single command.
- Update the module docstring to describe check 4 and its offline-skip semantics.
- Ruff `C901` and `S` are enforced — keep the new functions small (extract a helper rather than
    nesting), and if a `# noqa` is unavoidable, justify it inline.

### CI wiring

Add one job to `.github/workflows/ci.yml` (mirroring the `version-check` job's brevity):

```text
  release-workflow:
    name: Release workflow (action inputs)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      # exact tag: setup-uv publishes no floating major tag past v7 (v8/v9 are
      # exact release tags only), so @v9 does not resolve
      - uses: astral-sh/setup-uv@v9.0.0
      - name: Check release workflow action inputs
        run: <one line> uv run --no-project --with pyyaml python
             scripts/check_release_workflow.py --check-action-inputs
```

(The `run:` value is a **single line** — the wrap above is only for this document. `yamlfix` will
normalise indentation, so run `uv run prek run --files .github/workflows/ci.yml` after editing.)

`uv run --no-project --with pyyaml python scripts/check_release_workflow.py` was verified working in
this environment (exit 0, ~4s). This adds one job and one check name to CI — expected, and worth
calling out in the handoff so update-state does not read the changed job count as drift.

### Tests (network-free)

Extend `tests/test_check_release_workflow.py` in the existing style:

- `action_yml_urls` unit tests for the four ref shapes above (plain, sub-path, slash-containing ref,
    branch ref).
- `check_action_compat` with an in-memory workflow dict + fake fetcher: clean case → no errors;
    undeclared `with:` key → one error naming the key and the ref; undeclared step output → one
    error; fetcher returning `None` (skip) → no errors.
- A test that a local `run:` step's `steps.<id>.outputs.<x>` reference produces no error.
- Do **not** add an anchor test that fetches the real actions — the suite stays offline.

## Verification

- `uv run scripts/check_release_workflow.py` exits 0 (default mode unchanged, no network use)
- `uv run scripts/check_release_workflow.py --check-action-inputs` exits 0, and its output contains
    no `warning: skipped` line when the network is available
- Offline behaviour:
    `https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9 uv run   scripts/check_release_workflow.py --check-action-inputs`
    exits **0** and prints at least one `warning: skipped` line (transport failure degrades to a
    skip, never to a red gate)
- `uv run pytest tests/test_check_release_workflow.py` passes (existing tests plus the new ones)
- The suite is network-free:
    `https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9 uv run   pytest tests/test_check_release_workflow.py`
    passes identically
- `grep -q -- "--check-action-inputs" .github/workflows/ci.yml` and
    `grep -q -- "--check-action-inputs" docs/development.md`
- `uv run python -c "import yaml,pathlib;yaml.safe_load(pathlib.Path('.github/workflows/ci.yml').read_text())"`
    exits 0 (the new job parses)
- `uv run ruff check` → "All checks passed!" and `uv run ruff format --check` exits 0
- `mise run check` → all hooks `Passed` (the `check-release-workflow` prek hook still runs the
    default, network-free mode)
- `git status --porcelain .github/workflows/release.yml` is empty (the gate lands green; the
    workflow under test is not edited)

## Done When

`--check-action-inputs` validates every `release.yml` action ref against its published `action.yml`,
exits 0 at HEAD, degrades to a warning-only skip when the network is unavailable, is covered by
network-free pytest tests, and runs as a dedicated CI job — with all verification commands above
passing.
