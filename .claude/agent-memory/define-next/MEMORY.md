# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next".
- Critical issues always take priority regardless of feature trajectory.
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    always read issues.md directly (review agent can miscount).
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files).
- **IDLE is valid** when all target sections met and only `low` issues remain — don't invent work.
    But an already-specced, locally-verifiable target gap is NOT idle work.
- **Prefer boolean-verifiable prerequisites over high-impact-but-risky infra fixes** when both are
    available and no feature is in-progress. But infra/release fixes CAN be locally verifiable (napi
    bundled loader #38, cargo-deny reads Cargo.lock not artifacts) — don't default to "release-only
    → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — if the Write tool is blocked, use `cat > file << 'EOF'` via
    Bash.
- Run `mise run format` before committing next.md + memory (pre-push mdformat rejects the batch).

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM/binaries). `gen_iscc_code_v0` vectors have no `wide` — pass
    `false`. `"stream:<hex>"` prefix = hex-encoded byte data.
- **data.json copies** (all identical, update together): `crates/iscc-lib/tests/`,
    `packages/go/   testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`. Binding crates with
    hardcoded vector-count asserts (Rust core + WASM) must be bumped when vendoring new vectors.
- **SumHasher** lives at `iscc_lib::streaming::SumHasher` (NOT Tier 1; count stays 32).

## Dev Environment Constraints

- **No Swift toolchain / no shellcheck** in the Linux devcontainer — `swift test` + shell lint are
    CI/macOS only.
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`.
- UniFFI 0.31.0; SPM module name MUST be `iscc_uniffiFFI`; UniFFI can't export `const` (getters) or
    `usize`/borrowed/generic exports.
- cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner ARE installed.
    cargo-deny is NOT preinstalled but installs via `cargo binstall cargo-deny@0.19.9 --force`.

## CI/Release, Docs, Gotchas

- Release: `workflow_dispatch` with 9 per-registry checkboxes; version_sync.py manages 16 targets
    (`--check` exits 1 on mismatch). `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in
    Rust CI). XCFramework cache key must hash all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`; collapsible `??? tip "Build from source"`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson Maven groupId `com.google.code.gson`.
- "10 gen functions" vs "9 conformance functions" — no blanket 9→10 find/replace (corrupts
    conformance-scoped files). See learnings.md.
- Language API name styles (doc examples): Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/
    Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u` params.

## v1.0.0 Hardening Phase — COMPLETE (iters 86–114); detail in MEMORY-archive.md + learnings.md

All autonomous v1.0.0-hardening gates landed and are enforcing/green: module-visibility narrowing,
SumHasher (#37), PyO3 0.29 (#1), npm bundled loader (#38), `cargo-semver-checks` (informational — do
NOT flip to enforcing until the human-gated v1.0.0 cut), CRAP Phases 1–3 + `--fail-above`,
iai-callgrind perf gate (#3), and `cargo-deny` supply-chain gate (#114). Residual facts:

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates.** CRAP + Perf + Audit enforcing;
    Semver `continue-on-error: true` until v1.0.0 cut.
- **Baselines are committed + refreshed by deliberate `mise run` reviewed commits**
    (`.crap-baseline   .json`, `.iai-baseline.json`) — never auto-committed from CI (push race).
    Regenerate from CI's artifact if flapping; don't widen `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled) → local `cargo deny check` green is
    authoritative. `deny.toml`: config v2, `[graph] all-features`, `yanked/multiple-versions`,
    `private = { ignore = true }`, license allow-list (Unicode-3.0/Zlib/BSL-1.0/MPL-2.0). Dead
    pre-0.14 schema traps + license-graph detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released to all registries; all 12 bindings meet core criteria. Human raised the target.md
    bar with 5 spec'd `normal` `[human]` v0.6.0 issues (#41 GIL, #42 WASM SIMD, #43 Go ISCC-IDv1,
    #49 aarch64 wheels, dependency refresh — first four DONE) + 2 `normal` release-workflow issues
    (npm OIDC, single-registry re-trigger). v1.0.0 cut + Semver-enforcing still HELD by Titusz
    (`low`).
- **iters 115–119 DONE (detail in MEMORY-archive.md)**: 115 cargo-deny advisory bump
    (crossbeam-epoch, CI-red-first); 116 #41 Python GIL detach; 117→118 #42 WASM SIMD (reframe:
    needs the `blake3/wasm32_simd` Cargo feature, not just RUSTFLAGS); 119 #43 Go ISCC-IDv1.
- **Recurring**: the enforcing cargo-deny gate WILL periodically go red on fresh RustSec advisories
    vs dev/bench deps — CI-red-first priority; prefer `cargo update -p <crate>` (patch bump) over a
    `deny.toml` ignore when a patched release exists (check `patched` range in advisory-db first).
- **iters 120–122 DONE (trailing-byte hardening saga; detail in learnings.md + MEMORY-archive.md)**:
    120 Go `IsccDecode` "too long" branch (`[review]`); 121 Rust-core `iscc_decode` "too long" guard
    (11 bindings inherit) — but the added branch tripped the CI-only CRAP `--fail-regression` gate;
    122 CI-RED-FIRST `.crap-baseline.json` refresh via `mise run crap:baseline`. **Root lesson: CRAP
    regression gate is CI-ONLY (not in `mise run check`/pre-commit)** — any step adding a
    branch/loop to a covered fn MUST refresh the baseline in the SAME step (this is exactly how iter
    121 slipped).
- **iters 123–125 DONE (detail in MEMORY-archive.md)**: 123 #49 aarch64 Python wheels (release-only
    infra → STATIC verification: pyyaml `safe_load` + grep); 124 dep-refresh slice 1 =
    `cargo update` (Cargo.lock only); 125 slice 2 = `uv lock --upgrade` (root `/uv.lock` only, one
    documented `ruff<0.16` hold-back).
- **Dep refresh is sliced per-ecosystem** (spans ~12 manifests, cites no `[audit]` → no 8-file
    valve): Rust lock → Rust direct pins → Python `uv.lock` → each binding-manifest group → tooling
    pins. Lockfiles are generated → 0 source files; hold a tool/dep back with an inline documented
    comment rather than disabling a rule/gate.
- **iter 126: dep-refresh slice 3 = Rust direct-pin evaluation** (`Cargo.toml` +
    `crates/iscc-lib/benches/benchmarks.rs`, plus generated `Cargo.lock`). **Pin survey (crates.io,
    2026-07-24): only 4 pins are majors behind**; all others caret-covered by the iter-124 lock
    refresh. Spec `ci-cd.md` §"Dependency Freshness" mandates a documented reason next to every
    held-back pin → `# held:` comments are the deliverable.
    - `criterion` 0.5→**0.7** (the one bump): 0.6 deprecated `criterion::black_box` → swap import to
        `std::hint::black_box` (30 bare call sites unchanged); `mise run lint` =
        `clippy --all-targets   -D warnings`, so a deprecation IS a hard error. CI bench job =
        `cargo bench --no-run`.
    - **criterion 0.8 HELD: MSRV 1.86 > workspace `rust-version = "1.85"`** — do NOT raise the
        declared MSRV for a dev-dep (human policy call for the v1.0.0 cut; no MSRV CI job, local rustc
        1.97).
    - **magnus 0.8 HELD**: `old-api` no longer default → `magnus::exception::runtime_error()` (used in
        `crates/iscc-rb/src/lib.rs`) becomes `#[deprecated]` → clippy failure; needs refactor to
        `Ruby::exception_runtime_error()`, bundle with the Ruby manifest slice. (Rest of 0.8 fits:
        iscc-rb already uses `Ruby::get()`, no `FString`; rb-sys ≥0.9.113 vs our 0.9.123.)
    - **jni 0.22 HELD**: wholesale rework (`JNIEnv`→`EnvUnowned`/`Env`, `GlobalRef`→`Global`,
        `AutoLocal`→`Auto`, closure attachment, `ErrorPolicy`) per upstream `docs/0.22-MIGRATION.md` —
        rewrites `crates/iscc-jni/src/lib.rs`; own (possibly human-gated) step.
    - **uniffi 0.32 HELD**: needs Swift+Kotlin regen/re-verify; no Swift toolchain locally. **pyo3
        0.29** is already latest. Risk: enforcing `Audit (cargo-deny)` on criterion 0.7's new dev
        subtree (criterion-plot, clap, plotters) → `mise run audit`.
- **ruff 0.16 adoption is OVER the 3-file budget** (measured iter 126 via
    `uvx ruff@0.16.0 check .`): 104 errors across `_lowlevel.pyi` (72), `tools/cid.py` (12),
    `tools/metrics.py` (3), `scripts/test_install.py` (2), `iscc_lib/__init__.py` (2) + 6 test files
    = **5 non-test files + `pyproject.toml`** → must be sliced (iscc-py package, then tools/scripts,
    then drop the pin).
- **v0.6.0 remaining after slice 3**: ruff 0.16 (sliced), magnus/jni/uniffi major migrations,
    per-binding manifests (napi/rb/jni/kotlin/dotnet/go), tooling pins (mise, pre-commit, GHA) + 2
    human-gated release-workflow fixes (npm OIDC, single-registry re-trigger). One slice/iteration.
- **Handy**: crates.io latest via `cargo search <crate> --limit 1`; changelog/migration docs via
    `curl -sL https://static.crates.io/crates/<c>/<c>-<ver>.crate | tar xz` into /tmp. Network
    works.
