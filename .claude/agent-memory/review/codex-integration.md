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

**Rule of thumb:** expect a real finding whenever a diff adds a matching/parsing rule, an
exception-handling contract, or a user-facing factual claim. Convergence with my own suspicion is a
signal to VERIFY EMPIRICALLY, not to agree.

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
