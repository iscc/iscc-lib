# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `lint-tooling.md` (ruff/prek/mdformat). **Size budget: under 140 lines** —
archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then prove it is code-free with
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` (empty = nothing outside CI coverage).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 147): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    tracked docs `.md` **24** (23 pages + 1 `includes/` partial), ORDERED_PAGES + llms.txt links
    **23**; `docs/howto/*.md` 11; speedups 1.3x-158x; release.yml toggles 8; ffi extern 47; iscc-lib
    `#[test]` **328** (`grep -rc --include="*.rs" crates/iscc-lib/`, sum); ci.yml job entries **20**
    = `grep -cE '^  [a-z_-]+:$'` (raw **22**) minus 2.
- **GOTCHA — `git ls-files 'docs/**/*.md'` returns 13, NOT 24**: the pattern misses top-level
    `docs/*.md`. Use `git ls-files 'docs/*.md' 'docs/**/*.md'`, or just run
    `uv run scripts/check_docs_nav.py` (prints the authoritative page count).
- **version_sync TARGETS** = **21**
    (`uv run python scripts/version_sync.py --check | grep -c '^OK'`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates (all in ci.yml unless noted; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: nearby `continue-on-error` is the SEMVER job's.
- **release.yml static gate — ALL 3 CHECKS BUILT** (142/144, hardened 146); internals →
    `MEMORY-archive.md`. Offline prek hook + `release-workflow` CI job for the network mode. **FAILS
    OPEN by design**: only a double-404 reds it; timeouts/429/5xx print `warning: skipped` and exit
    0 → **verify the JOB LOG has zero `warning: skipped`**; the badge is not proof it ran.
- **Docs page-list parity (145)** — `scripts/check_docs_nav.py` asserts disk pages ⇄ `zensical.toml`
    nav ⇄ `ORDERED_PAGES` ⇄ `docs/llms.txt` are one set. Enforced by prek hook `check-docs-nav` +
    pytest in CI's `python-test` — **no dedicated CI job**. Takes 4 `Path` args (temp checkouts).
- **Coverage + CRAP** — ENFORCING: `--fail-regression` + `--fail-above` 30.0 (`.cargo-crap.toml`);
    baseline `.crap-baseline.json` (**98** entries, max ~22.3). **GOTCHA — `--fail-regression` is
    CI-ONLY, not in `mise run check`**: a new branch in a covered fn → exit 1 despite a GREEN local
    check (bit 121). Fix = refresh that entry in the SAME step.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9`, root `deny.toml` (v2, 2 dev-bench
    ignores). **GOTCHA — the live advisory DB flips this red with NO code change**; not in the
    devcontainer, so green CI is the only confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), does
    NOT flip the run. rust-core.md box `[ ]` = unmet (enforcing + >=1.0.0, HELD); ci-cd.md `[x]`.
- **`setup-uv` needs the EXACT tag `@v9.0.0`** (no floating major past v7) — bites any new CI job.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests and `publish=false`.
- `.github/workflows/ci.yml` — **20 YAML job entries → 21 jobs → 22 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job. `release.yml` — 8
    registry toggles; the Swift XCFramework step lives in `prepare-release` and is NOT a toggle.
- `packages/` layout detail (go pure-Go no-CGO, swift XCFramework 5 targets, kotlin JNA) →
    `MEMORY-archive.md`. `packages/go` is the ONLY binding not inheriting the freeze rule.
- **Unicode = 16.0.0 + SENTINEL freeze rule → read `unicode-contract.md` before ANY Unicode call.**
    Ruling 2026-07-26 REVERSED the design (map unassigned → `U+FFFF`, don't delete); `utils.rs`
    L103/L181 still `.filter(` so criterion 1 is UNMET; `decisions.md` keeps 2 SUPERSEDED designs
    that must not be implemented; `docs/unicode.md` is STALE.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`) —
    now gated (check-docs-nav). `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root,
    `SumHasher` only via `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification
    only — **don't prune** (archive).
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE).
    `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box`
    from `std::hint`); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` has **4**
    (criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32) + a pyo3 `# note:`; `pyproject.toml` has
    zero. MSRV: `cargo info <crate>@<v>`.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; prek/CI parity holes CLOSED
    (138, 139) — local prek is a strict SUPERSET. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory, GHA histograms + slice history** → `dep-refresh-survey.md`. ci.yml,
    docs.yml AND release.yml GHA refs ALL CURRENT (slice 9, 140); release.yml keeps **97 `uses:`**
    (18 distinct refs) and has **no setup-uv step**. `rubygems/configure-rubygems-credentials@main`
    (`release.yml:895`) = the ONLY unpinned ref — RULED `@v2.1.0`, CID-authorized, not yet applied.
    No Dependabot/Renovate; no `[tools]` in `mise.toml`; **`rb_sys` pinned in THREE places that move
    together** — Gemfile (exact `0.9.123`), `Gemfile.lock`, `tag:` at `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff). **Always
    diff `.claude/context/specs/`** — a `human(decide)` commit flips met→unmet with ZERO code change
    (132, and decisively at 147). **A review's "Issues found" can too** (145) — read issues.md diffs
    for NEW measured facts, not just headers.
- **When a ruling lands, re-verify the CODE against the NEW spec** — don't carry forward a "met"
    status. At 147 the shipped `.filter(` had been met for 14 iterations and became non-conformant
    overnight. Also grep `decisions.md` for `supersede`: it logs losing designs chronologically.
- **Reproduce `critical` binding bugs in seconds** rather than trusting the issue text — a 6-line
    `go run` confirmed the `Final_Sigma` divergence at 147. Cheap evidence beats inherited claims.
- **Issues diff**: a rewritten-in-place issue keeps the count flat. Diff issue *bodies*, and check
    `grep -c "HUMAN REVIEW REQUESTED"` — 0 means the human backlog cleared.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.

## Current State (assessed-at: 9aa25ad, iter 147)

- **IN_PROGRESS — CI GREEN** on origin/develop tip `dd65825`: **43** check-runs, **22** names, 0
    non-success. HEAD `9aa25ad` = **2 UNPUSHED** commits (`cid(log)` 146 + `human(decide)`);
    `origin/develop..HEAD` minus `.claude/` = EMPTY, so green CI covers all code. ~2x runs because
    PR **#44 (develop→main) is OPEN** (titled "Release 0.6.0" — NOT shipped; version **0.5.0**).
- **Iter 147 = a `human(decide)` commit**: 21 files, only 6 outside `.claude/` (iter-146 gate
    hardening); NO `crates/`/`packages/`. Load-bearing = `specs/rust-core.md` (+146/-27).
- **Statuses:** Rust-core partially met (**criterion 1 REGRESSED to unmet**); Other Bindings +
    Documentation **DOWNGRADED** (Go bug; stale `docs/unicode.md`); py/napi/wasm/ffi + benchmarks
    met; CI/CD partially met.
- **NEW CRITICAL — Go `Final_Sigma`**: `packages/go/utils.go:117` `strings.ToLower` → `ΛΟΓΟΣ` gives
    `λογοσ` not `λογος`. Reproduced at 147. Titusz sequenced it FIRST, before the sentinel work.
- **Human backlog CLEARED** (0 `HUMAN REVIEW REQUESTED`; the top ask for 3 states). AUTHORIZED for
    CID: rubygems `@v2.1.0` pin; major dep bumps **one per step** (magnus 0.8, jni 0.22 source
    rewrites one crate per step; xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x); exhaustive
    `specs/ci-cd.md` job table. DEFERRED: npm OIDC (not v0.6.0; don't even prepare the diff).
- **Cadence RESOLVED** — the ruling supplied unblocked user-facing work, so the 3-iteration
    escalation request is discharged. Do not repeat it.
- **10 issues: 1 critical, 4 normal, 5 low.** The 3 deferred gate-script remainders are `low` +
    **trigger-contingent** — CID must not act unprompted.
- **Don't re-flag as DONE work**: gate blind-spots 146; docs-list gate 145; action-input gate 144;
    Unicode docs page 143; release.yml static checks 142; Unicode boundary Rust half 141; GHA refs
    140; re-trigger + `.pyi` hooks 139; Markdown parity 138; ruff 0.16 A-E 131-137.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`); `--stat` shows it as a 0-line file (136).
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
