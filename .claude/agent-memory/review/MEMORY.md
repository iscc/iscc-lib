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
- **CI-only YAML**: `mise run check`
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- Cross-platform CI: bash syntax needs `shell: bash` if matrix includes Windows
- **Semver gate review** (iter 93): `semver` job is informational (`continue-on-error: true`)
    pre-1.0 — it adds a gate, doesn't weaken one. Verify locally:
    `cargo semver-checks check-release -p iscc-lib` (~7s; expect "2 major checks failed" = the
    post-0.4.0 `pub(crate)` narrowing, intended). Piping to `tail`/`head` masks the non-zero exit —
    read the "Summary … N checks failed" line

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
- CI: 17 YAML job entries + python-test matrix (`['3.10','3.14']`) → **18 actual jobs** (`semver`
    iter 93, `coverage` iter 94). Advance handoffs count YAML entries (17), not matrix expansion.
    Version sync: 16 targets (incl. Package.swift releaseTag). Release: 9 registry inputs
- **`Coverage + CRAP` CI job** (Phase 1 iter 94, Phase 2 iter 96, Phase 3 iter 97, ci-cd.md):
    standalone, no `needs:`, no `continue-on-error`. `cargo-llvm-cov` 0.8.7 + `cargo-crap` 0.2.2 in
    devcontainer. CI install MUST be `cargo binstall -y --force cargo-crap@0.2.2` (iter 100):
    `--force` is load-bearing — rust-cache restores `.crates.toml` metadata WITHOUT the binary, so
    plain binstall skips → `cargo crap` dies "no such command" → CI RED every run. Verify from repo
    root: `mise run crap` (exit 0, "97 functions; none exceed 30") + enforcing gate
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`
    (pass=`0 regressed/97 unchanged` exit 0; inflate baseline coverage → `N regressed` exit 1 — do
    NOT pipe to tail, masks `$?`) + `mise run crap:baseline` regenerates `.crap-baseline.json`
    byte-identical (97 entries, 10 iscc-lib files, COMMITTED, not gitignored). KEY GOTCHA:
    `.cargo-crap.toml` MUST exclude `crates/iscc-lib/benches/**` (built-in `benches/**` matches only
    repo-root) else `bench_cdc_chunks` leaks at CRAP 42. CAVEAT (Codex): `--fail-regression` exits 0
    for NEW high-CRAP funcs — filed [review] issue to add `--fail-above 30`
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

## C++ Wrapper Review

- C++ wrapper in `packages/cpp/` — header-only, no Rust crate. CMake + INTERFACE library
- Review shortcut: `cargo build -p iscc-ffi` + CMake configure/build/test + ASAN rebuild + clippy +
    `mise run check`
- CI `cpp` job: cmake + ASAN + test on ubuntu-latest
- `iscc.hpp` bundled in FFI release tarballs (flat, alongside `iscc.h`); pkg mgrs vcpkg/conan in
    `packages/cpp/`
- **C++ cmake build**: use `cmake -B build -DFFI_LIB_DIR=../../target/debug` from `packages/cpp/`

## Environment

- Python `iscc_lib`: compile with `cd crates/iscc-py && uv run maturin develop --release`
- `.pyi` stub sync: `ty check` catches mismatches, `mise run check` does not
- **Pre-push needs iscc_lib built**: `ty check` and `pytest` hooks import `iscc_lib` — build before
    pushing (same maturin command above), else push fails
- PyO3 GIL-release pattern (#39, iter 91): technique archived in `learnings-archive.md`. Post-0.26
    the GIL-release call is `Python::detach` (was `allow_threads`) — grep `detach` to count sites
- **PyO3 minor migration** (0.23→0.29, one minor per CID step; advisories clear only at 0.29): pin
    is root `Cargo.toml` `[workspace.dependencies]`, used by `iscc-py` alone. Python-only review +
    `cargo tree -p iscc-py -i pyo3` for resolved version. 0.23→0.24 AND 0.24→0.25 zero-source;
    0.25→0.26 FIRST hop needing edits: `allow_threads`→`detach` (7 sites) + `PyObject` alias →
    `Py<PyAny>` returns (17 sites); 0.26→0.27 SECOND hop: `to_pylist` cast rename
    `downcast`/`downcast_into_unchecked` → `cast`/`cast_into_unchecked` (mechanical, error type
    discarded). 0.27→0.28 (iter 104) compiled ZERO-edit clean BUT carried a SILENT runtime flip
    `-D warnings` can't catch: PyO3 0.28 changed the unspecified `#[pymodule]` `gil_used` default
    `true`→`false` (macros-backend `module.rs`: 0.27 `map_or(true,…)` → 0.28 `is_some_and(…)`). On
    free-threaded CPython source builds the module then imports WITHOUT re-enabling the GIL — unsafe
    for the raw borrowed `PyList_GetItem` ptrs in `extract_frame_sigs`. Review fixed by adding
    `gil_used = true` to the `#[pymodule]` (no-op on abi3 wheels/GIL builds; restores 0.27
    behavior). LESSON: each pyo3 hop, diff the macros-backend default handling — "compiles clean" ≠
    behavior-neutral. Verify deprecated APIs gone: grep
    `allow_threads`/`downcast`/`PyResult<PyObject>` = 0. `uv run maturin develop` then
    `uv run pytest` (286). NEXT 0.28→0.29 is FINAL (advisories clear, closes issue #1; `cargo audit`
    after). Watch `-D warnings` each hop

## Ruby Binding Review

- Magnus 0.7.1 pinned for Ruby 3.1 compat — 0.8 needs Ruby 3.2+
- `function!` macro does NOT accept `&Ruby` — use `Ruby::get().expect("called from Ruby")`
- `rb-sys` needs Ruby headers + `libclang-dev` — why `--exclude iscc-rb` in CI
- Ruby `JSON.generate` ignores `sort_keys: true` — use `.sort.to_h` before generate
- Streaming classes: `#[magnus::wrap(class = "...")]` + `RefCell<Option<inner>>` for one-shot
    finalize. Linting: Standard Ruby (`standard` gem) + `rubocop-minitest`, config `.standard.yml`
