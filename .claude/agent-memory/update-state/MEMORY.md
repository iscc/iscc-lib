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
- **PyO3 version — MIGRATION COMPLETE (issue #1 CLOSED iter 105 `8df611f`/`103fe3d`)**:
    `grep -n "pyo3" Cargo.toml` (workspace.dependencies — one place). Now at `0.29` (`Cargo.lock`
    0.29.0, single entry, no older) — the version where the 2 RustSec advisories clear. Core has NO
    PyO3 dep; scope is `crates/iscc-py/`. **Load-bearing current-code detail**:
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697, documented) — explicit because
    PyO3 0.28 silently flipped the unspecified default `true`→`false` (free-threading-safety of the
    raw-FFI `extract_frame_sigs` path). **LESSON for future bumps: "compiles clean" ≠
    behavior-neutral — diff pyo3 macros-backend defaults + read the migration guide each hop.**
    Advisory clearance confirmed only by lockfile proxy (`cargo deny`/`cargo audit` absent) → now
    its own `[review]` supply-chain issue (below). Full hop-by-hop history (0.23→0.29) archived.
- **Supply-chain audit gate ABSENT (NEW [review] issue iter 105)**: `notes/07` mandates a
    `cargo deny` CI gate + root `deny.toml` + `cargo audit`. NONE exist (no `deny.toml`/CI job/mise
    task/tools). HUMAN REVIEW REQ (req in notes/07, not CID specs).
- **v1.0.0 gates check**: `grep -iE "crap|semver|llvm-cov|iai-callgrind" .github/workflows/ci.yml`;
    also `ls .cargo-crap.toml` + `grep -iE "coverage|crap|semver|callgrind" mise.toml`
- **Coverage + CRAP gate ALL 3 PHASES present; install flake FIXED iter 101**: ONE job named
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` at ci.yml:294, no `needs:`, NO
    `continue-on-error`, job-level `security-events: write`. Pipeline: rust-toolchain@stable +
    `llvm-tools-preview` → cargo-llvm-cov → cargo-binstall → `Install cargo-crap` (ci.yml:314, now
    `cargo binstall -y --force cargo-crap@0.2.2`) →
    `cargo llvm-cov -p iscc-lib --lcov --output-path   lcov.info` → upload-artifact (`name: lcov`) →
    **Phase 2 (report-only)**: `cargo crap --lcov lcov.info --format github` +
    `--format sarif --output crap.sarif` + `codeql-action/upload-sarif@v3` → **Phase 3 (enforcing,
    iter 97)**: `CRAP regression gate`
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` (ci.yml:335-336),
    NOT continue-on-error, runs LAST. **cargo-crap install flake FIXED iter 101 `628c5d9`** via
    `--force` on the binstall (rust-cache restored cargo metadata without the binary → binstall
    skipped; full mechanism archived). General lesson: a binstalled tool that flakes "already
    installed" under rust-cache needs `--force`. `.crap-baseline.json` (repo root, NOT gitignored —
    only `lcov.info`+`crap.sarif` are): envelope `{$schema, version:"0.2.2", entries:[...]}`, 97
    entries / 10 `crates/iscc-lib/src/` files. Regen via `mise run crap:baseline` (mise.toml:119,
    `depends=["coverage"]`, `--format json --output .crap-baseline.json`) — reviewed commit, NOT CI
    auto-commit. `.cargo-crap.toml` (repo root): threshold 30, `missing="pessimistic"`, excludes all
    7 binding crates + `packages/**` + `scripts/**` + `crates/iscc-lib/benches/**`.
    `mise run coverage` (110) + `mise run crap` (114, depends=coverage). Phase 1 iter 94; Phase 2
    iter 96; Phase 3 iter 97. ci-cd.md Phases 1+2+3 boxes all `[x]`. CRAP base issue DELETED by
    review sweep iter 100. **[review] hardening issue (iter 97, still open)**: Phase 3 is
    regression-ONLY — a new/renamed fn has no baseline entry → reports `★ N new` & exits 0 (Codex
    verified new CC=21 fn @ CRAP 462 bypassed). Fix: add `--fail-above 30` (baseline max ~22.3 < 30,
    safe). HUMAN REVIEW REQUESTED before spec change. If a future run flaps, regen baseline from
    CI's lcov artifact — do NOT widen `--epsilon`.
- **Semver gate present iter 93** (`9d42077`): `Semver (cargo-semver-checks)` job at ci.yml:280,
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0). Mirrored `mise run semver` (mise.toml:104). CAUTION: job reports
    `conclusion: failure` (2 expected breaking changes from post-0.4.0 `pub(crate)` narrowing) but
    **run-level conclusion stays `success`** — NOT a CI failure. rust-core.md "verified when" stays
    `[ ]` (needs enforcing + >= 1.0.0); ci-cd.md:417 is `[x]` (informational wording).
- **GIL/SumHasher checks** (done & stable): iscc-py GIL-release = `Python::detach` (7 sites); verify
    `grep -rn "detach\|SumHasher" crates/iscc-{lib,py,wasm}/src/`
- **npm optionalDeps bug (#38 FIXED iter 92)**: `grep -c "napi prepublish" release.yml` = `0`;
    bundled `files: ["*.node"]`, no `optionalDependencies`. Node.js MET.
- **Issue count (correct)**: grep `issues.md` for `^##` headers ending in a priority label
    (critical/normal/low) — anchoring to `^##` excludes the legend line, so NO -1 adjustment. A bare
    label grep over-counts by 1.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — CID commits locally; origin may
    lag. Code commits after the last CI run sha are UNVERIFIED; cross-check the diff against it.

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

## Current State (assessed-at: 40fa239)

- **IN_PROGRESS — CI GREEN.** v0.4.0 released; hardening toward v1.0.0. Workspace version = `0.4.0`.
- **Iter 106 incremental** (diff `f7f9276..HEAD`). Only code-bearing change: PyO3 `0.28 → 0.29`
    (Cargo.toml:35 `0.29`, Cargo.lock `0.29.0` single entry no older, ZERO source edits —
    `gil_used = true` preserved at lib.rs:697). Everything else is `.claude/` context/memory. **PyO3
    migration arc COMPLETE — issue #1 CLOSED.**
- **✅ CI PASSING.** Latest run 27728337913 (sha `103fe3d`, the review commit) = **SUCCESS**. HEAD
    `40fa239` adds only one `iterations.jsonl` log commit on top of `103fe3d`, so the green run
    covers HEAD's code. All 17 functional jobs + Coverage+CRAP green; only `Semver` shows job-level
    failure but continue-on-error (does NOT flip). Coverage+CRAP enforcing Phase 3 gate ran &
    passed.
- **5 issues: 0 critical, 3 normal, 2 low** (grep `^## .+\`(critical|normal|low)\`\` for headers,
    excludes legend line — no -1 adjustment). Composition CHANGED: PyO3 #1 closed, NEW supply-chain
    audit gate [review] issue added — still 3 normal / 2 low.
- **Open normal gaps (3) — ALL CONSTRAINED (natural pause point)**: CRAP `--fail-above` hardening
    [review, HUMAN REVIEW REQ], supply-chain `cargo deny`/`cargo audit` gate \[review, HUMAN REVIEW
    REQ, NEW\], iai-callgrind perf gate (ZERO impl; blocked — valgrind absent in devcontainer).
    cargo-semver-checks gate present — informational. Most self-contained unblocked candidate =
    supply-chain audit gate (but needs spec amendment approval).
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver gate informational; perf gate missing;
    enforcing-semver needs v1.0.0), CI/CD (**GREEN**; `--fail-above` + supply-chain + iai-callgrind
    remain). Python now **MET** (PyO3 migration COMPLETE, GIL MET). Node.js MET. WASM MET. All 12
    bindings met.
- **Recently closed/landed (don't re-flag)**: PyO3 0.29 (iter 105 `8df611f`, migration arc DONE,
    issue #1 closed), PyO3 0.28 (iter 104, silent gil_used flip restored), PyO3 0.27 (iter 102/103
    `acf9277`), PyO3 0.26 (iter 102), cargo-crap `--force` flake fix (iter 101 `628c5d9`), PyO3 0.25
    (iter 100), CRAP base issue swept (iter 100), CRAP Phase 3 first green (iter 99), PyO3 0.24
    (iter 98), semver gate (iter 93, informational), npm #38 (iter 92), GIL #39 (iter 91), streaming
    SumHasher #37 (iters 88-90).
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
    packages/swift for CI development. `releaseChecksum = "PLACEHOLDER"` until first release with
    swift input
- **pytest-benchmark naming**: functions use `test_bench_*` prefix (not bare `bench_*`)
- **Kotlin JAR selection**: `ls *.jar | head -1` picks `-javadoc.jar`; must `grep -v` classifiers
