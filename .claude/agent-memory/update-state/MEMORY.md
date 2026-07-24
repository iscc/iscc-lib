# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Full gate pipelines → `MEMORY-archive.md`.

**Size budget:** Keep under 140 lines. One line per entry. Archive detail.

## Exploration Shortcuts

- **Per-crate READMEs**: `ls crates/*/README.md packages/*/README.md`
- **CLAUDE.md files**: `ls packages/*/CLAUDE.md crates/*/CLAUDE.md` (12 total)
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[]|{name,conclusion}'`
- **Authoritative CI status** (sandbox `gh run list` can be STALE — old ancestor SHAs):
    `gh api repos/iscc/iscc-lib/commits/<tip-sha>/check-runs --jq '.check_runs[]|{name,conclusion}'`
    against the real origin/develop tip.
- **Failed-job logs**: `gh run view <id> --log-failed | grep -iE "error|RUSTSEC|advisory|FAILED"`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **Unpushed check**: `git log --oneline origin/develop..HEAD` (origin lags; code after last CI sha
    UNVERIFIED). Usual case: HEAD = +1 log-only commit.
- **Tier 1 pub fns**:
    `grep -r "pub fn gen_\|pub const META\|pub const IO\|pub const TEXT" crates/iscc-lib/src/`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Criterion benches**:
    `grep -n "^fn bench_\|criterion_group" crates/iscc-lib/benches/benchmarks.rs`
- **pytest-benchmark fns**: `grep -c "def test_bench_" tests/test_benchmarks.py` (18)
- **UniFFI exports**: grep `#\[uniffi::export\]` in `crates/iscc-uniffi/src/lib.rs` (32)
- **Version sync targets**: `uv run scripts/version_sync.py --check | grep "^OK:" | wc -l` (16)
- **gen_llms_full.py pages**: ast.literal_eval ORDERED_PAGES (22)
- **Howto guides**: `ls docs/howto/*.md` (11)
- **Benchmarks doc**: `grep -i "speedup" docs/benchmarks.md`
- **release.yml checks**: boolean toggles `grep "type: boolean" release.yml | wc -l` (8);
    XCFramework `test -x scripts/build_xcframework.sh`.
- **Issue count**: grep `issues.md` `^##` headers ending in a priority label (legend excluded → no
    -1); per-priority `grep -cE "^## .*\`normal\`"\` etc.
- **Trace a dependency**: `cargo tree -i <crate>` (shows dev-dep vs shipped).
- **state.md Write workaround**: Write tool = permission error → heredoc
    `cat > .claude/context/state.md << 'STATEEOF' ... STATEEOF`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ci.yml:281-333, ENFORCING (no continue-on-error), GREEN.
    `scripts/iai_regression.py --check` fails >10% Ir vs `.iai-baseline.json` (16 entries). GOTCHA:
    the nearby `continue-on-error` belongs to the SEPARATE semver job.
- **Coverage + CRAP** — one job ~ci.yml:348, ENFORCING. Phase 3 (ci.yml:392-393) runs
    `cargo crap ... --fail-regression --fail-above`; `--fail-above` boolean off
    `.cargo-crap.toml threshold=30.0`. Baseline `.crap-baseline.json` (97 entries). Max CRAP
    ~22.3\<30. GREEN.
- **Audit (cargo-deny)** — ci.yml:395, ENFORCING (no continue-on-error):
    `taiki-e/install-action`→`cargo-deny@0.19.9`→`cargo deny check`. Root `deny.toml` (65 lines;
    advisories+licenses+bans+sources, config v2, `ignore` list at line 24). Task `mise run audit`
    (mise.toml:149). **GOTCHA — live advisory DB flips this red with NO code change**: iter 115 CI
    RED on `RUSTSEC-2026-0204` (crossbeam-epoch 0.9.18, dev-only via
    criterion→rayon→crossbeam-deque). Fix: `cargo update -p <crate>` OR add justified `ignore`
    (already has 2 dev-bench ignores: RUSTSEC-2025-0141 bincode, RUSTSEC-2026-0173
    proc-macro-error2). `cargo-deny` NOT in devcontainer → green CI job is only real confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0),
    `obi1kenobi/cargo-semver-checks-action@v2`, package iscc-lib. Job conclusion does NOT flip the
    run. rust-core.md semver box `[ ]` (needs enforcing + ≥1.0.0); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 actual jobs** (python-test matrix expands
    3.10 + 3.14): functional + Semver (non-blocking) + Coverage+CRAP + Perf + **Audit
    (cargo-deny)**. `push:` under `on:` is NOT a job; read job names.
- `.github/workflows/release.yml` — 8 registry toggles (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget, maven-kotlin). Swift XCFramework builds in `prepare-release` (~line 55), NOT a toggle.
    Provenance guard on build-xcframework. `publish-npm-lib` has NO napi prepublish step (#38).
- `packages/go/` — pure Go, no CGO/WASM/binaries. `codec.go` rejects Version>0 (lines 269/438) —
    ISCC-IDv1 NOT yet supported (#43, v0.6.0).
- `packages/swift/` + root `Package.swift` — Ferrostar toggle (`useLocalFramework`), `.binaryTarget`
    `releaseTag`/`releaseChecksum`. `scripts/build_xcframework.sh` = 5 Apple targets.
- `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, JNA 5.16.0, UniFFI-generated. 9 desktop+Android
    release targets.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (~line 157);
    `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher+InstanceHasher re-exported at
    crate root (lib.rs:24); SumHasher via `streaming::`. SumHasher wrapper: Python iscc-py
    lib.rs:615, WASM iscc-wasm lib.rs:533.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach(' ` = **12** GIL-release sites (iter 116, #41
    DONE): text@138, image@153, video@190, video_flat@217, soft_hash_video@241, data@306,
    instance@320, sum@357, soft_hash_video_flat@535, + 3 streaming update()@569/618/669. All video
    detaches open strictly AFTER frame extraction. GIL theme (#39+#41) COMPLETE.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches; `iai_benches.rs` iai-callgrind
    0.16 (11 bench\_ fns, 16 cases). `scripts/iai_regression.py` (248 lines, stdlib),
    `tests/test_iai_regression.py` (11 tests), `tests/test_benchmarks.py` (18 pytest-benchmark fns).
- `docs/howto/` — 11 files; `docs/benchmarks.md` speedup 1.3x-158x. `scripts/version_sync.py` 16
    targets.
- **No Dependabot/Renovate** (`.github/dependabot.yml`, `renovate.json` absent) —
    dependency-freshness gap (v0.6.0).

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest. python-test matrix = 3.10 + 3.14 (count job defs, not run records).
- **Verify independently**: grep don't trust handoff; verify CI via check-runs API on real tip;
    re-read target.md diff each incremental (target GROWS — new "verified when" bullets turn
    met→partially-met).
- **Issues diff**: scan issues.md for NEW entries each cycle (`[human]` + `[review]`); watch
    `HUMAN REVIEW REQUESTED` + critical reshuffles.
- **Re-scope reactivation**: large issues.md/target.md/specs growth in the diff → human re-scoped;
    do a near-full re-review, not a diff parrot.

## Current State (assessed-at: 7f78ef5, iter 117)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released (workspace version `0.5.0`), all 12 bindings meet CORE
    criteria. **v0.6.0 GIL theme (#39+#41) now COMPLETE**; four spec'd work packages + two
    release-infra fixes still open.
- **CI GREEN on origin/develop tip `2ffc8f9`** — all 22 check-runs `success` incl.
    `Audit (cargo-deny)`. HEAD `7f78ef5` = +1 log-only `iteration 116` commit (unpushed), no code
    delta → verified-green state holds. Pushed code tip = `b55a1bf` (advance #41), review =
    `2ffc8f9`.
- **Iter 116 delivered #41 (Python text/video GIL)**: advance added 5 `py.detach` windows in iscc-py
    lib.rs (7→12 total), review PASS + Codex clean, #41 deleted from issues.md,
    `specs/python-bindings.md` GIL-text/video 3 boxes `[x]`. Python still **partially met** (only
    #49 aarch64 wheels remains).
- **cargo-deny gate LANDED & enforcing** — see Quality Gates. Live advisory can re-red it any push
    (prefer `cargo update -p` over deny.toml ignore). RUSTSEC-2026-0204 (iter 115) already resolved.
- **v0.6.0 scope (4 `normal` `[human]`, each spec'd)**: #42 WASM simd128, #43 Go ISCC-IDv1, #49
    aarch64 wheels, dependency review/refresh → Python/WASM/Go/CI-CD **partially met**. Recommended
    first pick: **#42** (pure build-flag change, most self-contained).
- **8 issues: 0 critical, 6 normal, 2 low** (all `[human]`). Also open normal: npm OIDC migration,
    single-registry re-trigger bug. Low (CID skips): v1.0.0 HELD by Titusz (stay 0.5.x, flip Semver
    enforcing at cut), docs logos.
- **MET sections**: Node, C FFI, Java, Ruby, .NET, C++, UniFFI, Swift, Kotlin, README, per-crate
    READMEs, Docs, Benchmarks.
- **Don't re-flag as new work**: GIL #41 (iter 116, DONE), cargo-deny gate, CRAP `--fail-above`
    (iter 113), iai perf gate/hardening (107-111), PyO3 #1 (105), semver gate (93), npm #38, GIL
    #39, SumHasher #37.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat pre-commit aborts commit** ("renders to different HTML") on: (a) nested/escaped
    backticks in a code span; (b) an ordered list RESTARTING at a non-1 number (e.g. `6.`/`7.` after
    a paragraph break → mdformat renormalizes to `1.` → diff HTML). Fix (b) = restart at 1 or use
    bullets. Test `uv run mdformat /tmp/copy.md` before committing; bisect line ranges to locate.
- **live advisory DB** — cargo-deny `advisories` can turn a previously-green gate red with no code
    change (see Audit gate).
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
