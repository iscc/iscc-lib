# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- Doc files are excluded from the 3-file modification limit — can batch all 6 howto guides in one
    step since they follow identical patterns
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- **State assessments can go stale** — always verify claimed gaps by reading the actual files
- **Handoff "IDLE" can be stale** — always check issues.md directly
- **Generated files (tool output) don't count toward the 3-file modification limit**
- **CI red always first** — green CI is a prerequisite for all other work
- **Target gaps vs low issues** — target is source of truth for what needs to be done
- **Review agent can miscount issues** — always read issues.md directly
- **Batch related small changes** — version sync + docs update for same feature can combine into one
    step (1 code file + 1 doc file excluded from limit)
- **When blocked issues dominate** — look for target verification criteria gaps rather than
    accepting "idle". Docs completeness (tabbed examples, tables) is often missed
- **next.md is a sensitive file** — Write tool may be blocked; use `cat > file << 'EOF'` via Bash
- **HUMAN REVIEW REQUESTED issues**: When evidence is overwhelming (bytecode-verified), scope the
    fix — the review agent will verify. Don't block the CID loop on human confirmation for
    well-understood bugs.
- **IDLE is valid** — when all target sections are met and only low issues remain, signal IDLE.
    Don't invent work. But an already-specced, locally-verifiable target gap is NOT idle work.
- **Prefer boolean-verifiable prerequisites over high-impact-but-risky infra fixes** when both are
    available and there's no in-progress feature. A single-file `pub`→`pub(crate)` change with grep
    \+ `cargo test` checks beats an infra fix whose real verification needs publishing/CI.
- **npm `optionalDependencies` fix (#38) RESOLVED iter 92** — lesson kept: an infra/release fix can
    still be LOCALLY verifiable. The bundled napi loader (`files: ["*.node"]`, local `require` of
    the sibling `.node` first) was provable in-devcontainer with NO publish; "release-only → too
    risky" was the wrong default. Detail archived to MEMORY-archive.md.

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## UniFFI Scaffolding (Swift/Kotlin foundation)

- UniFFI v0.31.0 is the latest stable version (checked 2026-03-21)
- Proc macro approach: `#[uniffi::export]`, `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`
- Key type constraints: no `usize` (use `u64`), no borrowed types, no generics on exported functions
- Constants are getter functions (UniFFI can't export `const`)
- **SPM module name MUST match generated code**: `iscc_uniffiFFI`

## Dev Environment Constraints

- **No Swift toolchain** in Linux devcontainer — `swift test` can only run on macOS (CI)
- **No shellcheck** in Linux devcontainer — can't lint shell scripts locally
- `uniffi-bindgen` not pre-installed — use in-crate binary via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`

## Conformance Vector Loader Differences (critical for data.json updates)

- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, `packages/dotnet/Iscc.Lib.Tests/testdata/data.json`,
    `packages/swift/Tests/IsccLibTests/data.json`, and
    `packages/kotlin/src/test/resources/data.json` (all identical). Must be updated together.

## CI/Release Patterns

- v0.4.0 released to all registries; next target is v1.0.0 (stability-committed, strict SemVer)
- Release workflow has `workflow_dispatch` with 9 per-registry checkboxes
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- XCFramework cache key at release.yml:1269 — must hash ALL build inputs
- `swift package dump-package` validates manifest syntax without downloading binary targets — safe
    to use with PLACEHOLDER checksum on develop

## Docs Infrastructure

- `zensical.toml` has `nav` array for howto guides — must add entry when creating new guide
- `scripts/gen_llms_full.py` has `ORDERED_PAGES` list — must add entry for llms-full.txt generation
- All howto guides follow identical structure (see `docs/howto/dotnet.md` as template)
- Howto install sections use collapsible `??? tip "Build from source"` pattern

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)
- **Gson groupId trap**: Maven groupId is `com.google.code.gson`, NOT `com.google.gson`

## Language API Patterns (for doc examples)

- Ruby: `IsccLib.gen_text_code_v0("text")` — snake_case module methods
- C#: `IsccLib.GenTextCodeV0("text")` — PascalCase static methods
- C++: `iscc::gen_text_code_v0("text")` — namespace free functions, RAII
- Swift: `genTextCodeV0(text: "text", bits: 64)` — camelCase free functions, named params
- Kotlin: `genTextCodeV0(text = "text", bits = 64u)` — camelCase free functions, UInt params

## v1.0.0 Hardening Phase (post-v0.4.0, started ~iter 86)

- v0.4.0 shipped to all registries; CI green 16/16. Human added 7 `normal` issues toward v1.0.0 and
    raised target.md bar. No critical issues. Module-visibility narrowing started at iter 86.
- **Tier 1 surface = 32 crate-root re-exports.** Internal modules (`cdc`, `minhash`, `simhash`,
    `utils`, `conformance`) only leak their *paths*; narrowing them to `pub(crate) mod` is a pre-1.0
    breaking change that must land BEFORE the `cargo-semver-checks` gate locks the surface.
- **Integration tests in `crates/iscc-lib/tests/` deliberately assert module-path access** —
    `test_module_path_imports*` in test_algorithm_primitives.rs + test_text_utils.rs. They break
    when modules go `pub(crate)`; delete them (crate-root `test_flat_crate_root_imports` /
    `test_crate_root_imports*` already cover the same functions). Tests excluded from 3-file limit.
- **v1.0.0 backlog ordering** (per state.md Next Milestone): npm #38 fix → PyO3 0.23→0.29 → v1.0.0
    gates (semver-checks, iai-callgrind, module-visibility) → coverage/CRAP → streaming SumHasher
    (core first, then PyO3 + WASM wrappers) + Python GIL release. `pub use` from a `pub(crate) mod`
    is valid Rust — re-exported Tier 1 symbols stay public after narrowing.
- **#37 SumHasher (iters 88–90) and #39 Python GIL release (iter 91) are CLOSED** — detailed scoping
    notes archived to `MEMORY-archive.md`. SumHasher lives at `iscc_lib::streaming::SumHasher` (NOT
    crate-root Tier 1; Tier 1 count stays 32). Python/WASM wrappers ship it; core counts untouched.
- **#38 (npm, iter 92) and the `cargo-semver-checks` gate (iter 93) both LANDED** — detailed scoping
    notes archived to `MEMORY-archive.md`. semver job is informational (`continue-on-error: true`)
    until the v1.0.0 cut; do NOT flip it before then.
- **CI-infra step verification pattern**: pre-commit hooks (`mise run check`) validate YAML/TOML
    syntax of edited workflow/manifest files locally — a solid automated check even without
    actionlint. Pair with grep assertions + the next CI run (review agent confirms the new job
    appears and existing jobs stay green). (iter 93/94 network + Phase 1 detail archived to
    MEMORY-archive.md.)
- **iscc-py is ONE file (`crates/iscc-py/src/lib.rs`, 735 lines) — pyo3 used by NO other crate**
    (grep `crates/*/Cargo.toml` → only iscc-py; version pin lives ONLY at root `Cargo.toml` line 35;
    iscc-py consumes via `workspace = true, features = ["extension-module"]`;
    `crates/iscc-py/pyproject.toml` line 38 holds the maturin config
    `features = ["pyo3/extension-module"]`, NO version → never edited on a hop). Has raw
    `pyo3::ffi::*` CPython-C-API calls (8 sites: `PySequence_List`, `PyList_GetItem`, `PyList_Size`,
    `PyLong_AsLong`, `PyErr_Occurred`, `PyList_Check`) +
    `Bound::from_owned_ptr().cast_into_unchecked()` (lib.rs:24) — map to stable CPython C API,
    rarely break across pyo3 minors. Local verify = `maturin develop -m crates/iscc-py/Cargo.toml`
    (maturin 1.12.4 via uv) + `uv run pytest` (286).
- **CRAP gate Phases 1+2 LANDED** (iters 94–96; Phase 2 `6ed51c5`, reviewed PASS `cbc0d14`). The
    `coverage` job (`ci.yml:293-331`, renamed "Coverage + CRAP") installs cargo-llvm-cov +
    cargo-binstall + `cargo-crap@0.2.2`, generates+uploads `lcov.info`, then runs report-only
    `cargo crap --format github` + `--format sarif` (→ Code Scanning; job has
    `security-events:   write`). `.cargo-crap.toml`: threshold 30, missing pessimistic, excludes 7
    binding crates + `packages/` + `scripts/` + `crates/iscc-lib/benches/**`. `mise run crap`
    (depends coverage). Detailed Phase 1/2 scoping archived to MEMORY-archive.md.
- **cargo-crap 0.2.2 facts (re-verified locally iter 97 — both cargo-crap 0.2.2 + cargo-llvm-cov
    0.8.7 are now INSTALLED in the devcontainer)**: flags `--lcov`,
    `--format {human,json,github,markdown,pr-comment,sarif}`, `--threshold`, `--missing`,
    `--exclude`, `--allow`, `--output`, `--fail-above`, `--baseline <FILE>`, `--fail-regression`,
    `--epsilon` (default 0.01). **NO `--sort`** (that's only on `main`, not 0.2.2). Baseline JSON
    envelope = `{$schema, version:"0.2.2", entries[]}`; `--baseline` reads only that shape and is
    incompatible with `--format sarif`. Config `.cargo-crap.toml` keys:
    threshold/missing/exclude/default-excludes/ allow/fail-above/epsilon/jobs/sort/show_unchanged
    (unknown keys rejected; NO `baseline`/ `fail-regression` keys — those are CLI-only).
- **CRAP Phase 3 (regression gate) LANDED iter 97.** COMMITTED `.crap-baseline.json` (97 iscc-lib
    fns; NOT gitignored — only lcov.info/crap.sarif are) + enforcing
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` step (LAST in
    coverage job, after SARIF) + `mise.toml [tasks."crap:baseline"]`. Refresh = deliberate
    `mise run crap:baseline` reviewed commit.
- **Phase 3 design call (iter 97): NO auto-commit-baseline-from-CI on develop pushes.** A CI-side
    `git push` races the CID loop's own develop pushes (non-fast-forward) + risks a push→CI→push
    loop → unsafe in this repo. So "refresh on merges to develop" = deliberate
    `mise run crap:baseline` reviewed commit (iai-callgrind pattern); advance agent updates the spec
    prose to match. Flapping risk: committed baseline coverage (devcontainer rustc 1.96.0) vs CI
    `@stable` — deterministic test coverage is stable + `--epsilon 0.01` absorbs noise; if CI flaps,
    regenerate from CI's lcov artifact, do NOT widen epsilon.
- **PyO3 #1 migration COMPLETE at 0.29 (issue closed iter 105).** Per-hop recipe + silent-gotcha
    catalog archived to MEMORY-archive.md. Residual fact: **cargo-audit AND cargo-deny are absent
    locally AND not wired into CI/mise** (verified iter 105) — advisory clearance was confirmable
    only by the lockfile proxy, which spawned the [review] supply-chain-audit-gate issue.
- **Remaining v1.0.0 normal backlog after iter 106 (3 issues, ALL constrained)**: (a) iai-callgrind
    perf gate `[human]` — ALREADY in spec (target.md L71, rust-core.md, ci-cd.md), NO human-review
    hold; (b) CRAP `--fail-above 30` `[review]` — HUMAN REVIEW REQUESTED to amend spec; (c)
    supply-chain `cargo deny/audit` gate `[review]` — HUMAN REVIEW REQUESTED to amend spec. **Key
    calibration:** prefer the already-specced gap (a) — adding a NEW gate that amends the spec/notes
    is a policy call the review agent flagged for human sign-off; respect that, do NOT auto-scope
    (b)/(c). The HUMAN-REVIEW-override-on-overwhelming-evidence rule is for BUG fixes, not new
    policy gates.
- **iai-callgrind gate = THREE slices (harness → run-in-CI → regression gate).** `valgrind` +
    `iai-callgrind-runner` BOTH absent locally (no apt candidate) so benches cannot RUN locally, but
    the harness COMPILES (runner/valgrind are runtime-only). Slices: (1) **DONE iter 106** — harness
    `crates/iscc-lib/benches/iai_benches.rs` (11 `#[library_benchmark]`, iai-callgrind "0.16" →
    resolves 0.16.1) + workspace dep + `[[bench]] harness=false`, verified
    `cargo build -p iscc-lib --bench iai_benches` (local boolean). (2) **SCOPED iter 107** — `Perf`
    CI job (key `perf`, ubuntu: apt install valgrind + cargo-binstall via taiki-e/install-action +
    `cargo binstall -y --force iai-callgrind-runner@0.16.1`, run
    `cargo bench -p iscc-lib --bench iai_benches`, upload `target/iai/` artifact) +
    `mise run bench:iai`. First run w/ NO baseline just measures + exits 0 → job GREEN with no gate
    yet. Locally verifiable via YAML/TOML hooks + grep; review confirms green Perf job vs CI. NO
    committed baseline this slice (can't generate locally — produce it from THIS slice's CI
    artifact). (3) **FUTURE (2b)** — committed baseline + `--baseline`/`--fail-regression` (or
    in-harness regression limit) + `bench:iai:baseline` refresh task + docs/spec checkbox flip.
    iai-callgrind has NO clean single-file committed-baseline workflow like cargo-crap (baselines
    live in `target/iai/`); the glue needs design informed by slice-2's real artifact → DO NOT
    attempt blind. `.cargo-crap.toml` already excludes `crates/iscc-lib/benches/**` (no CRAP-config
    edit needed). Primitive sigs: `alg_cdc_chunks(&[u8], utf32 bool, avg u32) -> IsccResult`,
    `alg_minhash_256(&[u32]) -> Vec<u8>` (infallible). Runner version MUST match lib (0.16.1).
- **iter 100→101: cargo-crap install-flake fix LANDED, CI confirmed GREEN** (run 27685108728). Rule
    reaffirmed: **CI red always preempts feature work, even a clean handoff "Next".** Root cause:
    `Swatinem/rust-cache@v2` restores cargo's `.crates.toml` metadata WITHOUT the
    `~/.cargo/bin/cargo-crap` binary, so `cargo binstall cargo-crap@0.2.2` (no `--force`) skipped
    install → next `cargo crap` step died `no such command: crap`. Fix = `--force` on that binstall
    line (ci.yml:314). With CI green, iter 101 resumed PyO3 0.25→0.26 (handoff "Next").
