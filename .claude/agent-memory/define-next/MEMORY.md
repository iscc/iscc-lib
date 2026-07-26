# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next". Then `critical` issues, regardless of feature trajectory.
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    always read issues.md directly (review agent can miscount). **A `human(...)` commit after the
    last review invalidates the handoff wholesale** (iter 147: handoff said "write `## Step: NONE`",
    but the intervening ruling had filed a `critical` bug and cleared the backlog). Check
    `git log --oneline` for `human(` commits newer than the review commit before trusting "Next".
- **A human issue's suggested fix is a hypothesis, not a spec — probe it** (iter 147: the issue said
    hoist a package-level `cases.Caser`; x/text documents `Caser` as *not* goroutine-safe and the
    hoist measured no faster). Cheap throwaway probes (a `zz_probe_test.go` you delete, a `/tmp`
    module) settle these in a minute and turn `## Implementation Notes` into measured facts.
- **A crashed review role means the previous step has NO verdict** (`iterations.jsonl` FAIL/turns:1
    with no `cid(review)` commit) — handoff.md then holds only the advance section and resolved
    issues were never deleted. Re-verify the prior step's claims from the tree.
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files). **IDLE
    is valid** when all target sections are met and only `low` issues remain — but an
    already-specced, locally-verifiable target gap is NOT idle work, and infra/release fixes CAN be
    locally verifiable; don't default to "release-only → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — Write needs a prior Read of it; if Write is blocked, use
    `cat > file << 'EOF'` via Bash.
- **Escape sequences in a tool payload may get decoded before they hit the file** (observed iter
    149, NOT reproduced iter 150 \\u2014 transport-dependent): `\u0378` can land as the *character*,
    corrupting a snippet next.md must hand advance verbatim. Safest is `U+0378` prose notation;
    otherwise verify with `python3 ... .isascii()` / `grep -c 'u0378' <file>` after every write.

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
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`. UniFFI
    0.31.0; SPM module name MUST be `iscc_uniffiFFI`; no `const`/`usize`/borrowed/generic exports.
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

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates** (CRAP/Perf/Audit enforcing,
    Semver informational until the human-gated v1.0.0). **Baselines are committed and refreshed only
    by deliberate reviewed `mise run` commits** — never from CI; don't widen `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled artifacts) → a green local
    `cargo deny check` is authoritative. `deny.toml` detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released; all 12 bindings meet core criteria; the 4 spec'd v0.6.0 feature issues are DONE.
    Still open: the Unicode chain (below), the authorized major bumps (one per step), the
    `rubygems/configure-rubygems-credentials@v2.1.0` pin, and an exhaustive `specs/ci-cd.md` job
    table. HELD `low` by Titusz: v1.0.0 + Semver-enforcing, npm OIDC (token good to 2026-09-16).
- **iters 115–123 DONE (detail in MEMORY-archive.md)**. **Root lesson: the CRAP regression gate is
    CI-ONLY** — a step adding a branch to a covered fn MUST refresh the baseline in it.
- **iters 124–137 = the dependency-refresh slices, all 8 now CLOSED** → ledger, gotchas, hold-backs,
    version-lookup commands: [dep-refresh ledger](dep-refresh-ledger.md); read it before scoping any
    dep step. Headline rule: **never move a consumer floor (MSRV, `go` directive,
    `required_ruby_version`, a published binding's compiler) inside a refresh slice** (iter 128).
- **Lint/formatter tool bumps and hook-config changes have their own playbook** — slicing rules,
    `uvx ruff@<ver> --config` and `prek run -c /tmp/probe.yaml` probing recipes, and the gate-parity
    warnings: [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact
    file/finding count a pass/fail criterion** (it drifts with the CID agents' own commits — iter
    139 predicted 153, review measured 155); make the exit code the criterion.
- **Open Unicode backlog** (`human(decide)` `9aa25ad`; `specs/rust-core.md` is the authority): (1)
    Go `Final_Sigma` ✅147, (2) sentinel conversion ✅148, (3) the four **sequence** vectors + fixture
    guard ✅149, (4) fixture propagation into the 11 bindings — **slice 1 (Python + Go) scoped 150**,
    (5) the 1,112,064-scalar **and sequence-class** differential sweep as a runnable check. Never
    implement the superseded category override (`U+A7F1` injects a spurious `S`) or a 15.1.0
    declared version. Escapes, fixture facts, CRAP-in-tests →
    [unicode-freeze-facts](unicode-freeze-facts.md).
- **Before scoping any propagation slice** read the
    [propagation ledger](unicode-fixture-propagation.md) — loader taxonomy (which bindings read the
    canonical path vs. need a vendored `cp`), which binding artifacts are stale, Go's measured 9/12
    with its **per-case** (not per-code-point) skip list, and the planned slice order.
- **Gate/checker steps in `scripts/` have their own playbook** — prek-vs-CI placement, the Python
    3.10 floor (no `tomllib`), injected-`Path` shape, network/offline probing, docs-list wiring, and
    when a gate change needs Titusz: [gate scripts playbook](gate-scripts-playbook.md). Read it
    before scoping anything under `scripts/` or `.pre-commit-config.yaml`.
- **In a file no CI push exercises, split behavioural edits from mechanical ones** (iters 139/140:
    `release.yml`'s guard fixes vs the 97-ref `uses:` bump, so a broken release is bisectable).
    Recipe, job inventory, evidence rules for un-runnable workflow bumps (check `ci.yml` first;
    verify each floating tag via `gh api repos/<r>/git/ref/tags/<vN>`):
    [release.yml static gates](release-yml-static-gates.md).
- **Any step touching the text hot path trips two gates at once**: the CI-only CRAP
    `--fail-regression` baseline and the `.iai-baseline.json` 10% Ir gate. A `tests/`-only step
    usually trips **neither** — say so in next.md so advance doesn't refresh a baseline. Exception +
    measured constants + the Unicode-16 derivation recipe:
    [unicode-freeze-facts](unicode-freeze-facts.md).
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep —
    [ty gate trap](define-next-ty-generator-scripts.md).
- `uv run zensical build` (exits 0, "No issues found", ~8s) verifies any docs-only step.
- **Recurring**: the cargo-deny gate WILL periodically go red on fresh RustSec advisories vs
    dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore.
- **Watch the tooling-cadence flag in state.md.** With 3 of the last 4 iterations CI/lint/ workflow,
    prefer a user-facing item over a tracked `normal` tooling issue (iter 143) — but deferring is a
    **one-iteration** move, not a veto: count the window in `## Goal`. At the threshold with zero
    unblocked user-facing candidates the rule does not fire (iter 146) — then enumerate each blocked
    candidate + its blocker.
- Parked-work scoping lessons → `MEMORY-archive.md`; nothing is parked on Titusz today.
- **Binding artifacts are cheap probes, but each has its own age** — the Python editable install was
    CURRENT at iter 150 while the checked-in napi `.node` was months stale. Probe a *discriminating*
    input before trusting one (details + freshness table →
    [propagation ledger](unicode-fixture-propagation.md)); cross-check with
    `cargo test -p iscc-lib --lib <mod>::` (~seconds when built).
- **Probing a foreign binding without touching the repo**: a throwaway module in `/tmp` with a
    `replace` / path dependency back to the package (used at iter 150 to measure Go's 9/12 on the
    boundary fixture) turns `## Implementation Notes` into measured facts and leaves no tree diff.
