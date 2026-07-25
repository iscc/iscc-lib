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

All autonomous v1.0.0-hardening gates landed and are enforcing/green (module visibility, SumHasher
#37, PyO3 0.29 #1, npm bundled loader #38, CRAP `--fail-above`, iai-callgrind #3, cargo-deny #114);
`cargo-semver-checks` stays informational until the human-gated v1.0.0 cut. Residual facts:

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates.** CRAP + Perf + Audit enforcing;
    Semver `continue-on-error: true` until v1.0.0 cut.
- **Baselines (`.crap-baseline.json`, `.iai-baseline.json`) are committed and refreshed only by
    deliberate reviewed `mise run` commits** — never auto-committed from CI; don't widen
    `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled artifacts) → a green local
    `cargo deny check` is authoritative. `deny.toml` schema/license-graph detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released to all registries; all 12 bindings meet core criteria. Human raised the target.md
    bar with 5 spec'd `normal` `[human]` v0.6.0 issues (#41 GIL, #42 WASM SIMD, #43 Go ISCC-IDv1,
    #49 aarch64 wheels — all DONE — plus dependency refresh) + 2 `normal` release-workflow issues
    (npm OIDC, single-registry re-trigger). v1.0.0 cut + Semver-enforcing HELD by Titusz (`low`).
- **iters 115–123 DONE (detail in MEMORY-archive.md + learnings.md)**: 115 cargo-deny advisory bump;
    116 #41 Python GIL detach; 117→118 #42 WASM SIMD (reframe: needs the `blake3/wasm32_simd` Cargo
    feature, not just RUSTFLAGS); 119 #43 Go ISCC-IDv1; 120/121 trailing-byte "too long" guards (Go,
    then Rust core); 122 CI-RED-FIRST `.crap-baseline.json` refresh; 123 #49 aarch64 wheels
    (release-only infra → STATIC verification: pyyaml `safe_load` + grep). **Root lesson: the CRAP
    regression gate is CI-ONLY** (not in `mise run check`/pre-commit) — any step adding a
    branch/loop to a covered fn MUST refresh the baseline in the SAME step.
- **iters 124–131 = the dependency-refresh slices** → full ledger, gotchas, hold-back reasons,
    remaining slices and version-lookup commands live in
    [dep-refresh ledger](dep-refresh-ledger.md). Read it before scoping any dep step. Headline rule:
    **never move a consumer floor (MSRV, `go` directive, `required_ruby_version`, a published
    binding's compiler) inside a refresh slice** — iter 128 did it by accident and raised the
    published Kotlin consumer floor to 2.3 (open `[review]` issue, HUMAN REVIEW REQUESTED, CID must
    not decide it). All 7 per-ecosystem slices are closed; **slice 8 = ruff 0.16, itself split into
    3 sub-slices A/B/C by decision content** (see the ledger — B carries a live gate trap).
- **A lint-tool major bump is not one step.** Slice by *what decision each finding needs*
    (mechanical / gate-interacting / config-requiring), not by file. Probe candidate settings
    without touching the lock: `uvx ruff@<ver> check --config '<key> = <val>' --diff <paths>`.
- **Two `normal` `[review]` issues are human-gated (do NOT decide)**: the Kotlin consumer floor
    (iter 128) and the Rust-core Unicode-16/17 divergence from `iscc-core`/Go (iter 129 — the core
    is the outlier; no vendored vector catches it). Both are policy calls; the loop is still not
    idle because refresh slices + ruff 0.16 remain.
- **Recurring**: the enforcing cargo-deny gate WILL periodically go red on fresh RustSec advisories
    vs dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore
    when a patched release exists.
