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
- **iai-callgrind perf GATE — COMPLETE & ENFORCING (issue #3 done, CI-verified iter 110)**:
    `Perf (iai-callgrind)` CI job GREEN (`grep -n "Perf (iai-callgrind)" ci.yml`). Pipeline (ci.yml
    ~281-333): valgrind → binstall `iai-callgrind-runner@0.16.1 --force` → run benches (env
    `IAI_CALLGRIND_ALLOW_ASLR=true`) → `Assert non-zero instruction collection` guard
    (`grep -rEq '^summary: [1-9]' target/iai/`) → ENFORCING `Check perf regression`
    (`python3 scripts/iai_regression.py --check`, no continue-on-error, fails on >10% Ir regression)
    → upload `iai-baseline` artifact `if: always()`. Committed baseline `.iai-baseline.json` (repo
    root, NOT gitignored): `{metric:"Ir", tolerance_pct:10.0, benches:{<16 entries>}}`. Tasks:
    `bench:iai`, `bench:iai:check`, `bench:iai:baseline` (mise.toml ~126-144). Refresh = reviewed
    commit. NEW `[review]` hardening issue open (2 false-green edges in iai_regression.py: zero-Ir
    bench read as improvement; disappeared baseline bench only warns).
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
- **PyO3 — MIGRATION COMPLETE (issue #1 CLOSED iter 105)**: pinned `0.29` (`Cargo.toml`; lockfile
    single 0.29.0, no older). Core has NO PyO3 dep; scope = `crates/iscc-py/`. Load-bearing
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) — explicit b/c PyO3 0.28
    silently flipped that default `true`→`false`. Hop-by-hop history (0.23→0.29) + detail archived.
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
    LAST). Flake fix iter 101 = `--force` on binstall (mechanism archived). `.crap-baseline.json`
    (repo root, NOT gitignored): `{$schema, version:"0.2.2", entries:[...]}`, 97 entries / 10 src
    files. Regen via `mise run crap:baseline` — reviewed commit, NOT auto. `.cargo-crap.toml`:
    threshold 30, `missing="pessimistic"`, excludes 7 binding crates + `packages/**` + `scripts/**`
    \+ `benches/**`. ci-cd.md Phases 1+2+3 all `[x]`. **[review] hardening issue (still open)**:
    Phase 3 is regression-ONLY — a new/renamed fn (no baseline entry) reports `★ N new` & exits 0
    (Codex: new CC=21 fn @ CRAP 462 bypassed). Fix: add `--fail-above 30` (baseline max ~22.3 < 30,
    safe). HUMAN REVIEW REQ before spec change.
- **Semver gate present iter 93**: `Semver (cargo-semver-checks)` job ci.yml:280,
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0; `mise run semver` mise.toml:104). CAUTION: job reports `failure` (2
    expected breaking changes from post-0.4.0 `pub(crate)` narrowing) but **run conclusion stays
    `success`** — NOT a CI failure. rust-core.md "verified when" stays `[ ]` (needs enforcing +
    ≥1.0.0); ci-cd.md:417 `[x]` (informational wording).
- **GIL #39 (MET) + npm #38 (FIXED iter 92) — stable**: GIL-release = `Python::detach` (7 sites);
    npm bundled `files: ["*.node"]`, no `optionalDependencies`, `napi prepublish` removed.
- **Issue count (correct)**: grep `issues.md` for `^##` headers ending in a priority label
    (critical/normal/low) — anchoring to `^##` excludes the legend line, so NO -1 adjustment.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — origin lags; code after the last
    CI-run sha is UNVERIFIED, cross-check the diff. (Usual case: HEAD = +1 log-only commit.)

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
    `Perf (iai-callgrind)` (added iter 107; false-green FIXED iter 108; ENFORCING regression gate
    added iter 109 — apt valgrind → binstall `iai-callgrind-runner@0.16.1 --force` → bench (env
    `IAI_CALLGRIND_ALLOW_ASLR=true`) → `Assert non-zero instruction collection` GUARD →
    `Check perf regression` (`python3 scripts/iai_regression.py --check`, fails >10% Ir) → upload
    `target/iai/` as `iai-baseline` `if: always()`; no continue-on-error). `push:` under `on:` is
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
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (line ~157);
    `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher + InstanceHasher re-exported at
    crate root (lib.rs:24); SumHasher via `streaming::`. SumHasher wrapper in Python (iscc-py
    lib.rs:615) + WASM (iscc-wasm lib.rs:533). #37 closed iter 90.
- `crates/iscc-lib/benches/benchmarks.rs` — 12 benches in criterion_group!
- `crates/iscc-lib/benches/iai_benches.rs` — iai-callgrind 0.16 harness (iter 107 `072e746`), 11
    `bench_*` fns (9 `gen_*_v0` + cdc + minhash) in `library_benchmark_group!(iscc_benches)` (16
    parametrized runtime cases). `[[bench]] name="iai_benches" harness=false`; dep
    `iai-callgrind = "0.16"` (root + iscc-lib dev-dep). RUN IN CI by `Perf` job;
    `mise run bench:iai`. **`[profile.bench]` inherits release `strip=true`** → stripped
    `__iai_callgrind_wrapper` toggle symbols → false green; FIX (iter 108 `6982124`):
    `[profile.bench] strip=false, debug=true` (Cargo.toml:61) + `IAI_CALLGRIND_ALLOW_ASLR=true`.
    `#[library_benchmark]` fns use `//` not `///` (macro `abort!`s on `doc`).
- `scripts/iai_regression.py` (iter 109 `949f63f`, 195 lines, **stdlib-only**) — `--check` parses
    `target/iai/**/*.out` `summary:` lines vs committed `.iai-baseline.json`; fails on >10% Ir
    regression. Intersection-only (new benches warn, not fail). Has 2 known false-green edges (NEW
    `[review]` hardening issue): zero-Ir current bench read as improvement; disappeared baseline
    bench only warns. `--update` regenerates baseline (from `target/iai/` or `--from-dir`).
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
- **idle→active reactivation**: state.md idle but `git diff <hash>..HEAD --stat` shows large
    issues.md/target.md/specs growth → human re-scoped. Do a near-full re-review, not a diff parrot.

## Current State (assessed-at: bb9f02e)

- **IN_PROGRESS — CI GREEN on pushed tip.** v0.4.0 released; hardening toward v1.0.0. Workspace
    version = `0.4.0`.
- **Iter 110 incremental** (diff `6cb9642..HEAD`). Code change: iai-callgrind perf gate **slice 2b**
    (`949f63f`) — committed `.iai-baseline.json` (16 Ir entries) + `scripts/iai_regression.py` +
    enforcing `Check perf regression` CI step + `bench:iai:baseline`/`:check` mise tasks. Else
    `.claude/` context/memory. Issue #3 NOW FUNCTIONALLY COMPLETE & CI-verified (Perf job green with
    regression step passing) — awaiting review-agent deletion.
- **✅ CI GREEN on pushed tip; HEAD +1 (log only, code-clean).** origin/develop = `a5ce73c`, HEAD =
    `bb9f02e` (iter-109 log, `iterations.jsonl` only). Run 27750907410 (sha `a5ce73c`) = **SUCCESS**
    — confirmed via `gh api .../commits/a5ce73c/check-runs`: all 19 jobs green incl.
    `Perf (iai-callgrind)` (Check perf regression step passes vs committed baseline), only `Semver`
    failure (continue-on-error). Sandbox `gh run list` matched the API this time, but STILL prefer
    the check-runs API on the actual tip SHA — it has been stale before.
- **6 issues: 0 critical, 4 normal, 2 low** (count issues.md headers — lines starting with two
    hashes whose title ends in a priority label; the legend line is excluded, so no -1 adjustment).
    +1 normal vs iter 109 (NEW iai_regression.py false-green hardening); #3 still listed but
    functionally done.
- **Open normal gaps (3 actionable; #3 done)**: (1) NEW iai_regression.py false-green hardening
    [review, NO spec change, FULLY CID-ACTIONABLE — the obvious next slice]; (2) CRAP
    `--fail-above   30` hardening [review, HUMAN REVIEW REQ]; (3) supply-chain
    `cargo deny`/`cargo audit` gate [review, HUMAN REVIEW REQ]. cargo-semver-checks gate present —
    informational.
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver informational, enforcing needs v1.0.0 — ONLY
    remaining gap; perf gate now MET), CI/CD (**GREEN**; `--fail-above` + supply-chain + iai
    hardening remain). All 12 bindings + Benchmarks MET.
- **Recently closed/landed (don't re-flag)**: iai perf gate slice 2b (iter 109 `949f63f`, committed
    baseline + enforcing regression check), false-green strip fix (iter 108 `6982124`), Perf CI job
    slice 2a (iter 107), iai harness (iter 107), PyO3 migration #1 (iters 98-105), cargo-crap
    `--force` flake fix (iter 101), CRAP Phase 3 (iter 99), semver gate (iter 93, informational),
    npm #38 (iter 92), GIL #39 (iter 91), SumHasher #37 (iters 88-90).
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; ci-cd.md Phases 1+2+3 boxes `[x]`; **rust-core perf boxes NOW
    `[x]`** (both perf checkboxes + ci-cd.md perf box flipped iter 109); enforcing-semver still
    `[ ]` (flips only at v1.0.0 cut). Re-read on incremental review.

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
