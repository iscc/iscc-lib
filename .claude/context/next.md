# Next Work Package

## Step: Close the known blind spots in the two gate scripts (`check_docs_nav.py`, `--check-action-inputs`)

## Goal

Close both open `normal` `[review]` issues — "Harden the `--check-action-inputs` gate against its
known blind spots" and "`check_docs_nav.py` counts commented-out `zensical.toml` nav entries as
present" — so neither gate can report green while the invariant it names is broken. Both are
`scripts/`-local, share one test pattern, and both issue texts recommend bundling them.

**Cadence note (checked deliberately, not skipped):** the window is 142 tooling / 143 user-facing
docs / 144 tooling / 145 docs-fix + gate = at most 3 of 4 tooling-ish, at the threshold. I
re-checked every user-facing candidate before defaulting here and all are blocked: the Unicode
boundary-vector propagation is double-blocked (parked ordering ruling **and** the Go 15.0-tables
decision), the dependency remainder is human/major-gated, npm OIDC and the
`rubygems/configure-rubygems-credentials` pin need Titusz, the docs logos are `low` `[human]`, and
every other target section verifies at HEAD. There is no unblocked user-facing work to prefer, so
the deferral rule does not apply this iteration.

## Scope

- **Modify** (3 non-test/non-doc files — within the standard budget):
    - `scripts/check_release_workflow.py`
    - `scripts/check_docs_nav.py`
    - `.pre-commit-config.yaml` (comment only)
- **Modify** (tests + docs, excluded from the budget):
    - `tests/test_check_release_workflow.py`
    - `tests/test_check_docs_nav.py`
    - `docs/development.md` (the two bullets at lines 145–157 describe these gates)
- **Reference**:
    - `.claude/context/issues.md` — the two `[review]` issues (the numbered sub-items are the
        checklist for this step)
    - `.claude/context/decisions.md` — 2026-07-26 "The release-workflow action-input gate fails open
        on transport trouble" and 2026-07-26 "The docs page-list gate parses `zensical.toml` nav with
        a regex, not `tomllib`"
    - `.github/workflows/ci.yml` lines 416–430 (the `release-workflow` job)
    - `zensical.toml` lines 12–44 (the real nav block)

## Not In Scope

- **Do not make a zero-resolved run fatal.** "Fail if no ref resolved" is a *policy* change against
    `decisions.md` 2026-07-26, which accepted all-skipped-as-green by design and filed the stricter
    rule as a follow-up "if skips are ever observed in practice". None have been observed. This step
    makes the condition **visible** (a resolved/total summary line) and leaves the exit code alone;
    flipping it to fatal needs Titusz.
- **Do not scan job-level `uses:`** (reusable-workflow calls). `release.yml` has none at HEAD, and
    the `owner/repo/.github/workflows/x.yml@ref` form needs a different URL shape than
    `action_yml_urls` builds — a separate step if a reusable workflow is ever added.
- **Do not migrate `check_docs_nav.py` to `tomllib`/`tomli`** — CI's `python-test` matrix pins
    `['3.10', '3.14']`; the regex approach is the recorded decision. Fix the comment blindness
    inside the existing regex path.
- **Do not edit `.github/workflows/release.yml` or `ci.yml`.** The gate must land green against the
    file exactly as it is (verified below: 18/18 refs resolve, zero missing required inputs).
- No Unicode boundary-vector work, no dependency bumps, no new CI job.

## Implementation Notes

### A. `check_docs_nav.py` — comment-aware nav parsing

**Trap, measured while scoping: the issue text's claim that "no nav title or page path in this repo
contains `#`" is FALSE.** `zensical.toml` line 25 is `{ "C# / .NET" = "howto/dotnet.md" },`. A naive
`re.sub(r"#.*$", "", line, flags=re.M)` truncates that line and makes the gate report
`zensical.toml nav: missing 1 page(s): ['howto/dotnet.md']` — a false positive on the real tree.

So strip `#`-to-end-of-line only when the `#` is **outside** a double-quoted string. A small
character scan per line is enough (TOML basic strings here contain no escaped quotes):

```text
for each line: walk chars, toggle in_string on '"', truncate at the first '#' seen while not in_string
```

Apply it to the sliced `nav = [ ... ]` block before `NAV_MD_RE.findall`. Keep the helper
module-level and named (e.g. `strip_toml_comments`) so tests can hit it directly, and update the
comment above `NAV_MD_RE`.

### B. `.pre-commit-config.yaml` — record the deletion gap

Extend the comment above `check-docs-nav` (lines 73–75) to state that prek `files:` matching only
sees added/modified paths, so a bare `git rm docs/<page>.md` does not trigger this hook — the
`tests/test_check_docs_nav.py` anchor test catches it at pre-push and in CI. Comment only; do not
change the hook's `files:` pattern or stage.

### C. `check_release_workflow.py --check-action-inputs`

1. **Transport failures must never red the gate** (issue item 1). Add `import http.client` and widen
    the fallback handler in `fetch_action` to
    `except (OSError, http.client.HTTPException, yaml.YAMLError) as exc:`. Order matters — the
    existing `except urllib.error.HTTPError` clause must stay **first** (it is an `OSError`
    subclass). Test both new classes by monkeypatching `urllib.request.urlopen`: one raising
    `http.client.IncompleteRead(b"")`, one returning a body that provably breaks `yaml.safe_load`
    (e.g. `b"a: b\n- c\n"` → a mapping followed by a sequence). Both must yield `None` plus a
    `warning: skipped` line, not an exception.
2. **Reverse required-input check** (issue item 2). In `check_with_keys`, after the existing
    undeclared-key loop, report every `inputs.<k>` in the fetched metadata with `required: true`
    and **no** `default:` key that the step's `with:` omits, e.g.
    `action: job '<id>' omits required input '<k>' of '<uses>'`. Treat a `required` value of the
    string `"true"` as true as well (action authors quote it). Verified while scoping: **zero**
    such omissions exist at HEAD, so this lands green.
3. **Make an all-skipped run visible, not fatal** (issue item 3, narrowed — see Not In Scope). Give
    `check_action_compat` an optional `cache: dict | None = None` parameter that it passes to
    `functools.partial(cached_fetch, …)` instead of the inline `cache={}`; existing call sites and
    tests keep working. `main()` passes its own dict and, after the check, prints one stdout line:
    `action-inputs: resolved <R> of <T> action refs (<S> skipped)` where `T = len(cache)` and
    `R = number of non-None values`. Exit code unchanged.
4. **Latent false positives** (issue item 4). In `check_with_keys`: (a) when the fetched metadata
    has `runs.using == "docker"`, accept the GitHub-native `args` and `entrypoint` overrides in
    addition to the declared inputs; (b) skip non-`str` `with:` keys — PyYAML's YAML 1.1 booleans
    turn an input literally named `on`/`off`/`yes`/`no` into `True`/`False`, and the two sides
    cannot be compared reliably. Neither case exists at HEAD (verified: zero docker-type actions
    among the 18 refs), so both need synthetic fixtures in the existing in-memory `_fake_fetch`
    style.

Keep every function short and pure in the existing style; the only new I/O is the one summary
`print` in `main()`. Update the module docstring (check 4's description) and the two
`docs/development.md` bullets to match the new behaviour.

### Measured facts from scoping (do not re-litigate)

- `release.yml` has **18 distinct repo-action refs**; all 18 resolve today, none is a docker action,
    and none omits a required-without-default input.
- `raw.githubusercontent.com` is reachable unauthenticated from the devcontainer, so the network
    path is locally verifiable. Force the offline path with
    `https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9`.

## Verification

- `uv run pytest tests/test_check_docs_nav.py tests/test_check_release_workflow.py` passes with **≥
    33** tests (27 today: 8 + 19), including at least: a commented-out nav entry fires,
    `"C# / .NET"` still yields `howto/dotnet.md`, an omitted required input fires, a docker action's
    `args`/`entrypoint` does **not** fire, `IncompleteRead` → `None` + warning, malformed-YAML body
    → `None` + warning
- `uv run pytest` passes with **≥ 347** tests (341 today) and no failures
- `uv run scripts/check_docs_nav.py` exits 0 and prints a line containing
    `documentation pages consistent across nav, ORDERED_PAGES and llms.txt.`
- `uv run --no-project --with pyyaml python scripts/check_release_workflow.py --check-action-inputs`
    exits 0, prints a line matching `action-inputs: resolved [0-9]+ of [0-9]+ action refs`, and
    emits **no** `warning: skipped` line and no line starting with `action:`
- Fails open offline:
    `https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9 uv run --no-project --with pyyaml python scripts/check_release_workflow.py --check-action-inputs`
    exits **0** and prints `resolved 0 of` (transport failure is still not a red gate)
- `uv run scripts/check_release_workflow.py` (no flag, network-free) exits 0
- `uv run prek run check-docs-nav --all-files` and
    `uv run prek run check-release-workflow --all-files` both Pass
- `grep -c 'git rm' .pre-commit-config.yaml` ≥ 1 (the deletion-coverage comment is present)
- `mise run check` — all hooks Passed; `uv run ruff check` and `uv run ruff format --check` clean
- `git status --porcelain .github/workflows/ .claude/context/specs/ docs/unicode.md crates/ packages/`
    is empty (no workflow, spec, parked-docs or source changes in this step)

## Done When

All verification criteria pass: both gate scripts reject their previously-blind failure modes, both
still pass on the unmodified repository, the action-input gate still fails open offline, and the
docs/config prose matches the new behaviour.
