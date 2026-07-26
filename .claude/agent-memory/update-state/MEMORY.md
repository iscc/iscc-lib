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
- **Counts** (re-verified 146): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    tracked docs `.md` **24** (23 pages + 1 `includes/` partial), ORDERED_PAGES **23**,
    `docs/llms.txt` page links **23** (+1 to llms-full.txt); `docs/howto/*.md` 11; speedups
    1.3x-158x; release.yml toggles 8; ffi extern 47; iscc-lib `#[test]` = **328**
    (`grep -rc --include="*.rs" crates/iscc-lib/`, sum); ci.yml job entries **20** =
    `grep -cE '^  [a-z_-]+:$'` (raw **22**) minus 2.
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
- **release.yml static gate — ALL 3 CHECKS BUILT** (1-2 iter 142, 3 iter 144); internals →
    `MEMORY-archive.md`. Runs offline as a prek hook + `release-workflow` CI job for the network
    mode. **FAILS OPEN by design** (`decisions.md` 2026-07-26): only a double-404 reds it;
    timeouts/429/5xx print `warning: skipped` and exit 0 → **verify the JOB LOG has zero
    `warning: skipped` lines**, the badge alone is not proof it ran.
- **Docs page-list parity (NEW iter 145)** — `scripts/check_docs_nav.py` asserts disk pages ⇄
    `zensical.toml` nav ⇄ `ORDERED_PAGES` ⇄ `docs/llms.txt` are one set; exit 0 prints
    `OK: 23 documentation pages consistent…`. Enforced by prek hook `check-docs-nav` (scoped to the
    4 inputs) + `tests/test_check_docs_nav.py` (8 tests) in CI's `python-test` — **no dedicated CI
    job**. Known latent holes (filed): `NAV_MD_RE` matches inside TOML comments; the prek hook does
    not fire on page *deletion* (pytest catches it). Takes 4 `Path` args → point it at temp
    checkouts to verify.
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
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    filter MET 133 (`src/utils/unicode16.rs` 731 ranges, generator
    `scripts/gen_unicode16_unassigned.py`, pre-`nfkc`/`nfd` filter `utils.rs` L103/L181); Rust
    vectors MET 141 (`tests/unicode_boundary.json` is ASCII-**escaped** — `grep "1FAE9"` MISSES it,
    grep the filename; zero hits outside `crates/iscc-lib` at 146); docs MET 143 (`docs/unicode.md`
    plus a go.md note, carrying a DELIBERATE CPython-3.14 placeholder tied to the ordering ruling).
    UNMET and double-blocked: full-code-space sweep (needs the wording ruling) + BINDING half of the
    vectors (needs the Go go1.27 vendor-or-skip call).
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`) —
    now gated, see check-docs-nav above.
- `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via `streaming::`.
    iscc-wasm's `blake3 wasm32_simd` dep is feature-unification only — **don't prune** (archive).
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
    = the ONLY unpinned ref (human ruling pending). No Dependabot/Renovate; no `[tools]` in
    `mise.toml`; **`rb_sys` pinned in THREE places that move together** — Gemfile (exact `0.9.123`),
    `Gemfile.lock`, `tag:` at `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff). **Always
    diff `.claude/context/specs/` too** — a `human(decide)` commit can flip met→partially-met with
    zero code change (132). **A review's "Issues found" can too** (145: an llms.txt drift measured
    in review downgraded Documentation with no file changing) — read issues.md diffs for NEW
    measured facts, not just headers.
- **Issues diff**: a rewritten-in-place issue keeps the count flat (145→146: docs-list issue
    resolved, replaced by the docs-nav blind-spot issue, still 9). Diff issue *bodies*.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.

## Current State (assessed-at: 57ebcb8, iter 146)

- **IN_PROGRESS — CI GREEN** on origin/develop tip `06ff088`: **43** check-runs, **22** names, 0
    non-success. HEAD `57ebcb8` = +1 UNPUSHED `cid(log)` commit, `origin/develop..HEAD` minus
    `.claude/` = EMPTY stat. ~2x runs because PR **#44 (develop→main) is OPEN** (titled "Release
    0.6.0" — NOT a shipped release; version is still **0.5.0**).
- Iter 145 = docs-list fix + gate: diff `2d1c03c..HEAD` had exactly 5 non-`.claude` files
    (`.pre-commit-config.yaml`, `docs/development.md`, `docs/llms.txt` +6,
    `scripts/check_docs_nav.py` new, `tests/test_check_docs_nav.py` new); NO `crates/`, `packages/`,
    specs, `target.md`, manifest or workflow change → all library sections carried forward.
- **Section statuses at 146:** Rust-core partially met (binding vectors + sweep + semver/v1.0.0
    HELD); **Documentation UPGRADED back to met**; CI/CD partially met (gate blind spots +
    human-gated items); all bindings + benchmarks met.
- **Cadence — DRIFT WATCH**: 141 tests / 142 tooling / 143 docs / 144 tooling / 145 docs+tooling.
    146's recommendation (bundle the two gate blind-spot issues) would make it 4 tooling-ish in 6 —
    accepted only because every remaining user-facing item is blocked.
- **Dep-refresh: ALL 9 slices DONE** → `dep-refresh-survey.md`. Remainder human/major-gated ONLY:
    magnus 0.8, jni 0.22 (source rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **9 issues: 0 critical, 7 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule ordering diverges from iscc-core on *sequences*; spec-wording ruling, upstream
    iscc/iscc-core#137). Only 2 unblocked CID-doable: harden `--check-action-inputs`, and fix
    `check_docs_nav.py`'s comment/deletion blind spots. Everything else human/major-gated or
    double-blocked.
- **Don't re-flag as new work** (DONE): docs-list parity gate + llms.txt fix (145); release.yml
    action-input gate + CI job (144); Unicode consumer docs page (143); release.yml static checks
    1-2 (142); Unicode boundary Rust half (141); release.yml GHA refs (140); re-trigger guards +
    `.pyi` hooks (139); Markdown hook parity (138); ruff 0.16 A-E (131-137); freeze filter (133).
- **Pre-existing spec drift for Titusz**: `specs/ci-cd.md`'s CI job table = 14 rows vs 21 jobs
    (descriptive, not exhaustive) — NOT a regression. Review may not edit specs for
    `[review]`-sourced issues, so define-next must stop assigning spec edits to review.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`); `--stat` shows it as a 0-line file (136).
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
