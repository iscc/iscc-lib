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
- **A human issue's or handoff's suggested fix is a hypothesis, not a spec — probe it** (iter 147:
    the suggested `cases.Caser` hoist was both unsafe and no faster). Cheap throwaway probes settle
    these in a minute and turn `## Implementation Notes` into measured facts.
- **A crashed review role means the previous step has NO verdict** (`iterations.jsonl` FAIL/turns:1
    with no `cid(review)` commit) — handoff.md then holds only the advance section and resolved
    issues were never deleted. Re-verify the prior step's claims from the tree.
- **A `define-next` TIMEOUT leaves an uncommitted `next.md` in the tree** (iter 155): adopt-and-
    verify rather than starting blank, but **re-derive every numeric invariant it asserts** (they
    become hard-coded `EXPECTED_*` in code) — iter 155's prose contradicted its own snippet. A
    timeout is **not** a bounce: no advance/review ran, so say so in `## Goal`.
- **Run the expensive probe while scoping when it can invert the plan.** The full-code-space
    differential sweep (~60 s) inverted the milestone order at 155 and, re-run at 157, turned every
    constant in the gate spec into a measured fact.
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files). **IDLE
    is valid** when all target sections are met and only `low` issues remain — but an
    already-specced, locally-verifiable target gap is NOT idle work, and infra/release fixes CAN be
    locally verifiable; don't default to "release-only → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — Write needs a prior Read of it; if Write is blocked, use
    `cat > file << 'EOF'` via Bash (quoted EOF = no expansion, so backticks are safe).
- **Escape sequences in a tool payload may get decoded before they hit the file** (iter 149;
    transport-dependent). Use `U+XXXX` prose notation, build probe strings with `chr(0x...)` rather
    than backslash-u literals, and after every write print the file's non-ASCII character set.

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM). `gen_iscc_code_v0` vectors have no `wide` — pass `false`;
    `"stream:<hex>"` prefix = hex-encoded byte data.
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
    (issues.md line ~55 says "22" — wrong). `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb`
    in Rust CI). XCFramework cache key must hash all build inputs.
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
    Semver informational until the human-gated v1.0.0). **Baselines are refreshed only by deliberate
    reviewed `mise run` commits** — never from CI; don't widen `--epsilon`. `cargo deny check` reads
    Cargo.lock + metadata, so a green local run is authoritative (learnings.md).

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
- **Lint/formatter tool bumps and hook-config changes have their own playbook** (slicing rules,
    probing recipes, gate-parity warnings): [lint tooling lessons](lint-tooling-lessons.md).
    Headline: **never make an exact file/finding count a pass/fail criterion** — it drifts with the
    CID agents' own commits; make the exit code the criterion.
- **Open Unicode backlog** (`human(decide)` `9aa25ad`; `specs/rust-core.md` is the authority): (1)
    Go `Final_Sigma` ✅147, (2) sentinel conversion ✅148, (3) sequence vectors + fixture guard ✅149,
    (4) fixture propagation into the 11 bindings — ✅150 Python+Go, ✅151 WASM+Ruby, ✅152 drift gate,
    ✅153 napi+Java, ✅154 C#+Kotlin, **159 scoped: C FFI** via a generated header (→ 9 of 11; only
    Swift/C++ left, both CI-proof-only here), (4b) ✅156 `Final_Sigma` case-table freeze at 16.0.0,
    (5) ✅157 the criterion-4 sweep gate (`scripts/unicode_sweep.py` + `mise run unicode:sweep` + own
    3.14 CI job) — measured green while scoping at 157 **and 158**, 17,793,024 comparisons / 0
    divergences / ~60 s, (6) ✅158 hardened that gate (`--rebuilt` flag so a bare run can't
    false-green off a stale `.so`; divergence retention bounded to 20 samples with an exact count;
    no `default=0.0` vacuous freshness pass). Never implement the superseded category override
    (`U+A7F1` injects a spurious `S`) or a 15.1.0 declared version. Sweep constants, escapes,
    fixture facts, the `Final_Sigma`/U+0295 table shapes, CRAP-in-tests →
    [unicode-freeze-facts](unicode-freeze-facts.md).
- **Before scoping any propagation slice** read the
    [propagation ledger](unicode-fixture-propagation.md) — loader taxonomy, the **two-axis** cost
    rule (plumbing × text coverage; a one-axis ranking wrongly put C FFI first three times),
    artifact staleness, Go's 9/12 per-case skip list, and the slice order.
- **Gate/checker steps in `scripts/` have their own playbook** (prek-vs-CI placement, Python 3.10
    floor, injected-`Path` shape, docs-list wiring, when a gate needs Titusz):
    [gate scripts playbook](gate-scripts-playbook.md). Read before scoping under `scripts/`.
- **In a file no CI push exercises, split behavioural edits from mechanical ones** (iters 139/140:
    `release.yml` guard fixes vs the 97-ref `uses:` bump, so a broken release is bisectable). Recipe
    - evidence rules: [release.yml static gates](release-yml-static-gates.md).
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
- **Watch the tooling-cadence flag in state.md.** With 3 of the last 4 iterations CI/lint/workflow,
    prefer a user-facing item over a tracked `normal` tooling issue (iter 143) — a **one-iteration**
    deferral, not a veto; with zero unblocked user-facing candidates it does not fire (iter 146).
- **Binding artifacts are cheap probes, but each has its own age** — probe a *discriminating* input
    first, preferring probes that leave no tree diff: a `/tmp` module with a path/`replace` dep
    (Go), a `/tmp` MSBuild project at equal depth (C#), or a gitignored build output
    (`rake compile`, `maturin develop`). Freshness table →
    [propagation ledger](unicode-fixture-propagation.md).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (iter 159) — that table is only for byte-identical *copies*. Pair the no-op
    check with a pytest case proving the gate fires on a mutated source. C-header specifics (octal
    escapes, sibling-dir quoted include so `ci.yml` needs no `-I`) →
    [propagation ledger](unicode-fixture-propagation.md).
- **"Not buildable in this container" claims decay — re-probe before they veto a slice** (iter 154:
    state.md ruled Kotlin out; `./gradlew cleanTest test --offline` ran the suite in 6 s off cached
    `~/.gradle`). Corollary trap: a runner can report success **without executing** (gradle `test`
    UP-TO-DATE) — force a rerun and check the result-file mtime.
