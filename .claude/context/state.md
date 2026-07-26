<!-- assessed-at: 5f75fea91be882a7064fe2255158e7a445c72cdb -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the Unicode 16.0 contract is now documented for consumers; its binding-side conformance work stays human-parked

Iteration 143 was a docs-only step: `docs/unicode.md` publishes the declared Unicode 16.0.0 data
version, the freeze rule, the four boundary vectors and both known divergences, wired into the site
nav, `ORDERED_PAGES` and `llms.txt`. CI is green on the `origin/develop` tip. No `crates/`,
`packages/`, `.github/` or spec file moved, so every code-bearing criterion is unchanged from
iteration 142's assessment.

## Rust Core Crate

**Status**: partially met — criterion 3 half done; criterion 4 unmet; semver/v1.0.0 held

- Incremental scope: `git diff b6308e9..HEAD` touches **5 non-`.claude` files** — `docs/unicode.md`
    (new, 100 lines), `docs/howto/go.md`, `docs/llms.txt`, `scripts/gen_llms_full.py`,
    `zensical.toml`. **No `crates/`, no `packages/`, no `.claude/context/specs/`, no `target.md`, no
    `.github/`, no manifest change.** Library sections are carried forward with spot re-checks; the
    Documentation and CI/CD sections were re-verified from scratch.
- 8 crates in the workspace; `iscc-lib` still carries **328** `#[test]` functions (re-counted at
    HEAD, unchanged) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json`
    conformance vectors. Workspace version is still **0.5.0** — PR #44 is titled "Release 0.6.0" but
    is only the open `develop` → `main` PR, not a shipped release.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds the 731-range
    unassigned table (819,533 code points); `utils.rs:103` / `utils.rs:181` filter before `.nfkc()`
    / `.nfd()`; generator `scripts/gen_unicode16_unassigned.py` is checked in. Untouched.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer".
- **Boundary conformance vectors (criterion 3) — PARTIALLY met (Rust half only).**
    `crates/iscc-lib/tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` are unchanged
    at HEAD. The spec sentence requires the set to be exercised "by the Rust test suite **and** by
    every binding's conformance test"; no binding or `packages/` tree carries the boundary code
    points, so the criterion as written remains unmet. Documentation is *not* a substitute —
    `docs/unicode.md` correctly states only that the vectors are "exercised by the Rust test suite".
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites
    in `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64. Nothing under
    `crates/iscc-py/` or in `pyproject.toml` / `uv.lock` moved this iteration.
- Pending: the Unicode 16.0 boundary vectors — only fixture wiring is missing; the freeze-rule
    behaviour is inherited from the core automatically.

## Node.js Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched this iteration.
- Pending: the same Unicode 16.0 boundary vectors.

## WASM Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3` `wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched this iteration.
- Pending: the same Unicode 16.0 boundary vectors.

## C FFI

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes 47 `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched this iteration.
- Pending: the same Unicode 16.0 boundary vectors.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met, with one cross-cutting criterion pending — Go carries an extra caveat

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. The Kotlin
    2.3+ consumer floor documented in iteration 132 stands. No package source moved this iteration.
- **`packages/go` is the one binding that does not inherit the freeze rule** — an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the boundary fixture unless the 15.0→16.0 assigned
    delta is vendored or the vectors are skipped for Go with a tracking note. Neither option has
    been chosen; this is still a concrete blocker on finishing criterion 3.
- The divergence is now *documented* (a `!!! note "Unicode tables"` admonition in `docs/howto/go.md`
    plus a section in `docs/unicode.md`), which reduces user surprise but does not close the gap.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md` guides; MkDocs/zensical site
    with branding. `scripts/version_sync.py` still covers **21** targets (`--check` → 21 `OK`
    lines).
- **New this iteration:** `docs/unicode.md` ("Text and Unicode", Explanation section). Verified at
    HEAD: the declared version, the 731-range / 819,533-code-point table figures, the four boundary
    rows and both divergence notes match `crates/iscc-lib/src/utils/unicode16.rs` and
    `crates/iscc-lib/tests/unicode_boundary.json`. Wiring is complete and consistent across all
    three places that must agree — `zensical.toml` nav, `gen_llms_full.py` `ORDERED_PAGES` (now
    **23** pages, re-counted), and `docs/llms.txt`.
- The gap noted in the previous two assessments ("the declared Unicode data version appears nowhere
    in user-facing docs") is **closed**.
- The page carries one *deliberate* placeholder, agreed in review and recorded in `decisions.md` and
    as point 4 of the freeze-rule issue: the CPython 3.14 sentence is scoped to "single-code-point
    behaviour" because the sequence-ordering divergence is still unruled. That sentence must move in
    the same step as the ordering ruling.
- No gate enforces `zensical.toml` nav ↔ `ORDERED_PAGES` ↔ `docs/llms.txt` parity; all three were
    wired by hand this iteration. A page can land in one and be silently missing from the others.
    This is a real (small) hole, not currently filed as an issue.
- The only open docs item in `issues.md` is `Add programming language logos to docs site` (`low`,
    cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
    Untouched this iteration (docs-only change, baselines correctly left alone).

## CI/CD and Publishing

**Status**: partially met — the `release.yml` static gate covers checks 1–2; check 3 and the
human-gated items remain

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `cd56117`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `5f75fea` is one unpushed commit (`cid(log): iteration 143`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty and the working tree is clean,
    so the entire code change of this iteration is covered by the green run.
- `.github/workflows/ci.yml`, `release.yml` and `docs.yml` are **all byte-unchanged** this
    iteration. ci.yml: 19 YAML job entries (raw grep 21) → 20 jobs → 21 check names. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at **44 checked / 8
    unchecked**.
- `release.yml`: 97 `uses:` refs at current majors, 29 jobs with 28 carrying the
    `!cancelled() && !failure()` re-trigger guard, 8 registry toggles. It is
    `workflow_dispatch`-only, so its invariants are verified statically by
    `scripts/check_release_workflow.py` (prek hook scoped to that file + 11 pytest tests run by CI's
    `python-test` matrix). Checks 1–2 covered.
- **Open gaps:** check 3 (action-input compatibility vs each published `action.yml` — needs network,
    so a CI-only step); one floating-branch action ref
    (`rubygems/configure-rubygems-credentials@main`, needs a human tag-vs-SHA ruling); the `magnus`
    0.8 and `jni` 0.22 source rewrites plus deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk`
    18.x, Gradle wrapper, JUnit 6.x); npm OIDC trusted publishing (human-gated). No Dependabot or
    Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (count unchanged; two existing issues were amended in
place, none opened or closed). **One `[review]` entry still carries a HUMAN REVIEW REQUESTED
block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried from iteration 133):** "Freeze-rule ordering diverges from iscc-core on
    sequences". Stripping unassigned code points before normalization changes adjacency and unblocks
    contextual transforms the reference still blocks. The ruling needed is spec **wording**:
    criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as literally
    written, and the upstream proposal must state pre-normalization removal. Filed upstream as
    iscc/iscc-core#137. **Amended this iteration with point 4:** the ruling must also revisit the
    scoped CPython-3.14 sentence now published in `docs/unicode.md`.
- "Declare and gate a Unicode data version (DECIDED)" — **step (c), user-facing documentation, is
    now ticked**. Remainder: copy the fixture into 11 binding conformance suites and four sibling
    `data.json` locations. Explicitly blocked on the ordering ruling **and** the Go decision.
- **CID-doable, unblocked:** "Gate `release.yml` action-input compatibility in CI" (check 3) — an
    opt-in `--check-action-inputs` mode on the existing script that fetches each `uses:` ref's
    published `action.yml` and validates every `with:` key and every `steps.<id>.outputs.<x>`, plus
    a `ci.yml` step. Must skip cleanly offline and stay out of the network-free pytest suite.
- **Human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`" (tag-vs-SHA reopens a
    repo convention); npm OIDC trusted publishing; the parent dependency issue, now reduced to
    human/major-gated bumps only.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Escalate the two human-parked rulings to Titusz.** This recommendation carries over from iteration
143's assessment and is now stronger, not weaker: iteration 143 completed the *last* Unicode
sub-task that could be done without a ruling. Everything remaining on the two partially-met criteria
(3 and 4) is double-blocked. Both questions are short, self-contained, and fully written up:

1. The freeze-rule sequence-ordering **wording** ruling — reword criterion 4 to "single code points,
    with the sequence-adjacency delta enumerated and accepted", or specify a sequence-aware sweep.
    This alone unblocks criterion 4, de-risks the binding propagation, and unfreezes the deliberate
    placeholder now published in `docs/unicode.md`.
2. The Go 15.0-tables call — vendor the 15.0→16.0 assigned delta, or skip the boundary vectors for
    Go with a tracking note. This unblocks the Go slice of the binding propagation.

**If the loop must pick work anyway,** check 3 (`--check-action-inputs` + a CI step) is the only
fully unblocked item and is well scoped — keep it network-gated and out of pytest. Note the cadence
risk: 134–140 were seven consecutive tooling/CI packages, 141 was library test code, 142 tooling
again, 143 docs. Taking check 3 makes three of the last five iterations workflow tooling on a file
no CI run executes.

**Do not pick:** Unicode binding propagation (double-blocked), or the
`rubygems/configure-rubygems-credentials` pin (needs the tag-vs-SHA ruling).

**Smaller, genuinely unblocked alternative:** a parity check for `zensical.toml` nav ↔
`ORDERED_PAGES` ↔ `docs/llms.txt` — this iteration wired all three by hand with nothing to catch a
miss. Inventing a new gate needs human sign-off, so define-next should either scope it explicitly or
file it as an issue first.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
