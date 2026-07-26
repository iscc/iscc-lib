# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `lint-tooling.md` (ruff/prek/mdformat). **Size budget: under 140 lines** —
archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap instead of calling CI green-for-HEAD (bit at 148).
- **ALWAYS `tail -4 .claude/context/iterations.jsonl`** — the ONLY place a crashed role shows up.
    **A non-OK status does NOT mean no work: always corroborate with `git log` for the
    `cid(<role>):` commit.** Infra crash = `"status":"FAIL","turns":1,"cost_usd":~0.0006` AND no
    commit, as at iteration 147. Benign overrun = `"status":"TIMEOUT","turns":0` WITH the commit
    present, as at iteration 148 — since `4739a4b` the runner detects this and logs `recovered`.
    When review genuinely crashes: no verdict, handoff has only the advance section, and resolved
    issues stay in issues.md (deleting them is review's job) so counts over-report.
- **Verify a Go-package claim in ~60s**: `/tmp` module with
    `require github.com/iscc/iscc-lib/packages/go v0.0.0` +
    `replace … => /workspace/iscc-lib/packages/go`, then
    `GOFLAGS=-mod=mod CGO_ENABLED=0 go mod tidy && CGO_ENABLED=0 go run main.go`.
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 148): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    tracked docs `.md` **24** (23 pages + 1 `includes/` partial), ORDERED_PAGES + llms.txt links
    **23**; `docs/howto/*.md` 11; speedups 1.3x-158x; release.yml toggles 8; ffi extern 47; iscc-lib
    `#[test]` **334** (`grep -rc --include="*.rs" crates/iscc-lib/`, sum); `packages/go`
    `^func Test` **174**; ci.yml jobs **20** (`grep -cE '^  [a-z_-]+:$'` raw **22**, minus 2).
- **GOTCHA — `git ls-files 'docs/**/*.md'` returns 13, NOT 24**: it misses top-level `docs/*.md`.
    Use both globs, or run `uv run scripts/check_docs_nav.py` (prints the authoritative page count).
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
- **release.yml static gate — ALL 3 CHECKS BUILT** (142/144, hardened 146; internals →
    `MEMORY-archive.md`): offline prek hook + `release-workflow` CI job. **FAILS OPEN by design** —
    only a double-404 reds it, so **check the JOB LOG for zero `warning: skipped`**, not the badge.
- **Docs page-list parity (145)** — `scripts/check_docs_nav.py` asserts disk pages ⇄ `zensical.toml`
    nav ⇄ `ORDERED_PAGES` ⇄ `docs/llms.txt` are one set. Enforced by prek hook `check-docs-nav` +
    pytest in CI's `python-test` — **no dedicated CI job**. Takes 4 `Path` args (temp checkouts).
- **Coverage + CRAP** — ENFORCING: `--fail-regression` + `--fail-above` 30.0 (`.cargo-crap.toml`);
    baseline `.crap-baseline.json` (**100** entries; count with
    `python3 -c "import json;print(len(json.load(open('.crap-baseline.json'))['entries']))"` — the
    file is a dict with `$schema`/`version`/`entries`). **GOTCHA — `--fail-regression` is CI-ONLY,
    not in `mise run check`**: a new branch in a covered fn → exit 1 despite a GREEN local check
    (bit 121). Fix = refresh that entry in the SAME step.
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
    Criterion 1 **MET since 148** (`UNASSIGNED_SENTINEL` + `.map(` at both `utils.rs` call sites);
    `docs/unicode.md` rewritten in the SAME commit. Still open: sequence vectors + binding
    propagation (crit 3), differential sweep (crit 4, no harness in `scripts/`). `decisions.md`
    keeps 2 SUPERSEDED designs that must NOT be implemented.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
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
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change.
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede` (it logs losing designs chronologically). At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute binding claims yourself** — cheap probes beat inherited text (confirmed a
    `critical` bug at 147, its fix at 148). Deliberate deviations from an issue's fix sketch are
    legitimate: read the in-source comment before calling one a defect.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.
    Spec *prose* also rots: at 149 `rust-core.md` still described a defect fixed 2 iterations back.
- **A fixture can be structurally incapable of gating what it claims to gate** (149:
    single-code-point vectors wrapped in ASCII score the conformant and the non-conformant design
    identically). Ask what a test would have to *distinguish*, not just whether it passes.

## Current State (assessed-at: c597496, iter 149)

- **IN_PROGRESS — CI GREEN AND COVERING HEAD.** HEAD **==** `origin/develop` == `c597496`; 43
    check-runs, 22 names, 0 non-success, none in progress. Nothing unpushed. ~2x runs because PR
    **#44 (develop→main) is OPEN** ("Release 0.6.0" — NOT shipped; version **0.5.0**).
- **Iter 148's review logged TIMEOUT (turns 0, 3000s) but HAD committed** (`b9a600c` + `82ec3ac`) —
    a false failure. **A human fixed the loop**: `4739a4b` adds `_role_commit_landed()` (role that
    commits then overruns no longer fails the iteration, + 94 lines in `tests/test_cid.py`) and
    `c597496` raises the review timeout 3000→3600s. **Do not re-raise this as a loop defect.** New
    log field to expect: `"recovered": true`.
- **Statuses:** Rust-core partially met (crit 1 + 2 MET; crit 3 partial — no sequence vectors, zero
    binding propagation; crit 4 not started); Documentation now **met**; all binding sections +
    benchmarks met; CI/CD partially met. **Next = the 4 sequence vectors** in
    `unicode_boundary.json` + extend the ungated content guard (assert by code-point set, not
    shape); expected outputs are already tabulated in issues.md — don't re-derive.
- **`specs/rust-core.md` is STALE on Go `Final_Sigma`** (present tense, box unchecked) though fixed
    at 147 and verified at 148/149 — human-owned, CID doesn't edit specs.
- **Issues: 8** (4 `normal`, 4 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`). Review deleted both
    resolved entries at 148 and folded their residuals into the Unicode umbrella issue.
- **Human backlog CLEARED** (0 `HUMAN REVIEW REQUESTED`). AUTHORIZED for CID: rubygems `@v2.1.0`
    pin; major dep bumps **one per step** (magnus 0.8, jni 0.22 as source rewrites, one crate per
    step; xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x); exhaustive `specs/ci-cd.md` job
    table. DEFERRED: npm OIDC (not v0.6.0; don't even prepare the diff). **Cadence escalation is
    discharged — do not repeat it.** The 3 gate-script remainders are `low` + trigger-contingent:
    don't act.
- **Don't re-flag as DONE**: sentinel conversion + `docs/unicode.md` rewrite 148; Go `Final_Sigma`
    147; gate blind-spots 146; docs-list gate 145; action-input gate 144; Unicode docs page 143;
    release.yml static checks 142; boundary-vector Rust half 141; GHA refs 140 (pre-140 done-work →
    `MEMORY-archive.md` / issues.md slice log).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` bind mount): a mode-only commit
    shows nothing in `git status`/`diff` (`--stat` = a 0-line file). Use `git ls-files -s <path>`.
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
