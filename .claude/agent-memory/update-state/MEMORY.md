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
- **iai-callgrind harness + perf-GATE check**: `ls crates/iscc-lib/benches/iai_benches.rs`; CI
    `Perf` job EXISTS + GREEN (`grep -n "Perf (iai-callgrind)" ci.yml`,
    `grep -n "bench:iai"   mise.toml`). Job now collects REAL non-zero counts (iter-108 strip fix +
    zero-collection guard). REGRESSION GATE (slice 2b) still missing: no committed baseline
    (`git ls-files |   grep iai` shows only the harness), no >10% fail condition. Issue #3 open
    until 2b.
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
- **release.yml checks** (stable, rarely changes): inputs `grep "type: boolean" release.yml | wc -l`
    (=8); Swift `grep -i 'swift\|xcframework'`; Kotlin/Android `grep -A 20 "build-kotlin-native:"`;
    provenance `grep -c 'Verify main matches tag'`. XCFramework:
    `test -x scripts/build_xcframework.sh`.
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 — MIGRATION COMPLETE (issue #1 CLOSED iter 105)**: pinned `0.29` (one place in
    `Cargo.toml`; `Cargo.lock` single 0.29.0 entry, no older). Core has NO PyO3 dep; scope =
    `crates/iscc-py/`. Load-bearing: `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697)
    — explicit because PyO3 0.28 silently flipped that default `true`→`false`. Clearance confirmed
    only by lockfile proxy (`cargo deny`/`cargo audit` absent → own [review] issue below).
    Hop-by-hop history (0.23→0.29) archived.
- **Supply-chain audit gate ABSENT ([review] issue iter 105)**: `notes/07` mandates `cargo deny` CI
    gate + root `deny.toml` + `cargo audit`; NONE exist. HUMAN REVIEW REQ (req in notes/07, not
    specs).
- **Coverage + CRAP gate ALL 3 PHASES present & GREEN; install flake FIXED iter 101**: ONE job
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` (ci.yml:294), no `needs:`, NO
    `continue-on-error`, job-level `security-events: write`. Pipeline: llvm-cov → cargo-binstall →
    `Install cargo-crap` (ci.yml:314 `cargo binstall -y --force cargo-crap@0.2.2`) →
    `cargo llvm-cov -p iscc-lib --lcov` → upload-artifact `lcov` → **Phase 2 report-only**
    (`--format github` + `--format sarif` → `codeql-action/upload-sarif@v3`) → **Phase 3 enforcing**
    (ci.yml:335 `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`, runs
    LAST). Flake fix iter 101 `628c5d9` = `--force` on binstall (rust-cache restored metadata sans
    binary; mechanism archived). `.crap-baseline.json` (repo root, NOT gitignored — only
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
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — origin lags; code commits after the
    last CI-run sha are UNVERIFIED, cross-check the diff against it.

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
- `.github/workflows/ci.yml` — **18 YAML job entries → 19 actual jobs** (`python-test` matrix
    expands 3.10 + 3.14): functional jobs + non-blocking `Semver` (iter 93) +
    `Coverage + CRAP   (cargo llvm-cov + cargo crap)` (Phases 1-3, iters 94-97) +
    `Perf (iai-callgrind)` (added iter 107; false-green FIXED iter 108 — apt valgrind → binstall
    `iai-callgrind-runner@0.16.1` → bench (env `IAI_CALLGRIND_ALLOW_ASLR=true`) →
    `Assert non-zero   instruction collection` GUARD → upload `target/iai/` as `iai-baseline`; no
    continue-on-error). `push:` under `on:` is NOT a job; a bare `^  [a-z].*:$` grep over-counts —
    read the job names.
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
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (line ~157);
    `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher + InstanceHasher re-exported at
    crate root (lib.rs:24); SumHasher via `streaming::`. SumHasher wrapper in Python (iscc-py
    lib.rs:615) + WASM (iscc-wasm lib.rs:533). #37 closed iter 90.
- `crates/iscc-lib/benches/benchmarks.rs` — 12 benches in criterion_group!
- `crates/iscc-lib/benches/iai_benches.rs` — iai-callgrind 0.16 harness (iter 107 `072e746`), 11
    `bench_*` fns (9 `gen_*_v0` + cdc + minhash) in `library_benchmark_group!(iscc_benches)` (16
    parametrized runtime cases). `[[bench]] name="iai_benches" harness=false`; dep
    `iai-callgrind = "0.16"` (root Cargo.toml:43 + crates/iscc-lib/Cargo.toml:34 dev-dep). RUN IN CI
    by `Perf` job under valgrind; `mise run bench:iai` (mise.toml:126). **CORRECTION (iter 109,
    review-confirmed empirically): `[profile.bench]` DOES inherit release `strip=true`** → stripped
    the `__iai_callgrind_wrapper` toggle symbols → every bench `summary: 0` while exiting 0 (FALSE
    GREEN). FIX (iter 108 `6982124`): `[profile.bench] strip=false, debug=true` (Cargo.toml:58) +
    `IAI_CALLGRIND_ALLOW_ASLR=true` (mise.toml + ci.yml; skips kernel-blocked `setarch -R`, ASLR
    doesn't affect Ir) + ci.yml guard step `grep -rEq '^summary: [1-9]' target/iai/`.
    `#[library_   benchmark]` fns use `//` not `///` (macro `abort!`s on `doc`). Issue #3 REGRESSION
    GATE (committed baseline + >10% fail) NOT done (slice 2b) — valgrind absent locally.
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

## Current State (assessed-at: 6cb9642)

- **IN_PROGRESS — CI GREEN on pushed tip.** v0.4.0 released; hardening toward v1.0.0. Workspace
    version = `0.4.0`.
- **Iter 109 incremental** (diff `77a5fb3..HEAD`). Code change: iai-callgrind FALSE-GREEN fix
    (`6982124`) — see iai_benches landmark. Else `.claude/` context/memory. Issue #3 slice 2a now
    GENUINELY done (real non-zero counts, guard passes); slice 2b (committed baseline + >10% gate)
    PENDING → #3 stays open.
- **✅ CI GREEN on pushed tip; HEAD +1 (log only, code-clean).** origin/develop = `1463edb`, HEAD =
    `6cb9642` (iter-108 log, `iterations.jsonl` only). Run 27746693860 (sha `1463edb`) = **SUCCESS**
    — confirmed via `gh api .../commits/1463edb/check-runs`: all 19 jobs green incl.
    `Perf (iai-callgrind)` (guard passed = real counts), only `Semver` failure (continue-on-error).
    Sandbox `gh run list` matched the API this time, but STILL prefer the check-runs API on the
    actual tip SHA — it has been stale before.
- **5 issues: 0 critical, 3 normal, 2 low** (count issues.md headers — lines starting with two
    hashes whose title ends in a priority label; the legend line is excluded, so no -1 adjustment).
    Unchanged from iter 107-108.
- **Open normal gaps (3)**: CRAP `--fail-above 30` hardening [review, HUMAN REVIEW REQ],
    supply-chain `cargo deny`/`cargo audit` gate [review, HUMAN REVIEW REQ], iai-callgrind perf
    REGRESSION GATE (slice 2b: inspect uploaded `iai-baseline` artifact for layout → commit baseline
    → add >10% fail → `bench:iai:baseline` refresh task; CI-verify only, no local valgrind).
    cargo-semver-checks gate present — informational. Obvious next slice = perf gate 2b.
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver informational; perf regression gate pending;
    enforcing-semver needs v1.0.0), CI/CD (**GREEN**; `--fail-above` + supply-chain + perf-2b
    remain). Python MET, Node.js MET, WASM MET. All 12 bindings met.
- **Recently closed/landed (don't re-flag)**: iai-callgrind false-green strip fix (iter 108
    `6982124`, Perf now real counts), Perf CI job slice 2a (iter 107), iai harness (iter 107), PyO3
    migration #1 (iters 98-105), cargo-crap `--force` flake fix (iter 101), CRAP Phase 3 (iter 99),
    semver gate (iter 93, informational), npm #38 (iter 92), GIL #39 (iter 91), SumHasher #37 (iters
    88-90).
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; ci-cd.md Phases 1+2+3 boxes all `[x]`; rust-core perf criterion
    - enforcing-semver still `[ ]` (perf checkbox flips only when slice 2b lands). Re-read on
        incremental review.

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
- **pytest-benchmark naming**: functions use `test_bench_*` prefix (not bare `bench_*`)
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 `armv7`→`arm`, dual
    Package.swift manifests, Kotlin JAR classifier pick) archived → `MEMORY-archive.md`.
