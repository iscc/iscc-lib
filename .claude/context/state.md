<!-- assessed-at: 8183ffb44567717a1e2845c7ea3ff9346021102a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the Unicode boundary fixture is now complete and design-discriminating in the Rust core; what remains is propagation to the bindings and the differential sweep

Iteration 149 added the four multi-code-point sequence vectors, closing the gap flagged at 148: the
fixture can now actually distinguish the ruled `U+FFFF` sentinel map from the superseded
delete-filter design, and review mutation-probed four degradation modes to prove the guards are
non-vacuous. CI is green on the pushed tip and covers every line of working-tree code. Rust-core
criterion 3 is still partially met — the Rust half is done, but zero of the 11 bindings and none of
the 4 sibling `data.json` copies have the fixture yet.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 half done (Rust complete, bindings
zero); criterion 4 not started

- Incremental scope: `git diff c597496..HEAD --stat` = 19 files, **3 outside `.claude/`**:
    `crates/iscc-lib/tests/unicode_boundary.json`, `crates/iscc-lib/tests/test_unicode_boundary.rs`,
    `docs/unicode.md`. `git diff c597496..HEAD -- .claude/context/specs/` is **empty** and
    `-- .github/` is empty — no spec moved, no workflow moved, so no met→unmet flip from either
    side. No file under `crates/iscc-lib/src/` changed (review confirmed the same).
- 8 crates; `iscc-lib` now carries **335** `#[test]` functions (was 334 — one new fixture guard).
    All 10 `gen_*_v0` functions still pass the vendored `iscc-core/data.json` vectors. Version
    **0.5.0**.
- **Criterion 1 (freeze rule) — MET**, re-verified in the source at HEAD: `utils.rs:35` defines
    `const UNASSIGNED_SENTINEL: char = '\u{FFFF}'`, both call sites (L122, L213) map to it inside
    the fused iterator, and `grep -c "filter(|&c| !is_unassigned_in_unicode16"` is **0** — the
    superseded pre-normalization delete filter is fully gone. Category-`C`/`CMP` filters unchanged
    from the reference; vendored 731-range / 819,533-code-point table untouched.
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0) both qualify and are explicitly freely-upgradable.
- **Criterion 3 (boundary vectors) — PARTIALLY met. The Rust half is now COMPLETE.**
    `tests/unicode_boundary.json` holds **7 `text_clean` + 5 `text_collapse`** cases: the original 4
    single-code-point vectors plus the four sequence vectors (`e U+0378 U+0301` → `e U+0301`;
    `U+1100 U+0378 U+1161` → `U+1100 U+1161`; `e U+A7F1 U+0301` → `e U+0301`;
    `U+0391 U+03A3 U+0378 U+0392` → `U+03B1 U+03C2 U+03B2`). Verified by decoding the fixture to
    numeric code points, not by reading glyphs.
    - **The structural weakness reported at 149 is fixed.** Every sequence row records an output that
        a delete filter would *not* produce, so an implementation that deletes rather than maps now
        fails the gated vector tests. Two **ungated** guards back this:
        `test_boundary_fixture_metadata` (version string, derived per-section counts, exact non-ASCII
        code-point set) and the new `test_boundary_fixture_sequence_vectors` (exactly-one match per
        input, expected equality, delete-filter inequality). Both are driven by consts in the test
        source, not by the fixture, so neither is self-referential.
    - **Remaining gap: propagation is still zero.** `grep -rn unicode_boundary` outside
        `crates/iscc-lib/` finds only context/memory files — the one apparent hit in
        `tests/test_text_utils.py` is an unrelated function name (`test_text_trim_unicode_boundary`).
        None of the 11 bindings and none of the 4 sibling `data.json` copies (`packages/go/testdata/`,
        `packages/dotnet/Iscc.Lib.Tests/testdata/`, `packages/swift/Tests/IsccLibTests/`,
        `packages/kotlin/src/test/resources/`) carry the fixture. The spec requires "every binding's
        conformance test".
- **Criterion 4 (differential sweep) — NOT started.** `ls scripts/` confirms no sweep harness is
    checked in (9 entries, none of them a sweep). Review's 1,270-case probe at 148 was ad hoc and is
    gone with the session. The requirement is **zero** divergence over both the **1,112,064 Unicode
    scalar values** (surrogates excluded — `&str` cannot carry them) **and** sequence classes; a
    per-code-point-only sweep is explicitly forbidden as false assurance.
- **A wrong oracle value reached published docs this iteration and was caught by review, not by a
    gate.** next.md tabulated `e U+015A` as the "delete filter" output for the `U+A7F1` row; that is
    the *category-override* design's output — a delete filter yields `U+00E9`. Review corrected the
    const, the docs cell and the issues.md source table, and added a "do not relabel this back"
    note. Relevant now because that table is the propagation source for 11 bindings.
- **Stale spec text worth a human's attention (not CID's to edit):** `specs/rust-core.md:152-157`
    still describes the Go `Final_Sigma` defect in the present tense and leaves its **Verified
    when** box unchecked, although the fix landed at iteration 147 and has since passed CI. The
    criterion-1 box (line 169) is likewise unchecked despite being met — spec checkboxes are not a
    progress signal here.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved. Inherits the sentinel
    fix automatically; only boundary-vector fixture wiring is missing.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched this iteration.

## WASM Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes 47 `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. Nothing moved
    this iteration. Kotlin 2.3+ consumer floor stands.
- Go `Final_Sigma` remains fixed and CI-covered (`packages/go/utils.go:125` uses
    `cases.Lower(language.Und)`; the per-call `cases.Caser` is deliberate — do not hoist it). 174
    named `func Test`.
- Go's separate, *accepted* divergence (Unicode 15.0 stdlib tables) stands as ruled: Go **skips**
    the two table-dependent boundary vectors until go1.27. The `Final_Sigma` sequence vector is
    table-independent and Go **must** take it — that vector now exists, so this is live work.

## Documentation

**Status**: met

- `docs/unicode.md` gained a sequence-vector section in step with the fixture: a four-row table
    (input / function / expected output / delete-filter output) plus a paragraph explaining why the
    ASCII-wrapped single code points above it cannot make the distinction, and a note that the
    `U+A7F1` row additionally guards the category-override hazard. The row-3 oracle was corrected to
    `U+00E9` by review before the commit landed.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt` (no page added). 12 crate/package READMEs, 12 CLAUDE.md
    files, 11 `docs/howto/*.md`, `scripts/version_sync.py` covers 21 targets.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`,
    `[human]`).

## Benchmarks

**Status**: met

- `benches/benchmarks.rs`: 12 criterion benches (criterion 0.7). `benches/iai_benches.rs`:
    iai-callgrind 0.16, 11 functions / 16 cases. 18 pytest-benchmark fixtures; documented speedups
    of 1.3x–158x. Untouched — this iteration was test-fixture and docs only, so no perf surface
    moved and `.iai-baseline.json` was correctly left alone.

## CI/CD and Publishing

**Status**: partially met — green and covering all working-tree code

- **Latest CI: GREEN.** check-runs API on `f2aeeac` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** HEAD is `8183ffb`, one commit ahead — but
    that commit is `cid(log): iteration 149` and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty**, so the green run covers
    every line of code. (~2x runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- ci.yml / `release.yml` / `docs.yml` byte-unchanged since the last assessment: 22 raw job entries →
    20 YAML jobs → 21 jobs → 22 check names, matching the API exactly. `release.yml` keeps 97
    `uses:` refs, 8 registry toggles, `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44
    checked / 8 unchecked.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression`,
    `--fail-above` 30.0; baseline held at 100 entries and correctly untouched — review's CI-exact
    re-run reported 0 regressed / 0 new / 2 moved / 98 unchanged), `cargo-deny`.
    `cargo-semver-checks` informational.
- The iteration-148 loop defect (a role that commits then overruns failing the iteration) stays
    fixed — iteration 149 logged three clean `OK` roles and a `PASS_WITH_NOTES` summary. Do not
    re-raise it.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0` +
    inline comment (verified still `@main` at `release.yml:895`); major dependency bumps **one per
    step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8
    source rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
Unchanged in count; the umbrella Unicode issue was updated rather than closed, correctly, since only
half of its remainder (b) is done. Nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Two tracked remainders: **(b)** propagate the fixture to the 11 bindings and the 4 sibling
    `data.json` copies (the sequence-vector half is now ✅ with mutation evidence), and **(a2)** the
    criterion-4 differential sweep wired in as a runnable check (scalar values *and* sequence
    classes). Its `**Upstream:**` section carries a human-owned task (update `iscc-core#137` to
    propose the sentinel mechanism) — explicitly *not* CID's.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (all trigger-contingent — do
    not act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Propagate `unicode_boundary.json` to the bindings and the four sibling `data.json` locations.**
This is the right next step because the Rust fixture is now complete *and* design-discriminating —
it is a finished propagation source, and until it reaches the bindings the sentinel behaviour is
guarded in exactly one crate while 11 language surfaces ship ungated. Go takes the `Final_Sigma`
sequence vector (its blocker was cleared at 147) and skips the two table-dependent single-code-point
vectors per the 2026-07-26 ruling.

Slice it — the diff is wide by nature (11 bindings × conformance loaders + 4 `data.json` copies) and
each loader needs individual inspection: the fixture is a *second* vector file, so a binding that
hard-codes `data.json` needs new plumbing rather than a copy. A sensible first slice is the four
sibling `data.json` locations, then bindings in language groups.

Two hazards to carry into that work. First, every sibling fixture is ASCII-escaped and the Edit tool
decodes `\uXXXX` into literal UTF-8 — edit through Python (`json.dumps(..., ensure_ascii=True)`) and
verify with `raw.isascii()` plus numeric `ord()` checks, never by looking at glyphs. Second, the
expected outputs and the delete-filter oracles are pinned in `issues.md` and in the
`SEQUENCE_VECTORS` const **after** review's correction — copy those, and do not relabel the
`e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the category-override design's output,
not a delete filter's).

The remaining Unicode item after propagation is **(a2)**, the criterion-4 differential sweep as a
runnable check covering both the 1,112,064 scalar values and the sequence classes. Neither task
touches a hot path, so no perf or CRAP gate should react.
