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
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`; **C FFI
    externs**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Counts** (re-verified 129): pytest-benchmark (18); UniFFI exports (32); llms-full ORDERED_PAGES
    (22); `docs/howto/*.md` (11); speedup 1.3x-158x; release.yml toggles (8); ffi extern (47);
    iscc-lib `#[test]` = **320** (needs `grep -rc --include="*.rs" crates/iscc-lib/` — src/\*.rs
    alone is 270); ci.yml job entries = 19 via `grep -cE '^  [a-z_-]+:$' ci.yml` minus 2 for
    `push`/`pull_request`.
- **version_sync TARGETS** = 20 entries (`grep -cE '^    \(' scripts/version_sync.py`) — the list at
    ~L260-288 is the authoritative set of version-synced files; anything absent rots silently.
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
    conclusion does NOT flip the run. rust-core.md box `[ ]` = the one genuinely-unmet core
    criterion (needs enforcing + ≥1.0.0, HELD by Titusz); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols). iscc-uniffi: 32 exports, 21 tests, `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs → 21 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job.
- `.github/workflows/release.yml` — 8 registry toggles (crates-io/pypi/npm/maven/ffi/rubygems/nuget/
    maven-kotlin). Swift XCFramework in `prepare-release` (~L55), NOT a toggle. `publish-npm-lib`
    has NO napi prepublish step (#38).
- `packages/go/` — pure Go, no CGO/WASM/binaries. **ISCC-IDv1 DONE (119, #43)**: `iscc_id.go` =
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result`; `codec.go` `VSV1` + `decodeHeader` allows
    Version=1 ONLY for MTId (~L271). **Trailing-byte fix DONE (120)**: `IsccDecode` has BOTH "too
    short" (L594) + "too long" (L597) branches. Rust-core `iscc_decode` parallel fix DONE (121).
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`
    `releaseTag`/`releaseChecksum`; `scripts/build_xcframework.sh` = 5 Apple targets.
    `packages/kotlin/` — Kotlin/JVM, Gradle wrapper 8.12.1 (major deferred), UniFFI-gen, 9
    desktop+Android. JVM pins after slice 5 + the **consumer-floor trap** (a compiler bump inside a
    PUBLISHED binding is a support-policy change, not a pin) → `dep-refresh-survey.md`.
- **`packages/kotlin/README.md` is NOT in version_sync TARGETS** → stuck at `0.3.1` (only stale
    version string in repo, found iter 129). Fix = bump line 13 + add to TARGETS.
    `specs/kotlin-bindings.md` L150+L165 still say `jna:5.16.0@aar` (human-owned, drift).
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
- `crates/iscc-wasm/Cargo.toml` — carries `blake3 = { features = ["wasm32_simd"] }` dep (iter 118,
    #42 DONE) SOLELY for feature-unification (no `use blake3` in lib.rs — don't prune as unused);
    that **Cargo feature**, not RUSTFLAGS, is what activates the SIMD backend (`v128` opcodes = WEAK
    signal; prove with `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3`). Also
    `wasm-opt --enable-simd` + simd128 RUSTFLAGS in ci.yml (wasm test) and release.yml (build).
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE);
    video detach opens strictly AFTER frame extraction; meta/audio/mixed stay attached by design.
- `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint` since 126); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **No Dependabot/Renovate** (`dependabot.yml`, `renovate.json` absent) — freshness gap (v0.6.0).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41) and
    `pyproject.toml` (`ruff<0.16`); MSRV recipe: `cargo info <crate>@<ver>`.
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

## Current State (assessed-at: c441740, iter 129)

- **IN_PROGRESS — CI GREEN.** v0.5.0 released, all 12 bindings meet CORE criteria. Partially met:
    Rust-core (semver-enforcing/v1.0.0 HELD by Titusz), CI/CD (dep freshness in progress), Per-Crate
    READMEs (stale `packages/kotlin/README.md` 0.3.1). Kotlin = met on criteria but has a
    release-blocking `[review]` issue. Every other section MET.
- **CI GREEN on origin/develop tip `4a93fe9`** (= iter-128 review PASS commit; HEAD `c441740` = +1
    UNPUSHED log-only commit, iterations.jsonl only). **41** check-runs, 21 distinct names, 0
    non-success, 0 running. Count is ~2x the job count because PR **#44 "Release 0.6.0"
    (develop→main) is OPEN** → every develop commit fires both a `push` and a `pull_request` run.
- **Dependency-refresh SLICED, in progress** (`normal` `[human]`, spec `ci-cd.md`→Dependency
    Freshness; no `[audit]` cite = no 8-file valve). Slices 1-5 DONE (Cargo.lock 124, uv.lock 125,
    Rust pins 126, GHA refs 127, JVM manifests 128); remaining + rationale →
    `dep-refresh-survey.md`. Next best: napi `package.json` + dotnet `.csproj` (tiny) or rb
    manifests.
- **6 issues: 0 critical, 4 normal, 2 low.** NEW iter 128: "Kotlin binding silently raised the
    consumer Kotlin floor to 2.3" `normal` `[review]` + HUMAN REVIEW REQUESTED → the *choice*
    (accept & document 2.3+ vs hold KGP 2.1.x) is Titusz's; CID may only execute the docs-only
    option once picked. Other normal = dep refresh (CID-doable) + npm OIDC + single-registry
    re-trigger (both human-gated). low (CID skips) = v1.0.0 (HELD), docs logos.
- **Don't re-flag as new work** (all DONE): dep slices 1-5 + c-cpp anchor (124-128), aarch64 wheels
    #49 (123), CRAP baseline (122), trailing-byte fixes (120-121), Go IDv1 #43 (119), WASM SIMD #42
    (118), GIL #39+#41, cargo-deny + CRAP `--fail-above` (113), iai perf gate (107-111), PyO3 #1,
    semver gate (93), npm #38, SumHasher #37. CID infra (audit role, metrics.jsonl, decisions.md,
    escape valve) = meta, NOT target sections — ignore for met/not-met.
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
- **Gradle flakes on this bind mount** (incremental-state `Unable to delete file …/build/kotlin/…`)
    — always `./gradlew clean` before believing a Kotlin build failure; check the test XML first.
- Go = pure Go only (no WASM/wazero/binaries). **csbindgen** runs on every `cargo build`
    (`crates/iscc-ffi/build.rs`).
- **UniFFI** = proc-macro, no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH
    `java.library.path` AND `jna.library.path`.
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 armv7→arm, dual
    Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
