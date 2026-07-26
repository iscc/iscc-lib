# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
pipelines, release internals), `dep-refresh-survey.md` (pin inventory), `unicode-contract.md`,
`lint-tooling.md` (ruff/prek/mdformat). **Size budget: under 165 lines** — archive detail eagerly;
the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view` with `--log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then prove it is code-free with
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` (empty = nothing outside CI coverage).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 145): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    tracked `docs/**/*.md` **24**, ORDERED_PAGES **23**, `docs/llms.txt` page links **17** (+1 to
    llms-full.txt) — see docs-drift entry; `docs/howto/*.md` 11; speedup 1.3x-158x; release.yml
    toggles 8; ffi extern 47; iscc-lib `#[test]` = **328**
    (`grep -rc --include="*.rs" crates/iscc-lib/`, sum); ci.yml job entries **20** =
    `grep -cE '^  [a-z_-]+:$'` (raw **22**) minus 2.
- **version_sync TARGETS** = **21**
    (`uv run python scripts/version_sync.py --check | grep -c '^OK'`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144). Always diff
    `.claude/context/specs/`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: nearby `continue-on-error` is the SEMVER job's.
- **release.yml static gate — ALL 3 CHECKS BUILT** (1-2 iter 142, 3 iter 144).
    `scripts/check_release_workflow.py` (23 fns; `uv run` it; exit 0 + one `OK:` line): guard shape
    (**29 jobs, 28 guarded**, only `prepare-release` bare), input wiring, artifact wiring with
    `matrix.include` expansion, `needs:` graph — all offline. `--check-action-inputs` (opt-in,
    NETWORK) validates `with:` keys + `steps.<id>.outputs.<x>` against 18 distinct published
    `action.yml`. Prek hook `check-release-workflow` (release.yml only, offline) +
    `tests/test_check_release_workflow.py` (**19** tests, fetcher-injected so network-free);
    `pyyaml` is a dev dep. **FAILS OPEN by design** (`decisions.md` 2026-07-26): only a double-404
    reds it; timeouts/429/5xx print `warning: skipped` and exit 0 → **verify the JOB LOG has zero
    `warning: skipped` lines**, the badge alone is not proof it ran.
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
- `.github/workflows/ci.yml` — **20 YAML job entries → 21 jobs → 22 distinct check names** (job
    `release-workflow` / check "Release workflow (action inputs)" added 144): `python-test` =
    3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job. `release.yml` — 8
    registry toggles; the Swift XCFramework step lives in `prepare-release` and is NOT a toggle.
- `packages/` layout detail (go pure-Go no-CGO, swift XCFramework 5 targets, kotlin JNA) →
    `MEMORY-archive.md`. `packages/go` is the ONLY binding not inheriting the freeze rule.
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    filter MET 133 (`src/utils/unicode16.rs` 731 ranges, generator
    `scripts/gen_unicode16_unassigned.py`, pre-`nfkc`/`nfd` filter `utils.rs` L103/L181); Rust
    vectors MET 141 (`tests/unicode_boundary.json` is ASCII-**escaped** — `grep "1FAE9"` MISSES it,
    grep the filename; zero hits outside `crates/iscc-lib` at 145); docs MET 143 (`docs/unicode.md`
    plus a go.md note, carrying a DELIBERATE CPython-3.14 placeholder tied to the ordering ruling).
    UNMET and double-blocked: full-code-space sweep (needs the wording ruling) + BINDING half of the
    vectors (needs the Go go1.27 vendor-or-skip call).
- **Docs 3-way wiring has NO gate, and it HAS drifted**: a page must be hand-added to
    `zensical.toml` nav + `scripts/gen_llms_full.py` ORDERED_PAGES + `docs/llms.txt`. Measured 144
    review / re-verified 145: nav 24 + ORDERED_PAGES 23 agree (both minus `includes/`, a snippet
    partial), but **`docs/llms.txt` lists only 17** — missing the `howto/` pages for c-cpp, dotnet,
    kotlin, ruby and swift plus `ruby-api.md`, i.e. 5 of 11 languages. Violates
    `specs/documentation.md` L32 ("links to all documentation pages") → Documentation is **partially
    met**, not met. Command: `grep -o 'https://lib.iscc.codes/[^)]*' docs/llms.txt | sort` vs
    `git ls-files 'docs/**/*.md'`.
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
    diff `.claude/context/specs/` too** — a `human(decide)` commit can flip met→partially- met with
    zero code change (132). **A review's "Issues found" can too** (145: the llms.txt drift measured
    at 144's review downgraded Documentation with no file changing) — read issues.md diffs for NEW
    measured facts, not just for headers.
- **Issues diff**: scan for NEW/removed `[human]`/`[review]` entries + `HUMAN REVIEW REQUESTED`; a
    rewritten-in-place issue keeps the count flat, so diff issue *bodies*, not just headers.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.

## Current State (assessed-at: 2d1c03c, iter 145)

- **IN_PROGRESS — CI GREEN.** Version still **0.5.0** (PR #44 titled "Release 0.6.0" is just the
    open develop→main PR — do NOT read it as a shipped release). Iter 144 = the
    `--check-action-inputs` gate: diff `5f75fea..HEAD` had exactly 4 non-`.claude` files (`ci.yml`
    +15, `docs/development.md`, `scripts/check_release_workflow.py` +175,
    `tests/test_check_release_workflow.py` +131); NO `crates/`, `packages/`,
    `.claude/context/specs/`, `target.md`, `release.yml` or manifest change → every library section
    carried forward.
- **CI GREEN on origin/develop tip `b915582`**; HEAD `2d1c03c` = +1 UNPUSHED `cid(log)` commit,
    `origin/develop..HEAD` minus `.claude/` = EMPTY stat. **43** check-runs, **22** names, 0
    non-success. ~2x jobs because PR **#44 (develop→main) is OPEN** → every develop commit fires a
    `push` AND a `pull_request` run.
- **Section statuses at 145:** Rust-core partially met (binding vectors + sweep + semver/v1.0.0
    HELD); **Documentation DOWNGRADED met → partially met** (llms.txt drift, above); CI/CD partially
    met (new gate's 4 blind spots + human-gated items); all bindings + benchmarks met.
- **Cadence — DRIFT WATCH.** 140 tooling / 141 tests / 142 tooling / 143 docs / 144 tooling. States
    143, 144 AND 145 all recommend **escalating the two parked rulings** (freeze-rule ordering
    wording; Go 15.0-tables vendor-or-skip) — criteria 3+4 double-blocked, no CID path exists.
- **Dep-refresh: ALL 9 slices DONE** → `dep-refresh-survey.md`. Remainder human/major-gated ONLY:
    magnus 0.8, jni 0.22 (source rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **9 issues: 0 critical, 7 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule ordering diverges from iscc-core on *sequences*; spec-wording ruling, upstream
    iscc/iscc-core#137). Net +1 at 144: the action-input-gate issue was RESOLVED+DELETED, two new
    `[review]` filed — (a) **docs-list parity gate + the 6-link llms.txt data fix** (unblocked, live
    user-facing gap, breaks the tooling cadence → recommended next), (b) harden
    `--check-action-inputs` (widen `except` to `http.client.HTTPException`/`yaml.YAMLError`;
    required-input reverse check; fail when 0 refs resolve; Docker `args`/`entrypoint` + YAML-1.1
    bool false positives; job-level `uses:` unscanned). **Never read a flat count as "nothing
    happened"** — bodies move every iteration.
- **Don't re-flag as new work** (DONE): release.yml action-input gate + `release-workflow` CI job
    (144); Unicode consumer docs page (143); release.yml static checks 1-2 (142); Unicode boundary
    Rust half (141); release.yml GHA refs (140); re-trigger guards + `.pyi` hooks (139); Markdown
    hook parity (138); ruff 0.16 A-E (131-137); freeze filter (133). Pre-133 → `MEMORY-archive.md`.
- **Pre-existing spec drift for Titusz**: `specs/ci-cd.md`'s CI job table = 14 rows vs 21 jobs
    (descriptive, not exhaustive) — NOT a regression. Review may not edit specs for
    `[review]`-sourced issues, so define-next must stop assigning spec edits to review.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`) or `git diff --summary` — `--stat` shows it as a 0-line file (136).
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
