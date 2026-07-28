# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next". Then `critical` issues, regardless of feature trajectory.
- **`tools/cid.py` never pushes — review pushes the whole batch on PASS**, so "get unpushed commits
    under CI" is never a step; it only affects *sizing* — when HEAD carries commits no gate has
    seen, scope a small diff so a red CI run is attributable (162).
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale.
    **A `human(...)` commit newer than the last `cid(review)` invalidates the handoff wholesale**
    (147) — check `git log` before trusting "Next".
- **A human issue's or handoff's suggested fix is a hypothesis, not a spec — probe it** (147: the
    suggested `cases.Caser` hoist was both unsafe and no faster); cheap probes turn
    `## Implementation Notes` into measured facts.
- **A crashed review role means the previous step has NO verdict** (`iterations.jsonl` FAIL/turns:1,
    no `cid(review)` commit) — resolved issues were never deleted; re-verify claims from the tree.
- **A `define-next` TIMEOUT leaves an uncommitted `next.md`** (155): adopt-and-verify rather than
    starting blank, but **re-derive every numeric invariant it asserts**. A timeout is **not** a
    bounce — no advance/review ran; say so in `## Goal`.
- **A "remaining item" in an issue body can already be done — probe the external source before
    scoping it** (170: three artifacts still listed the `release.yml` action refresh; one `gh api`
    loop showed zero refs behind). Same for a remembered version range: read the constant out of the
    tool's own artifact (165; recipes → [ledger](dep-refresh-ledger.md)).
- **Any upstream-API claim in Implementation Notes must be read out of
    `~/.cargo/registry/src/*/<crate>-<ver>/`, with enough context to see which item an attribute
    belongs to** (168: a `#[deprecated]` misattributed by two functions cost iteration 169). A wrong
    fact in next.md costs a whole extra iteration. Registry manifests also carry `rust-version`.
- **When a fix touches surfaces no test exercises, scope fix + missing tests as ONE step** (169) —
    tests are outside the 3-file budget, and splitting ships an unguarded change.
- **Run the expensive probe while scoping when it can invert the plan** — the ~60 s sweep reordered
    the milestones at 155/157; the 5-min Swift fetch at 161 killed a 10-iteration-old veto.
- **Generated/tool-output files (Cargo.lock, bindings) and docs don't count toward the 3-file
    limit** — batch related small changes (version sync + docs; several fixes in one crate).
- **IDLE is valid** when all target sections are met and only `low` issues remain — but an
    already-specced, locally-verifiable target gap or infra/release fix is NOT idle work.
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — Write needs a prior Read; if blocked, use `cat > file << 'EOF'`
    (quoted EOF = no expansion). Then run `uv run prek run mdformat --files <path>`: it reflows and
    can inject a double space inside inline code — reword instead of leaving the artifact.

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM). `gen_iscc_code_v0` vectors have no `wide` — pass `false`;
    `"stream:<hex>"` prefix = hex-encoded byte data.
- **SumHasher** lives at `iscc_lib::streaming::SumHasher` (NOT Tier 1; count stays 32).

## Dev Environment Constraints

- **No shellcheck** (shell lint is CI-only). `cmake` runs via `uv run --with cmake cmake …` (PyPI
    wheel, iter 160); **Swift runs here too** (swift.org debian12 tarball, 784 MB / ~5 min / no sudo
    — recipe → [propagation ledger](unicode-fixture-propagation.md)).
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen` (UniFFI pin →
    ledger); SPM module name MUST be `iscc_uniffiFFI`; no `const`/`usize`/borrowed/generic exports.
- `dotnet` 8.0.423 is on `$PATH` with nuget.org reachable, so .NET restore/test steps are locally
    verifiable; only `packages/dotnet` had no committed lockfile as of 170.
- cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner ARE installed.
    cargo-deny is NOT preinstalled but installs via `cargo binstall cargo-deny@0.19.9 --force`.
- **rustup carries stable (1.97.1) *and* 1.85.0**, so every MSRV claim is locally verifiable:
    `cargo +1.85.0 check -p <crate> --locked`. Never defer an MSRV assertion to a CI job.
- **`mise run lint` is only `cargo fmt --check` + clippy + ruff** — taplo/mdformat/hygiene run in
    prek (`mise run check`). Never credit a TOML/Markdown format check to `lint` in a criterion.

## CI/Release, Docs, Gotchas

- Release: `workflow_dispatch` per-registry checkboxes; version_sync.py manages **21** targets.
    `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in Rust CI); the XCFramework cache key
    must hash all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`; collapsible `??? tip "Build from source"`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson Maven groupId `com.google.code.gson`.
- "10 gen functions" vs "9 conformance functions" — never blanket-replace 9→10 (see learnings.md).
- Language API name styles (doc examples): Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/
    Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u` params.
- **4 quality gates**: Semver (informational until the human-gated v1.0.0) + Coverage/CRAP + Perf +
    Audit (enforcing). **Baselines move only via deliberate reviewed `mise run` commits**, never
    from CI; don't widen `--epsilon`. `cargo deny check` is authoritative locally (reads
    Cargo.lock).

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released; all 12 bindings meet core criteria; the 4 spec'd v0.6.0 feature issues and the
    Unicode chain are DONE (161). 169 closed the JNI debt, 170 the `packages/dotnet` lockfile gap.
- **2026-07-28 the human unblocked v0.6.0**: `target.md` names it the active milestone; 171
    (`criterion` 0.8) and 172 (`uniffi` 0.32) closed the dependency issue. Facts →
    [dep-refresh ledger](dep-refresh-ledger.md). Trigger-only: go1.27 + the Go freeze table (~Aug
    2026). HELD `low` by Titusz: v1.0.0 + Semver-enforcing, the MSRV verification job, npm OIDC.
- **A dep major can leave a *declaration* false without breaking anything** (172→173): uniffi 0.32
    pushed `iscc-uniffi`'s real floor to 1.91 via `cargo-platform`. Declaring the true floor on an
    **unpublished** crate is CID-doable and a 1-line step; moving the **root/published**
    `rust-version` stays Titusz's call.
- **2026-07-28 (174): out-of-loop commit `2c4e487` REOPENED scope AFTER a review IDLE/NONE handoff**
    — target grew to 33 Tier 1 symbols + experimental **ISCC-IDv1** on core + all 11 surfaces (issue
    #43). The 173 handoff still said "emit NONE"; it was **stale** (update-state `e157270`
    reassessed at `2c4e487`). Trust the fresh state.md over an IDLE handoff whenever a non-`cid()`
    commit sits between the last `cid(review)` and now.
- **ISCC-IDv1 (#43) decomposes into Part 1 then Part 2 — spec says so explicitly.** Part 1 = codec
    accepts Version 1 for MainType `Id` only (`codec::Version` gains `V1`+`#[non_exhaustive]`;
    `decode_header`/`encode_header`/`encode_component` gate on `Id`). **One core `codec.rs` edit
    flips 10 of 11 surfaces at once** because py/napi/wasm/ffi/jni/rb/uniffi all call the core
    `iscc_decode`/`iscc_decompose`; only Go is a separate port (already did its accept out-of-loop).
    Part 2 = new symbol `gen_iscc_id_v1(timestamp:u64, hub_id:u16, realm:u8)` (Part 2 depends on
    Part 1's encode path). Adding `#[non_exhaustive]` to `Version` IS a semver-major
    (`enum_marked_non_exhaustive`) — advance must write an `**API-BREAK:**` handoff note; not gated
    (semver job is `continue-on-error`). **No `decode_iscc_id_v1`** (Titusz). Differential test is
    Python-only; no new JSON fixture (one 16-char constant, pinned inline per surface).
- **Closed phases: 115–123 (features) and the dep slices 124–137 + 162–172** — detail in
    MEMORY-archive.md + the [dep-refresh ledger](dep-refresh-ledger.md) (read before any dep step).
    Lessons that still bite: the CRAP regression gate is **CI-only**; **never move a consumer
    floor** (MSRV, `go` directive, `required_ruby_version`, a published binding's compiler) inside a
    refresh slice (128); a major's real design question is a behavioural contract the new API would
    silently change — preserve it with a built-in option, never by editing tests (168).
- **Lint/formatter tool bumps and hook-config changes have their own playbook**:
    [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact file/finding
    count a pass/fail criterion** — it drifts with the CID agents' own commits; use the exit code.
- **Unicode chain DONE (147–161)**: sentinel freeze, `Final_Sigma` freeze, the
    `mise run unicode:sweep` gate, boundary vectors on all 11 surfaces; never revive the superseded
    category override or a 15.1.0 declared version. A 13th vector costs 12 suites at once — any
    fixture edit is its own slice. Details → [unicode-freeze-facts](unicode-freeze-facts.md) +
    [propagation ledger](unicode-fixture-propagation.md).
- **Gate/checker steps in `scripts/` have their own playbook** (placement, Python floor, docs-list
    wiring, when a gate needs Titusz): [gate scripts playbook](gate-scripts-playbook.md).
- **In a file no CI push exercises, split behavioural edits from mechanical ones** (139/140:
    `release.yml`) — recipe: [release.yml static gates](release-yml-static-gates.md).
- **Any step touching the text hot path trips two gates at once**: the CI-only CRAP
    `--fail-regression` baseline and the `.iai-baseline.json` 10% Ir gate. A `tests/`-only step
    usually trips **neither** — say so in next.md so advance doesn't refresh a baseline.
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep —
    [ty gate trap](define-next-ty-generator-scripts.md).
- `uv run zensical build` (exits 0, "No issues found", ~8s) verifies any docs-only step.
- **Recurring**: the cargo-deny gate WILL periodically go red on fresh RustSec advisories vs
    dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore.
- **Watch the tooling-cadence flag in state.md**: 3 of 4 recent iterations CI/lint/workflow → prefer
    a user-facing item once (143) — a one-iteration deferral; with no such candidate it doesn't
    fire.
- **Binding artifacts are cheap probes, but each has its own age** — probe a *discriminating* input
    first, via a path that leaves no tree diff (`/tmp` clone, gitignored build dir). Freshness table
    → [propagation ledger](unicode-fixture-propagation.md).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (159, copies only). Pair the no-op check with a pytest case proving the gate
    fires on a mutated source — unless a third-party generator emits non-hook-clean bytes (172).
- **"Not buildable in this container" claims decay — re-probe before they veto a slice.** Kotlin
    (154), C++ (160) and Swift (161) were each ruled out by state.md and each ran within minutes.
    Ladder: `$PATH` → `uv run --with <tool>` → cached toolchain dir → upstream tarball for *this*
    distro. Trap: a runner can report success **without executing** (gradle UP-TO-DATE) — force it.
- **When state.md names the file to edit, re-derive it** (160: it pointed at the public `iscc`
    INTERFACE target for a test-only include dir that vcpkg/conan consumers inherit).
