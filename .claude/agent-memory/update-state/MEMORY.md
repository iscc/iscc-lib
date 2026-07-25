# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Full gate pipelines → `MEMORY-archive.md`.

**Size budget:** Keep under 140 lines. One line per entry. Archive detail.

## Exploration Shortcuts

- **Per-crate READMEs / CLAUDE.md** (12 each): `ls crates/*/{README.md,CLAUDE.md} packages/*/…`
- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs):
    `gh api repos/iscc/iscc-lib/commits/<tip-sha>/check-runs --jq '.check_runs[]|{name,conclusion}'`
    on the real origin/develop tip. **Failed logs**: `gh run view <id> --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` (code after the last CI sha is
    UNVERIFIED). Usual: HEAD = +1 log-only commit — NOT guaranteed (iter 121 had none).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts**: pytest-benchmark (18); UniFFI exports (32); version-sync (16); llms-full ORDERED_PAGES
    (22); `docs/howto/*.md` (11); speedup 1.3x-158x; release.yml toggles (8).
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING, GREEN. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: the nearby `continue-on-error` is the SEMVER job's.
- **Coverage + CRAP** — one job, ENFORCING: cargo crap `--fail-regression` + `--fail-above` (30.0
    via `.cargo-crap.toml`); baseline `.crap-baseline.json` (97 entries), max CRAP ~22.3. **GOTCHA —
    `--fail-regression` is CI-ONLY, not in `mise run check`/pre-commit**: a new branch/loop in a
    covered fn → `↑ N regressed` → exit 1 despite a GREEN local check (bit iter 121). Fix = refresh
    that entry in the SAME step as the source change.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9` → `cargo deny check`; root `deny.toml`
    (v2, 2 dev-bench ignores); `mise run audit`. **GOTCHA — live advisory DB flips this red with NO
    code change**: fix via `cargo update -p <crate>` (preferred) or justified `ignore`. NOT in
    devcontainer → green CI job is the only real confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0); its
    conclusion does NOT flip the run. rust-core.md box `[ ]` (needs enforcing + ≥1.0.0, HELD);
    ci-cd.md `[x]`. `decisions.md`: input-domain narrowing ≠ SemVer break.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR asserting `needs.python-test.result == success`
    (so check-runs show "Python 3.10/3.14" AND "Python (ruff, pytest)"). `push:` under `on:` is NOT
    a job. Plus Semver (non-blocking), Coverage+CRAP, Perf, Audit (cargo-deny).
- `.github/workflows/release.yml` — 8 registry toggles (crates-io/pypi/npm/maven/ffi/rubygems/nuget/
    maven-kotlin). Swift XCFramework in `prepare-release` (~L55), NOT a toggle. `publish-npm-lib`
    has NO napi prepublish step (#38).
- `packages/go/` — pure Go, no CGO/WASM/binaries. **ISCC-IDv1 DONE (119, #43)**: `iscc_id.go` =
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result`; `codec.go` `VSV1` + `decodeHeader` allows
    Version=1 ONLY for MTId (~L271). **Trailing-byte fix DONE (120)**: `IsccDecode` has BOTH "too
    short" (L594) + "too long" (L597) branches; `DecodeIsccID` inherits, `IsccDecompose` untouched.
    Rust-core `iscc_decode` parallel fix DONE (121, lib.rs L235+L241) — independent (Go = reimpl).
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`
    `releaseTag`/`releaseChecksum`; `scripts/build_xcframework.sh` = 5 Apple targets.
    `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, JNA 5.16.0, UniFFI-gen, 9 desktop+Android.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
- `crates/iscc-wasm/Cargo.toml` — carries `blake3 = { features = ["wasm32_simd"] }` dep (iter 118,
    #42 DONE) SOLELY for feature-unification (no `use blake3` in lib.rs — don't prune as unused).
    Activates blake3 wasm32 SIMD backend. `[package.metadata.wasm-pack.profile.release] wasm-opt`
    has `--enable-simd`. simd128 RUSTFLAGS live in ci.yml (wasm test step) + release.yml (build).
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE):
    all heavyweight compute (data/instance/image/sum/text/video + 3 streaming update()); video
    detach opens strictly AFTER frame extraction; meta/audio/mixed stay attached by design.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint` since iter 126); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases) +
    `scripts/iai_regression.py`.
- **No Dependabot/Renovate** (`dependabot.yml`, `renovate.json` absent) — freshness gap (v0.6.0).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41) and
    `pyproject.toml` (`ruff<0.16`). Update when a hold-back lifts; MSRV recipe:
    `cargo info <crate>@<ver>`. Reasons → `dep-refresh-survey.md`.
- **Full dependency-pin inventory** (GHA refs per workflow, tooling pins, per-binding manifests;
    re-verified iter 128) → `dep-refresh-survey.md`. Headlines: ci.yml + docs.yml GHA refs are
    CURRENT (setup-uv = EXACT tag `@v9.0.0`); release.yml still lags and has **no setup-uv step**
    (issues.md claims otherwise — wrong); `mise.toml` has no `[tools]` section.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on real tip (don't trust handoff — a review
    PASS with green `mise run check` can still fail CI-only gates like CRAP `--fail-regression`).
    python-test matrix = 3.10+3.14. Re-read target/specs diff (they GROW → boxes flip met→partial).
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.

## Current State (assessed-at: 571ed7c, iter 128)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released, all 12 bindings meet CORE criteria. Only Rust-core
    (semver-enforcing/v1.0.0 HELD by Titusz) + CI/CD (dep freshness in progress) are partially met;
    every other section MET.
- **CI GREEN on origin/develop tip `b071517`** (= review PASS commit; HEAD `571ed7c` = +1 UNPUSHED
    log-only commit, iterations.jsonl only). **41** check-runs, 0 non-success, 0 running. Count is
    ~2x the job count because PR **#44 "Release 0.6.0" (develop→main) is OPEN** → every develop
    commit fires both a `push` and a `pull_request` run.
- **Dependency-refresh SLICED, in progress** (`normal` `[human]`, spec `ci-cd.md`→Dependency
    Freshness; no `[audit]` cite = no 8-file valve). ✅ s1 Rust `Cargo.lock` (124) ✅ s2 Python
    `uv.lock` (125, iscc-core 1.3.0 = zero vector drift, `ruff<0.16` hold-back) ✅ s3 Rust direct
    pins (126: criterion 0.5→0.7, 4 `# held:` comments) ✅ s4 GHA refs in ci.yml + docs.yml (127; 9
    refs, setup-uv exact-tag). Remaining: (a) JVM manifests (jni pom + kotlin gradle — both
    CI-exercised, best next slice), (b) small manifests (rb Gemfile/gemspec+lock, go.mod, napi
    package.json; dotnet csproj already wildcards), (c) ruff 0.16 adoption, (d) magnus 0.8 + (e) jni
    0.22 — each its own source-rewrite step. **Deferred/human-timed**: release.yml GHA refs (nothing
    there is exercised by a CID push).
- **5 issues: 0 critical, 3 normal `[human]`, 2 low `[human]`** — headers unchanged since iter 124;
    only the dep-refresh issue's Progress block grows. normal = dep refresh (CID-doable) + npm OIDC
    migration + single-registry re-trigger bug (both human-gated). low (CID skips) = v1.0.0 (HELD:
    stay 0.5.x, flip Semver enforcing at the cut), docs logos. NO CID-actionable
    `[review]`/`[audit]`.
- **Don't re-flag as new work**: dep slices 1-4 + c-cpp anchor fix (124-127), #49 aarch64 wheels
    (123), CRAP baseline refresh (122), Rust iscc_decode trailing-byte (121), Go IsccDecode (120),
    #43 Go ISCC-IDv1 (119), #42 WASM SIMD (118), GIL #41 (116), cargo-deny gate, CRAP `--fail-above`
    (113), iai perf gate (107-111), PyO3 #1 (105), semver gate (93), npm #38, GIL #39, SumHasher
    #37. CID infra (audit role, metrics.jsonl, decisions.md, escape valve) = meta, NOT target
    sections — ignore for met/not-met.
- **Known non-regression**: `proc-macro-error2 v2.0.1` future-incompat warning on cargo test/bench
    comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only), NOT magnus/rb-sys. No fixed
    upstream release; re-check when bumping iai-callgrind (must match CI's runner version).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat pre-commit aborts commit** ("renders to different HTML") on: (a) nested/escaped
    backticks in a code span; (b) ordered list RESTARTING at non-1 (mdformat renormalizes → diff
    HTML) — fix = restart at 1 or bullets. Test `uv run mdformat /tmp/copy.md` before committing.
- **live advisory DB** — cargo-deny `advisories` can turn a previously-green gate red with no code
    change (see Audit gate).
- **blake3 WASM SIMD (RESOLVED 118 #42)** — gated by the `blake3/wasm32_simd` **Cargo feature** (NOT
    RUSTFLAGS alone; `v128` opcodes = WEAK signal). Proof:
    `cargo tree -p iscc-wasm --target   wasm32-unknown-unknown -i blake3`.
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
