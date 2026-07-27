# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `quality-gates.md`, `lint-tooling.md`. **Size budget: under 140 lines** —
archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap instead of calling CI green-for-HEAD (bit at 148).
- **ALWAYS `tail -5 .claude/context/iterations.jsonl`** — the ONLY place a crashed role shows up
    (also the `audit` role, every 10th iteration). **A non-OK status does NOT mean no work:
    corroborate with `git log` for the `cid(<role>):` commit** — infra crash vs benign overrun and
    the crashed-review fingerprint → `MEMORY-archive.md`.
- **ALWAYS `git status --porcelain` too.** A TIMEOUT role dies before committing but **leaves its
    written file dirty** — at 155 define-next timed out with 0 turns yet left a 440-line `next.md`
    carrying the iteration's most important finding. `git diff <file>` it; treat as an unverified
    lead, not fact.
- **Verify a Go-package claim in ~60s**: `/tmp` module requiring
    `github.com/iscc/iscc-lib/packages/go v0.0.0` + a `replace` to the repo, then
    `GOFLAGS=-mod=mod CGO_ENABLED=0 go mod tidy && CGO_ENABLED=0 go run main.go`.
- **Counts** (155): crate/pkg READMEs & CLAUDE.md 12 each (**scope the glob to
    `crates/*/   packages/*/`** — bare grep gives 14); pytest-benchmark 18; UniFFI 32; docs pages
    **23**; `docs/howto/*.md` 11; speedups 1.3x-158x; ffi extern **47** (`'#\[unsafe(no_mangle)\]'`;
    bare `no_mangle` gives 48); iscc-lib `#[test]` **335**; `packages/go` `^func Test` **177**.
- **GOTCHA — `git ls-files 'docs/**/*.md'` returns 13, NOT 24** (misses top-level `docs/*.md`); use
    `uv run scripts/check_docs_nav.py` for the authoritative count.
- **version_sync TARGETS** = **21** (`scripts/version_sync.py --check`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md` (priority+source tag). **Trace a dep**:
    `cargo tree -i <crate> --target all`. **Tier 1 fns**: `grep -n "pub fn gen_" .../src/lib.rs`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir, 16 baseline entries); coverage + CRAP (100 entries —
    `--fail-regression` is **CI-ONLY**, so a green `mise run check` proves nothing); cargo-deny;
    docs page-list parity (prek hook + a pytest in `python-test`, no dedicated CI job).
- **Red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml static
    gate (job log must show zero `warning: skipped`). **Informational:** cargo-semver-checks.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests, `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the core's Unicode behaviour.
- `ci.yml` — **20 YAML job entries → 21 jobs → 22 check names**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR; `push:` under `on:` is NOT a job. `release.yml`
    — 8 registry toggles; the Swift XCFramework step is in `prepare-release`, NOT a toggle.
- **Unicode = 16.0.0 + SENTINEL freeze rule → read `unicode-contract.md` before ANY Unicode call**
    (criterion status, per-surface fixture-plumbing table, the OPEN `Final_Sigma` case-table defect,
    2 SUPERSEDED designs that must NOT be implemented). Inline landmark: **surefire's default CWD is
    the pom basedir**, NOT the `mvn -f` dir — why the Java suites' `../../iscc-lib/tests/…` resolve.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
    `iscc-py` has **12** `.detach(` sites. `benches/`: 12 criterion (0.7) + iai 0.16 (11 fns, 16
    cases). **Inline `# held:`** = pin rationale — root `Cargo.toml` **4**, `pyproject.toml` zero.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; local prek is a strict
    SUPERSET of CI. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory + slice history** → `dep-refresh-survey.md`. All GHA refs CURRENT
    (140); release.yml keeps **97 `uses:`**, `rubygems/configure-rubygems-credentials@main` (L895)
    the ONLY unpinned one. No Dependabot/Renovate; **`rb_sys` pinned in THREE linked places**.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change, as can pure MEASUREMENT with an empty diff (156).
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede`. At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute inherited claims yourself, ideally by a DIFFERENT method** — cheap probes beat
    inherited text (confirmed bugs at 147 and 156, refuted a stale-napi-`.node` claim at 151-153).
    At 156 a classification-level diff both confirmed the sweep claim and bounded it (100 → 1).
    Deliberate deviations from a fix sketch are legitimate: read the in-source comment first.
- **Spec checkboxes are NOT a progress signal** — most specs sit at 0/N checked though MET; only
    `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up. Spec *prose* rots too, and can be
    outright FALSIFIED by measurement (156: "Rust `str::to_lowercase()` does the same").
- **Cost-rank a propagation slice on THREE axes** (153/154): fixture plumbing × target-API coverage
    × **local buildability** (`command -v`). API coverage alone wrongly put C++ at "cheap"; the
    third axis demotes Swift/C++ (no toolchain → CI-only proof).
- **Fourth axis (154): is the fixture a DECLARED INPUT of that build system?** Gradle/MSBuild/CMake
    can report UP-TO-DATE and skip the suite — 13 Kotlin tests silently did not run while every
    surface gate was green. Only a mutate-then-rerun probe (no `clean`) exposes it; CI is immune.

## Current State (assessed-at: 12524f9, iter 156)

- **IN_PROGRESS — CI GREEN and covering all code, but green proves NOTHING about the open defect.**
    `origin/develop` == `ceb32fd` (unmoved): 43 check-runs, 22 names, 0 non-success; HEAD `12524f9`
    is 3 `.claude`-only commits ahead. PR **#44 (develop→main) OPEN** (v0.6.0 NOT shipped;
    **0.5.0**).
- **Iteration 155 PRODUCED NO CODE**: `define-next` **TIMEOUT** (1200 s, 0 turns), advance/review
    never ran, runner still wrote `cid(log)`. It left an **uncommitted `next.md`** — a full work
    package for the `Final_Sigma` fix; central claims verified at 156, but its `CASED_RANGES` 152 /
    `CASE_IGNORABLE_RANGES` 452 shape numbers are NOT verified.
- **Statuses:** Rust-core partially met (crit 3 at **8 of 11** surfaces + the pure-Go port; **crit 4
    would land RED** — `unicode-contract.md`); all else met except CI/CD (partial). All 8 gated
    suites ride EXISTING CI jobs, each rebuilding its native first.
- **Propagation invariant:** `git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** paths,
    matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` (152 drift gate, rides
    `python-test`). Register every new copy; keep canonical basenames.
- **Next = FIX the `Final_Sigma` case-table dependence FIRST** (vendor 16.0.0
    `Cased`/`Case_Ignorable`), *then* land the crit-4 sweep green. Do NOT extend
    `unicode_boundary.json` there — nine suites assert exactly 7/5 counts. It WILL trip CRAP and iai
    (`text_collapse` is benched), unlike the last six iterations. Propagation queues behind both.
- **`specs/rust-core.md` L149-157 is STALE** (Go `Final_Sigma`, fixed 147) and its
    `str::to_lowercase` claim is falsified (156); crit-1/crit-3 boxes unchecked though met.
    Human-owned — CID doesn't edit specs.
- **Issues: 8** (4 `normal`, 4 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`); human backlog
    CLEARED. AUTHORIZED for CID: rubygems `@v2.1.0` pin; major dep bumps **one per step** (magnus
    0.8 / jni 0.22 = source rewrites; xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x);
    exhaustive `specs/ci-cd.md` job table. DEFERRED: npm OIDC.
- **Don't re-flag as DONE**: C#+Kotlin 154, napi+Java 153, drift gate 152, WASM+Ruby 151, Python+Go
    150, sequence vectors 149, sentinel 148 (earlier → `MEMORY-archive.md`).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on rewrap edge cases (list → `lint-tooling.md`). Recipe:
    `cp f /tmp/c.md   && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once
    to confirm.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false`): a mode-only commit shows nothing
    in `git status`/`diff`. Use `git ls-files -s <path>`.
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Grep binding APIs CASE-INSENSITIVELY** (152): napi is **snake_case**, Java/Kotlin/Swift
    camelCase, C# PascalCase. Also `crates/iscc-napi/__tests__` (two underscores).
- **Container toolchains:** `dotnet`, `mvn`, `gcc`, `java`, `go`, `ruby`, `node`, `wasm-pack`,
    `rustc` present; **`cmake`, `swift`, `gradle` ABSENT** → `packages/{cpp,swift}` CI-verified
    only. **Kotlin IS locally testable via `./gradlew … --offline`** (warm cache). Probe with
    `command -v`. Other env gotchas → `MEMORY-archive.md`.
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro.
    **Kotlin** uses JNA — needs BOTH `java.library.path` AND `jna.library.path`. **Release-workflow
    internals** → `MEMORY-archive.md`.
