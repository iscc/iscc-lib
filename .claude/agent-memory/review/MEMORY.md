# Review Agent Memory

Review patterns, quality gate knowledge, and common issues accumulated across CID iterations. See
`review-patterns.md` for detailed documentation/verification/issues patterns and gotchas.

## Quality Gate Details

- `mise run check` runs 15 pre-commit hooks (file hygiene, formatting, linting incl. Ruby)
- Pre-push hooks (clippy, cargo test, pytest, etc.) not in `mise run check` — verify clippy
    separately with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests: run `mvn test` explicitly. Go tests: `cd packages/go && mise exec -- go test ./...`
- `check-added-large-files` threshold is `--maxkb=256`

## Pre-push Hook Gotchas

- **`ty check` external Python files**: Python files importing packages not in project venv (e.g.,
    `conanfile.py` importing `conan`) fail `ty check`. Fix: `[tool.ty.src] exclude` in
    `pyproject.toml`. This is proper scope exclusion, not gate circumvention. Advance agents should
    add this when creating Python files with external deps

## Common Issues

- Go `go get` adds deps as `// indirect` — run `go mod tidy` after
- Verification grep patterns may false-positive — verify match specificity. `grep -qv 'pattern'` is
    always true for multi-line files (matches ANY line not containing pattern) — use
    `! grep -q 'pattern'` to verify absence
- next.md test specs may have wrong expected values — verify against Rust implementation
- Advance agent test counts often inaccurate — always run tests to verify
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples
- **prek stash conflict**: untracked files with formatting issues cause prek stash/restore to fail
    during commit. Fix: move untracked files to /tmp before committing, restore after
- **next.md relative path errors**: define-next agent may specify wrong relative paths (e.g.,
    `../../README.md` counting wrong). Always verify relative paths by `realpath` during review
- **CI find cross-arch bug**: when multiple targets extract to same CWD with same lib name, `find`
    patterns must include target name. Generic wildcards match all extracted dirs
- **Advance agent idle claims**: always verify remaining issue priorities independently — advance
    agents may incorrectly claim "only low-priority issues remain" when normal issues still exist
- **Docs site URL**: `https://lib.iscc.codes/` NOT `https://iscc-lib.iscc.io/`. Advance agents
    consistently get this wrong — always verify in review
- **`mise run check` mdformat Failure on context files**: recurring — define-next writes `next.md`
    and `define-next/MEMORY.md` non-mdformat-conforming, so `prek --all-files` reformats them every
    cycle. NOT a regression in advance work. `git commit` (staged-only) is unaffected, BUT the
    pre-push mdformat hook runs on the whole push range and WILL reject the batch (those files were
    committed non-conforming by define-next). Fix (iter 101): STAGE the mdformat-reformatted
    `next.md` + `define-next/MEMORY.md` into the review commit (mechanical 100-col rewrap, zero
    semantic change) so HEAD is conforming and push passes. Never stage `iterations.jsonl`
    (runner-owned)
- **Concurrent CID loops (iter 97, resolved iter 98)**: spurious `mise run check` "files were
    modified by this hook" on a file the advance never touched (e.g. `standardrb-fix` flagging when
    NO `.rb` is dirty) + a working-tree `state.md`/context change appearing mid-review = a SECOND
    CID loop racing the branch. Confirm with `ps aux | grep -E 'cid:run|claude -p CID iteration'`
    (two `mise run cid:run` trees / two different `iteration N` agents). Flag HUMAN REVIEW
    REQUESTED, do NOT push, do NOT kill processes yourself. RESOLUTION: a later review re-checks
    `ps aux` — once a SINGLE `mise run cid:run` remains, the duplicate is gone and the unpushed
    backlog pushes as a fast-forward (`git rev-list --left-right --count origin/<b>...HEAD` =
    `0 N`); scan ALL `@{upstream}..HEAD` commits for gate circumvention before that batch push

## Review Shortcuts

- **Rust-only**: `cargo test -p iscc-lib` + clippy + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Docs-only**: `mise run check` + clippy
- **Python-only**: `mise run check` + `pytest`
- **Go-only**: `mise run check` + `cd packages/go && mise exec -- go test ./...`
- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`)
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`
- **Config-only**: `mise run check` + `cargo check -p <crate>`
- **Version sync addition**: `mise run check` + `uv run scripts/version_sync.py --check` + clippy
- **CI-only YAML**: `mise run check` + validate structure with
    `uv run python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` (jobs list,
    placement, `needs:`/`continue-on-error`). Note: a gate running a tool absent locally (e.g.
    `cargo-deny`/`cargo-audit`) has a CI-only criterion — confirm post-push. (valgrind +
    iai-callgrind-runner ARE present — see Perf gate review below)
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- Cross-platform CI: bash syntax needs `shell: bash` if matrix includes Windows
- **Semver gate review** (iter 93): informational pre-1.0 — full verify recipe in
    `MEMORY-archive.md`
- **Perf gate review — COMPLETE & HARDENED (slice 2a iter 107/108, 2b iter 109, hardening iter 110;
    #3 closed + CI GREEN on a5ce73c)**: valgrind 3.19 + `iai-callgrind-runner` 0.16.1 ARE in the
    devcontainer (iter-107 "absent" claim WRONG). `mise run bench:iai` RUNS locally (bakes in
    `IAI_CALLGRIND_ALLOW_ASLR=true`; kernel blocks the `personality` syscall iai's `setarch -R`
    needs). 2b VERIFY: `mise run bench:iai` → 16 `.out`, then
    `python3 scripts/iai_regression.py --check` → 16 within 10% exit 0 (committed
    `.iai-baseline.json` is CI-sourced; local 1.96.0 agrees ≤1.66%). Script logic test in /tmp:
    `--update` from synthetic `summary:` dirs, self-check exit 0, tamper one baseline −50% → exit 1,
    missing baseline → exit 1, `.out.old` excluded. RACE: `--check` immediately after `cargo bench`
    exits can match 9/16 mid-flush — re-run; CI runs it as a separate step so unaffected. STRIP
    GOTCHA: `[profile.bench] strip = false, debug = true` load-bearing (else 0
    `__iai_callgrind_wrapper` symbols → all `summary: 0` false green). FALSE-GREEN EDGES HARDENED
    (iter 110, [review] issue closed): `check_regressions(run, baseline, allow_missing=False)` now
    FAILS on any shared bench with current Ir 0 (zero-count guard, independent of `--allow-missing`)
    AND on a disappeared baselined bench (unless `--allow-missing` downgrades to warning);
    `only_run` new-bench still warns only. Script-only review shortcut:
    `uv run pytest   tests/test_iai_regression.py -q` (11 synthetic-fixture tests, loads script by
    path via `spec_from_file_location` like test_cid.py) + ruff check/format + `ty check`. NO
    `continue-on-error` (enforcing); post-push Perf-step-green is CI-only confirm

## Codex Review Integration

- Codex findings are advisory — cross-reference with your own analysis
- Use `--commit HEAD` for advance commit (verify with `git log` first)
- Go codec Codex findings: dismiss — Go mirrors Rust reference faithfully
- Codex `.NET version` findings: `dotnet-version: '8.0'` is valid for `actions/setup-dotnet@v4` —
    resolves to latest 8.0.x SDK. Dismiss "use `8.0.x`" suggestions
- Codex Conan recipe findings: MinGW ABI and MSVC cxxflags concerns are valid but non-blocking when
    recipe went from completely broken to functional. Assess severity relative to baseline

## Feature Flag Review

- Use Rust-only shortcut. Verify 3 configs: default, no-default, text-processing only
- `serde_json` non-optional (conformance.rs dependency)
- **`--no-default-features --all-targets` fails on `benchmarks` bench** (pre-existing, NOT a
    regression): benches import `gen_meta`/`gen_text` needing default features. Scope no-default
    clippy to the lib (`--no-default-features -- -D warnings`, no `--all-targets`). CI never runs it
- `streaming::SumHasher` is core-only (full path `iscc_lib::streaming::SumHasher`), NOT a Tier 1
    re-export; `gen_sum_code_v0` wraps `SumHasher::finalize(bits,wide,add_units)`. #37 fully closed

## Binding State

- 9 binding crates (py, napi, wasm, ffi, jni, go, rb, uniffi), all 32/32 Tier 1 symbols, all met.
    NAPI `index.js`/`.d.ts`/`*.node` gitignored (auto-gen by `napi build`). npm `@iscc/lib` bundled
    single-package (#38 closed): `files: ["*.node"]` ships all 5 binaries, NO
    `optionalDependencies`; `napi prepublish` must NOT run in `publish-npm-lib`. FFI constant count
    in module docstring must match additions (now 5)
- CI: 18 YAML job entries + python-test matrix (`['3.10','3.14']`) → **19 actual jobs** (`semver`
    iter 93, `coverage` iter 94, `perf` iter 107). Advance handoffs count YAML entries, not matrix
    expansion. Version sync: 16 targets (incl. Package.swift releaseTag). Release: 9 registry inputs
- **`Coverage + CRAP` CI job** (iter 94/96/97, ci-cd.md): standalone, no
    `needs:`/`continue-on-error`. Verify from repo root: `mise run crap` (exit 0, "97 functions;
    none exceed 30") + enforcing gate
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`
    (pass=`0 regressed/97 unchanged`; do NOT pipe to tail — masks `$?`) + `mise run crap:baseline`
    regenerates `.crap-baseline.json` byte-identical (COMMITTED). CI install MUST be
    `cargo binstall -y --force cargo-crap@0.2.2` (`--force` load-bearing, rust-cache poisoning).
    GOTCHA: `.cargo-crap.toml` MUST exclude `crates/iscc-lib/benches/**` else `bench_cdc_chunks`
    leaks at CRAP 42. CAVEAT: `--fail-regression` exits 0 for NEW high-CRAP funcs — [review] issue
    open
- **iscc-rb workspace exclusion**: `--exclude iscc-rb` in CI `rust` job is permanent — Rust job
    lacks Ruby headers/libclang-dev. Dedicated `ruby` job handles iscc-rb clippy/compile/test
- .NET + Swift bindings fully complete (32/32 Tier 1, CI, version sync, docs, release)

## Binding Propagation Shortcuts

- napi-rs: `npm test` + clippy + `mise run check`
- WASM: `wasm-pack test --node` (with and without `--features conformance`) + clippy. To run one
    test file: `--test <name>` goes BEFORE the `--` (cargo arg); after `--` the runner rejects it
- C FFI: `cargo test -p iscc-ffi` + clippy. Header tracked in git
- Java JNI: `cargo build -p iscc-jni` + clippy + `mvn test`
- Ruby: `pushd crates/iscc-rb && bundle exec rake compile && bundle exec rake test; popd`
- .NET: `cargo build -p iscc-ffi` + `dotnet build packages/dotnet/Iscc.Lib/` +
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug`
    - `mise run check`
- Kotlin: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy workspace
    - `mise run check`

## UniFFI Review

- `crates/iscc-uniffi/` — shared scaffolding for Swift+Kotlin. `uniffi = "0.31"`, proc macros only
    (no UDL/build.rs/uniffi.toml). 32 `#[uniffi::export]` (30 free fns + 2 impl blocks).
    `publish =   false`. `bindgen` feature: `uniffi/cli` → `uniffi-bindgen` binary
- Review shortcut: `cargo test -p iscc-uniffi` + `cargo clippy -p iscc-uniffi -- -D warnings` +
    `cargo clippy --workspace --all-targets -- -D warnings` + `mise run check`

## Kotlin Binding Review

- **Gradle multi-JAR artifact**: `withSourcesJar()` + `withJavadocJar()` produce 3 JARs in
    `build/libs/`. When selecting runtime JAR from glob, filter out `-sources.jar`/`-javadoc.jar` —
    alphabetical `head -1` picks `-javadoc.jar` first
- Generated `iscc_uniffi.kt` (~112KB, 3214 lines) — do NOT manually edit, regenerate via
    uniffi-bindgen. `@file:Suppress("NAME_SHADOWING")` is UniFFI boilerplate, not gate circumvention
    (review shortcut in Binding Propagation Shortcuts above)
- JNA native lib loading: `java.library.path` alone NOT sufficient for JNA `Native.register()`. Must
    also set `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env var in test task
- Kotlin bindings fully complete: CI job, gradlew perms, version sync, docs/README, release
    workflow. Codex is confused by large generated Kotlin/Swift diffs — findings advisory
- Kotlin Maven Central: `useInMemoryPgpKeys` (not `useGpgCmd`), staging to `build/staging-deploy/`,
    curl bundle upload to Central Portal REST API. JNA resource dirs differ from JNI (linux-x86-64
    vs linux-x86_64)

## Environment

- Python `iscc_lib`: compile with `cd crates/iscc-py && uv run maturin develop --release`
- `.pyi` stub sync: `ty check` catches mismatches, `mise run check` does not
- **Pre-push needs iscc_lib built**: `ty check` and `pytest` hooks import `iscc_lib` — build before
    pushing (same maturin command above), else push fails
- **PyO3 is `0.29`, migration COMPLETE** (#1 closed): per-hop recipe + GIL-detach pattern + gotcha
    catalog in `learnings-archive.md`. Verify pin: `cargo tree -p iscc-py -i pyo3` (single 0.29.0).
    Keep explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) — 0.28 silently
    flipped that default `true`→`false`. LESSON for future bumps: "compiles clean under
    `-D warnings`" ≠ behavior-neutral; diff pyo3-macros-backend default handling each hop. Advisory
    clearance NOT tool-confirmable — `cargo deny`/`cargo audit` absent from devcontainer + CI
    ([review])

## Ruby Binding Review

- Magnus 0.7.1 pinned for Ruby 3.1 compat — 0.8 needs Ruby 3.2+
- `function!` macro does NOT accept `&Ruby` — use `Ruby::get().expect("called from Ruby")`
- `rb-sys` needs Ruby headers + `libclang-dev` — why `--exclude iscc-rb` in CI
- Ruby `JSON.generate` ignores `sort_keys: true` — use `.sort.to_h` before generate
- Streaming classes: `#[magnus::wrap(class = "...")]` + `RefCell<Option<inner>>` for one-shot
    finalize. Linting: Standard Ruby (`standard` gem) + `rubocop-minitest`, config `.standard.yml`
