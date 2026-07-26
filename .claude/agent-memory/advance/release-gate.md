---
name: release-gate
description: release.yml static gate mechanics (check_release_workflow.py) and release workflow shape
metadata:
  type: project
---

# release.yml static gate + workflow shape

## Static gate (iters 142+144+146)

- `scripts/check_release_workflow.py` checks guard shape, artifact wiring via matrix-include
    expansion + symmetric glob match, and the `needs:` graph — prek hook `check-release-workflow` +
    `tests/test_check_release_workflow.py` in CI. Any release.yml edit must keep it green. `pyyaml`
    is an explicit dev dep.
- Opt-in `--check-action-inputs`: fetches each ref's published action.yml from raw GitHub, validates
    `with:` keys (docker `runs.using` gets native `args`/`entrypoint`; non-str keys skipped — YAML
    1.1 bools), required-without-default inputs (quoted `"true"` counts), action step outputs; 404 =
    error, transport/parse failure (incl. `HTTPException`, `YAMLError`) = stderr warning + exit 0;
    prints `action-inputs: resolved R of T` summary (all-skipped visible, NOT fatal — policy needs
    Titusz); runs only in the `release-workflow` CI job (tests inject a fake fetcher).

## Workflow shape (`release.yml`)

- 9 boolean inputs → build → **smoke test** → publish (inputs, auth, CI internals →
    MEMORY-archive.md). `build-wheels` 4 targets incl native-ARM aarch64; `test-wheels` matrixed,
    artifact name = `wheels-<os>-<target>`.
- All 28 non-`prepare-release` jobs carry `!cancelled() && !failure()` `if:` guards (iter 139); 97
    `uses:` refs at current majors (iter 140: checkout@v7, upload/download-artifact v7/v8 pair,
    gh-release@v3 — statically verified only, `workflow_dispatch`).
- Lint edits via `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7` (cached, offline-safe).
