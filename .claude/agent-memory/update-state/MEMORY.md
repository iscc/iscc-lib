# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Full gate pipelines → `MEMORY-archive.md`.

**Size budget:** Keep under 150 lines. One line per entry. Archive detail.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view` with `--log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then prove it is code-free with
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` (empty = nothing outside CI coverage).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`; **C FFI
    externs**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts** (re-verified 138): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    llms-full ORDERED_PAGES 22; `docs/howto/*.md` 11; speedup 1.3x-158x; release.yml toggles 8; ffi
    extern 47; iscc-lib `#[test]` = **325** (`grep -rc --include="*.rs" crates/iscc-lib/`, sum);
    ci.yml job entries 19 = `grep -cE '^  [a-z_-]+:$'` (raw 21) minus 2.
- **version_sync TARGETS** = **21** (since 129; issues.md's "22" is WRONG); ground truth
    `uv run python scripts/version_sync.py --check | grep -c '^OK'`. Its list = the authoritative
    set of version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING, GREEN. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: the nearby `continue-on-error` is the SEMVER job's.
- **Coverage + CRAP** — one job, ENFORCING: cargo crap `--fail-regression` + `--fail-above` (30.0
    via `.cargo-crap.toml`); baseline `.crap-baseline.json` (**98** entries), max ~22.3. **GOTCHA —
    `--fail-regression` is CI-ONLY, not in `mise run check`**: a new branch/loop in a covered fn →
    exit 1 despite a GREEN local check (bit 121). Fix = refresh that entry in the SAME step.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9`, root `deny.toml` (v2, 2 dev-bench
    ignores). **GOTCHA — the live advisory DB flips this red with NO code change**; fix by updating
    the crate. Not in the devcontainer, so the green CI job is the only confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), does
    NOT flip the run. rust-core.md box `[ ]` = unmet (enforcing + ≥1.0.0, HELD); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs → 21 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job.
- `.github/workflows/release.yml` — 8 registry toggles (crates-io/pypi/npm/maven/ffi/rubygems/nuget/
    maven-kotlin); Swift XCFramework lives in `prepare-release` (~L55) and is NOT a toggle.
- `packages/go/` — pure Go, no CGO/WASM/binaries (x/text 0.40.0). `packages/swift/` + root
    `Package.swift` — `useLocalFramework` toggle, `.binaryTarget` `releaseTag`/`releaseChecksum`,
    `scripts/build_xcframework.sh` = 5 Apple targets. `packages/kotlin/` — Kotlin/JVM + JNA,
    `kotlin("jvm") 2.4.10`; JVM pins + the **consumer-floor trap** → `dep-refresh-survey.md`.
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    freeze filter **MET iter 133** (`src/utils/unicode16.rs` 731-range table + PEP 723 generator
    `scripts/gen_unicode16_unassigned.py` + pre-`nfkc`/`nfd` filter at `utils.rs` L103/L181); unmet
    = boundary vectors + full-code-space sweep (Go exposed until go1.27; an open HUMAN ruling on
    sequence-adjacency blocks the sweep wording). Vector check:
    `grep -rl "1FAE9\|113C5\|20C1" crates/ packages/ tests/ scripts/` → at 138 still only the two
    core files (`utils.rs`, `utils/unicode16.rs`), zero bindings, no fixture file.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
- `crates/iscc-wasm/Cargo.toml` — the `blake3 wasm32_simd` dep (118, #42) is feature-unification
    only, no `use blake3` — **don't prune it** → `MEMORY-archive.md`.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE).
    `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box`
    from `std::hint`); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41).
    `pyproject.toml` now has **zero** `held:` pins (the `ruff<0.16` one died at 137). MSRV recipe:
    `cargo info <crate>@<ver>`.
- **ruff config since 135** (verify via tomllib, not grep — long inline code spans get mangled by
    mdformat rewrapping): `[tool.ruff]` sets `src` to repo root + `crates/iscc-py/python`, and
    `[tool.ruff.lint.isort]` sets `combine-as-imports`. BOTH load-bearing: no `src` → `iscc_lib`
    sorts third-party; no `combine-as-imports` → the `_lowlevel` re-export block shatters into ~60
    statements. `extend-select` = S, C901, I, RUF022, RUF100 with still **NO `select` key** — that
    absence is load-bearing (a bare `select` drops ruff's `E4`/`E7`/`E9`/`F` defaults). Since 135
    `ruff check --fix` auto-sorts imports at commit time. **Project ruff = 0.16.0 since 137** (pin
    dropped, `uv run ruff --version` is ground truth); `uv run ruff check` → exit 0.
- **ruff 0.16 Markdown asymmetry (open `[review]` issue, 137)**: bare `uv run ruff format --check`
    (ci.yml L71 + `mise run lint`) formats Python fences inside `.md` → **153** files vs 25
    pre-0.16; the prek `ruff-check`/`ruff-format` hooks are `types: [python]`
    (`.pre-commit-config.yaml` L42/ L48) and pre-push never runs `ruff format`. So local green + CI
    red is possible on docs edits. 129 tracked `.md` files are clean today. `decisions.md` (137)
    rejected a `[tool.ruff.format]` exclude as scope exclusion — the fix must widen the local
    surface, not shrink the CI one.
- **`rb_sys` pinned in THREE places that must move together** (130): `crates/iscc-rb/Gemfile` (exact
    `0.9.123`), `Gemfile.lock`, `tag:` at `release.yml:853` (cross-gem image must match).
- **Full dependency-pin inventory + slice history** → `dep-refresh-survey.md` (re-verified 129).
    Headlines: ci.yml/docs.yml GHA refs CURRENT (setup-uv = EXACT `@v9.0.0`); release.yml lags and
    has **no setup-uv step** (issues.md claims otherwise — wrong); no Dependabot/Renovate config
    exists at all (freshness gap, v0.6.0); `mise.toml` has no `[tools]`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff — a review
    PASS with green `mise run check` can still fail CI-only gates like CRAP `--fail-regression`).
    **Always diff `specs/` too — a `human(decide)` commit can add criteria and flip
    met→partially-met with zero code change (that IS iteration 132).**
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + the `rust-core.md` semver box are
    maintained. Verify in code, never read boxes as done/not-done.

## Current State (assessed-at: 8629752, iter 138)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released. Iter 137 = ruff sub-slice E only (`pyproject.toml` +
    `uv.lock`, 2 files — no Rust/`specs/`/`docs/`, so every binding/bench/docs section carried
    forward verbatim). Still partially met: Rust-core (boundary vectors + sweep + semver/v1.0.0
    HELD) and CI/CD (release.yml refs + the new Markdown gate-parity gap).
- **CI GREEN on origin/develop tip `3063fc6`**; HEAD `8629752` = +1 UNPUSHED log commit,
    `origin/develop..HEAD` minus `.claude/` = EMPTY stat. **41** check-runs, 21 names, 0
    non-success. ~2x jobs because PR **#44 (develop→main) is OPEN** → every develop commit fires a
    `push` AND a `pull_request` run.
- **Open tension (unsettled at 138)**: 133 wanted a Rust-only boundary fixture; the 134-137 reviews
    all say wire NO vectors until the human ruling lands. 138 = close the Markdown gate-parity gap.
- **Dep-refresh: ALL 8 slices DONE** (124-130 ecosystems, 131-137 ruff sub-slices A-E: 104→26→12→3→0
    findings, then the pin drop). Remaining in that issue is human/major-gated only: magnus 0.8, jni
    0.22 (source rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the
    release.yml GHA refs (bundle with the `if:`-guard fix). **Never `ruff@0.16 check --fix .`** —
    deletes load-bearing `# noqa`; all landed slices used `--select`.
- **8 issues: 0 critical, 6 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule pre-normalization ordering diverges from iscc-core on *sequences*; a spec-wording
    ruling, not a redesign → `unicode-contract.md`; still open at 138). NEW at 137: the ruff
    Markdown gate-parity `[review]` issue (CID-doable, one config file). Contested: Unicode step
    (b). Human-gated: npm OIDC, single-registry re-trigger; low (CID skips) = v1.0.0, docs logos.
- **Don't re-flag as new work** (all DONE): ruff sub-slices A-E (131-137), Unicode freeze filter +
    generator (133), Kotlin 2.3+ floor docs (132), dep slices 1-7 (124-130), aarch64 wheels #49
    (123), CRAP baseline (122), trailing-byte fixes (120-121), Go IDv1 #43 (119), WASM SIMD #42
    (118), GIL #39+#41, cargo-deny (113), iai perf gate (107-111), semver gate (93). CID infra
    (audit role, metrics.jsonl, decisions.md) = meta, NOT target — ignore.
- **Known non-regression**: the `proc-macro-error2 v2.0.1` future-incompat warning on cargo
    test/bench comes from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys; no upstream fix yet.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: hook args are `--wrap 100 --number` — always
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff /tmp/c.md f`, then
    `cp /tmp/c.md f` to adopt, and re-run once for idempotence (bare `mdformat` renumbers lists to
    all-`1.`). It ABORTS the commit on nested/escaped backticks in a code span or a wrapped line
    starting `+`/`-`/`>`, and escapes a re-wrapped bare `<number>.` at line start to `26\.` — reword
    those (e.g. "26 findings" instead of a leading "26.").
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`) or `git diff --summary` — `--stat` shows it as a 0-line file (136).
- **metrics.jsonl counts include gitignored build artifacts** — post-`rake compile` `crates/iscc-rb`
    jumped 8→12 files / 9→18 `unsafe` purely from `tmp/*/stage/` copies. Never read a metrics delta
    as real code change without `git diff --stat`.
- **Gradle flakes on this bind mount** (`Unable to delete file …/build/kotlin/…`) —
    `./gradlew clean` before believing a Kotlin build failure; check the test XML first.
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32
    armv7→arm, dual Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
