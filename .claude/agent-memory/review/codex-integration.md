---
name: codex-integration
description: How to run and weigh the independent Codex second-opinion review — what it reliably catches, what it reliably misses, and which findings to dismiss
metadata:
  type: project
---

# Codex second-opinion review

Findings are **advisory** — cross-reference with my own analysis; the second opinion never sets the
verdict. Use `--commit HEAD` (confirm with `git log` first). A clean, empty, or unavailable verdict
is a note, never grounds for NEEDS_WORK.

## What it reliably catches (trust, then verify empirically)

- **Dependency internals** — blake3 feature gating and `#[target_feature]` (iters 117–118). When it
    cites a dep's `build.rs` or feature wiring, read the dep source (and build it) before
    dismissing.

- **Downstream-consumer breakage** local gates cannot see — the Kotlin plugin floor (iter 128).

- **Input-validation edges** — trailing-byte decode aliasing (iter 119).

- **Algorithm-vs-its-own-docstring gaps** — the `gem-*` vs `*-linux` counter-example (iter 142).

- **Prose-vs-reality gaps in user-facing docs** — both false Unicode claims incl. the U+A7CB
    counter-example (iter 143). Do run it on docs-only diffs; that is where it has been most
    valuable.

- **Contract-vs-handler mismatches in new tooling** — `http.client.IncompleteRead` escaping an
    `except OSError` that a docstring promised would fail open (iter 144).

- **Regex-parses-a-structured-format holes** — `NAV_MD_RE` matching inside TOML comments, so a
    commented-out `zensical.toml` nav entry still counted as present (iter 145). Its single P2 was
    the only real finding on that diff and it reproduced in one temp-copy mutation. Three iterations
    running (143/144/145) the gate/docs-script diffs have each yielded exactly one true positive.

- **Build-config lines whose effect only shows on the SECOND run** — the iter-154 Gradle
    `UP-TO-DATE` stale green: a `Test` task does not track a fixture outside its project tree, so a
    post-run fixture edit silently skipped all 13 boundary tests. Every surface gate was green
    (13/13, mutation probe under `cleanTest`) and the hazard was invisible to all of them.

**Rule of thumb:** expect a real finding whenever a diff adds a matching/parsing rule, an
exception-handling contract, a user-facing factual claim, or a build-config line — i.e. wherever a
green run proves less than it appears to. Convergence with my own suspicion is a signal to VERIFY
EMPIRICALLY, not to agree.

## Weighing a finding (iters 148/150/154)

1. **Reproduce the mechanism yourself.** A no-findings verdict sometimes asserts its own evidence
    (iter 148 claimed a match "across all Unicode scalar values" it never ran) and a real finding
    sometimes rests on a future toolchain (iter 150 ran the Go suite under `go1.27rc1`). Run the
    probe either way.
2. **Judge the proposed remedy separately from the diagnosis.** A correct diagnosis can carry a
    gate-weakening cure — iter 150's fix was "version-gate the skip list", i.e. pre-emptive test
    skipping. Record the rejection in `decisions.md` so the next agent does not re-adopt it.
3. **Fix it here or file it.** Iter 154's fix was one additive line
    (`inputs.file(…).withPathSensitivity(PathSensitivity.NONE)`), green at HEAD, gate-strengthening
    — inside the minor-fix bar. Anything behavioural or architectural goes to `issues.md` instead.

## What it reliably misses (my job to cover)

- **Tool-internal classification** — it passed the iter-138 prek `types:` diff while missing that
    `.pyi` is tagged `pyi`, not `python`.
- **Gate self-weakening** — it called the iter-141 fixture "accurate" without asking what happens if
    the fixture itself is weakened, and it never asks whether a gate is one-directional or whether a
    fully-skipped run is distinguishable from a pass.

It reviews what the code *does*, not what the gates *cannot catch*.

## Dismiss without further work

- Go codec findings — `packages/go` mirrors the Rust reference faithfully and deliberately.
- "Use `8.0.x`" for `dotnet-version: '8.0'` — valid syntax for `setup-dotnet@v4`.
- Large generated Kotlin/Swift binding diffs confuse it; treat as advisory noise.
