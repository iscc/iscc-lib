# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Exploration Shortcuts

- **Per-crate READMEs**: `ls crates/*/README.md packages/*/README.md 2>&1`
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --branch "$(git branch --show-current)" --limit 3 --json status,conclusion,url,databaseId`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **Tier 1 pub fns in Rust core**:
    `grep -r "pub fn gen_\|pub const META\|pub const IO\|pub const TEXT" crates/iscc-lib/src/`
- **Doc nav check**: `grep -A 15 "Reference" zensical.toml`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Benchmark functions**:
    `grep -n "^fn bench_\|criterion_group" crates/iscc-lib/benches/benchmarks.rs`
- **iai-callgrind harness + perf-GATE check**: `ls crates/iscc-lib/benches/iai_benches.rs`; gate
    present? `grep -in "iai\|callgrind\|valgrind\|perf" .github/workflows/ci.yml mise.toml` (none
    yet).
- **Authoritative CI status (sandbox `gh run list` is STALE — returns old ancestor SHAs)**:
    `gh api repos/iscc/iscc-lib/commits/<tip-sha>/check-runs --jq '.check_runs[]|{name,conclusion}'`
    against the ACTUAL origin/develop tip SHA. `gh run view <id> --json conclusion,headSha` also
    works.
- **pytest-benchmark functions**: `grep -c "def test_bench_" tests/test_benchmarks.py`
- **gen_llms_full.py page count**: Python ast.literal_eval on ORDERED_PAGES list (now 22 entries)
- **UniFFI export count**: Use Grep for `#\[uniffi::export\]` in `crates/iscc-uniffi/src/lib.rs`
- **state.md Write workaround**: Write tool = permission error. Use heredoc:
    `cat > .claude/context/state.md << 'STATEEOF' ... STATEEOF`
- **CLAUDE.md files**: `ls packages/*/CLAUDE.md crates/*/CLAUDE.md 2>&1`
- **Howto guides**: `ls docs/howto/*.md | sort`
- **Version sync targets**: `uv run scripts/version_sync.py --check 2>&1 | grep "^OK:" | wc -l`
- **Release workflow inputs**: `grep "type: boolean" .github/workflows/release.yml | wc -l`
- **XCFramework verify**:
    `test -x scripts/build_xcframework.sh && bash -n scripts/build_xcframework.sh`
- **Swift release workflow check**: `grep -i 'swift\|xcframework' .github/workflows/release.yml`
- **Kotlin/Android targets**:
    `grep -A 20 "build-kotlin-native:\|android" .github/workflows/release.yml`
- **Provenance guard check**: `grep -c 'Verify main matches tag' .github/workflows/release.yml`
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 — MIGRATION COMPLETE (issue #1 CLOSED iter 105)**: pinned `0.29`
    (`grep -n "pyo3"   Cargo.toml`, one place; `Cargo.lock` 0.29.0 single entry, no older — version
    where the 2 RustSec advisories clear). Core has NO PyO3 dep; scope = `crates/iscc-py/`.
    Load-bearing: `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) — explicit because
    PyO3 0.28 silently flipped that default `true`→`false`. LESSON for future bumps: "compiles
    clean" ≠ behavior-neutral — diff macros-backend defaults + read the migration guide each hop.
    Clearance confirmed only by lockfile proxy (`cargo deny`/`cargo audit` absent → own [review]
    issue below). Full hop-by-hop history (0.23→0.29) archived.
- **Supply-chain audit gate ABSENT (NEW [review] issue iter 105)**: `notes/07` mandates a
    `cargo deny` CI gate + root `deny.toml` + `cargo audit`. NONE exist (no `deny.toml`/CI job/mise
    task/tools). HUMAN REVIEW REQ (req in notes/07, not CID specs).
- **Coverage + CRAP gate ALL 3 PHASES present & GREEN; install flake FIXED iter 101**: ONE job
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` (ci.yml:294), no `needs:`, NO
    `continue-on-error`, job-level `security-events: write`. Pipeline: llvm-cov → cargo-binstall →
    `Install cargo-crap` (ci.yml:314 `cargo binstall -y --force cargo-crap@0.2.2`) →
    `cargo llvm-cov -p iscc-lib --lcov` → upload-artifact `lcov` → **Phase 2 report-only**
    (`--format github` + `--format sarif` → `codeql-action/upload-sarif@v3`) → **Phase 3 enforcing**
    (ci.yml:335 `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`, runs
    LAST). Flake fix iter 101 `628c5d9`: rust-cache restored cargo metadata sans binary → binstall
    skipped; `--force` fixes (lesson: binstalled tool flaking "already installed" under rust-cache
    needs `--force`). `.crap-baseline.json` (repo root, NOT gitignored — only
    `lcov.info`+`crap.sarif` are): `{$schema, version:"0.2.2", entries:[...]}`, 97 entries / 10 src
    files. Regen via `mise run crap:baseline` (mise.toml:119, `depends=["coverage"]`) — reviewed
    commit, NOT auto. `.cargo-crap.toml`: threshold 30, `missing="pessimistic"`, excludes 7 binding
    crates + `packages/**` + `scripts/**` + `benches/**`. ci-cd.md Phases 1+2+3 all `[x]`.
    **[review] hardening issue (still open)**: Phase 3 is regression-ONLY — a new/renamed fn (no
    baseline entry) reports `★ N new` & exits 0 (Codex: new CC=21 fn @ CRAP 462 bypassed). Fix: add
    `--fail-above 30` (baseline max ~22.3 < 30, safe). HUMAN REVIEW REQ before spec change. If a
    future run flaps, regen baseline from CI's lcov artifact — do NOT widen `--epsilon`.
- **Semver gate present iter 93**: `Semver (cargo-semver-checks)` job ci.yml:280,
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0; `mise run semver` mise.toml:104). CAUTION: job reports `failure` (2
    expected breaking changes from post-0.4.0 `pub(crate)` narrowing) but **run conclusion stays
    `success`** — NOT a CI failure. rust-core.md "verified when" stays `[ ]` (needs enforcing +
    ≥1.0.0); ci-cd.md:417 `[x]` (informational wording).
- **GIL #39 (MET) + npm #38 (FIXED iter 92) — stable**: GIL-release = `Python::detach` (7 sites,
    `grep -rn "detach\|SumHasher" crates/iscc-{lib,py,wasm}/src/`); npm bundled `files: ["*.node"]`,
    no `optionalDependencies`, `grep -c "napi prepublish" release.yml` = `0`.
- **Issue count (correct)**: grep `issues.md` for `^##` headers ending in a priority label
    (critical/normal/low) — anchoring to `^##` excludes the legend line, so NO -1 adjustment.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — CID commits locally; origin lags.
    Code commits after the last CI-run sha are UNVERIFIED; cross-check the diff against it.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols)
- `.claude/context/specs/` — per-binding spec files
- `packages/go/` — pure Go module (no WASM, no binary artifacts)
- `packages/swift/` — SPM package with UniFFI-generated bindings (2400-line iscc_uniffi.swift)
- `Package.swift` — **root manifest** — Ferrostar toggle (`useLocalFramework`), `.binaryTarget` with
    `releaseTag`/`releaseChecksum`, two targets (iscc_uniffiFFI binary + IsccLib)
- `scripts/build_xcframework.sh` — builds XCF for 5 Apple targets, lipo fat binaries, ditto zip
- `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, UniFFI-generated (3214-line iscc_uniffi.kt), JNA
    5.16.0; conformance tests (9 methods, 50 vectors); docs + release workflow complete
- `.github/workflows/ci.yml` — **17 YAML job entries → 18 actual jobs** (`python-test` matrix
    expands 3.10 + 3.14): 16 functional jobs (incl. root Package.swift dump-package smoke test) +
    the non-blocking `Semver` job (iter 93) + the `Coverage + CRAP (cargo llvm-cov + cargo crap)`
    job (Phase 1 iter 94, Phase 2 iter 96, Phase 3 enforcing step iter 97). `push:` under `on:` is
    NOT a job; a bare `^  [a-z].*:$` grep over-counts — read the job names.
- `.github/workflows/release.yml` — **8 registry input toggles** (`type: boolean`): crates-io, pypi,
    npm, maven, ffi, rubygems, nuget, maven-kotlin. Swift XCFramework is NOT a toggle — it builds in
    `prepare-release` (line ~55). **provenance guard** on build-xcframework. After #38 fix (iter 92)
    the `publish-npm-lib` job has NO `napi prepublish` step.
- `crates/iscc-uniffi/` — UniFFI scaffolding: 32 exports, 21 tests; `publish = false`
- `docs/howto/` — **11 files**: rust, python, nodejs, wasm, go, java, c-cpp, ruby, dotnet, swift,
    kotlin
- `docs/benchmarks.md` — full speedup comparison (1.3x-158x), Criterion native results, methodology
- `scripts/gen_llms_full.py` — **22 entries** in ORDERED_PAGES (includes benchmarks.md)
- `scripts/version_sync.py` — **16 sync targets** (releaseTag added for Package.swift)
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (core struct
    added iter 88, line ~157); `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher +
    InstanceHasher re-exported at crate root (lib.rs:24); SumHasher reachable via `streaming::`.
    SumHasher streaming wrapper now in all 3 consumers: Python (iscc-py lib.rs:615), WASM (iscc-wasm
    lib.rs:533). #37 fully closed iter 90.
- `crates/iscc-lib/benches/benchmarks.rs` — 12 benches in criterion_group!
- `crates/iscc-lib/benches/iai_benches.rs` — iai-callgrind 0.16 harness (landed iter 107 `072e746`),
    11 `bench_*` fns (9 `gen_*_v0` + cdc + minhash) in `library_benchmark_group!(iscc_benches)`.
    `[[bench]] name="iai_benches" harness=false`; dep `iai-callgrind = "0.16"` (root Cargo.toml:43 +
    crates/iscc-lib/Cargo.toml:34 dev-dep). COMPILE-ONLY: NO CI `Perf` job/baseline/`mise` task yet
    (needs valgrind, absent in devcontainer). `#[library_benchmark]` fns use `//` not `///` (macro
    `abort!`s on `doc` attr). Issue #3 perf GATE still open (plumbing not done).
- `tests/test_benchmarks.py` — 18 pytest-benchmark functions (9 gen\_\*\_v0 x 2 implementations)
- **CLAUDE.md files & per-crate READMEs**: 12 each (all crates + all packages)

## Recurring Patterns

- **Incremental review**: compare assessed-at hash vs HEAD --stat first, then re-verify only
    affected sections. Carry forward unchanged sections. (python-test matrix = 3.10 + 3.14; count
    job definitions, not run records.)
- **Verify claims independently**: always grep rather than trusting handoff; verify CI independently
    (a review PASS = LOCAL checks only, CI can still flake — see iter-100 CRAP break); re-read
    target.md diff on incremental review; verify "partially met" claims rather than parroting.
- **Issues diff**: check issues.md for NEW entries each cycle (human AND `[review]`-sourced). Watch
    `[review]` + `HUMAN REVIEW REQUESTED` flags and any critical ones that reshuffle priorities.

## Current State (assessed-at: 4748669)

- **IN_PROGRESS — CI GREEN on pushed tip.** v0.4.0 released; hardening toward v1.0.0. Workspace
    version = `0.4.0`.
- **Iter 107 incremental** (diff `40fa239..HEAD`). Only code-bearing change: iai-callgrind bench
    HARNESS landed (`072e746`) — `crates/iscc-lib/benches/iai_benches.rs` (new), dep
    `iai-callgrind = "0.16"`, second `[[bench]] harness=false`. COMPILE-ONLY (no Perf CI
    job/baseline/ mise task — needs valgrind). Everything else `.claude/` context/memory. Issue #3
    perf gate PARTIALLY progressed (harness done, gate plumbing NOT).
- **✅ CI GREEN on pushed tip, but HEAD is 7 commits ahead (UNPUSHED).** origin/develop = `103fe3d`,
    HEAD = `4748669`. Run 27728337913 (sha `103fe3d`) = **SUCCESS** — confirmed via
    `gh api repos/iscc/iscc-lib/commits/103fe3d/check-runs` (all 18 jobs green except `Semver`
    continue-on-error). The iai harness commit `072e746` is UNPUSHED → locally-verified (handoff:
    build/clippy/fmt/test/`mise run check` all pass) but NOT CI-verified; `Bench (compile check)`
    will exercise it on push. **GOTCHA: sandbox `gh run list` returns a STALE snapshot (old ancestor
    SHAs like 6ff1f896 "Fix stale 0.3.1 versions", databaseIds LOWER than the real latest). Use
    `gh api .../commits/<sha>/check-runs` against the actual tip SHA instead — it is
    authoritative.**
- **5 issues: 0 critical, 3 normal, 2 low** (grep `^## .+\`(critical|normal|low)\`\` for headers,
    excludes legend line — no -1 adjustment). Unchanged from iter 106.
- **Open normal gaps (3) — ALL CONSTRAINED (natural pause point)**: CRAP `--fail-above` hardening
    [review, HUMAN REVIEW REQ], supply-chain `cargo deny`/`cargo audit` gate \[review, HUMAN REVIEW
    REQ\], iai-callgrind perf GATE (harness done iter 107; gate=CI Perf job+baseline+mise still
    missing; blocked — valgrind absent in devcontainer). cargo-semver-checks gate present —
    informational. Obvious next slice = finish iai perf gate (harness exists), though it needs the
    CI valgrind runner.
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver gate informational; perf gate missing;
    enforcing-semver needs v1.0.0), CI/CD (**GREEN**; `--fail-above` + supply-chain + iai-callgrind
    remain). Python now **MET** (PyO3 migration COMPLETE, GIL MET). Node.js MET. WASM MET. All 12
    bindings met.
- **Recently closed/landed (don't re-flag)**: iai-callgrind HARNESS (iter 107 `072e746`,
    compile-only slice of #3 — perf GATE still open), PyO3 migration 0.24→0.29 (iters 98-105, arc
    DONE, #1 closed, `gil_used=true` restored iter 104), cargo-crap `--force` flake fix (iter 101
    `628c5d9`), CRAP Phase 3 first green (iter 99), semver gate (iter 93, informational), npm #38
    (iter 92), GIL #39 (iter 91), streaming SumHasher #37 (iters 88-90).
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; ci-cd.md Phases 1+2+3 boxes all `[x]`; rust-core perf criterion
    - enforcing-semver still `[ ]`. Re-read on incremental review.

## Pattern: idle→active reactivation

- state.md idle but `git diff <hash>..HEAD --stat` shows large issues.md/target.md/specs growth →
    human re-scoped. Do a near-full re-review of affected sections, not a parrot of the diff.

## Gotchas

- **state.md Write**: Write tool = permission error. Only reliable method:
    `cat > file << 'EOF' ... EOF` via Bash tool
- **mdformat pre-commit can abort the commit** ("Could not format / renders to different HTML"):
    triggered by nested/escaped backticks inside a code span. Keep literal regex backticks OUT of
    code spans in MEMORY.md (describe in prose); test with `uv run mdformat /tmp/copy.md` first.
- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- **csbindgen**: `crates/iscc-ffi/build.rs` runs csbindgen on every `cargo build`
- **UniFFI proc macro approach**: no uniffi.toml or build.rs needed
- **Kotlin UniFFI bindings**: Uses JNA (not JNI); needs BOTH `java.library.path` AND
    `jna.library.path` at runtime
- **Kotlin release workflow**: Uses `useInMemoryPgpKeys` (env vars) instead of Java's GPG keyring;
    Central Portal upload via curl REST API (no Gradle plugin)
- **JNA ARM32 canonicalization**: JNA 5.16.0 `Platform.getNativeLibraryResourcePrefix()` maps
    `armv7` → `arm`, so resource dir must be `android-arm/` not `android-armv7/`
- **Root Package.swift**: Two manifests coexist — root for distribution (binaryTarget),
    packages/swift for CI dev. `releaseChecksum = "PLACEHOLDER"` until first swift-input release.
- **pytest-benchmark naming**: functions use `test_bench_*` prefix (not bare `bench_*`)
- **Kotlin JAR selection**: `ls *.jar | head -1` picks `-javadoc.jar`; must `grep -v` classifiers
