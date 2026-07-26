---
name: quality-gates
description: Every enforcing quality gate in ci.yml and prek — what it checks, whether it can fail without a code change, and how to verify it is actually gating rather than failing open
metadata:
  type: project
---

# Quality gates: what enforces, what fails open, what fails without a code change

All in `.github/workflows/ci.yml` unless noted. Full pipeline listings → `MEMORY-archive.md`.

**Why this file exists:** several of these gates look green while gating nothing, and two can go red
with zero code change. A green badge is not evidence — know which is which before reporting CI
status.

**How to apply:** consult before writing the CI/CD section of state.md, and before believing any
handoff claim that "the gate caught it".

## Enforcing

- **Perf (iai-callgrind)** — `scripts/iai_regression.py --check` fails on >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: the `continue-on-error` sitting near it in the YAML
    belongs to the SEMVER job, not this one.
- **Coverage + CRAP** — `--fail-regression` + `--fail-above` 30.0 (`.cargo-crap.toml`); baseline
    `.crap-baseline.json`, **100** entries. Count with
    `python3 -c "import json;print(len(json.load(open('.crap-baseline.json'))['entries']))"` — the
    file is a dict (`$schema` / `version` / `entries`), not a list. **GOTCHA — `--fail-regression`
    is CI-ONLY, not in `mise run check`**: a new branch in an already-covered function exits 1 in CI
    despite a fully green local check (bit at 121). The fix is to refresh that entry in the SAME
    step, not to touch the threshold.
- **Audit (cargo-deny)** — `cargo-deny@0.19.9`, root `deny.toml` (v2, 2 dev-bench ignores). **GOTCHA
    — the live advisory DB flips this red with NO code change.** Not installed in the devcontainer,
    so green CI is the only possible confirmation.
- **Docs page-list parity (145)** — `scripts/check_docs_nav.py` asserts disk pages ⇄ `zensical.toml`
    nav ⇄ `ORDERED_PAGES` ⇄ `docs/llms.txt` are one set. Enforced by the prek hook `check-docs-nav`
    plus a pytest inside CI's `python-test` — there is **no dedicated CI job**, so don't look for
    one. Takes 4 `Path` args (it is written to run against temp checkouts).

## Fails open — check the log, not the badge

- **release.yml static gate** — all 3 checks built (142/144, hardened 146; internals →
    `MEMORY-archive.md`): an offline prek hook plus the `release-workflow` CI job. **Fails open by
    design** — only a double-404 turns it red. To know whether it actually ran, read the JOB LOG and
    confirm zero `warning: skipped` lines.

## Informational only

- **Semver (cargo-semver-checks)** — `continue-on-error: true` until v1.0.0, so it never flips a
    run. `specs/rust-core.md`'s box is `[ ]` (unmet: it requires enforcing *and* >=1.0.0, which is
    HELD); `specs/ci-cd.md`'s equivalent box is `[x]`. Both are correct — they ask different things.

## Setup gotcha

- **`setup-uv` needs the EXACT tag `@v9.0.0`** — there is no floating major past v7. This bites
    every newly added CI job that touches Python.
