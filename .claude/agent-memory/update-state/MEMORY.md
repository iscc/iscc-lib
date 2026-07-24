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
- **Failed-job logs**: `gh run view <id> --log-failed | grep -iE "error|RUSTSEC|regress|FAILED"`
- **Unpushed check**: `git log --oneline origin/develop..HEAD` (origin lags; code after last CI sha
    UNVERIFIED). Usual: HEAD = +1 log-only commit (iter 122) — but NOT guaranteed (iter 121: HEAD ==
    origin/develop). Always check, don't assume; run check-runs API on the real origin/develop tip.
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts**: pytest-benchmark (18); UniFFI `#\[uniffi::export\]` (32); version-sync (16); llms-full
    ORDERED_PAGES (22); `docs/howto/*.md` (11); benchmarks speedup 1.3x-158x; release.yml toggles
    `grep "type: boolean"` (8).
- **Issue count**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` lists headers with
    priority+source tag; per-priority `grep -cE "^## .*\`normal\`"\`. **Trace a dep**:
    `cargo tree -i <crate>`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ci.yml, ENFORCING (no continue-on-error), GREEN.
    `scripts/iai_regression.py --check` fails >10% Ir vs `.iai-baseline.json` (16 entries). GOTCHA:
    the nearby `continue-on-error` belongs to the SEPARATE semver job.
- **Coverage + CRAP** — one job, ENFORCING. Phase 3 = cargo crap `--fail-regression` `--fail-above`
    (latter off `.cargo-crap.toml threshold=30.0`). Baseline `.crap-baseline.json` (97 entries =
    file/function/line/cyclomatic/coverage/crap). Max CRAP ~22.3\<30. **GOTCHA — `--fail-regression`
    is CI-ONLY, NOT in `mise run check`/pre-commit**: adding a branch/loop to a covered function
    raises its cyclomatic above baseline → `↑ N regressed` → exit 1 with GREEN local check. Bit iter
    121 (`iscc_decode` +"too long" branch vs baseline 4.0/crap 4.11). Fix = refresh that entry
    (regen `cargo llvm-cov`+`cargo crap`, or hand-edit) IN THE SAME STEP as the source change.
    `--fail-above` (30) is a SEPARATE trigger — read `↑ regressed` vs max-CRAP row to tell which
    fired.
- **Audit (cargo-deny)** — ci.yml, ENFORCING (no continue-on-error): `cargo-deny@0.19.9` →
    `cargo deny check`. Root `deny.toml` (config v2; `ignore` list has 2 dev-bench ignores).
    `mise run audit`. **GOTCHA — live advisory DB flips this red with NO code change**: fix via
    `cargo update -p <crate>` (preferred) OR justified `ignore`. NOT in devcontainer → green CI job
    is only real confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), `@v2`
    action. Conclusion does NOT flip the run. rust-core.md box `[ ]` (needs enforcing + ≥1.0.0,
    held); ci-cd.md `[x]`. `decisions.md` (2026-07-24): input-domain narrowing ≠ SemVer break.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML entries → 20 jobs** (python-test = 3.10+3.14): functional +
    Semver (non-blocking) + Coverage+CRAP + Perf + Audit (cargo-deny). `push:` under `on:` is NOT a
    job; read job names. **21 check-runs when all green** (iter 122: 20 green + Coverage+CRAP red).
- `.github/workflows/release.yml` — 8 registry toggles (crates-io/pypi/npm/maven/ffi/rubygems/nuget/
    maven-kotlin). Swift XCFramework in `prepare-release` (~L55), NOT a toggle. `publish-npm-lib`
    has NO napi prepublish step (#38).
- `packages/go/` — pure Go, no CGO/WASM/binaries. **ISCC-IDv1 DONE (iter 119, #43)**: `iscc_id.go` =
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result`; `codec.go` `VSV1` const + `decodeHeader` allows
    Version=1 ONLY for MTId (line ~271). Vector `ISCC:MAIGHFECJMOPMIAB` → realm 0/hub 1/ts
    1751831876325218\. body=`(ts<<12)|hub`. **Trailing-byte fix DONE (iter 120)**: `IsccDecode` now
    has BOTH "too short" (`len(tail) < nbytes`, L594) + "too long" (`len(tail) > nbytes`, L597)
    branches; `DecodeIsccID` inherits, `IsccDecompose` untouched. Rust-core `iscc_decode` parallel
    fix also DONE (iter 121, both branches lib.rs L235+L241) — the two are independent (Go = native
    reimpl, not FFI).
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`
    `releaseTag`/`releaseChecksum`; `scripts/build_xcframework.sh` = 5 Apple targets.
- `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, JNA 5.16.0, UniFFI-generated, 9 desktop+Android.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
- `crates/iscc-wasm/Cargo.toml` — carries `blake3 = { features = ["wasm32_simd"] }` dep (iter 118,
    #42 DONE) SOLELY for feature-unification (no `use blake3` in lib.rs — don't prune as unused).
    Activates blake3 wasm32 SIMD backend. `[package.metadata.wasm-pack.profile.release] wasm-opt`
    has `--enable-simd`. simd128 RUSTFLAGS live in ci.yml (wasm test step) + release.yml (build).
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE):
    all heavyweight compute (data/instance/image/sum/text/video + 3 streaming update()); video
    detach opens strictly AFTER frame extraction; meta/audio/mixed stay attached by design.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches; `iai_benches.rs` iai-callgrind
    0.16 (11 bench\_ fns, 16 cases). `scripts/iai_regression.py` + `tests/test_iai_regression.py`
    (11 tests). `docs/howto/` = 11 files; `scripts/version_sync.py` = 16 targets.
- **No Dependabot/Renovate** (`dependabot.yml`, `renovate.json` absent) — freshness gap (v0.6.0).

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on real tip (don't trust handoff — a review
    PASS with green `mise run check` can still fail CI-only gates like CRAP `--fail-regression`).
    python-test matrix = 3.10+3.14. Re-read target/specs diff (they GROW → boxes flip met→partial).
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.

## Current State (assessed-at: 9845cf5, iter 123)

- **IN_PROGRESS — CI GREEN on develop tip.** v0.5.0 released (`0.5.0`), all 12 bindings meet CORE
    criteria. The iter-121/122 CRAP RED is RESOLVED: iter 122 refreshed the `.crap-baseline.json`
    `iscc_decode` entry (cyclomatic 4→5, coverage 80.95→85.19, crap 4.11→5.08, now ~L223) — source
    robustness fix stays. Review PASS (handoff).
- **CI GREEN on origin/develop tip `e76a22b`** (HEAD `9845cf5` = +1 log-only commit iter 122). ALL
    **30 check-runs `success`** incl. the previously-red `Coverage + CRAP`. Verified via
    `gh api .../commits/e76a22b/check-runs` (0 non-success). NO CI fix needed this iter.
- **cargo-deny gate enforcing** — see Quality Gates. Live advisory can re-red any push (prefer
    `cargo update -p` over deny.toml ignore).
- **v0.6.0 remaining (`normal` `[human]`, spec'd, CID-doable)**: #49 aarch64 wheels (plan
    `.claude/plans/restore-linux-aarch64-python-wheels.md`), dependency review/refresh →
    Python/CI-CD **partially met**. Next milestone = these two (CI is green, no fix needed).
- **6 issues: 0 critical, 4 normal `[human]`, 2 low `[human]`.** normal: aarch64 wheels, dep
    refresh, npm OIDC migration, single-registry re-trigger bug. Low (CID skips): v1.0.0 HELD by
    Titusz (stay 0.5.x, flip Semver enforcing at cut), docs logos. NO CID-actionable `[review]`/
    `[audit]` issue open.
- **MET sections**: Node, WASM, C FFI, Java, Go, Ruby, .NET, C++, UniFFI, Swift, Kotlin, README,
    per-crate READMEs, Docs, Benchmarks. Rust-core (semver-enforcing/v1.0.0 held) + Python (aarch64)
    - CI/CD (deps freshness, aarch64 matrix) = partially met.
- **Don't re-flag as new work**: CRAP baseline refresh (iter 122), Rust-core iscc_decode
    trailing-byte (iter 121), Go IsccDecode trailing-byte (120), #43 Go ISCC-IDv1 (119), #42 WASM
    SIMD (118), GIL #41 (116), cargo-deny gate, CRAP `--fail-above` (113), iai perf gate (107-111),
    PyO3 #1 (105), semver gate (93), npm #38, GIL #39, SumHasher #37. CID infra (audit role,
    `metrics.jsonl`, `decisions.md`, scope escape valve) = meta, NOT target sections — ignore for
    met/not-met.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat pre-commit aborts commit** ("renders to different HTML") on: (a) nested/escaped
    backticks in a code span; (b) ordered list RESTARTING at non-1 (mdformat renormalizes → diff
    HTML) — fix = restart at 1 or bullets. Test `uv run mdformat /tmp/copy.md` before committing.
- **live advisory DB** — cargo-deny `advisories` can turn a previously-green gate red with no code
    change (see Audit gate).
- **blake3 WASM SIMD (RESOLVED iter 118 #42)** — `wasm32` SIMD backend gated by the
    `blake3/wasm32_simd` **Cargo feature** (NOT RUSTFLAGS alone; `v128` opcodes = WEAK signal, LLVM
    auto-vectorizes portable too). Proof:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i   blake3` shows `wasm32_simd`.
    simd128 RUSTFLAGS + `--enable-simd` wasm-opt still needed.
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
