---
name: gate-scripts-playbook
description: How to scope steps that add or harden a checker script in scripts/ — CI vs prek placement, Python floor, network probing, docs-list wiring, and when a gate change needs human sign-off
metadata:
  type: project
---

Playbook for the family of steps that add or harden a **gate script** under `scripts/`
(`check_release_workflow.py`, `check_docs_nav.py`, `version_sync.py`, `gen_llms_full.py`).
Accumulated iters 142–146.

**Why:** four of the last six CID steps were gate work, and each one re-learned the same placement
and verification facts. **How to apply:** read before scoping anything under `scripts/` or
`.pre-commit-config.yaml`.

## Placement and enforcement

- **CI runs no prek** — it calls `ruff`/`pytest`/`cargo` directly. A new prek hook is therefore
    single-place enforcement *unless* a pytest test carries it into CI. The established pattern
    (iter 142, repeated 144/145): the checker is exercised both by a scoped prek hook **and** by a
    test that runs it against the real tracked file.
- **Look for gates that exist but run in only one place** (`S`/`C901` were pre-push-hook-only, CI
    never saw them). Broadening an *existing* gate to CI needs no human sign-off; inventing a new
    policy gate does.
- A network-needing check gets its own CI job and is kept out of prek/pytest (the `release-workflow`
    job in `ci.yml` is the model), with an explicit fails-open contract.

## Python floor and loading

- **No `tomllib` in any script the pytest suite imports** — CI's `python-test` matrix runs Python
    **3.10** (and `version-check` too), where `tomllib` is absent. Repo convention for TOML in
    `scripts/` is stdlib regex (`version_sync.py`).
- Gate scripts are loaded in tests via `importlib.util.spec_from_file_location`
    (`tests/test_check_release_workflow.py` is the model).
- Give the checker's core function **explicit `Path` / injected-fetcher arguments** rather than
    reading module constants: review can then point it at `git archive HEAD | tar -x` copies and
    mutate them freely. This is what made iters 144/145 cheap to verify.

## Verifying a gate

- **Probe the network path before scoping.** `raw.githubusercontent.com` is reachable
    unauthenticated from the devcontainer, so "needs network → CI-only" items are still locally
    verifiable, and the probe proves the gate lands **green** rather than as a fix. Force the
    offline branch with `https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9` — that is the
    boolean fails-open criterion.
- **Never make an exact file/finding count the pass/fail criterion** — counts drift with the CID
    agents' own commits. Make the exit code, or a regex on the output line, the criterion.
- A green gate is not a working gate: require at least one criterion that asserts the gate *fires*
    on a synthetic break.

## Judgment calls

- **An issue's suggested fix recipe is a hypothesis — probe it against the real file first.** Iter
    146: the filed fix "strip `#`-to-EOL before `NAV_MD_RE`, no nav title contains `#`" would have
    produced a false positive, because `zensical.toml` contains
    `{ "C# / .NET" = "howto/dotnet.md" }`. Grep turned a broken one-liner into a string-aware scan.
- **A follow-up item that would reverse a `decisions.md` entry must be narrowed, not obeyed.** Iter
    146: "fail if zero action refs resolve" contradicts the recorded fails-open decision, so the
    step scoped only the *observability* half (a `resolved R of T` summary line, exit code
    untouched) and named the policy flip as human-gated in `Not In Scope`.

## Docs page lists

New docs page checklist: `zensical.toml` nav + `scripts/gen_llms_full.py ORDERED_PAGES` +
`docs/llms.txt` (hand-maintained `## Reference` bullets, absolute `https://lib.iscc.codes/<p>.md`
URLs; 23 real pages = 24 tracked `docs/**/*.md` minus the `includes/` snippet partial). Since iter
145 a `scripts/check_docs_nav.py` prek hook + pytest test gates all three lists against disk.
`gen_llms_full.py` prints `Auto-discovered …` for any page missing from `ORDERED_PAGES` → "output
contains no `Auto-discovered` line" is the boolean wiring check. `uv run zensical build` (exit 0,
"No issues found", ~8s) verifies any docs-only step; run it *before* `gen_llms_full.py` (the build
wipes `site/`).
