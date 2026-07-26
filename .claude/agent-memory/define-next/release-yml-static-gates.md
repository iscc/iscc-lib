---
name: release-yml-static-gates
description: How to scope and statically verify edits to .github/workflows/release.yml — job/guard inventory, actionlint via go run, and the parse-based invariant checks
metadata:
  type: project
---

# Verifying `.github/workflows/release.yml` without running it

Nothing in `release.yml` is exercised by a CID push (`workflow_dispatch` only, 8 registry toggles),
so every criterion for a step touching it must be **static**.

**Why:** a mistake here is invisible until Titusz cuts a release, and a release is exactly when a
mistake is most expensive.

**How to apply:** when scoping a `release.yml` step, build the criteria from the five checks below,
and keep behavioural edits (`if:` conditions, `needs:`) in a different commit from mechanical ones
(`uses:` ref bumps) so a broken release is bisectable.

## Inventory (verified at HEAD, iter 139)

- 1345 lines, **29 jobs**, 97 `uses:` lines over 18 distinct action refs.
- `prepare-release` is the only job whose `if:` is (correctly) unguarded: `inputs.version != ''`.
    Guarding it would make the tag-pushing job run on registry-only dispatches — never do it.
- Registry-token histogram across all job `if:` expressions (a strong "no condition was dropped"
    invariant):
    `version:29, crates-io:1, pypi:4, npm:6, maven:4, ffi:3, nuget:4, rubygems:3, maven-kotlin:4`.
- Guard shape in use: `${{ !cancelled() && !failure() && (<original condition>) }}`. `!failure()`
    still reflects the job's own `needs`, so publishes stay blocked after a failed build/test — only
    the skip propagated from `prepare-release` is neutralised. `always()` would break that.

## `uses:` refresh targets (scoped iter 140)

| Ref                           | HEAD before | Target | Count |
| ----------------------------- | ----------- | ------ | ----- |
| `actions/checkout`            | v4          | v7     | 23    |
| `actions/download-artifact`   | v4          | **v8** | 20    |
| `actions/upload-artifact`     | v4          | v7     | 11    |
| `actions/setup-java`          | v4          | v5     | 6     |
| `actions/setup-node`          | v4          | v7     | 5     |
| `actions/setup-dotnet`        | v4          | v6     | 3     |
| `actions/setup-python`        | v5          | v7     | 2     |
| `softprops/action-gh-release` | v2          | v3     | 2     |
| `actions/cache`               | v4          | v6     | 1     |

Unchanged (already current floating major): `dtolnay/rust-toolchain@stable` 7,
`Swatinem/rust-cache@v2` 7, `ruby/setup-ruby@v1` 3, `PyO3/maturin-action@v1` 2, plus one each of
`nttld/setup-ndk@v1`, `oxidize-rb/actions/cross-gem@v1`, `pypa/gh-action-pypi-publish@release/v1`,
`rubygems/configure-rubygems-credentials@main`, `rust-lang/crates-io-auth-action@v1`. **There is no
`setup-uv` step and none is needed — the file runs no `uv` command at all.**

Notes that make the bump defensible without a release run:

- `upload-artifact` (max major **7**) and `download-artifact` (max major **8**) have *different*
    latest majors but share `@actions/artifact` v4 since upload v5 / download v6, so v7↔v8
    interoperate. Move them in the same commit anyway.
- `download-artifact@v5`'s breaking change is scoped to single downloads by `artifact-ids:` — this
    file uses none (only `name:` / `pattern:` + `merge-multiple:`, all still in the v8
    `action.yml`). `@v8` also flips `digest-mismatch` from warn to **error**; keep the strict
    default.
- Everything else in these lines is a Node 20 → 24 + ESM repackaging.
- `setup-python@v7` dropped the `pip-install` input; `setup-node@v7` dropped a dummy
    `NODE_AUTH_TOKEN` export — neither is used here.

## The five static checks

1. `uv run python` + `yaml.safe_load(...)["jobs"]` — assert job count, per-job `if:` regex shape,
    and the registry-token histogram. Parse-based, immune to line-number drift.
2. Artifact wiring: collect `upload-artifact` `name:` values (11 at HEAD) and `download-artifact`
    `name:`/`pattern:` values (20), then assert every `pattern:` matches some upload name after
    `${{ … }}` → `*` substitution. Prints `unmatched: []` at HEAD.
3. `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 <file>` — **works in the devcontainer**
    (Go 1.26.1, network OK, module now cached; no `actionlint` binary is installed). Exit 0 with no
    output at HEAD on both workflows; validates GH expression syntax that YAML parsing cannot.
4. `uv run prek run check-yaml --files <file>` → Passed.
5. `uv run prek run yamlfix --files <file>` → Passed with no `files were modified by this hook`
    (works because the file is tracked).

## Making the checks executable (scoped iter 142)

Checks 1–2 were scoped into `scripts/check_release_workflow.py` + a scoped prek hook
(`files: ^\.github/workflows/release\.yml$`, `pass_filenames: false`) + a pytest test that runs the
checker against the real workflow. Check 3 (action-input compatibility) stays out — it needs network
and belongs in a CI-only follow-up.

Facts that shaped the design:

- **CI does not run prek** (`ci.yml` calls `ruff`/`pytest`/`cargo` directly), so a prek-only hook is
    single-place enforcement. The cheap CI carrier is a **pytest test that runs the checker on the
    tracked file** — no `ci.yml` edit needed.
- **PyYAML parses the top-level `on:` key as boolean `True`** (YAML 1.1). Read the trigger block as
    `wf.get("on", wf.get(True))` or the registry-flag list comes back empty and every check passes
    vacuously.
- Registry flags are derivable: `on.workflow_dispatch.inputs` keys minus `version`. Derive
    structurally — do **not** freeze the token histogram into the script (it drifts with any
    legitimate job addition).
- `pyyaml` 6.0.3 is only a *transitive* dev dep (yamlfix/zensical). An in-process pytest import
    needs it declared in `[dependency-groups] dev`; PEP 723 does not help here (would need network
    in CI).
- Artifact matching that resolves all 20 downloads at HEAD: expand `${{ matrix.<k> }}` from the
    job's `strategy.matrix.include` when present (only `test-wheels` needs it), else `${{ … }}` →
    `*`; then upload name → regex (`*` → `.*`) and `re.fullmatch` the download ref with its own `*`
    stripped.

## Check 3 — action-input compatibility (scoped iter 144)

Probed live before scoping; all figures verified at HEAD:

- All 18 distinct refs are compatible **today** — every `with:` key is a declared `inputs` key. The
    step must land green, not as a fix.
- Raw fetch needs **no auth**:
    `https://raw.githubusercontent.com/<owner>/<repo>/<ref>/<sub>/action.yml` (split the ref on the
    first `@`; fall back to `action.yaml`). Network works in the devcontainer.
- Four ref shapes to parse: plain, **sub-path** (`oxidize-rb/actions/cross-gem@v1` →
    `<repo>/<ref>/cross-gem/action.yml`), **slash in the ref**
    (`pypa/gh-action-pypi-publish@release/v1`), and branch refs (`@stable`, `@main`).
- Only 3 of the 5 `steps.<id>.outputs.<x>` references are action-sourced (`crates-auth`→`token`,
    `setup-ndk`→`ndk-path`, `xcf-cache`→`cache-hit`); `check`/`version` are local `run:` steps and
    must be ignored — resolve ids **per job**.
- Error vs skip is the design crux: a 404 on both filenames is a real defect (error); any transport
    failure (URLError/timeout/403/429/5xx) is a warning-only skip. Boolean offline check:
    `https_proxy=http://127.0.0.1:9 … --check-action-inputs` must still exit 0.
- CI carrier is a **dedicated job** (`uv run --no-project --with pyyaml python scripts/…` —
    verified, ~4s), not the `python-test` matrix (would fetch 3×). Expect the CI job/check-name
    count to rise by one; flag it so update-state does not read it as drift.

## Related

- GHA `uses:` refresh facts and the "floating `@vN` is a convention, not a guarantee" rule live in
    [[dep-refresh-ledger]].
- `.claude/skills/release/SKILL.md` documents the re-trigger recovery path and must be updated in
    the same step whenever release-job gating changes.
