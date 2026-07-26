<!-- assessed-at: 57ebcb83a77cecca3076b8c3581179ce8ffb72d3 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the measured docs-list defect is fixed and gated; remaining unblocked work is gate-hardening only

Iteration 145 closed the `docs/llms.txt` drift found at 144: all 23 real pages are linked, and
`scripts/check_docs_nav.py` now asserts parity across the four hand-wired lists. CI is green on the
`origin/develop` tip with 22 distinct checks. With that gap closed, every remaining unblocked issue
is small gate-hardening in `scripts/`; the two substantive criteria (Unicode boundary propagation,
full-code-space sweep) stay double-blocked on human rulings.

## Rust Core Crate

**Status**: partially met — criterion 3 half done; criterion 4 unmet; semver/v1.0.0 held

- Incremental scope: `git diff 2d1c03c..HEAD` touches **5 non-`.claude` files** —
    `.pre-commit-config.yaml` (+10), `docs/development.md` (+5), `docs/llms.txt` (+6),
    `scripts/check_docs_nav.py` (+121, new), `tests/test_check_docs_nav.py` (+120, new). **No
    `crates/`, no `packages/`, no `.claude/context/specs/`, no `target.md`, no manifest, no workflow
    change.** Library sections carried forward with spot re-checks; Documentation, CI/CD and Open
    Issues re-verified from scratch.
- 8 crates in the workspace; `iscc-lib` still carries **328** `#[test]` functions (re-counted at
    HEAD, unchanged) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json`
    conformance vectors. Workspace version is still **0.5.0**. PR #44 is titled "Release 0.6.0" but
    is only the open `develop` → `main` PR, not a shipped release.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds the 731-range
    unassigned table; `utils.rs:103` / `utils.rs:181` filter before `.nfkc()` / `.nfd()`; generator
    `scripts/gen_unicode16_unassigned.py` checked in. Untouched.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer".
- **Boundary conformance vectors (criterion 3) — PARTIALLY met (Rust half only).**
    `crates/iscc-lib/tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` unchanged. The
    spec requires the set to be exercised "by the Rust test suite **and** by every binding's
    conformance test"; no binding or `packages/` tree carries the boundary code points.
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

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched.
- Pending: the same Unicode 16.0 boundary vectors.

## WASM Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3` `wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- Pending: the same Unicode 16.0 boundary vectors.

## C FFI

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes 47 `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
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
    `docs/unicode.md`).

## Documentation

**Status**: met — the `llms.txt` drift measured at iteration 144 is fixed and now gated

- **Re-verified at HEAD by measurement, not by handoff:** 24 tracked `docs/**/*.md` files minus the
    `docs/includes/abbreviations.md` snippet partial = **23 pages**; `docs/llms.txt` carries **23**
    `https://lib.iscc.codes/<path>.md` links (24 URLs total, the extra being `llms-full.txt`);
    `zensical.toml` nav and `gen_llms_full.py` `ORDERED_PAGES` both list 24 / 23 respectively as
    before. `uv run scripts/check_docs_nav.py` exits 0 with
    `OK: 23 documentation pages consistent across nav, ORDERED_PAGES and llms.txt.` The six
    previously-missing pages (`howto/c-cpp.md`, `howto/dotnet.md`, `howto/kotlin.md`,
    `howto/ruby.md`, `howto/swift.md`, `ruby-api.md`) are present, so all eleven supported languages
    are visible to `llms.txt` consumers. `specs/documentation.md` line 32 is now true.
- **Parity is now enforced** by `scripts/check_docs_nav.py` (121 lines, four `Path` args so it can
    be pointed at temp checkouts) via a prek `check-docs-nav` hook scoped to
    `docs/*.md | docs/llms.txt | zensical.toml | scripts/gen_llms_full.py`, plus
    `tests/test_check_docs_nav.py` (**8** tests) which runs in CI's `python-test` matrix. Review
    independently confirmed the real pre-fix regression fires and that all three malformed-input
    paths fail closed.
- Two latent blind spots in the new gate are filed, neither active at HEAD: `NAV_MD_RE` matches
    inside TOML comments (a commented-out nav entry would still count as present; `zensical.toml`
    has zero comment lines today), and the prek hook does not fire on a page *deletion* (`files:`
    matching only sees added/modified paths — the pytest anchor test catches it at pre-push/CI).
- Otherwise intact: 12 crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md` guides;
    MkDocs/zensical site with branding; `scripts/version_sync.py` covers **21** targets.
    `docs/development.md` gained an accurate description of the new checker.
- `docs/unicode.md` still carries the *deliberate* placeholder recorded in `decisions.md`: the
    CPython 3.14 sentence is scoped to "single-code-point behaviour". It must move in the same step
    as the ordering ruling.
- Remaining cosmetic docs item: `Add programming language logos to docs site` (`low`, `[human]`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
    Untouched this iteration.

## CI/CD and Publishing

**Status**: partially met — all automated gates green; remaining items are human-gated or
gate-hardening

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `06ff088`:
    **43 check-runs, 22 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `57ebcb8` is one unpushed commit (`cid(log): iteration 145`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty and the working tree is clean,
    so the entire code change of this iteration is covered by the green run.
- ci.yml is unchanged: **22 raw job entries → 20 YAML jobs → 21 jobs → 22 check names**, matching
    the API exactly. The new docs-nav gate deliberately adds **no** CI job — it rides the existing
    `python-test` matrix via pytest plus the prek pre-commit stage, which is the cheaper and
    network-free placement.
- `.github/workflows/release.yml` and `docs.yml` are **byte-unchanged**. `release.yml`: 97 `uses:`
    refs at current majors, 29 jobs / 28 guarded, 8 registry toggles, `workflow_dispatch`-only.
    `scripts/check_release_workflow.py` (23 functions, three checks) and its 19 tests unchanged; the
    `release-workflow` CI job with `--check-action-inputs` remains green. `specs/ci-cd.md` stands at
    **44 checked / 8 unchecked** (unchanged).
- All other quality gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit;
    `cargo-semver-checks` informational only.
- **Open gaps:** four known blind spots in `--check-action-inputs` (transport failures that are not
    `OSError` — `IncompleteRead`, `yaml.YAMLError` — still red it, contradicting its fails-open
    contract; no reverse check for newly-*required* inputs; an all-skipped run is indistinguishable
    from a pass; Docker `args`/`entrypoint` false positive plus unscanned job-level `uses:`), and
    the two `check_docs_nav.py` items above. Plus the pre-existing human-gated set:
    `rubygems/configure-rubygems-credentials@main` (tag-vs-SHA ruling), npm OIDC trusted publishing,
    the `magnus` 0.8 / `jni` 0.22 source rewrites and deferred majors (xunit 3.x,
    `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit 6.x). No Dependabot or Renovate config.
- **Pre-existing spec drift for Titusz, not a regression:** `specs/ci-cd.md`'s CI job table has 14
    rows against 21 jobs. It reads as descriptive rather than exhaustive.

## Open Issues

**9 open — 0 critical, 7 normal, 2 low** (count flat, but bodies moved: the docs-list parity issue
was resolved and replaced in place by the `check_docs_nav.py` blind-spot issue). **One `[review]`
entry still carries a HUMAN REVIEW REQUESTED block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried from iteration 133):** "Freeze-rule ordering diverges from iscc-core on
    sequences". Stripping unassigned code points before normalization changes adjacency and unblocks
    contextual transforms the reference still blocks. The ruling needed is spec **wording**:
    criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as literally
    written. Filed upstream as iscc/iscc-core#137. Point 4 also requires revisiting the scoped
    CPython-3.14 sentence in `docs/unicode.md`.
- "Declare and gate a Unicode data version (DECIDED)" — steps (a1) and (c) done, Rust half of (b)
    done. Remainder: copy the fixture into 11 binding conformance suites and four sibling
    `data.json` locations. Explicitly blocked on the ordering ruling **and** the Go decision.
- **CID-doable and unblocked:** "Harden the `--check-action-inputs` gate against its known blind
    spots" (four small items) and **NEW** "`check_docs_nav.py` counts commented-out `zensical.toml`
    nav entries as present" (two small items). Both live in `scripts/`, share a test pattern, and
    the issue text itself recommends bundling them.
- **Human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`"; npm OIDC trusted
    publishing; the parent dependency issue, now reduced to human/major-gated bumps only.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos (`[human]`, needs asset
    sourcing and licensing).

## Next Milestone

**Bundle the two gate blind-spot fixes into one step** — `check_docs_nav.py` (strip `#`-to-EOL
before `NAV_MD_RE` matching; add the explanatory comment about deletion coverage to
`.pre-commit-config.yaml`) plus `check_release_workflow.py --check-action-inputs` (widen the
`except` to `http.client.HTTPException` and `yaml.YAMLError`, add the required-input reverse check,
fail when zero refs resolve, and skip Docker-action `args`/`entrypoint`). Both are pure-local
`scripts/` changes with an existing test pattern, and both close gaps where a gate reports green
while the invariant it names is broken — the worst failure mode for a gate.

**Cadence caveat, stated deliberately rather than ignored:** 141 tests / 142 tooling / 143 docs /
144 tooling / 145 docs+tooling; this makes it four tooling-ish steps in six. That is acceptable
*only because* every remaining user-facing item is blocked: the Unicode work is double-blocked, the
dependency remainder is human/major-gated, npm OIDC and the rubygems pin need Titusz, and the docs
logos need licensed assets. If define-next can find genuinely unblocked user-facing work, prefer it.

**Still the highest-value thing a human can do — escalate the two parked rulings to Titusz.** This
is the third consecutive state to say so; both are short, self-contained and fully written up, and
criteria 3 and 4 are double-blocked without them:

1. The freeze-rule sequence-ordering **wording** ruling — reword criterion 4 to "single code points,
    with the sequence-adjacency delta enumerated and accepted", or specify a sequence-aware sweep.
    This unblocks criterion 4, de-risks the binding propagation, and unfreezes the deliberate
    placeholder in `docs/unicode.md`.
2. The Go 15.0-tables call — vendor the 15.0→16.0 assigned delta, or skip the boundary vectors for
    Go with a tracking note. Unblocks the Go slice of the binding propagation.

**Do not pick:** Unicode binding propagation (double-blocked), or the
`rubygems/configure-rubygems-credentials` pin (needs the tag-vs-SHA ruling).

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
