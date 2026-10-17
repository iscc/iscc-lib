# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md`,
`counts.md`, `dep-refresh-survey.md`, `unicode-contract.md`, `quality-gates.md`, `lint-tooling.md`,
`env-gotchas.md`. **Size budget: under 140 lines** — archive detail eagerly.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`. **A check-RUN
    `failure` does NOT mean CI failed** — a `continue-on-error: true` job (e.g. `Semver`) reds its
    check-run while the SUITE stays `success`. Judge CI by the check-SUITE conclusion,
    cross-checking `continue-on-error` in ci.yml for any red check-run.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap (fired 162; fired again 184: a NEEDS_WORK advance
    commit stays in local history unreverted and its code sits at HEAD uncovered by green).
- **ALWAYS `tail -6 iterations.jsonl` + `git status --porcelain`.** jsonl is the ONLY place a
    crashed/`audit` role shows; a non-OK status ≠ no work (corroborate `git log`, fingerprints →
    `MEMORY-archive.md`); out-of-loop `cid(loop):` commits leave NO jsonl entry — diff them. A
    TIMEOUT role leaves its file dirty (155: 440-line `next.md`) — unverified lead. Audit cadence
    UNRELIABLE (none at 170) — never park a finding for it.
- **Counts + the glob/grep trap for each → `counts.md`** (194: 12 READMEs, 22 docs pages, 353
    iscc-lib `#[test]`, 179 Go, 49 ffi externs, 102 CRAP, 21 version_sync, 5 fixture copies).
- **YAML probes need `uv run python`** (system `python3` has NO `yaml`); job-table parity gated
    (163).
- **The project venv is a ready-made conformance oracle**: `iscc_core` AND `iscc_lib` both import
    under `uv run python` (check `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` mtime — stale
    `.so` measures the previous commit). `uv run --with iscc-core` = live IDv1 oracle. →
    `unicode-contract.md`.
- **Issue headers**: `grep -nE '^## ' issues.md`. **Trace a dep**: `cargo tree -i <crate>`. **Specs
    live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` is empty for ANY commit
    and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir); coverage + CRAP (`--fail-regression` is **CI-ONLY**, so a green
    `mise run check` proves nothing); cargo-deny; docs page-list parity; `unicode-sweep` (157); CI
    job-table parity (163). Last three = **prek hook + a pytest**; the pytest makes it a CI gate.
- **Red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml static
    gate (job log must show zero `warning: skipped`). "no floating branch ref" in
    `check_release_workflow.py` is new policy, NOT a gap.
- **cargo-semver-checks INFORMATIONAL — `semver` job `continue-on-error: true`** (`ci.yml:355`,
    enforcing only from v1.0.0). Check-run CAN report `failure` (exit 100:
    `enum_marked_non_exhaustive` on `enum Version`) while the check-SUITE stays `success` — judge by
    the SUITE. `mise run check` doesn't run it. → `quality-gates.md`.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi); iscc-uniffi has 21 tests,
    `publish=false`. `packages/` layout → `MEMORY-archive.md`; `packages/go` is the ONLY binding not
    inheriting the core's Unicode behaviour.
- `ci.yml` — **20 YAML job entries** (194; `unicode-sweep` removed): `python-test` = 3.10/3.14
    matrix, `python` (L72) is an `if: always()` AGGREGATOR; `push:` under `on:` is NOT a job.
    `release.yml` = 8 registry toggles (Swift XCFramework is a `prepare-release` step, NOT a
    toggle). `specs/ci-cd.md` job table EXHAUSTIVE since 163 (rows==keys, gated); prose counts
    unpinned (HAND edit).
- **Unicode freeze REMOVED 2026-07-30** (decisions.md): no pin, no sweep gate, no boundary fixture —
    `text_clean`/`text_collapse` use the dependencies' tables. `unicode-contract.md` is history.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`),
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` via
    `streaming::` only. iscc-wasm `blake3 wasm32_simd` = feature-unification, don't prune. `iscc-py`
    12 `.detach(` sites. Benches in `crates/iscc-lib/benches/` (NOT root): 12 criterion (0.8.2) +
    iai 0.16 (11 fns, 16 cases). Root `Cargo.toml` zero `# held:`/`authorized` since 172 — grep BOTH
    for a hold.
- **Ruff/prek/mdformat → `lint-tooling.md`** (ruff 0.16.0 since 137; local prek is a strict SUPERSET
    of CI; probe hooks with `prek run <hook> --files <f>`). **Dep-pin inventory →
    `dep-refresh-survey.md`** (GHA refs current 171, no Dependabot, `rb_sys` pinned in 3 places).

## Recurring Patterns

- **No assessed-at hash (human reset)?** Diff from the hash in THIS file's Current State header to
    scope the full review fast (194). **Incremental review**: assessed-at vs HEAD `--stat` first,
    re-verify only affected sections, carry forward the rest, CI via check-runs API on the real tip.
    **Always diff `.claude/context/specs/`** (132,147) and read issues.md *bodies* (145) — either
    flips met→unmet with ZERO code change (as can pure MEASUREMENT, empty diff, 156).
- **When a ruling lands, re-verify CODE against the NEW spec**; grep `decisions.md` for `supersede`
    (147: a 14-iteration "met" went unmet).
- **Reproduce/refute inherited claims by a DIFFERENT method** (bugs confirmed 147/156; "unbuildable"
    REFUTED). A generator is never its own oracle; an N-passes suite may cover a SUBSET. Spec
    checkboxes are NOT a progress signal (most sit 0/N though MET); spec *prose* rots.
- **Fixtures as DECLARED build inputs**: Gradle/MSBuild can report UP-TO-DATE and skip a suite; only
    mutate-then-rerun exposes it (all 12 probed, CI immune). "Not verifiable in this container" is
    UNPROVEN (cmake/Kotlin/C++/Swift all fell) → `env-gotchas.md`.

## Current State (assessed-at: ad69c8e, iter 194 — FULL review after a human-reset state.md)

- **Loop paused 193 → 2026-10-17; everything since was HUMAN** (`cid(human):` commits, no jsonl
    entries). v0.6.0 shipped 2026-07-30, **v0.7.0 shipped 2026-10-17**; `develop`==`main`==`d49b953`
    CI fully green (Perf, CRAP, Semver, Swift, Kotlin all success). Iter-193 Perf blocker resolved
    by the human raising the `four_units` iai baseline (decisions.md 2026-07-29).
- **Target = v0.8.0 ISCC-SEQ** (`encode_seq`/`decode_seq` Tier 1 33→35 on all 11 surfaces + Tier 2
    `read_unit`/`SEQ_MAINTYPES`; spec `rust-core.md` ~L221). **ZERO implementation at 194** — the
    only repo hit is a comment at `packages/go/conformance.go:27`. Probe:
    `grep -rlnE 'encode_seq|decode_seq|EncodeSeq|encodeSeq|read_unit|SEQ_MAINTYPES' crates packages`.
- Seq vectors (4 encode + 4 decode, `bytes:<hex>` outputs) are vendored in all 5 identical
    `data.json` copies but run by NO harness; Go `parseConformanceData` loads only `gen_*`.
- **6 open `normal` issues, ALL actionable** (c-ffi-api unprefixed type names; Go `isccClean` absent
    = prerequisite for Go seq; wasm `Cargo.toml` no license/description; Ruby `json` 2.21.1 +
    Dependabot alert #1 open; Ruby `File.read` w/o encoding; `mise.toml` still passes
    `--fallback-model opus`). One-line probes for each are cheap — re-run them, don't carry.
- **Stale target fact (human-owned):** "Tag push `v*.*.*` triggers all publish jobs" — release.yml
    is `workflow_dispatch`-only by design since `8f444eb` (workflow creates the tag). PR #48
    (`fix/image-code-dct-quadrants`→main) open though superseded by #49.
- **IDv1 (#43) fully closed — do NOT re-flag.** Golden:
    `gen_iscc_id_v1(1751831876325218,1,0) == "ISCC:MAIGHFECJMOPMIAB"`. Validation-order contract
    (ts→hub→realm before narrowing) → `rust-core.md`.
- **dotnet gotcha:** csbindgen `build.rs` regenerates tracked `NativeMethods.g.cs` on every build;
    any FFI-symbol step (seq!) must regen `iscc.h` + `NativeMethods.g.cs` in-step.

## Durable Facts (carried, not per-iteration)

- **`target.md` carries a "Current Release Milestone — v0.6.0" section** — always diff `target.md`.
    Unicode work FINISHED (161, 11/11). Dep refresh DONE (172, uniffi 0.32 + criterion 0.8; zero
    `# held:`/`authorized`). `iscc-uniffi/Cargo.toml` declares `rust-version = "1.91"` (real floor);
    other 7 crates inherit workspace 1.85. Version **0.7.0** (194); releases via PR develop→main
    then `/release`.
- **Review policy widened at 171:** `review` may correct a *mechanically checkable fact* (version,
    path, count) in a sub-spec under `.claude/context/specs/` with NO escalation; `target.md` +
    rationale/criteria/scope still need the human. Report stale spec FACTS.
- **Every ecosystem has a lockfile** (170: .NET `packages.lock.json` `--locked-mode`; a bump needs
    `--force-evaluate` same commit). Propagation invariant: `git ls-files -- '*data.json'` = **5**
    (canonical + 4 vendored) == `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- **Loop infra (162):** `ARTIFACT_BUDGETS` (`tools/cid.py`) caps state 200; `decisions.md` rotates
    into `decisions-archive.md` (grep BOTH) — dirty `decisions*.md` = runner, not crash.
- **Issue count: never carry forward** — re-grep `^## ` headers each iteration (a legend inflates a
    naive `grep -c`); composition flips while the count holds. **Don't re-flag as DONE**: IDv1 mint
    fan-out (all 11 surfaces, cpp 188 last), uniffi 0.32 172, criterion 0.8 171 (≤170 → archive).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: run `uv run prek run mdformat --files <f>` after writing (bare `uv run mdformat` is
    NOT the hook), then grep for `` `…  …` `` (2+ spaces in backticks — a span across a line break
    joins with indent and corrupts a path) and `\` (a reflowed `)` becomes `172\)`). Reword so no
    span/paren straddles a break. Edge cases → `lint-tooling.md`.
- **Toolchain presence (`$PATH` fallbacks), invisible exec bits, case-sensitive binding-API greps,
    csbindgen/UniFFI/JNA side effects, the Go probe recipe → `env-gotchas.md`.**
