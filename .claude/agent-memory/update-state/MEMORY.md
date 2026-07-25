# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
pipelines, release internals), `dep-refresh-survey.md` (pin inventory), `unicode-contract.md`,
`lint-tooling.md` (ruff/prek/mdformat). **Size budget: under 140 lines** — archive detail.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view` with `--log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then prove it is code-free with
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` (empty = nothing outside CI coverage).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`; **C FFI
    externs**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts** (re-verified 139): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    llms-full ORDERED_PAGES 22; `docs/howto/*.md` 11; speedup 1.3x-158x; release.yml toggles 8; ffi
    extern 47; iscc-lib `#[test]` = **325** (`grep -rc --include="*.rs" crates/iscc-lib/`, sum);
    ci.yml job entries 19 = `grep -cE '^  [a-z_-]+:$'` (raw 21) minus 2.
- **version_sync TARGETS** = **21** (ground truth
    `uv run python scripts/version_sync.py --check | grep -c '^OK'`); its list = the authoritative
    set of version-synced files, absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: the nearby `continue-on-error` is the SEMVER job's.
- **Coverage + CRAP** — one ENFORCING job: `--fail-regression` + `--fail-above` 30.0
    (`.cargo-crap.toml`); baseline `.crap-baseline.json` (**98** entries, max ~22.3). **GOTCHA —
    `--fail-regression` is CI-ONLY, not in `mise run check`**: a new branch in a covered fn → exit 1
    despite a GREEN local check (bit 121). Fix = refresh that entry in the SAME step.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9`, root `deny.toml` (v2, 2 dev-bench
    ignores). **GOTCHA — the live advisory DB flips this red with NO code change** (fix = bump the
    crate); not in the devcontainer, so green CI is the only confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), does
    NOT flip the run. rust-core.md box `[ ]` = unmet (enforcing + ≥1.0.0, HELD); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests and `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs → 21 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job. `release.yml` — 8
    registry toggles; the Swift XCFramework step lives in `prepare-release` and is NOT a toggle.
- `packages/go/` — pure Go, no CGO/WASM/binaries (x/text 0.40.0). `packages/swift/` + root
    `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`, `build_xcframework.sh` = 5 Apple
    targets. `packages/kotlin/` — Kotlin/JVM + JNA 2.4.10; pins → `dep-refresh-survey.md`.
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    freeze filter **MET iter 133** (`src/utils/unicode16.rs` 731-range table + PEP 723 generator
    `scripts/gen_unicode16_unassigned.py` + pre-`nfkc`/`nfd` filter at `utils.rs` L103/L181); unmet
    = boundary vectors + full-code-space sweep (Go exposed until go1.27; an open HUMAN ruling on
    sequence-adjacency blocks the sweep wording). Vector check
    `grep -rl "1FAE9\|113C5\|20C1" crates/ packages/ tests/ scripts/` → at 139 still only the two
    core files, zero bindings, no fixture file.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
    `crates/iscc-wasm/Cargo.toml`'s `blake3 wasm32_simd` dep (118, #42) is feature-unification only,
    no `use blake3` — **don't prune it** → `MEMORY-archive.md`.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE).
    `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box`
    from `std::hint`); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41);
    `pyproject.toml` has **zero** (the `ruff<0.16` one died at 137). MSRV: `cargo info <crate>@<v>`.
- **Ruff/prek/mdformat detail → `lint-tooling.md`.** Headlines: ruff **0.16.0** since 137 (ground
    truth = `uv run ruff --version`; `ruff check` exits 0); `pyproject.toml` `src` +
    `combine-as-imports` + the missing `select` key are all load-bearing; **prek type tags ≠ ruff's
    recursive discovery** — Markdown half closed at 138, `.pyi` half still OPEN (both hooks skip
    `_lowlevel.pyi`). Probe hooks with `prek run <hook> --files <probe>`, never `git ls-files`.
- **Full dependency-pin inventory + slice history** → `dep-refresh-survey.md`. Headlines:
    ci.yml/docs.yml GHA refs CURRENT (setup-uv = EXACT `@v9.0.0`); release.yml lags and has **no
    setup-uv step** (issues.md claims otherwise — wrong); no Dependabot/Renovate at all; `mise.toml`
    has no `[tools]`; **`rb_sys` is pinned in THREE places that must move together** — the Gemfile
    (exact `0.9.123`), `Gemfile.lock`, and `tag:` at `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff — a PASS
    with green `mise run check` can still fail CI-only gates). **Always diff `specs/` too — a
    `human(decide)` commit can flip met→partially-met with zero code change (iteration 132).**
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + the `rust-core.md` semver box are
    maintained. Verify in code, never read boxes as done/not-done.

## Current State (assessed-at: 862c128, iter 139)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released. Iter 138 = the prek `ruff-format` widening only
    (`.pre-commit-config.yaml` + `CLAUDE.md` + `docs/development.md`, 3 files — no
    Rust/`specs/`/`target.md`, so every binding/bench section carried forward verbatim). Still
    partially met: Rust-core (boundary vectors + sweep + semver/v1.0.0 HELD) and CI/CD (release.yml
    refs + the new `.pyi` hook hole).
- **CI GREEN on origin/develop tip `2b7ec6f`**; HEAD `862c128` = +1 UNPUSHED log commit,
    `origin/develop..HEAD` minus `.claude/` = EMPTY stat. **41** check-runs, 21 names, 0
    non-success. ~2x jobs because PR **#44 (develop→main) is OPEN** → every develop commit fires a
    `push` AND a `pull_request` run.
- **Cadence warning**: iters 134-138 were ALL Python dev-tooling config; library code has not moved
    since 133 and cannot on the Unicode track until the human ruling lands. Flag a third parallel
    tooling thread as drift. **Open tension**: 133 wanted a Rust-only boundary fixture, but every
    134-138 review says wire NO vectors until the ruling lands.
- **Dep-refresh: ALL 8 slices DONE** (124-130 ecosystems, 131-137 ruff A-E) →
    `dep-refresh-survey.md`. Remaining is human/major-gated only: magnus 0.8, jni 0.22 (source
    rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus release.yml GHA refs
    (bundle with the `if:`-guard fix).
- **8 issues: 0 critical, 6 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule pre-normalization ordering diverges from iscc-core on *sequences*; a spec-wording
    ruling, not a redesign → `unicode-contract.md`; still open at 139). Count held at 8 across 138:
    Markdown gate-parity deleted (resolved), `.pyi` hook-gap `[review]` took its place. Contested:
    Unicode step (b). Human-gated: npm OIDC, single-registry re-trigger; low = v1.0.0, docs logos.
- **Don't re-flag as new work** (all DONE): Markdown hook parity (138), ruff A-E (131-137), Unicode
    freeze filter (133), Kotlin 2.3+ floor docs (132), dep slices 1-7 (124-130), aarch64 wheels #49
    (123), CRAP baseline (122), trailing-byte fixes (120-121), Go IDv1 #43 (119), WASM SIMD #42
    (118), GIL #39+#41, cargo-deny (113), iai perf gate (107-111), semver gate (93). CID infra
    (audit role, metrics.jsonl, decisions.md) = meta, NOT target — ignore.
- **Known non-regression**: the `proc-macro-error2 v2.0.1` future-incompat warning on cargo
    test/bench comes from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys; no upstream fix yet.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`) or `git diff --summary` — `--stat` shows it as a 0-line file (136).
- **metrics.jsonl counts include gitignored build artifacts** (post-`rake compile` iscc-rb jumped
    8→12 files purely from `tmp/*/stage/`) — never read a metrics delta as real code change without
    `git diff --stat`. **Gradle flakes on this bind mount** (`Unable to delete file …/build/…`) —
    `./gradlew clean` before believing a Kotlin failure; check the test XML first.
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32
    armv7→arm, dual Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
