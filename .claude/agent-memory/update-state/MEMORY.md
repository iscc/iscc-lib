# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Full gate pipelines → `MEMORY-archive.md`.

**Size budget:** Keep under 140 lines. One line per entry. Archive detail.

## Exploration Shortcuts

- **Per-crate READMEs / CLAUDE.md** (12 each): `ls crates/*/{README.md,CLAUDE.md} packages/*/…`
- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs):
    `gh api "repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` on the real
    origin/develop tip — **QUOTE the path**, zsh treats `?` as a glob and errors "no matches found".
    **Failed logs**: `gh run view <id> --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` (code after the last CI sha is
    UNVERIFIED). Usual: HEAD = +1 log-only commit — NOT guaranteed (iter 121 had none).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`; **C FFI
    externs**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts** (re-verified 131): pytest-benchmark 18; UniFFI exports 32; llms-full ORDERED_PAGES 22;
    `docs/howto/*.md` 11; speedup 1.3x-158x; release.yml toggles 8; ffi extern 47; iscc-lib
    `#[test]` = **320** (`grep -rc --include="*.rs" crates/iscc-lib/`; src/\*.rs alone = 270);
    ci.yml job entries 19 = `grep -cE '^  [a-z_-]+:$' ci.yml` minus 2 (`push`/`pull_request`).
- **version_sync TARGETS** = **21** since iter 129; ground truth =
    `uv run python scripts/version_sync.py --check | grep -c '^OK'` (handoff/learnings say "22" —
    wrong). The list (~L259-289) is the authoritative set of version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING, GREEN. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: the nearby `continue-on-error` is the SEMVER job's.
- **Coverage + CRAP** — one job, ENFORCING: cargo crap `--fail-regression` + `--fail-above` (30.0
    via `.cargo-crap.toml`); baseline `.crap-baseline.json` (97 entries), max ~22.3. **GOTCHA —
    `--fail-regression` is CI-ONLY, not in `mise run check`**: a new branch/loop in a covered fn →
    exit 1 despite a GREEN local check (bit 121). Fix = refresh that entry in the SAME step.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9` → `cargo deny check`; root `deny.toml`
    (v2, 2 dev-bench ignores); `mise run audit`. **GOTCHA — live advisory DB flips this red with NO
    code change**: fix via `cargo update -p <crate>` (preferred) or justified `ignore`. NOT in
    devcontainer → green CI job is the only real confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), its
    conclusion does NOT flip the run. rust-core.md box `[ ]` = unmet (needs enforcing + ≥1.0.0, HELD
    by Titusz); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs → 21 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job.
- `.github/workflows/release.yml` — 8 registry toggles (crates-io/pypi/npm/maven/ffi/rubygems/nuget/
    maven-kotlin); Swift XCFramework lives in `prepare-release` (~L55) and is NOT a toggle.
- `packages/go/` — pure Go, no CGO/WASM/binaries; deps refreshed iter 129 (x/text 0.40.0). ISCC-IDv1
    (#43, 119) + trailing-byte fixes (120/121) DONE → details in `MEMORY-archive.md`.
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`
    `releaseTag`/`releaseChecksum`; `scripts/build_xcframework.sh` = 5 Apple targets.
    `packages/kotlin/` — Kotlin/JVM + JNA, Gradle wrapper major deferred; JVM pins + the
    **consumer-floor trap** → `dep-refresh-survey.md`. README fixed to `0.5.0` + version_sync'd
    (129); `specs/kotlin-bindings.md` L150+L165 still say `jna:5.16.0@aar` (human-owned).
- **Unicode version = UNPINNED cross-impl variable** (iter 129 `[review]`, HUMAN REVIEW):
    `utils.rs:28-32` strips `GeneralCategory::Unassigned` via `unicode-general-category` 1.1.0 =
    **U16** (+`unicode-normalization` 0.1.25 = U17) vs Go 15.0.0 / CPython 3.13 15.1.0 → U16/U17
    chars (`Ɤ` U+A7CB) KEPT by Rust, STRIPPED by Go+`iscc-core`; 5,813 `text_clean` diffs →
    divergent Meta/Text codes. Rust core = the outlier; no gate catches it (vectors predate U16).
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
- `crates/iscc-wasm/Cargo.toml` — the `blake3 = { features = ["wasm32_simd"] }` dep (118, #42) is
    feature-unification only, no `use blake3` — **don't prune it** → `MEMORY-archive.md`.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE);
    video detach opens strictly AFTER frame extraction; meta/audio/mixed stay attached by design.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint` since 126); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **No Dependabot/Renovate** (`dependabot.yml`, `renovate.json` absent) — freshness gap (v0.6.0).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41) and
    `pyproject.toml` (`ruff<0.16`); MSRV recipe: `cargo info <crate>@<ver>`.
- **`rb_sys` pinned in THREE places that must move together** (iter 130): `crates/iscc-rb/Gemfile`
    (exact `0.9.123`), `Gemfile.lock`, `tag:` at `release.yml:853` — it bundles
    `rake-compiler-dock (= 1.10.0)`, must match the cross-gem Docker image. 2nd `held:` =
    `minitest ~> 5.0` (6.x needs Ruby ≥3.2 > gemspec floor 3.1). Gemspec has NO dev deps.
- **Full dependency-pin inventory + slice history** → `dep-refresh-survey.md` (re-verified 129).
    Headlines: ci.yml/docs.yml GHA refs CURRENT (setup-uv = EXACT tag `@v9.0.0`); release.yml lags
    and has **no setup-uv step** (issues.md claims otherwise — wrong); `mise.toml` has no `[tools]`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on real tip (don't trust handoff — a review
    PASS with green `mise run check` can still fail CI-only gates like CRAP `--fail-regression`).
    python-test matrix = 3.10+3.14. Re-read target/specs diff (they GROW → boxes flip met→partial).
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + the `rust-core.md` semver box are
    maintained. Verify in code, never read boxes as done/not-done.

## Current State (assessed-at: 2dee913, iter 131)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released, all 12 bindings meet CORE criteria. Partially met:
    Rust-core (semver-enforcing/v1.0.0 HELD **plus** the Unicode conformance divergence), CI/CD (dep
    freshness: only ruff 0.16 left autonomously). Kotlin = met on criteria but has a
    release-blocking `[review]` issue. Every other section MET.
- **CI GREEN on origin/develop tip `03a92f5`** (= iter-130 review PASS commit; HEAD `2dee913` = +3
    UNPUSHED context-only commits — `iterations.jsonl` + `metrics.jsonl`). **41** check-runs, 21
    distinct names, 0 non-success, 0 running. Count is ~2x the job count because PR **#44 "Release
    0.6.0" (develop→main) is OPEN** → every develop commit fires a `push` AND a `pull_request` run.
- **Dependency-refresh: ALL 7 per-ecosystem slices DONE** (`normal` `[human]`, spec
    `ci-cd.md`→Dependency Freshness; no `[audit]` cite = no 8-file valve): Cargo.lock 124, uv.lock
    125, Rust pins 126, GHA refs 127, JVM 128, go.mod 129, Ruby 130; napi+dotnet+pre-commit closed
    verified-current. **Last CID-doable item = ruff 0.16**, then magnus 0.8 / jni 0.22 (source
    rewrites) → after ruff the loop likely goes IDLE pending the 2 `[review]` calls.
- **7 issues: 0 critical, 5 normal, 2 low.** TWO carry HUMAN REVIEW REQUESTED (`[review]`, policy
    calls Titusz owns; CID may only execute the docs-only option once picked): Kotlin consumer floor
    2.3 (128) + Unicode 16/17 divergence (129). Other normal = dep refresh (CID-doable), npm OIDC,
    single-registry re-trigger (human-gated). low (CID skips) = v1.0.0 (HELD), docs logos.
- **Don't re-flag as new work** (all DONE): dep slices 1-7 + c-cpp anchor (124-130), aarch64 wheels
    #49 (123), CRAP baseline (122), trailing-byte fixes (120-121), Go IDv1 #43 (119), WASM SIMD #42
    (118), GIL #39+#41, cargo-deny (113), iai perf gate (107-111), semver gate (93). CID infra
    (audit role, metrics.jsonl, decisions.md, escape valve) = meta, NOT target — ignore.
- **Known non-regression**: the `proc-macro-error2 v2.0.1` future-incompat warning on cargo
    test/bench comes from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys; no upstream fix yet.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: hook args are `--wrap 100 --number` — always
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff /tmp/c.md f`, then
    `cp /tmp/c.md f` to adopt (bare `mdformat` renumbers lists to all-`1.`). It ABORTS the commit on
    nested/escaped backticks in a code span or a wrapped line starting `+`/`-`/`>` — reword those.
- **live advisory DB** — cargo-deny `advisories` can turn a previously-green gate red with no code
    change (see Audit gate).
- **metrics.jsonl counts include gitignored build artifacts** — post-`rake compile`,
    `crates/iscc-rb` jumped 8→12 files / 9→18 `unsafe` purely from `tmp/*/stage/` copies (`vendor/`
    IS excluded). Never read a metrics delta as real code change without `git diff --stat`.
- **Gradle flakes on this bind mount** (incremental-state `Unable to delete file …/build/kotlin/…`)
    — always `./gradlew clean` before believing a Kotlin build failure; check the test XML first.
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
