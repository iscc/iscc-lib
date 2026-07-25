---
name: lint-tooling-lessons
description: How to scope lint/formatter tool bumps, hook-config changes and gate-parity claims — probing recipes and slicing rules from the ruff 0.15→0.16 thread (iters 131–139)
metadata:
  type: project
---

# Scoping lint-tool bumps and hook-config changes

Distilled from the ruff 0.15.22 → 0.16.0 adoption (dependency-refresh slice 8, sub-slices A–E,
iterations 131–137) and the two follow-on hook-parity steps (138–139). The ruff thread is
**closed**; these rules generalise to the next tool bump (mdformat, taplo, yamlfix, prek, `ty`).

**Why:** every one of these was learned by mis-scoping a step first.

**How to apply:** read this before scoping any step that changes a linter/formatter pin, a
`pyproject.toml` lint config, or a `.pre-commit-config.yaml` hook.

## Slicing

- **A lint-tool major bump is not one step.** Slice by *what decision each finding needs* —
    mechanical / gate-interacting / config-requiring — not by file.
- **A lint *config* change moves the finding set, it does not only shrink it** (iter 135: an isort
    `src` setting cleaned 3 test files and dirtied a previously-green benchmark file). Re-run the
    full-tree check *with* the candidate config before counting files against the file budget.
- **A tool bump's real risk may be file *discovery*, not new rules** (iter 137: ruff 0.16's new
    rules were a no-op, but its formatter started covering `.md` fences, widening the bare
    `ruff format --check` gate from 25 to 153 files). Before scoping a pin drop, run the *bare* gate
    command under the new version and compare the reported file count, not just the exit code.

## Probing without touching the repo

- Probe a setting without touching the lock:
    `uvx ruff@<ver> check --config '<key> = <val>' --diff <paths>`, then re-probe with the *pinned*
    tool + `--extend-select` to see whether the rule is already enforceable pre-upgrade.
- Probe a hook-config change with `prek run -c /tmp/probe.yaml <hook> --files <path>` — the global
    `-c` accepts any path while still resolving files inside the repo, so define-next never has to
    edit `.pre-commit-config.yaml` to find out what a hook would do.
- prek reports `files were modified by this hook` **only for tracked files**; an untracked probe
    file is silently fixed and reported `Passed`. Stage the probe if you need the signal.

## Gate parity

- **Gate-parity claims from a review handoff are hypotheses — measure the surfaces yourself.** Iter
    138: the filed issue said no local hook covered Markdown, but `mdformat-ruff` (pulled in by
    `mdformat-mkdocs[recommended]`) already formatted the `python` fence tag; the real gap was three
    other fence tags. Check `entry_points(group='mdformat.codeformatter')` before scoping.
- **Look for gates that exist but run in only one place.** Ruff's `S` and `C901` selections were
    pre-push-hook-only for months; CI never saw them. Broadening an *existing* gate to CI needs no
    human sign-off; inventing a new gate does.
- Never make an exact file/finding count a pass/fail criterion — it drifts with the CID agents' own
    commits. Make the exit code the criterion and the count an aside.
- Recursive ruff discovery honours `.gitignore`, but explicitly-named paths bypass exclusions unless
    `--force-exclude` is passed. prek's `python` type tag does **not** match `.pyi` (that is the
    `pyi` tag) — a hook covering stubs needs both.

## Related

- Per-ecosystem bump history, hold-backs and version-lookup commands: [[dep-refresh-ledger]].
- Never run `ruff@0.16 check --fix .` on this tree — it deletes load-bearing `# noqa` directives.
