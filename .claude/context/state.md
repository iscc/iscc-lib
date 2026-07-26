<!-- assessed-at: c2aabf56120cbf30dfdef4b2bdab7e0c34421fce -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the critical Go `Final_Sigma` bug is fixed (verified independently), but iteration 147's review role crashed and the fix is unpushed, so nothing but this assessment has verified it

Iteration 147 landed the `packages/go` `Final_Sigma` fix, and I reproduced the corrected behaviour
at HEAD (`ΛΟΓΟΣ` → `λογος`). Two process gaps make that the headline: the iteration-147 **review
role failed at turn 1** (`status":"FAIL","turns":1,"cost_usd":0.000606` in `iterations.jsonl` — an
infra crash, not a verdict), and HEAD is now **6 commits ahead of `origin/develop` with real code
among them**, so the green CI run does *not* cover the Go change. The Unicode sentinel conversion —
the largest correctness item — is still untouched.

## Rust Core Crate

**Status**: partially met — criterion 1 still **unmet**; 3 partial; 4 unmet

- Incremental scope: `git diff 9aa25ad..HEAD --stat` = 14 files, **6 outside `.claude/`**, all under
    `packages/go/`. **No `crates/` file moved.** `git diff 9aa25ad..HEAD -- .claude/context/specs/`
    is **empty** — no spec moved this iteration, so no met→unmet flip from the spec side.
- 8 crates; `iscc-lib` carries **328** `#[test]` functions (re-counted at HEAD, unchanged); all 10
    `gen_*_v0` functions still pass the vendored `iscc-core/data.json` vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — STILL UNMET.** Re-read the source at HEAD:
    `crates/iscc-lib/src/utils.rs` **lines 103 and 181 still hold
    `.filter(|&c| !is_unassigned_in_unicode16(c))`**, and
    `grep -n FFFF crates/iscc-lib/src/utils.rs` returns **nothing** — the sentinel const does not
    exist yet. The spec requires
    `.map(|c| if is_unassigned_in_unicode16(c) { '\u{FFFF}' } else { c })`. The iteration-133
    pre-filter is real non-conformance (IEP-0003 defines conformance as output-equivalence with the
    reference): deleting changes adjacency and unblocks canonical composition, Hangul jamo
    composition and `Final_Sigma` — **42 of 140 sequence cases** fail. The vendored 731-range table
    and its generator are unaffected; the fix is one token per call site.
- **Trap for the next step:** `decisions.md` contains an intermediate **category-override** design
    that a later entry explicitly SUPERSEDES. It scores 1 single-code-point and 10 sequence failures
    (`U+A7F1` decomposes to `S` under Unicode 17 and injects a spurious letter). Do not implement
    it. Lowering the declared version to 15.1.0 is likewise rejected.
- **Criterion 2 (table deps) — MET**, unchanged and explicitly freely-upgradable.
- **Criterion 3 (boundary vectors) — PARTIALLY met (Rust half).** `tests/unicode_boundary.json` +
    `tests/test_unicode_boundary.rs` unchanged. Binding propagation is unblocked on both former
    counts (Go table ruling **and** the Go `Final_Sigma` fix now landed) but must still follow the
    sentinel conversion, which changes expected sequence outputs.
- **Criterion 4 (differential sweep) — NOT started.** Wording is settled: **zero** divergence over
    the **1,112,064 Unicode scalar values** (not 1,114,112 — `&str` cannot carry surrogates) **and**
    sequence classes. A per-code-point-only sweep is explicitly forbidden as false assurance.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved. Inherits the core's
    freeze-rule fix automatically; only boundary-vector fixture wiring is missing.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched.

## WASM Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes 47 `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met — the critical Go defect is **fixed and independently verified**, pending CI

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. Only
    `packages/go` moved. Kotlin 2.3+ consumer floor stands.

- **`Final_Sigma` FIXED.** `packages/go/utils.go` `TextCollapse` now calls
    `cases.Lower(language.Und).String(norm.NFD.String(text))`;
    `grep -rn strings.ToLower packages/go/` returns only the test-only `codec_test.go:390`.
    **Verified by building a throwaway module against the package at HEAD** (not by trusting the
    handoff):

    | input      | shipped at HEAD | reference / Rust | verdict            |
    | ---------- | --------------- | ---------------- | ------------------ |
    | `ΛΟΓΟΣ`    | `λογος`         | `λογος`          | fixed              |
    | `ΑΣ`       | `ας`            | `ας`             | fixed              |
    | `ΣΣ`       | `σς`            | `σς`             | fixed              |
    | `ΑΣΒ`      | `ασβ`           | `ασβ`            | no over-correction |
    | `İstanbul` | `istanbul`      | `istanbul`       | no over-correction |

- Static gates re-run here: `gofmt -l .` prints nothing, `CGO_ENABLED=0 go vet ./...` exits 0,
    `git diff 9aa25ad..HEAD -- packages/go/go.mod packages/go/go.sum` is **empty** (no dependency
    change — `golang.org/x/text` was already required). Named `func Test` count **174** (was 165).

- The `cases.Caser` is constructed **per call**, with an in-source comment forbidding hoisting to a
    package-level var (x/text documents `Caser` as possibly stateful / not goroutine-safe). The
    issues.md fix sketch asked for a package-level Caser; the step deliberately deviated. Do not
    "optimize" this later — it is a data-race hazard, not dead cost (~140ns).

- Go's separate, *accepted* divergence (Unicode 15.0 stdlib tables) stands as ruled: Go **skips**
    the two table-dependent boundary vectors with a tracking note until go1.27. The `Final_Sigma`
    boundary vector is table-independent and is now **unblocked**.

## Documentation

**Status**: partially met — `docs/unicode.md` still contradicts the ruled design

- Page-list machinery intact and green: `uv run scripts/check_docs_nav.py` → 23 pages consistent
    across nav, `ORDERED_PAGES` and `llms.txt`; 12 crate/package READMEs and 12 CLAUDE.md files; 11
    `docs/howto/*.md`; `scripts/version_sync.py` covers 21 targets. No docs file moved this
    iteration.
- **Content still stale.** `docs/unicode.md` lines 29–40 describe the superseded mechanism: "code
    points unassigned in Unicode 16.0.0 are **removed** from the input **before** any normalization
    or category lookup" and "Because removal happens first…". Under the ruling, removal happens
    exactly where the reference does it and the `U+FFFF` sentinel is what provides table-invariance.
    Two further specified edits remain undone: widen the deliberately-scoped CPython-3.14 sentence
    (line ~73) back to unqualified agreement, and add a **"How much does this matter?"** section.
- `docs/howto/go.md:292` still carries only the Unicode-tables note, which remains accurate. It
    never documented the `Final_Sigma` bug, so the fix needs no docs edit there.
- Remaining cosmetic item: `Add programming language logos to docs site` (`low`, `[human]`).

## Benchmarks

**Status**: met

- `benches/benchmarks.rs`: 12 criterion benches (criterion 0.7). `benches/iai_benches.rs`:
    iai-callgrind 0.16, 11 functions / 16 cases. 18 pytest-benchmark fixtures; documented speedups
    of 1.3x–158x. Untouched.

## CI/CD and Publishing

**Status**: partially met — the last green run **no longer covers HEAD**

- **Latest CI: GREEN, but stale.** check-runs API on the real `origin/develop` tip `dd65825`: **43
    check-runs, 22 distinct check names, 0 non-success.** (~2x runs because PR **#44** `develop` →
    `main` is OPEN, titled "Release 0.6.0" — not shipped; version is still **0.5.0**.)
- **HEAD is 6 commits ahead of `origin/develop`**, and unlike the last three iterations
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **NOT empty** — 6 files / 114
    insertions under `packages/go/`. **The Go fix has never run through CI**, so the `go-test` job
    has not executed the 9 new tests, and no CI evidence exists for it. Pushing `develop` is the
    cheapest way to close this.
- **Iteration 147's review role produced no verdict:** `iterations.jsonl` records
    `"role":"review","status":"FAIL","turns":1,"cost_usd":0.000606` and there is **no `cid(review)`
    commit** for 147. `handoff.md` therefore contains only the advance section. Combined with the
    unpushed code this means the Go fix carries exactly one verification: the reproduction in this
    assessment. Two consequences: (a) the resolved `critical` issue was **not deleted** from
    `issues.md` (that is the review agent's job), so the count below over-reports by one; (b) if
    this failure mode repeats, the loop is running without its verification stage.
- ci.yml/`release.yml`/`docs.yml` byte-unchanged: 22 raw job entries → 20 YAML jobs → 21 jobs → 22
    check names, matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression`,
    `--fail-above 30.0`, 98-entry baseline), `cargo-deny`. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0` +
    inline comment (still `@main` at `release.yml:895` — verified unchanged); major dependency bumps
    **one per step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 /
    `magnus` 0.8 source rewrites, one crate per step); make the `specs/ci-cd.md` job table
    exhaustive.
- **Deferred by the ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid
    until 2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**10 entries in `issues.md` — 1 critical, 4 normal, 5 low — but the `critical` one is de-facto
resolved** and only still listed because iteration 147's review agent (which deletes resolved issues
after verification) never ran. Effective backlog: **4 normal, 5 low**. Zero `HUMAN REVIEW REQUESTED`
blocks remain — nothing is parked on Titusz.

- **`critical` — "`packages/go` lowercases without `Final_Sigma` context": fixed at HEAD**, verified
    above. Needs a CI run and a review verdict, then deletion from `issues.md`.
- **NORMAL, unblocked, now the top item:** "Convert the freeze rule from a pre-filter to a sentinel
    map (RULED)" — a 6-point work package (code, both docstrings, 4 regression tests + a normalizer
    pass-through assertion, hot-path gates, and the two `docs/unicode.md` rewrites).
- **NORMAL, unblocked:** "Declare and gate a Unicode data version" — remainder is copying the
    fixture into 11 binding suites and 4 sibling `data.json` locations. Both former blockers are
    cleared (Go table ruling + Go `Final_Sigma` fix); must follow the sentinel conversion.
- **NORMAL, unblocked:** the rubygems `@v2.1.0` pin; the `specs/ci-cd.md` exhaustive job table; the
    dependency issue's major-bump remainder (one per step).
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (all trigger-contingent — do
    not act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**The Unicode sentinel conversion**, exactly as the six-point work package in `issues.md` specifies.
It is the largest open correctness item, it gates every remaining Unicode task, and with the Go bug
fixed there is nothing sequenced ahead of it. Land in **one commit**: `utils.rs` lines 103/181
`filter` → `map` onto a named `U+FFFF` sentinel const, both function docstrings (they currently cite
pre-normalization ordering as the reason for invariance — wrong on both counts), the four **Verified
when** regression cases including `text_clean("e\u{A7F1}\u{0301}") == "e\u{0301}"` plus a normalizer
pass-through assertion, **and both `docs/unicode.md` rewrites** — the page currently documents a
mechanism the spec has rejected, so splitting it leaves the docs actively wrong.

**Two hot-path gates react and must be handled in the same commit:** the iai-callgrind perf gate
(10% Ir budget — expect Ir **flat**, since a `map` replaces a `filter` in the same fused iterator; a
significant move in either direction means the implementation is not the one specified) and the
CI-only CRAP `--fail-regression` check (a new branch in a covered function fails CI even when
`mise run check` is green locally — refresh `.crap-baseline.json` in the same step).

**Two process items worth doing first because they are nearly free:** push `develop` so the Go fix
gets CI coverage and the `go-test` job actually executes its 9 new tests; and if the review role
crashes again at turn 1, treat that as a loop defect rather than a silent PASS — the current
iteration shipped code with no review verdict at all.
