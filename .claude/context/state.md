<!-- assessed-at: 2d1c03c3d16cf24460620a7d95107a916f68414a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — release-workflow invariants are now fully gated; a measured docs-list drift is the newest concrete defect

Iteration 144 closed the last unautomated `release.yml` invariant: `--check-action-inputs` validates
every `with:` key and `steps.<id>.outputs.<x>` read against each action's published `action.yml`, in
a dedicated CI job. CI is green on the `origin/develop` tip, and the new job's log confirms it truly
ran (zero `warning: skipped` lines). Review of that step surfaced a real user-facing gap that
downgrades Documentation from met: `docs/llms.txt` is missing 6 of 23 real pages.

## Rust Core Crate

**Status**: partially met — criterion 3 half done; criterion 4 unmet; semver/v1.0.0 held

- Incremental scope: `git diff 5f75fea..HEAD` touches **4 non-`.claude` files** —
    `.github/workflows/ci.yml` (+15), `docs/development.md` (+6/-2),
    `scripts/check_release_workflow.py` (+175), `tests/test_check_release_workflow.py` (+131). **No
    `crates/`, no `packages/`, no `.claude/context/specs/`, no `target.md`, no manifest, no
    `release.yml` change.** Library sections carried forward with spot re-checks; CI/CD,
    Documentation and Open Issues re-verified from scratch.
- 8 crates in the workspace; `iscc-lib` still carries **328** `#[test]` functions (re-counted at
    HEAD, unchanged) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json`
    conformance vectors. Workspace version is still **0.5.0** (re-confirmed in the CI
    version-consistency log — all 21 sync targets report `0.5.0`). PR #44 is titled "Release 0.6.0"
    but is only the open `develop` → `main` PR, not a shipped release.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds the 731-range
    unassigned table (819,533 code points); `utils.rs:103` / `utils.rs:181` filter before `.nfkc()`
    / `.nfd()`; generator `scripts/gen_unicode16_unassigned.py` checked in. Untouched.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer".
- **Boundary conformance vectors (criterion 3) — PARTIALLY met (Rust half only).**
    `crates/iscc-lib/tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` unchanged at
    HEAD. The spec requires the set to be exercised "by the Rust test suite **and** by every
    binding's conformance test"; no binding or `packages/` tree carries the boundary code points.
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
    been chosen; still a concrete blocker on finishing criterion 3.
- The divergence is documented (`!!! note "Unicode tables"` in `docs/howto/go.md` plus a section in
    `docs/unicode.md`), which reduces user surprise but does not close the gap.

## Documentation

**Status**: partially met — `docs/llms.txt` violates its spec line, with measured drift

- `specs/documentation.md` line 32 requires "`docs/llms.txt` exists with site metadata and links to
    **all** documentation pages". Verified at HEAD: **24 tracked `docs/**/*.md` files**; `llms.txt`
    carries **17 page links** (plus one to `llms-full.txt`) and is missing `howto/c-cpp.md`,
    `howto/dotnet.md`, `howto/kotlin.md`, `howto/ruby.md`, `howto/swift.md` and `ruby-api.md`.
    `docs/includes/abbreviations.md` is a snippet partial and legitimately excluded. **Five of the
    eleven supported languages are invisible to `llms.txt` consumers today.** This is a real
    user-facing defect, not a tooling nicety — hence the downgrade from "met".
- The other two lists are correct and agree with each other: `zensical.toml` nav 24 entries,
    `gen_llms_full.py` `ORDERED_PAGES` **23** (both differ only by the snippet partial).
- **No gate enforces parity** across the three hand-wired lists; the drift above is exactly what
    that absence allows. Now filed as a `normal` `[review]` issue that scopes the data fix and the
    checker together, so it is CID-doable and unblocked.
- Otherwise intact: 12 crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md` guides;
    MkDocs/zensical site with branding; `scripts/version_sync.py` covers **21** targets (all `OK` at
    `0.5.0` in the CI log). `docs/unicode.md` (iter 143) unchanged and still accurate.
- `docs/development.md` gained an accurate description of the new `--check-action-inputs` mode
    (network-only, CI-job-only, fails open on transport trouble).
- `docs/unicode.md` still carries the *deliberate* placeholder recorded in `decisions.md`: the
    CPython 3.14 sentence is scoped to "single-code-point behaviour". It must move in the same step
    as the ordering ruling.
- Remaining cosmetic docs item: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases; the CI log
    confirms "16 benchmark(s) within 10% of baseline (Ir)".
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
    Untouched this iteration.

## CI/CD and Publishing

**Status**: partially met — all three `release.yml` static checks are now gated; only human-gated
items and the new gate's blind spots remain

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `b915582`:
    **43 check-runs, 22 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `2d1c03c` is one unpushed commit (`cid(log): iteration 144`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty and the working tree is clean,
    so the entire code change of this iteration is covered by the green run.
- **New job `release-workflow` ("Release workflow (action inputs)")** — ci.yml is now **20 YAML job
    entries** (raw grep 22, minus `push`/`pull_request`) → 21 jobs → **22 check names**, matching
    the API exactly. Job is `checkout@v7` + `setup-uv@v9.0.0` (exact tag, commented) +
    `uv run --no-project --with pyyaml … --check-action-inputs`.
- **Verified the gate actually ran, not just passed:** the job log shows `Installed 1 package`, the
    `OK: … passed release-workflow static checks.` line and **zero `warning: skipped` lines**. This
    matters because `decisions.md` (2026-07-26) records the accepted fails-open trade-off — a fully
    rate-limited run would be green, so the log is the signal, not the badge.
- `scripts/check_release_workflow.py` now has 23 functions covering all three checks: guard shape
    (29 jobs, 28 guarded), input wiring, artifact wiring with `matrix.include` expansion, `needs:`
    graph, and action-input compatibility over 18 distinct refs.
    `tests/test_check_release_workflow.py` is at **19 tests** (11 + 8 new, fetcher injected so the
    suite stays network-free) and runs in CI's `python-test` matrix; the prek hook remains
    network-free and scoped to `release.yml`.
- `.github/workflows/release.yml` and `docs.yml` are **byte-unchanged**. `release.yml`: 97 `uses:`
    refs at current majors, 29 jobs / 28 guarded, 8 registry toggles, `workflow_dispatch`-only.
    `specs/ci-cd.md` stands at **44 checked / 8 unchecked** (unchanged — review correctly declined
    to edit a spec from a `[review]`-sourced issue).
- All other quality gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit;
    `cargo-semver-checks` informational only.
- **Open gaps:** four known blind spots in the new gate (transport failures that are *not* `OSError`
    — `IncompleteRead`, `yaml.YAMLError` — still red it, contradicting its own contract; no reverse
    check for newly-*required* inputs; an all-skipped run is indistinguishable from a pass; two
    latent false positives plus unscanned job-level `uses:`). Plus the pre-existing human-gated set:
    `rubygems/configure-rubygems-credentials@main` (tag-vs-SHA ruling), npm OIDC trusted publishing,
    the `magnus` 0.8 / `jni` 0.22 source rewrites and deferred majors (xunit 3.x,
    `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit 6.x). No Dependabot or Renovate config.
- **Pre-existing spec drift for Titusz, not a regression:** `specs/ci-cd.md`'s CI job table has 14
    rows against 21 jobs (no `dotnet`, `cpp`, `swift`, `kotlin`, `coverage`, `release-workflow`). It
    reads as descriptive rather than exhaustive.

## Open Issues

**9 open — 0 critical, 7 normal, 2 low** (net +1: the action-input gate issue was resolved and
deleted, two new `[review]` issues were filed). **One `[review]` entry still carries a HUMAN REVIEW
REQUESTED block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried from iteration 133):** "Freeze-rule ordering diverges from iscc-core on
    sequences". Stripping unassigned code points before normalization changes adjacency and unblocks
    contextual transforms the reference still blocks. The ruling needed is spec **wording**:
    criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as literally
    written, and the upstream proposal must state pre-normalization removal. Filed upstream as
    iscc/iscc-core#137. Point 4 also requires revisiting the scoped CPython-3.14 sentence in
    `docs/unicode.md`.
- "Declare and gate a Unicode data version (DECIDED)" — steps (a1) and (c) done, Rust half of (b)
    done. Remainder: copy the fixture into 11 binding conformance suites and four sibling
    `data.json` locations. Explicitly blocked on the ordering ruling **and** the Go decision.
- **NEW, CID-doable and unblocked:** "Gate parity of the three hand-wired docs page lists" — comes
    with the measured `llms.txt` drift above, so it is data fix + local checker in one step (natural
    home `scripts/check_docs_nav.py` + a prek hook, no network).
- **NEW, CID-doable:** "Harden the `--check-action-inputs` gate against its known blind spots" — the
    four items listed under CI/CD, all small.
- **Human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`" (tag-vs-SHA reopens a
    repo convention); npm OIDC trusted publishing; the parent dependency issue, now reduced to
    human/major-gated bumps only.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Fix the `docs/llms.txt` drift and gate the three page lists.** This is the only open item that is
both fully unblocked *and* a live user-facing defect: five of eleven supported languages have no
entry in the file LLM consumers read, and the spec sentence requiring "links to all documentation
pages" is currently false. Scope the data fix (six URLs) and a small pure-local parity checker
(`docs/**/*.md` ⇄ `zensical.toml` nav ⇄ `ORDERED_PAGES` ⇄ `llms.txt`, with a commented allowlist for
`includes/`) into the same step so the gate lands green. This also breaks the tooling-heavy cadence:
140 tooling / 141 tests / 142 tooling / 143 docs / 144 tooling.

**Still the highest-value thing a human can do — escalate the two parked rulings to Titusz.** Both
are short, self-contained and fully written up, and criteria 3 and 4 are double-blocked without
them:

1. The freeze-rule sequence-ordering **wording** ruling — reword criterion 4 to "single code points,
    with the sequence-adjacency delta enumerated and accepted", or specify a sequence-aware sweep.
    This unblocks criterion 4, de-risks the binding propagation, and unfreezes the deliberate
    placeholder in `docs/unicode.md`.
2. The Go 15.0-tables call — vendor the 15.0→16.0 assigned delta, or skip the boundary vectors for
    Go with a tracking note. Unblocks the Go slice of the binding propagation.

**Second choice if the docs step is taken elsewhere:** harden `--check-action-inputs` (widen the
`except` to `http.client.HTTPException` and `yaml.YAMLError`, add the required-input reverse check,
fail when zero refs resolve). Worth doing, but it hardens tooling against hypotheticals rather than
fixing a live gap, and it would make four of the last six iterations workflow tooling.

**Do not pick:** Unicode binding propagation (double-blocked), or the
`rubygems/configure-rubygems-credentials` pin (needs the tag-vs-SHA ruling).

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
