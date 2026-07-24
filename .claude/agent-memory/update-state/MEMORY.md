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
- **Criterion benches**: `grep -n "^fn bench_" crates/iscc-lib/benches/benchmarks.rs`
- **Counts**: pytest-benchmark (18); UniFFI `#\[uniffi::export\]` iscc-uniffi/src/lib.rs (32);
    version-sync `version_sync.py --check` (16); llms-full ORDERED_PAGES (22); `docs/howto/*.md`
    (11); `docs/benchmarks.md` speedup 1.3x-158x.
- **release.yml toggles**: `grep "type: boolean" release.yml | wc -l` (8).
- **Issue count**: grep `issues.md` `^##` headers ending in a priority label (legend excluded);
    per-priority `grep -cE "^## .*\`normal\`"\`. **Trace a dep**: `cargo tree -i <crate>`.
- **state.md Write workaround**: Write tool = permission error → heredoc
    `cat > .claude/context/state.md << 'STATEEOF' ... STATEEOF`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ci.yml, ENFORCING (no continue-on-error), GREEN.
    `scripts/iai_regression.py --check` fails >10% Ir vs `.iai-baseline.json` (16 entries). GOTCHA:
    the nearby `continue-on-error` belongs to the SEPARATE semver job.
- **Coverage + CRAP** — one job, ENFORCING. Phase 3 runs cargo crap `--fail-regression`
    `--fail-above` (latter off `.cargo-crap.toml threshold=30.0`). Baseline `.crap-baseline.json`
    (97). Max CRAP ~22.3\<30. GREEN.
- **Audit (cargo-deny)** — ci.yml, ENFORCING (no continue-on-error): `cargo-deny@0.19.9` →
    `cargo deny check`. Root `deny.toml` (config v2; `ignore` list has 2 dev-bench ignores).
    `mise run audit`. **GOTCHA — live advisory DB flips this red with NO code change**: fix via
    `cargo update -p <crate>` (preferred) OR justified `ignore`. NOT in devcontainer → green CI job
    is only real confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0),
    `obi1kenobi/cargo-semver-checks-action@v2`. Conclusion does NOT flip the run. rust-core.md box
    `[ ]` (needs enforcing + ≥1.0.0, held); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 actual jobs** (python-test matrix expands
    3.10 + 3.14): functional + Semver (non-blocking) + Coverage+CRAP + Perf + **Audit
    (cargo-deny)**. `push:` under `on:` is NOT a job; read job names.
- `.github/workflows/release.yml` — 8 registry toggles (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget, maven-kotlin). Swift XCFramework builds in `prepare-release` (~line 55), NOT a toggle.
    `publish-npm-lib` has NO napi prepublish step (#38).
- `packages/go/` — pure Go, no CGO/WASM/binaries. `codec.go` rejects Version>0 (lines 269/438) —
    ISCC-IDv1 NOT yet supported (#43, v0.6.0).
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`
    `releaseTag`/`releaseChecksum`; `scripts/build_xcframework.sh` = 5 Apple targets.
- `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, JNA 5.16.0, UniFFI-generated, 9 desktop+Android.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher`. Only
    DataHasher+InstanceHasher re-exported at crate root; SumHasher via `streaming::` (drives
    `gen_sum_code_v0`). SumHasher wrappers: Python iscc-py, WASM iscc-wasm.
- `crates/iscc-wasm/Cargo.toml` — carries `blake3 = { features = ["wasm32_simd"] }` dep (iter 118,
    #42 DONE) SOLELY for feature-unification (no `use blake3` in lib.rs — don't prune as unused).
    Activates blake3 wasm32 SIMD backend. `[package.metadata.wasm-pack.profile.release] wasm-opt`
    has `--enable-simd`. simd128 RUSTFLAGS live in ci.yml (wasm test step) + release.yml (build).
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL theme #39+#41
    DONE): all heavyweight compute (data/instance/image/sum/text/video + 3 streaming update()).
    Video detaches open strictly AFTER frame extraction; meta/audio/mixed stay attached by design.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches; `iai_benches.rs` iai-callgrind
    0.16 (11 bench\_ fns, 16 cases). `scripts/iai_regression.py` + `tests/test_iai_regression.py`
    (11 tests). `docs/howto/` = 11 files; `scripts/version_sync.py` = 16 targets.
- **No Dependabot/Renovate** (`dependabot.yml`, `renovate.json` absent) — freshness gap (v0.6.0).

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on real tip (don't trust handoff). python-test
    matrix = 3.10 + 3.14. Re-read target.md/specs diff (they GROW — new "verified when" boxes flip
    met→partially-met).
- **Issues diff**: scan issues.md for NEW `[human]`/`[review]` entries + removed (resolved) ones;
    watch `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.

## Current State (assessed-at: a3db8ac, iter 119)

- **IN_PROGRESS — CI GREEN on pushed tip.** v0.5.0 released (workspace version `0.5.0`), all 12
    bindings meet CORE criteria. GIL theme (#39+#41) COMPLETE. **#42 WASM SIMD DONE (iter 118).**
    Three spec'd v0.6.0 feature packages + two release-infra fixes still open.
- **CI GREEN on origin/develop tip `6571c1b`** (iter-118 review commit) — all check-runs `success`
    incl. `WASM (wasm-pack test)` with the SIMD change, `Audit (cargo-deny)`, Perf, Coverage+CRAP,
    Semver. HEAD `a3db8ac` = +1 log-only commit (`cid(log): iteration 118`), unpushed, no source →
    nothing to verify.
- **#42 WASM simd128 RESOLVED (iter 118, PASS_WITH_NOTES)**: fix = direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep in `iscc-wasm/Cargo.toml`
    (feature-unification only, NO `use blake3` — don't prune as unused). Activates
    `Platform::detect()==WASM32_SIMD`. Landed `RUSTFLAGS=-C target-feature=+simd128` (ci.yml +
    release.yml) + `--enable-simd` wasm-opt stay in place. Verified by cargo-tree feature graph +
    byte-identical conformance + 1993→5370 `v128`-opcode jump. Native builds inert. WASM now
    **met**, all 4 "WASM SIMD" spec boxes `[x]`.
- **cargo-deny gate LANDED & enforcing** — see Quality Gates. Live advisory can re-red it any push
    (prefer `cargo update -p` over deny.toml ignore). RUSTSEC-2026-0204 (iter 115) already resolved.
- **v0.6.0 scope (3 `normal` `[human]` feature pkgs, each spec'd)**: #43 Go ISCC-IDv1 (recommended
    NEXT — vector `ISCC:MAIGHFECJMOPMIAB`), #49 aarch64 wheels, dependency review/refresh →
    Python/Go/CI-CD **partially met**.
- **7 issues: 0 critical, 5 normal, 2 low** (all `[human]`). Also open normal: npm OIDC migration,
    single-registry re-trigger bug. Low (CID skips): v1.0.0 HELD by Titusz (stay 0.5.x, flip Semver
    enforcing at cut), docs logos.
- **MET sections**: Node, WASM, C FFI, Java, Ruby, .NET, C++, UniFFI, Swift, Kotlin, README,
    per-crate READMEs, Docs, Benchmarks.
- **Don't re-flag as new work**: #42 WASM SIMD (DONE iter 118 — blake3/wasm32_simd feature + simd128
    flags), GIL #41 (iter 116, DONE), cargo-deny gate, CRAP `--fail-above` (iter 113), iai perf gate
    (107-111), PyO3 #1 (105), semver gate (93), npm #38, GIL #39, SumHasher #37.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat pre-commit aborts commit** ("renders to different HTML") on: (a) nested/escaped
    backticks in a code span; (b) an ordered list RESTARTING at a non-1 number (e.g. `6.`/`7.` after
    a paragraph break → mdformat renormalizes to `1.` → diff HTML). Fix (b) = restart at 1 or use
    bullets. Test `uv run mdformat /tmp/copy.md` before committing; bisect line ranges to locate.
- **live advisory DB** — cargo-deny `advisories` can turn a previously-green gate red with no code
    change (see Audit gate).
- **blake3 WASM SIMD (RESOLVED iter 118 #42 — kept as reference)** — BLAKE3's `wasm32` SIMD backend
    (blake3 1.8.x) is gated by the `blake3/wasm32_simd` **Cargo feature** (NOT by RUSTFLAGS alone;
    `v128` opcodes are a WEAK signal — LLVM auto-vectorizes the portable path too). FIX = direct
    `blake3 = { features = ["wasm32_simd"] }` dep in iscc-wasm/Cargo.toml (feature-unifies, native
    inert). Proof = `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3` shows
    `wasm32_simd`. simd128 RUSTFLAGS + `--enable-simd` wasm-opt still needed (auto-vec + wasm-opt).
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
