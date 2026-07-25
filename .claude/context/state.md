<!-- assessed-at: b6308e9a498b8c83f59375a3e526f2d7cf4f5e84 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the `release.yml` invariants became an executable gate; the Unicode 16.0 boundary work remains half landed and human-parked

Iteration 142 converted three hand-retyped `release.yml` invariants into
`scripts/check_release_workflow.py`, wired into prek (scoped to that one file) and into CI through
an 11-test pytest suite. CI is green on the `origin/develop` tip. The Unicode 16.0 boundary
criterion is unchanged since iteration 141 — Rust half done, binding propagation double-blocked on
two parked human rulings.

## Rust Core Crate

**Status**: partially met — criterion 3 half done; criterion 4 unmet; semver/v1.0.0 held

- Incremental scope: `git diff 2ae5604..HEAD` touches **7 non-`.claude` files** —
    `scripts/check_release_workflow.py` (new, 274 lines), `tests/test_check_release_workflow.py`
    (new, 171 lines), `.pre-commit-config.yaml`, `pyproject.toml`, `uv.lock`, `CLAUDE.md`,
    `docs/development.md`. **No `crates/`, no `packages/`, no `specs/`, no `target.md`, no
    `.github/workflows/` change.** All library sections below are carried forward with spot
    re-checks; the CI/CD and Documentation sections were re-verified from scratch.
- 8 crates in the workspace; `iscc-lib` still carries **328** `#[test]` functions (re-counted,
    unchanged) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance
    vectors. Workspace version is still **0.5.0** — PR #44 is titled "Release 0.6.0" but is only the
    open `develop` → `main` PR, not a shipped release.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds the 731-range
    unassigned table; `utils.rs:103` / `utils.rs:181` filter before `.nfkc()` / `.nfd()`; generator
    `scripts/gen_unicode16_unassigned.py` is checked in. Untouched this iteration.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer".
- **Boundary conformance vectors (criterion 3) — PARTIALLY met (Rust half only).**
    `crates/iscc-lib/tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` (1 ungated
    shape/content guard + 2 `text-processing`-gated vector tests) are unchanged at HEAD. The spec
    sentence requires the set to be exercised "by the Rust test suite **and** by every binding's
    conformance test"; no binding or `packages/` tree carries the boundary code points, so the
    criterion as written is still unmet.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites
    in `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64. Nothing under
    `crates/iscc-py/` moved this iteration.
- The only Python-adjacent change is at repo root: `pyyaml` added to the `dev` dependency group
    (with an inline rationale comment) plus the matching `uv.lock` edit, so the new checker and its
    pytest suite can parse `release.yml` in-process. `uv.lock` resolves clean.
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
    2.3+ consumer floor documented in iteration 132 stands. Untouched this iteration.
- **`packages/go` is the one binding that does not inherit the freeze rule** — it is an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the boundary fixture unless the 15.0→16.0 assigned
    delta is vendored or the vectors are skipped for Go with a tracking note. Both options are
    recorded; neither has been chosen. This remains a concrete blocker on finishing criterion 3.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md` guides; llms-full generation
    over 22 ordered pages; MkDocs site with branding. `scripts/version_sync.py` covers 21 targets.
- Two documentation edits this iteration, both accurate and matching the code: the pre-commit hook
    list in root `CLAUDE.md` and the hook list in `docs/development.md` now name
    `scripts/check_release_workflow.py`, its `release.yml` scoping, and the reason (the workflow is
    `workflow_dispatch`-only) plus its CI enforcement path.
- Observation (not a spec gap, unchanged): the declared Unicode data version appears nowhere in
    user-facing `docs/` or `README.md`, so consumers cannot discover the contract. The fixture's
    `_metadata.unicode_data_version` field is developer-facing only.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
    Untouched this iteration.

## CI/CD and Publishing

**Status**: partially met — a real gate now covers `release.yml` checks 1–2; check 3 and the
human-gated items remain

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `df44f6f`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `b6308e9` is one unpushed commit (`cid(log): iteration 142`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so the entire code change of
    this iteration is covered by the green run. Working tree is clean.
- **New this iteration — `release.yml` static gate (checks 1–2).**
    `scripts/check_release_workflow.py` derives everything structurally from the parsed document
    (nothing hardcoded but the `prepare-release` job id, with its "why" comment) and covers: the
    registry-guard shape plus the declared-vs-referenced `workflow_dispatch` input cross-check;
    artifact wiring with `${{ matrix.<k> }}` expansion over `strategy.matrix.include`; and the
    `needs:` graph. Verified at HEAD: `uv run scripts/check_release_workflow.py` exits **0**;
    `.github/workflows/release.yml` is byte-unmodified; the prek hook `check-release-workflow` is
    `stages: [pre-commit]`, `pass_filenames: false`, `files: ^\.github/workflows/release\.yml$`. CI
    coverage is indirect but real — `tests/test_check_release_workflow.py` holds **11** tests
    (anchor test against the tracked workflow + mutation + unit tests) and CI's `python-test` matrix
    runs a bare `uv run pytest`. Known accepted limitation, recorded in `decisions.md`:
    `references_match` is a strict approximation of glob intersection (can false-alarm on a future
    edit, can never pass broken wiring).
- `.github/workflows/ci.yml` **unchanged** — still 19 YAML job entries (raw grep 21) → 20 jobs → 21
    check names. All quality gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage
    \+ CRAP (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- `release.yml` itself unchanged since iteration 140: 97 `uses:` refs at current majors, 29 jobs
    with 28 carrying the `!cancelled() && !failure()` re-trigger guard, 8 registry toggles. It is
    `workflow_dispatch`-only, so its invariants are still verified statically only — but now by a
    re-running gate instead of a throwaway heredoc.
- **Open gaps:** check 3 (action-input compatibility, needs network → CI-only step); one
    floating-branch action ref (`rubygems/configure-rubygems-credentials@main`, needs a human
    tag-vs-SHA ruling); the `magnus` 0.8 and `jni` 0.22 source rewrites plus deferred majors (xunit
    3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x); npm OIDC trusted publishing
    (human-gated). No Dependabot or Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (count unchanged: the `[review]` gate issue was **rewritten
in place**, not closed, because check 3 remains). **One `[review]` entry still carries a HUMAN
REVIEW REQUESTED block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried from iteration 133):** "Freeze-rule ordering diverges from iscc-core on
    sequences". Stripping unassigned code points before normalization changes adjacency and unblocks
    contextual transforms the reference still blocks. The ruling needed is spec **wording**:
    criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as literally
    written, and the upstream proposal must state pre-normalization removal. Filed upstream as
    iscc/iscc-core#137.
- "Declare and gate a Unicode data version (DECIDED)" — unchanged this iteration. Remainder: copy
    the fixture into 11 binding conformance suites and four sibling `data.json` locations.
    Explicitly blocked on the ordering ruling **and** the Go decision.
- **CID-doable, unblocked:** "Gate `release.yml` action-input compatibility in CI" (check 3) — an
    opt-in `--check-action-inputs` mode on the existing script that fetches each `uses:` ref's
    published `action.yml` and validates every `with:` key and every `steps.<id>.outputs.<x>`, plus
    a `ci.yml` step. Must skip cleanly offline and stay out of the network-free pytest suite.
- **Human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`" (tag-vs-SHA reopens a
    repo convention); npm OIDC trusted publishing; the parent dependency issue, now reduced to
    human/major-gated bumps only.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Cadence flag first.** Iterations 134–140 were seven consecutive tooling/CI packages; 141 moved
library test code; 142 went back to tooling. Iteration 141's assessment explicitly said "if a second
consecutive tooling iteration is proposed after [142], treat that as drift and escalate the parked
rulings instead." Check 3 is the natural continuation of 142 and is genuinely unblocked, but taking
it makes three of the last four iterations pure workflow-tooling on a file no CI run executes, while
the two rulings that unblock real library work stay parked.

**Recommended: escalate the two human-parked rulings to Titusz rather than open another tooling
package.** Both are short, self-contained questions with the evidence already written up:

1. The freeze-rule sequence-ordering **wording** ruling (reword criterion 4 to "single code points,
    with the sequence-adjacency delta enumerated and accepted", or specify a sequence-aware sweep).
    This alone unblocks criterion 4 and de-risks step (b).
2. The Go 15.0-tables call (vendor the 15.0→16.0 assigned delta, or skip the boundary vectors for Go
    with a tracking note). This unblocks the Go slice of step (b).

**If the loop must pick work anyway,** check 3 (`--check-action-inputs` + a CI step) is the only
fully unblocked item and is well scoped — keep it network-gated and out of pytest.

**Do not pick:** Unicode step (b) binding propagation (double-blocked), or the
`rubygems/configure-rubygems-credentials` pin (needs the tag-vs-SHA ruling).

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
