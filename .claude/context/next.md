# Next Work Package

## Step: Land the `release.yml` static checks 1–2 as an executable gate

## Goal

Turn the two pure-local `release.yml` invariants that have been hand-retyped into throwaway heredocs
in three consecutive iterations (139, 140, 141) — the registry-guard shape and the artifact wiring —
into a committed checker script wired into both prek (local, on every edit of that file) and pytest
(so CI enforces it too). This picks up the `normal` `[review]` issue **"Land the `release.yml`
static checks as an executable gate"**, the only fully-unblocked CID-doable item; the Unicode
propagation half stays parked on Titusz.

## Scope

- **Create**: `scripts/check_release_workflow.py`, `tests/test_check_release_workflow.py`
- **Modify**: `.pre-commit-config.yaml`, `pyproject.toml` (declare `pyyaml` in the dev group),
    `uv.lock` (regenerated), `CLAUDE.md` (pre-commit hook list, ~line 181), `docs/development.md`
    (pre-commit hook list, ~line 143)
- **Reference**: `.github/workflows/release.yml` (the file being gated), `scripts/iai_regression.py`
    \+ `tests/test_iai_regression.py` (the house pattern for a checker script and its by-path
    `importlib` test harness), `.claude/context/issues.md` → "Land the `release.yml` static checks
    as an executable gate", `.claude/context/decisions.md` 2026-07-25 (why the
    `!cancelled() && !failure()` guard shape is only safe in exactly this form)

Non-test, non-doc file budget used: 3 (`scripts/check_release_workflow.py`,
`.pre-commit-config.yaml`, `pyproject.toml`). `uv.lock` is generated.

## Not In Scope

- **Check 3 (action-input compatibility)** — validating every `with:` key against each action ref's
    published `action.yml` needs network, so it belongs in a CI-only follow-up step. Do not add it,
    not even behind an offline skip.
- **Editing `.github/workflows/release.yml`.** The gate must pass on the file exactly as it stands;
    if a check fires, the check is wrong, not the workflow. (If you find a genuine workflow bug,
    report it in the handoff instead of fixing it here.)
- Adding an `actionlint` invocation, a `ci.yml` step, or a new `mise` task — CI enforcement comes
    free via the pytest test, and prek covers the local edit path.
- The `rubygems/configure-rubygems-credentials@main` pin (human-gated tag-vs-SHA ruling) and any
    other `uses:` change.
- Unicode step (b): propagating `crates/iscc-lib/tests/unicode_boundary.json` into the bindings —
    double-blocked on the parked ordering ruling and the Go 15.0-tables decision.
- Refactoring `scripts/iai_regression.py` or extracting shared script helpers.

## Implementation Notes

**Script shape.** `scripts/check_release_workflow.py`: module docstring explaining why the file
needs a static gate (`workflow_dispatch`-only → no CI run and no CID push ever executes it), a
positional **optional** path argument defaulting to `.github/workflows/release.yml` (tests and
probes run it against mutated temp copies), short pure functions returning `list[str]` of error
strings, and a `main()` that prints one error per line and exits 1 if any. Keep functions small —
`C901` and the `S` rules are enforced by `uv run ruff check`.

**PyYAML gotcha:** YAML 1.1 parses the top-level `on:` key as the boolean `True`, so read the
trigger block as `wf.get("on", wf.get(True))`. Use `yaml.safe_load`, never `yaml.load`.

**Check 1 — registry-guard shape** (verified structurally at HEAD: 29 jobs):

- `prepare-release` is the one deliberately unguarded job; its `if` must be exactly
    `inputs.version != ''`. Pin its name in a module constant with a comment saying why wrapping it
    would make the tag-pushing job run on registry-only dispatches.
- Every other job's `if` must match
    `^\$\{\{ !cancelled\(\) && !failure\(\) && \((?P<inner>.+)\) \}\}$`, and its `inner` must
    contain `inputs.version != ''` as an alternative.
- Every `inputs.<name>` referenced in any job `if` must be a declared `on.workflow_dispatch.inputs`
    key, and every declared registry input (all inputs except `version`) must be referenced by at
    least one job — that pair catches both a typo'd flag and a silently dropped one.
- **Do not hardcode the registry-token histogram** (`version:29, npm:6, …`). Frozen counts drift
    with any legitimate job addition and would turn the gate into a maintenance tax; derive
    everything structurally from the parsed document.

**Check 2 — artifact wiring.** Collect `with.name` from every step whose `uses:` starts with
`actions/upload-artifact` (11 at HEAD) and `with.name` / `with.pattern` from every
`actions/download-artifact` step (20 at HEAD). Expand `${{ matrix.<k> }}` against the job's
`strategy.matrix.include` entries when the key is present there (`test-wheels` downloads
`${{ matrix.artifact }}`, whose include block supplies two literal values); otherwise replace each
`${{ … }}` span with `*`. Matching rule: build a regex from an upload name by escaping it and
turning `*` into `.*`, then require every download reference — with its own `*` characters removed —
to `re.fullmatch` at least one upload regex. That resolves all 20 downloads at HEAD (`wheels-*`,
`jni-*`, `ffi-*`, `gem-*`, `kotlin-native-*`, `nuget-package`, `wasm-pkg`, `kotlin-jar`, …). An
upload nobody downloads is not an error.

**Check 3 of the issue is out of scope, but add a cheap third structural check:** every entry in a
job's `needs:` must be a declared job id. It costs five lines and catches a rename typo that would
otherwise surface only on release day.

**Wiring.** Add a local prek hook in the pre-commit block:

```yaml
  - id: check-release-workflow
    name: Release workflow static checks
    entry: uv run scripts/check_release_workflow.py
    language: system
    files: ^\.github/workflows/release\.yml$
    stages: [pre-commit]
    pass_filenames: false
```

CI does not run prek, so CI coverage comes from `tests/test_check_release_workflow.py`: load the
script by path with `importlib.util.spec_from_file_location` (copy the header of
`tests/test_iai_regression.py`), then include one test that runs **all** checks against the real
`.github/workflows/release.yml` and asserts zero errors, plus mutation tests that write a modified
copy into `tmp_path` and assert the specific check fires — at minimum: guard wrapper stripped from
one job, a registry flag renamed to an undeclared input, an upload `name:` renamed so a download no
longer resolves, and a broken `needs:` entry. Never mutate the tracked workflow file.

**Dependency.** `pyyaml` 6.0.3 is currently only a transitive dev dep (via yamlfix/zensical).
Declare it explicitly in `[dependency-groups] dev` in `pyproject.toml` with a short inline comment
and run `uv lock` — the pytest test imports it in-process, so a PEP 723 script would not help here
and would need network in CI. This does not create a hold-back: no version pin is added.

**No Rust source changes**, so neither `.crap-baseline.json` nor `.iai-baseline.json` needs
refreshing — do not touch either. Do not run a blanket `ruff check --fix .` (it deletes load-bearing
`# noqa: S603/S607` directives).

**Docs.** Add the new hook to the pre-commit bullet list in `CLAUDE.md` (~line 181) and
`docs/development.md` (~line 143), phrased like the neighbouring entries.

## Verification

- `uv run scripts/check_release_workflow.py` exits 0 (no arguments → gates the tracked
    `.github/workflows/release.yml`).
- `git status --porcelain .github/workflows/release.yml` prints nothing — the gate passes on the
    workflow as it stands, unmodified.
- Mutation probe, working-tree safe (writes only under `/tmp`):
    `mkdir -p /tmp/relprobe && sed 's/!cancelled() && !failure() && //' .github/workflows/release.yml > /tmp/relprobe/release.yml && uv run scripts/check_release_workflow.py /tmp/relprobe/release.yml`
    exits **non-zero** and names the guard check.
- `uv run pytest tests/test_check_release_workflow.py` passes, and its output includes a test that
    runs the checker against the real `.github/workflows/release.yml`.
- `uv run prek run check-release-workflow --files .github/workflows/release.yml` → `Passed`.
- `uv run prek run check-release-workflow --files pyproject.toml` → `Skipped` (hook is scoped to
    `release.yml` only).
- `uv run ruff check` exits 0 and `uv run ty check` exits 0 (the script is not excluded from `ty`).
- `uv lock --check` exits 0 and `uv run python -c "import yaml"` exits 0.
- `mise run check` exits 0 with no hook reporting `Failed` (do not assert a hook count — it drifts).
- `uv run pytest --timeout=120` exits 0 (full Python suite, no regression in the existing tests).

## Done When

`scripts/check_release_workflow.py` passes on the unmodified `.github/workflows/release.yml`, fails
on each mutated temp copy exercised by `tests/test_check_release_workflow.py`, runs automatically
both via the scoped prek hook and via pytest in CI, and every verification command above exits 0.
