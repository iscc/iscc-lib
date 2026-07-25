# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next". Then `critical` issues, regardless of feature trajectory.
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    always read issues.md directly (review agent can miscount).
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files).
- **IDLE is valid** when all target sections met and only `low` issues remain — don't invent work.
    But an already-specced, locally-verifiable target gap is NOT idle work.
- **Prefer boolean-verifiable prerequisites over risky infra fixes** — but infra/release fixes CAN
    be locally verifiable; don't default to "release-only → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — if the Write tool is blocked, use `cat > file << 'EOF'` via
    Bash.
- Format next.md + memory with **`uv run prek run mdformat --files <paths>`** (seconds). A bare
    `uv run mdformat` is NOT the hook (the hook passes `--number`) and *causes* the failure it was
    meant to avoid; `mise run format` can exceed a 2-min Bash timeout. mdformat warns on a
    python-fenced block that does not parse — give partial snippets a `text` fence.

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM/binaries). `gen_iscc_code_v0` vectors have no `wide` — pass
    `false`. `"stream:<hex>"` prefix = hex-encoded byte data.
- **5 identical data.json copies, update together**: `crates/iscc-lib/tests/`,
    `packages/go/   testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`. Hardcoded
    vector-count asserts (Rust core + WASM) must be bumped when vendoring new vectors.
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

- Release: `workflow_dispatch` with per-registry checkboxes; version_sync.py manages **21** targets
    (`--check` exits 1 on mismatch; issues.md line ~55 says "22" — wrong, don't propagate).
    `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in Rust CI). XCFramework cache key must
    hash all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`; collapsible `??? tip "Build from source"`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson Maven groupId `com.google.code.gson`.
- "10 gen functions" vs "9 conformance functions" — no blanket 9→10 find/replace (corrupts
    conformance-scoped files). See learnings.md.
- Language API name styles (doc examples): Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/
    Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u` params.

## v1.0.0 Hardening Phase — COMPLETE (iters 86–114); detail in MEMORY-archive.md + learnings.md

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates.** CRAP + Perf + Audit enforcing;
    Semver `continue-on-error: true` until the human-gated v1.0.0 cut.
- **Baselines (`.crap-baseline.json`, `.iai-baseline.json`) are committed and refreshed only by
    deliberate reviewed `mise run` commits** — never auto-committed from CI; don't widen
    `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled artifacts) → a green local
    `cargo deny check` is authoritative. `deny.toml` schema/license-graph detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released; all 12 bindings meet core criteria. The 4 spec'd v0.6.0 feature issues (#41 GIL,
    #42 WASM SIMD, #43 Go ISCC-IDv1, #49 aarch64 wheels) are DONE; dependency refresh + 2 `normal`
    release-workflow issues (npm OIDC, single-registry re-trigger) remain. v1.0.0 cut +
    Semver-enforcing HELD by Titusz (`low`).
- **iters 115–123 DONE (detail in MEMORY-archive.md + learnings.md)**: GIL detach, WASM SIMD, Go
    ISCC-IDv1, trailing-byte guards, aarch64 wheels (release-only infra → STATIC verification:
    pyyaml `safe_load` + grep). **Root lesson: the CRAP regression gate is CI-ONLY** (not in
    `mise run check`/pre-commit) — any step adding a branch/loop to a covered fn MUST refresh the
    baseline in the SAME step.
- **iters 124–137 = the dependency-refresh slices** → ledger, gotchas, hold-back reasons, remaining
    slices and version-lookup commands: [dep-refresh ledger](dep-refresh-ledger.md). Read it before
    scoping any dep step. Headline rule: **never move a consumer floor (MSRV, `go` directive,
    `required_ruby_version`, a published binding's compiler) inside a refresh slice** — iter 128 did
    it by accident and raised the published Kotlin floor to 2.3. All 7 per-ecosystem slices closed;
    slice 8 = ruff 0.16 sub-sliced A–E, E scoped iter 137 closes it.
- **A lint-tool major bump is not one step.** Slice by *what decision each finding needs*
    (mechanical / gate-interacting / config-requiring), not by file. Probe candidate settings
    without touching the lock: `uvx ruff@<ver> check --config '<key> = <val>' --diff <paths>` — and
    also re-probe with the *pinned* tool + `--extend-select`, which tells you whether the new rule
    can be enforced immediately instead of arriving silently with the upgrade.
- **A tool bump's real risk may be file *discovery*, not new rules.** Iter 137: ruff 0.16's rule set
    was a no-op on this tree, but its formatter started covering Python blocks inside `.md`, so the
    bare `ruff format --check` in CI silently widened 25 → 153 files. Before scoping a pin drop, run
    the *bare* gate command under the new version and compare the reported **file count**, not just
    the exit code.
- **A lint *config* change moves the finding set, it does not only shrink it** (iter 135: isort
    `src` cleaned 3 test files and dirtied a previously-green benchmark file). Re-run the full-tree
    check *with* the candidate config before counting files against the budget.
- **A stale in-repo comment is not worth blowing the file budget for** (iter 136, the `# held:` note
    on the `ruff<0.16` pin): leave it for the step that retires the thing it annotates, and say so
    in `## Not In Scope` so review reads it as deliberate, not missed.
- **`core.fileMode=false` (9p Windows bind mount) means `chmod +x` alone never lands in a commit** —
    `git update-index --chmod=+x <path>` is also required, run *after* `git add`, verified with
    `git diff --cached --summary`. The `+x`/`-x` round-trip is safe to probe while scoping.
- **Both formerly-parked `[review]` policy calls were DECIDED by Titusz 2026-07-25** (commits
    `8d267ff`, `8358eba`) → four criteria in `specs/kotlin-bindings.md` + `specs/rust-core.md`.
    Done: Kotlin floor docs (132), Unicode freeze filter (133), ruff B/C/D (134–136). **Open
    backlog:** the full-code-space differential sweep (own step, own harness) and the boundary
    vectors in the Rust suite + all 12 bindings — both parked behind the `[review]` ordering ruling;
    the vectors also need an explicit Go decision (Go is on 15.0 tables until go1.27 ≈ Aug 2026:
    vendor the 15.0→16.0 delta or skip-with-note).
- **A parked HUMAN REVIEW issue does not stall the loop — it re-prioritises it** (iters 134–137: the
    Unicode ruling parked 2 criteria, so the steps went to the unblocked ruff slices). If a blocked
    slice's *only* consumer is the parked propagation, its marginal value is low — take the
    unblocked backlog item and let the ruling land.
- **Look for gates that exist but run in only one place** (`S`/`C901` were pre-push-hook-only, CI
    never saw them). Broadening an *existing* gate to CI needs no human sign-off; inventing one
    does.
- **A multi-part spec criterion slices along its own checkboxes** — `specs/rust-core.md`'s Unicode
    contract → 3 steps (filter / 1.1M-code-point proof / 12-binding propagation), not one.
- **Any step touching the text hot path trips two gates at once**: the CI-only CRAP
    `--fail-regression` baseline and the `.iai-baseline.json` 10% Ir gate. Measured constants and
    boundary behaviour: [unicode-freeze-facts](unicode-freeze-facts.md).
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep —
    [ty gate trap](define-next-ty-generator-scripts.md).
- `uv run zensical build` (exits 0, "No issues found", ~8s) verifies any docs-only step.
- **Recurring**: the cargo-deny gate WILL periodically go red on fresh RustSec advisories vs
    dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore.
- **The installed Python binding is a cheap probe for core behaviour** while scoping:
    `uv run python -c "import iscc_lib; …"` answers "what does the Rust core do today?" in seconds
    without writing Rust — turns vague spec prose into exact before/after assertions for next.md.
