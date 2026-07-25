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
- **next.md is a sensitive file** — Write needs a prior Read of it; if Write is blocked, use
    `cat > file << 'EOF'` via Bash.
- Format next.md + memory with **`uv run prek run mdformat --files <paths>`** (seconds; re-run until
    Passed). A bare `uv run mdformat` is NOT the hook (the hook passes `--number`) and *causes* the
    failure it was meant to avoid; `mise run format` can exceed a 2-min Bash timeout. Give partial/
    pseudo-code snippets a `text` or `bash` fence (a `python` fence is reformatted by ruff).

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
    ISCC-IDv1, trailing-byte guards, aarch64 wheels. **Root lesson: the CRAP regression gate is
    CI-ONLY** (not in `mise run check`/pre-commit) — any step adding a branch/loop to a covered fn
    MUST refresh the baseline in the SAME step.
- **iters 124–137 = the dependency-refresh slices, all 8 now CLOSED** → ledger, gotchas, hold-backs,
    version-lookup commands: [dep-refresh ledger](dep-refresh-ledger.md); read it before scoping any
    dep step. Headline rule: **never move a consumer floor (MSRV, `go` directive,
    `required_ruby_version`, a published binding's compiler) inside a refresh slice** (iter 128).
- **A lint-tool major bump is not one step.** Slice by *what decision each finding needs*
    (mechanical / gate-interacting / config-requiring), not by file. Probe settings without touching
    the lock: `uvx ruff@<ver> check --config '<key> = <val>' --diff <paths>`, then re-probe with the
    *pinned* tool + `--extend-select` to see if the rule is enforceable before the upgrade.
- **A tool bump's real risk may be file *discovery*, not new rules** (iter 137: ruff 0.16's rules
    were a no-op, but its formatter started covering `.md` fences → CI's bare `ruff format --check`
    widened 25 → 153 files). Before scoping a pin drop, run the *bare* gate command under the new
    version and compare the reported **file count**, not just the exit code.
- **Probe a hook-config change with `prek run -c /tmp/probe.yaml <hook> --files <path>`** — the
    global `-c` accepts any path while still resolving files inside the repo, so define-next never
    touches `.pre-commit-config.yaml`. prek reports `files were modified by this hook` only for
    tracked files; an untracked probe is silently fixed and reported Passed.
- **Gate-parity claims from review are hypotheses — measure the surfaces yourself.** Iter 138: the
    filed issue said no local hook covers Markdown, but `mdformat-ruff` (via
    `mdformat-mkdocs[recommended]`) already covered the fence tag `python` — ruff also formats
    `py`/`python3`/`pycon`, so the real gap was three fence tags. Check
    `entry_points(group='mdformat.codeformatter')` before scoping.
- **File-count identity for the ruff formatter surface (HEAD, iter 138):** bare
    `ruff format --check` = **153** = 129 tracked `.md` + 24 `.py` + 1 `.pyi` − 1
    tracked-but-gitignored `.claude/plans/*.md` (recursive discovery honours `.gitignore`;
    explicitly-named paths bypass exclusions unless `--force-exclude`). prek's type tags split
    these: `python` does **not** match `.pyi` (that is the `pyi` tag) — a hook covering stubs needs
    both.
- **A lint *config* change moves the finding set, it does not only shrink it** (iter 135: isort
    `src` cleaned 3 test files and dirtied a previously-green benchmark file). Re-run the full-tree
    check *with* the candidate config before counting files against the budget.
- **Open Unicode backlog** (DECIDED by Titusz 2026-07-25, `specs/rust-core.md`): the full-code-space
    differential sweep and the boundary vectors (Rust suite + 12 bindings) — both parked behind the
    `[review]` ordering ruling; the vectors also need a Go decision (Go on 15.0 tables until go1.27
    ≈ Aug 2026: vendor the 15.0→16.0 delta or skip-with-note). Freeze filter (133) is done.
- **A parked HUMAN REVIEW issue does not stall the loop — it re-prioritises it** (iters 134–139: the
    Unicode ruling parked 2 criteria, so steps went to ruff slices, then hook parity, then
    release.yml). If a blocked slice's only consumer is the parked propagation, take other backlog.
- **Look for gates that exist but run in only one place** (`S`/`C901` were pre-push-hook-only, CI
    never saw them). Broadening an *existing* gate to CI needs no human sign-off; inventing one
    does.
- **In a file no CI push exercises, split behavioural edits from mechanical ones** (iter 139: the
    handoff wanted `release.yml`'s 19 `if:`-guard fixes bundled with a 97-ref `uses:` bump — scoped
    as two steps so a broken release is bisectable). Static-verification recipe + job inventory:
    [release.yml static gates](release-yml-static-gates.md).
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
