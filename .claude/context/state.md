<!-- assessed-at: 9aa25adfd545a9ffffe973df059261d2d29bb48c -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — a human ruling reversed the freeze-rule design and filed a critical Go conformance bug; the human backlog is cleared and work is unblocked

HEAD is a `human(decide)` commit that changes **no code** but rewrites `specs/rust-core.md`: the
Unicode freeze rule must now **map** unassigned code points to the noncharacter `U+FFFF`, not delete
them. The shipped implementation still deletes, so a criterion that read *met* for 14 iterations is
now **unmet** — a spec-side regression, not a code-side one. The same commit files a **critical**
wrong-output bug in `packages/go` (missing `Final_Sigma` case mapping), which I reproduced. CI is
green and, for the first time in several iterations, nothing is parked on Titusz.

## Rust Core Crate

**Status**: partially met — criterion 1 **regressed to unmet by the ruling**; 3 partial; 4 unmet

- Incremental scope: `git diff 57ebcb8..HEAD` touches 21 files, but only **6 outside `.claude/`**:
    `.pre-commit-config.yaml`, `docs/development.md`, `scripts/check_docs_nav.py`,
    `scripts/check_release_workflow.py`, `tests/test_check_docs_nav.py`,
    `tests/test_check_release_workflow.py` (all iteration 146's gate-hardening). **No `crates/`, no
    `packages/`.** The load-bearing change is `.claude/context/specs/rust-core.md` (+146/-27) — the
    exact case my memory flags: a `human(decide)` commit flipping met→unmet with zero code change.
- 8 crates; `iscc-lib` carries **328** `#[test]` functions (re-counted at HEAD, unchanged); all 10
    `gen_*_v0` functions still pass the vendored `iscc-core/data.json` vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — NOW UNMET.** The spec requires
    `.map(|c| if is_unassigned_in_unicode16(c) { '\u{FFFF}' } else { c })`. I read the source:
    `crates/iscc-lib/src/utils.rs` **lines 103 and 181 still hold
    `.filter(|&c|   !is_unassigned_in_unicode16(c))`** — the iteration-133 pre-filter. The ruling
    classes that as real non-conformance (IEP-0003 defines conformance as output-equivalence with
    the reference): deleting changes adjacency and unblocks canonical composition, Hangul jamo
    composition and `Final_Sigma` — **42 of 140 sequence cases** fail. The vendored 731-range table
    and its generator are unaffected; the fix is one token per call site.
- **Trap for the next step:** `decisions.md` contains an intermediate **category-override** design
    (line 514) that a later entry (line 640) explicitly SUPERSEDES. It scores 1 single-code-point
    and 10 sequence failures (`U+A7F1` decomposes to `S` under Unicode 17 and injects a spurious
    letter). Do not implement it. Lowering the declared version to 15.1.0 is likewise rejected.
- **Criterion 2 (table deps) — MET**, unchanged and now explicitly freely-upgradable.
- **Criterion 3 (boundary vectors) — PARTIALLY met (Rust half).** `tests/unicode_boundary.json` +
    `tests/test_unicode_boundary.rs` unchanged. Binding propagation is **now unblocked** (Go ruling
    landed) but must follow the sentinel conversion, since that changes expected sequence outputs.
- **Criterion 4 (differential sweep) — NOT started**, but its wording is now settled: it demands
    **zero** divergence over the **1,112,064 Unicode scalar values** (not 1,114,112 — `&str` cannot
    carry surrogates) **and** sequence classes. A per-code-point-only sweep is explicitly forbidden.
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

**Status**: partially met — **Go carries a critical, reproduced conformance defect**

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. No package
    source moved. Kotlin 2.3+ consumer floor stands.

- **CRITICAL — `packages/go/utils.go:117` uses `strings.ToLower`**, which applies unconditional
    simple case mapping and never the conditional `Final_Sigma` special case. **Reproduced at HEAD**
    by running both mappings against the existing `golang.org/x/text` dependency:

    | input   | `strings.ToLower` (shipped) | `cases.Lower(language.Und)` (correct) |
    | ------- | --------------------------- | ------------------------------------- |
    | `ΛΟΓΟΣ` | `λογοσ`                     | `λογος`                               |
    | `ΑΣ`    | `ασ`                        | `ας`                                  |
    | `ΑΣΒ`   | `ασβ`                       | `ασβ` (agrees — Σ followed by cased)  |

    This is **not** bounded by Unicode-version drift or rare code points: word-final sigma is
    pervasive in Greek (`-ος`, `-ης`, `-ας`), so any uppercase or mixed-case Greek text yields a
    different Text-Code and Meta-Code from Go than from every other implementation. Already shipped
    in v0.5.0. The vendored vectors contain no Greek, which is why CI never caught it. The fix needs
    no new dependency — `golang.org/x/text` is already required.

- Go's separate, *accepted* divergence (Unicode 15.0 stdlib tables) is now **ruled**: Go **skips**
    the two table-dependent boundary vectors with a tracking note until go1.27, rather than
    vendoring the 5,813-code-point delta. This does **not** cover the `Final_Sigma` vector —
    `U+0378` is unassigned in Go's 15.0 tables too, so that vector is table-independent and the
    `ToLower` bug is its only blocker.

## Documentation

**Status**: partially met — **downgraded**: `docs/unicode.md` now contradicts the ruled design

- The docs *page-list* machinery is intact and green: `uv run scripts/check_docs_nav.py` exits 0
    with `OK: 23 documentation pages consistent across nav, ORDERED_PAGES and llms.txt.`; 12
    crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md`; `scripts/version_sync.py`
    covers 21 targets.
- **But the content is now stale.** `docs/unicode.md` lines 29–40 still describe the superseded
    mechanism — "code points unassigned in Unicode 16.0.0 are **removed** from the input before any
    normalization or category lookup" and "Because removal happens first…". Under the ruling,
    removal happens exactly where the reference does it; the sentinel is what provides
    table-invariance. Two further edits are specified and undone: widen the deliberately-scoped
    CPython-3.14 sentence (line 74, "agrees … on the single-code-point behaviour") back to
    unqualified agreement, and add a **"How much does this matter?"** section so the page reads
    proportionally.
- Nothing was deleted or broken this iteration — this is documentation that describes an
    implementation which itself must change. It should land in the **same step** as the sentinel
    conversion, not as separate follow-up work.
- Remaining cosmetic item: `Add programming language logos to docs site` (`low`, `[human]`).

## Benchmarks

**Status**: met

- `benches/benchmarks.rs`: 12 criterion benches (criterion 0.7). `benches/iai_benches.rs`:
    iai-callgrind 0.16, 11 functions / 16 cases. 18 pytest-benchmark fixtures; documented speedups
    of 1.3x–158x. Untouched.

## CI/CD and Publishing

**Status**: partially met — all automated gates green; the remaining items are now mostly authorized

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `dd65825`:
    **43 check-runs, 22 distinct check names, 0 non-success.** (~2x runs because PR #44 `develop` →
    `main` is open, so each push fires both a `push` and a `pull_request` run.)
- **HEAD is 2 commits ahead of `origin/develop`** (`f35fade` cid(log), `9aa25ad` human(decide)), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — both are context-only, so
    the green run covers all code at HEAD.
- ci.yml unchanged: 22 raw job entries → 20 YAML jobs → 21 jobs → 22 check names, matching the API
    exactly. `release.yml` and `docs.yml` byte-unchanged; `release.yml` keeps 97 `uses:` refs, 8
    registry toggles, `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Iteration 146's gate hardening is in the tree and CI-covered: `check_release_workflow.py` (509 L,
    397 L of tests) and `check_docs_nav.py` (143 L, 160 L of tests).
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression`,
    `--fail-above 30.0`, 98-entry baseline), `cargo-deny`. `cargo-semver-checks` informational.
- **Newly authorized for CID by the ruling** (previously human-blocked): pin
    `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0` + inline comment (still
    `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit 3.x, Test.Sdk
    18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source rewrites — one crate
    per step); make the `specs/ci-cd.md` job table exhaustive (14 rows vs 21 jobs — no longer just
    spec drift for Titusz, now an authorized work item).
- **Deferred by the ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid
    until 2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**10 open — 1 critical, 4 normal, 5 low** (was 9: +2 new, −1 resolved, 1 downgraded). **Zero
`HUMAN REVIEW REQUESTED` blocks remain** — the ruling cleared the entire human backlog, so for the
first time in three iterations nothing is parked on Titusz.

- **CRITICAL, and Titusz explicitly sequenced it first:** "`packages/go` lowercases without
    `Final_Sigma` context". Ships in v0.6.0, no separate patch release.
- **NORMAL, unblocked:** "Convert the freeze rule from a pre-filter to a sentinel map (RULED)" — a
    6-point work package (code, docstrings, 4 regression tests + a normalizer pass-through
    assertion, hot-path gates, and the two `docs/unicode.md` rewrites).
- **NORMAL, unblocked:** "Declare and gate a Unicode data version" — remainder is copying the
    fixture into 11 binding suites and 4 sibling `data.json` locations. Both former blockers are
    cleared; must follow the sentinel conversion.
- **NORMAL, unblocked:** the rubygems `@v2.1.0` pin; the `specs/ci-cd.md` exhaustive job table; the
    dependency issue's major-bump remainder (one per step).
- **LOW / CID skips:** gate-script remainders deliberately deferred at 146 (all three are
    trigger-contingent — do not act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Fix the `packages/go` `Final_Sigma` bug — Titusz named it explicitly: "fix this next, ahead of the
freeze-rule work below."** It is the only `critical` issue, it is a wrong-output defect on ordinary
Greek text in an already-published package, the fix is small and needs no new dependency, and it
unblocks the `Final_Sigma` boundary vector for Go. Replace `strings.ToLower(norm.NFD.String(text))`
in `TextCollapse` with a **package-level** `cases.Caser` (construct once — `cases.Lower` allocates a
transformer per call), add Greek regression cases to `utils_test.go`, and audit `TextClean` and the
codec helpers for other `strings.ToLower` uses (`codec_test.go:390` is test-only and fine).

**Then the sentinel conversion** — the largest correctness item, and the one that gates everything
Unicode-shaped. Land the `utils.rs` change, both docstrings, the four **Verified when** regression
cases, and **both `docs/unicode.md` rewrites in the same commit**, since the page currently
documents a mechanism the spec has rejected. Only after that should the boundary vectors be
propagated into the bindings, because the sentinel changes the expected output of every sequence
vector.

**Cadence note, resolved.** The previous three states flagged four tooling-ish steps in six and
asked for genuinely unblocked user-facing work. The ruling supplied it: the next two steps are both
user-visible correctness fixes. The escalation request that stood for three iterations is discharged
and should not be repeated.

**Two gates react to any change on the text hot path** and must be handled in the same commit as the
sentinel conversion: the iai-callgrind perf gate (10% Ir budget — expect Ir to be **flat**, since a
`map` replaces a `filter` in the same fused iterator; a significant move in either direction means
the implementation is not the one specified) and the CI-only CRAP `--fail-regression` check (a new
branch in a covered function fails CI even when `mise run check` is green locally).
