# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next". Then `critical` issues, regardless of feature trajectory.
- **`tools/cid.py` never pushes — the review agent pushes the whole batch on PASS**, so "get
    unpushed commits under CI" is never a step. What it *should* change is sizing: when HEAD carries
    commits no gate has seen (out-of-loop `human(...)`/`cid(loop)` work), scope a deliberately small
    diff so a red CI run is attributable to that untested code, not to this step (iter 162).
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    read issues.md directly (review can miscount). **A `human(...)` commit newer than the last
    `cid(review)` invalidates the handoff wholesale** (iter 147) — check `git log` before trusting
    "Next".
- **A human issue's or handoff's suggested fix is a hypothesis, not a spec — probe it** (iter 147:
    the suggested `cases.Caser` hoist was both unsafe and no faster). Cheap throwaway probes settle
    these in a minute and turn `## Implementation Notes` into measured facts.
- **A crashed review role means the previous step has NO verdict** (`iterations.jsonl` FAIL/turns:1,
    no `cid(review)` commit) — resolved issues were never deleted; re-verify claims from the tree.
- **A `define-next` TIMEOUT leaves an uncommitted `next.md`** (iter 155): adopt-and-verify rather
    than starting blank, but **re-derive every numeric invariant it asserts** (they become
    hard-coded `EXPECTED_*`). A timeout is **not** a bounce — no advance/review ran; say so in
    `## Goal`.
- **A remembered "supported versions" range is a docs ceiling, not an enforced one — read the
    constant out of the tool's own artifact** (165; recipe → [ledger](dep-refresh-ledger.md)).
- **A "remaining item" in an issue body can already be done — probe the external source before
    scoping it** (170: state.md, handoff and issues.md all still listed the `release.yml` action
    refresh; one `gh api` loop showed zero refs behind). The freshness fact rots in the *opposite*
    direction from most: it can only become true again when an upstream ships a major.
- **Any upstream-API claim I put in Implementation Notes must be read out of
    `~/.cargo/registry/src/*/<crate>-<ver>/`, with enough context lines to see which item an
    attribute belongs to** — at 168 a `#[deprecated]` note two functions above got attributed to
    `Env::byte_array_from_slice`, and advance dutifully hand-rolled a slower replacement (169 undid
    it). A wrong fact in next.md costs a whole extra iteration.
- **When a fix touches surfaces that no test exercises, scope the fix and the missing tests as ONE
    step** (169: 3 of 5 edited JNI return sites had no Java test). Tests don't count against the
    3-file budget, and splitting them ships an unguarded change.
- **Run the expensive probe while scoping when it can invert the plan** — the ~60 s sweep reordered
    the milestones at 155/157; the 5-min Swift fetch at 161 killed a 10-iteration-old veto.
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in one crate). **IDLE is valid**
    when all target sections are met and only `low` issues remain — but an already-specced,
    locally-verifiable target gap is NOT idle work, and infra/release fixes CAN be locally verified.
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — Write needs a prior Read of it; if Write is blocked, use
    `cat > file << 'EOF'` via Bash (quoted EOF = no expansion, so backticks are safe).
- **Escape sequences in a tool payload may get decoded before they hit the file** (149): use
    `U+XXXX` prose and `chr(0x...)` probes, and print the file's non-ASCII set after every write.

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
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`. UniFFI
    0.31.0; SPM module name MUST be `iscc_uniffiFFI`; no `const`/`usize`/borrowed/generic exports.
- `dotnet` 8.0.423 is on `$PATH` with nuget.org reachable, so .NET restore/test steps are locally
    verifiable; only `packages/dotnet` had no committed lockfile as of 170.
- cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner ARE installed.
    cargo-deny is NOT preinstalled but installs via `cargo binstall cargo-deny@0.19.9 --force`.

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
    Audit (enforcing). **Baselines are refreshed only by deliberate reviewed `mise run` commits** —
    never from CI; don't widen `--epsilon`. `cargo deny check` reads Cargo.lock + metadata, so a
    green local run is authoritative. v1.0.0 hardening phase (iters 86–114) → MEMORY-archive.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released; all 12 bindings meet core criteria; the 4 spec'd v0.6.0 feature issues are DONE;
    the **Unicode chain is CLOSED at 161**. **After 168 the CID-schedulable dependency work is
    done**; 169 scoped the JNI debt review filed against 168 (byte-array regression + 7 untested
    natives, batched). **At 170 the `release.yml` action pass turned out to be a no-op** (every ref
    in all three workflows already on its publisher's latest major — recipe in
    [release.yml static gates](release-yml-static-gates.md)), so 170 scoped the unfiled
    `packages/dotnet` lockfile gap plus the Ruby doc-drift line as a ride-along. Trigger-only:
    go1.27 + the Go freeze table (~Aug 2026). HELD `low` by Titusz: v1.0.0 + Semver-enforcing, npm
    OIDC (token to 2026-09-16).
- **Closed phases: 115–123 (features) and the dep slices 124–137 + 162–168** — detail in
    MEMORY-archive.md and the [dep-refresh ledger](dep-refresh-ledger.md) (read it before any dep
    step). Lessons that still bite: the CRAP regression gate is **CI-only** (a new branch in a
    covered fn needs the baseline refreshed in the same step); **never move a consumer floor**
    (MSRV, `go` directive, `required_ruby_version`, a published binding's compiler) inside a refresh
    slice (128); a major's real design question is a behavioural contract the new API would silently
    change — preserve it with a built-in option, never by editing tests (168).
- **Lint/formatter tool bumps and hook-config changes have their own playbook**:
    [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact file/finding
    count a pass/fail criterion** — it drifts with the CID agents' own commits; use the exit code.
- **Unicode chain DONE (147–161)**: sentinel freeze, `Final_Sigma` freeze, the
    `mise run unicode:sweep` gate (17,793,024 comparisons / 0 divergences / ~60 s), boundary vectors
    on all 11 surfaces. Never revive the superseded category override or a 15.1.0 declared version.
    Constants/escapes/tables → [unicode-freeze-facts](unicode-freeze-facts.md); loaders, vendoring,
    toolchains, the two-axis cost rule → [propagation ledger](unicode-fixture-propagation.md). A
    13th vector costs 12 suites at once — any fixture edit is its own deliberate slice.
- **Gate/checker steps in `scripts/` have their own playbook** (prek-vs-CI placement, Python 3.10
    floor, injected-`Path` shape, docs-list wiring, when a gate needs Titusz):
    [gate scripts playbook](gate-scripts-playbook.md). Read before scoping under `scripts/`.
- **In a file no CI push exercises, split behavioural edits from mechanical ones** (iters 139/140:
    `release.yml` guard fixes vs the 97-ref `uses:` bump, so a broken release is bisectable). Recipe
    - evidence rules: [release.yml static gates](release-yml-static-gates.md).
- **Any step touching the text hot path trips two gates at once**: the CI-only CRAP
    `--fail-regression` baseline and the `.iai-baseline.json` 10% Ir gate. A `tests/`-only step
    usually trips **neither** — say so in next.md so advance doesn't refresh a baseline.
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep —
    [ty gate trap](define-next-ty-generator-scripts.md).
- `uv run zensical build` (exits 0, "No issues found", ~8s) verifies any docs-only step.
- **Recurring**: the cargo-deny gate WILL periodically go red on fresh RustSec advisories vs
    dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore.
- **Watch the tooling-cadence flag in state.md.** With 3 of the last 4 iterations CI/lint/workflow,
    prefer a user-facing item over a tracked `normal` tooling issue (iter 143) — a **one-iteration**
    deferral, not a veto; with zero unblocked user-facing candidates it does not fire (iter 146).
- **Binding artifacts are cheap probes, but each has its own age** — probe a *discriminating* input
    first, preferring probes that leave no tree diff: a `/tmp` module clone, a `/tmp` project at
    equal depth, or a gitignored build output. Freshness table →
    [propagation ledger](unicode-fixture-propagation.md).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (iter 159) — that table is only for byte-identical *copies*. Pair the no-op
    check with a pytest case proving the gate fires on a mutated source.
- **"Not buildable in this container" claims decay — re-probe before they veto a slice.** Kotlin
    (154, cached `~/.gradle`), C++ (160, cmake-from-PyPI) and Swift (161, swift.org debian12
    tarball) were each ruled out by state.md and each ran within minutes. Ladder: `$PATH` →
    `uv run --with <tool>` → cached toolchain dir → upstream tarball for *this* distro. Trap: a
    runner can report success **without executing** (gradle `test` UP-TO-DATE) — force a rerun.
- **When state.md names the file to edit, re-derive it** (iter 160: it pointed at the public `iscc`
    INTERFACE target for a test-only include dir, which vcpkg/conan consumers inherit). Test-only
    plumbing belongs on the test target.
