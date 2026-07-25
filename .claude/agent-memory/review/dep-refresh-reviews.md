---
name: dep-refresh-reviews
description: Per-slice review recipes for the v0.6.0 "Dependency review and refresh" issue (Rust lockfile, Python uv.lock, Rust direct pins, GitHub Actions) — what to verify and where each slice actually breaks
metadata:
  type: project
---

# Dependency-refresh review recipes (v0.6.0 `[human]` issue, sliced per-ecosystem)

**Why:** the refresh is deliberately sliced so each slice has its own verification surface; using
the generic "config/lockfile-only" shortcut on these under-verifies them. **How to apply:** find the
slice that matches the diff, run its gate set, and verify the hold-back claims rather than trusting
the advance handoff. Progress log lives in `.claude/context/issues.md`.

## Slice 1 — Rust `Cargo.lock` (`cargo update`, no `-p`) — iter 124

Verify `git diff --name-only -- Cargo.toml` is EMPTY (pins untouched: uniffi 0.31 / pyo3 0.29 /
criterion 0.5 / iai-callgrind 0.16 / magnus 0.7 / jni 0.21 / napi 3), then run the full 4-gate set
`mise run test` + `lint` + `audit` + `bench:iai:check` — not the lockfile-only shortcut, since a
bare `cargo update` can shift Ir and pull new transitive licenses. `cargo-deny` is the main risk.
Perf-gate tools are NOT in a fresh container — install per the learnings.md Tooling note first.
GOTCHA: next.md's pin grep `'uniffi = { version = "0.31"'` (inline table) FAILS — the manifest uses
the plain string `uniffi = "0.31"`. That is a next.md defect, not an advance miss; verify the actual
pin form.

## Slice 2 — Python `uv.lock` (`uv lock --upgrade`) — iter 125

`git diff --name-only` must be `uv.lock` + (at most) `pyproject.toml`, NO source. Rebuild the
extension first: `uv sync --group dev` UNINSTALLS the editable `iscc-lib`, so follow with
`uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`. Then `uv lock --check` +
`mise run test` / `lint` / `check` + `uv run prek run --all-files --hook-stage pre-push` (the `ty`
jump is the risk — 0.0.18 → 0.0.63 passed) + `zensical build` + `gen_llms_full.py`.

A single dev-tool hold-back pin (e.g. `ruff<0.16`) is legitimate scope discipline, NOT gate
weakening: it defers adoption of NEW default lint rules while the held minor still enforces the
exact prior rule set. VERIFY the hold-back is genuine rather than a mask — it is cheap:
`uvx ruff@0.16.0 check .` reproduced the claimed 104 errors (72 in `_lowlevel.pyi`) exactly. Require
an inline `# held: …` comment stating the reason plus a deferred-adoption follow-up.

## Slice 3 — Rust direct pins (`Cargo.toml` pin text changes) — iter 126

Same 4-gate set as slice 1, plus `cargo bench --no-run` (what CI's `Bench (compile check)` runs) — a
dev-only bump can still red clippy via a NEW deprecation (criterion 0.6 deprecated
`criterion::black_box`; `-D warnings` makes it fatal; the fix is `use std::hint::black_box`).

VERIFY HOLD-BACK CLAIMS, do not take them on faith: `cargo info <crate>@<ver>` prints `rust-version`
(criterion 0.8 needs 1.86 vs our declared 1.85 ✓), and a `grep -c` of the named API in the affected
binding proves a claimed migration cost is real (`exception::runtime_error` ×5 in iscc-rb;
`JNIEnv|GlobalRef|AutoLocal` ×41 in iscc-jni). Confirm taplo (`mise run check`) preserved the
`# held:` comments and left no rewrite in the tree.

## Slice 4 — GitHub Actions in `ci.yml` / `docs.yml` — iter 127

Prove the diff is purely mechanical:

```
git diff <base>..HEAD -- .github/ | grep -E '^[+-]' \
  | grep -vE '^(\+\+\+|---)|^[+-] *(- )?uses: '
```

should return only comment lines. Then run the next.md greps + `mise run check` + the REAL gate:

```
gh api "repos/iscc/iscc-lib/commits/<sha>/check-runs?per_page=100" \
  --jq '[.check_runs[]|select(.conclusion!="success")]|length'
```

→ 0. No ellipsis in the sha — `8f76d48...` yields HTTP 422 (Codex caught this in a handoff).

Independently confirm each bumped ref is really the latest major
(`gh api repos/<o>/<r>/releases/latest --jq .tag_name`) AND that a floating `@vN` tag exists
(`gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`) — the two are NOT the same claim.
`astral-sh/setup-uv` publishes no floating major past v7, so `@v9` fails to resolve and `@v9.0.0` is
correct (see `decisions.md` 2026-07-25). Also inventory ALL refs
(`grep -hoE 'uses: [^ ]+' … | sort | uniq -c`) to catch ones the bump table missed. `docs.yml` runs
only on push to `main` → statically verified only; `release.yml` is a separate deferred slice (see
the release.yml entry in [[MEMORY]]).

Bookkeeping: `ci.yml` sets `cancel-in-progress: true` per ref, and every develop commit triggers two
runs (push + `pull_request` from the open develop→main PR) — so check-run totals read ~2× the job
count, and a follow-up push cancels the prior sha's run.

## Remaining slices

Per-binding manifests (napi `package.json`, rb `Gemfile`/gemspec, jni `pom.xml`, kotlin
`build.gradle.kts`, dotnet `.csproj`, go `go.mod`) — these ARE CI-validated on develop pushes, so
demand green CI. Then `release.yml` GHA refs (not CI-exercised) and the deferred ruff 0.16 / magnus
0.8 / jni 0.22 migrations, each its own step.
