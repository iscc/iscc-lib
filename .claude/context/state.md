<!-- assessed-at: 2ae56046ecf469a14abc039a07b099c221244093 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — library code moved again after seven tooling iterations; the Unicode 16.0 boundary work is half landed (Rust core done, binding propagation blocked on a human ruling)

Iteration 141 landed the Unicode 16.0 boundary vectors as a Rust-core fixture plus loader — the
first non-config change since iteration 133. CI is green on the `origin/develop` tip. The remaining
half of that criterion (propagation into 11 bindings and four sibling `data.json` copies) is
double-blocked on the parked HUMAN REVIEW ordering ruling and the Go Unicode-15.0-tables decision.

## Rust Core Crate

**Status**: partially met — criterion 3 advanced but still unmet; criterion 4 unmet; semver/v1.0.0
held

- Incremental scope: `git diff 2cd0d6d..HEAD` touches exactly three non-`.claude` files —
    `crates/iscc-lib/tests/unicode_boundary.json` (new, 74 lines),
    `crates/iscc-lib/tests/test_unicode_boundary.rs` (new, 104 lines) and one bullet in
    `crates/iscc-lib/CLAUDE.md`. `.claude/context/specs/` and `target.md` are byte-identical to the
    previous assessment, so no criterion moved underneath the code. All binding, benchmark, docs and
    CI findings below are carried forward with spot re-checks.
- 8 crates in the workspace; `iscc-lib` now carries **328** `#[test]` functions (was 325; +3 from
    the new boundary target) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json`
    conformance vectors. Workspace version is still **0.5.0** — PR #44 is titled "Release 0.6.0" but
    is only the open `develop` → `main` PR, not a shipped release.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` before `.nfkc()` / `.nfd()`, with the checked-in
    PEP 723 generator `scripts/gen_unicode16_unassigned.py`. Untouched this iteration.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer".
- **Boundary conformance vectors (criterion 3) — PARTIALLY met (Rust half done).** Verified at HEAD:
    `tests/unicode_boundary.json` is a `data.json`-shaped, fully ASCII-escaped fixture with
    `_metadata.unicode_data_version = "16.0.0"` and 4 cases each under `text_clean` and
    `text_collapse`, covering U+1FAE9 (assigned `So`, retained), U+113C5 (assigned `Mc`, retained by
    clean / dropped by collapse), U+20C1 and U+A7F1 (unassigned in 16.0, stripped). The loader
    `tests/test_unicode_boundary.rs` has one **ungated** shape+content guard
    (`test_boundary_fixture_metadata`, which pins the exact non-ASCII code-point set against a
    `BOUNDARY_CODE_POINTS` constant so a hollowed-out fixture cannot pass silently) and two
    `text-processing`-gated vector tests. The spec sentence, however, requires the set to be
    exercised "by the Rust test suite **and** by every binding's conformance test" — a fresh grep
    confirms zero boundary code points in any binding or `packages/` tree, so the criterion as
    written is still unmet.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites
    in `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64. Nothing under
    `crates/iscc-py/` moved this iteration.
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (the workspace's only tracked `.pyi`) remains
    covered by both local ruff hooks since iteration 139.
- Pending: the Unicode 16.0 boundary vectors. The binding inherits the freeze-rule behaviour
    automatically — only the fixture wiring is missing, and the fixture it should copy now exists.

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
    2.3+ consumer floor closed in iteration 132 remains documented in all three consumer docs.
- **`packages/go` is the one binding that does not inherit the freeze rule** — it is an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the new boundary fixture unless the 15.0→16.0
    assigned delta is vendored or the vectors are skipped for Go with a tracking note. Both options
    are recorded; neither has been chosen. This is now the concrete blocker on finishing criterion
    3\.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; 11 `docs/howto/*.md` guides; llms-full generation
    over 22 ordered pages; MkDocs site with branding. `scripts/version_sync.py` covers 21 targets.
- The only documentation change this iteration is one bullet in `crates/iscc-lib/CLAUDE.md` pointing
    at `unicode_boundary.json` as the propagation source (verified present, single occurrence).
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

**Status**: partially met — dependency freshness is closed; what remains is human-gated or newly
filed gate work

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `b40b1bb`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `2ae5604` is one unpushed commit (`cid(log): iteration 141`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so the entire code change of
    this iteration is covered by the green run. Working tree is clean.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- `release.yml` unchanged since iteration 140: 97 `uses:` refs all at current majors, 29 jobs with
    28 carrying the `!cancelled() && !failure()` re-trigger guard, 8 registry toggles. It is
    `workflow_dispatch`-only, so neither the guards nor the action majors have ever executed —
    statically verified only, an accepted risk in `decisions.md` (2026-07-25).
- **Open gaps:** the hand-retyped `release.yml` static checks are still not an executable gate (open
    `[review]` issue); one floating-branch action ref
    (`rubygems/configure-rubygems-credentials@main`, needs a human tag-vs-SHA ruling); the `magnus`
    0.8 and `jni` 0.22 source rewrites plus deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk`
    18.x, Gradle 8.12.1, JUnit 6.x); npm OIDC trusted publishing (human-gated). No Dependabot or
    Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (unchanged count; no issue opened or closed this
iteration). **One `[review]` entry still carries a HUMAN REVIEW REQUESTED block**, so an item
remains parked on Titusz.

- **HUMAN-PARKED (carried from iteration 133):** "Freeze-rule ordering diverges from iscc-core on
    sequences". Stripping unassigned code points before normalization changes adjacency and unblocks
    contextual transforms the reference still blocks. The ruling needed is spec **wording**:
    criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as literally
    written, and the upstream `iscc-core` proposal must state pre-normalization removal. Filed
    upstream as iscc/iscc-core#137.
- "Declare and gate a Unicode data version (DECIDED)" — updated this iteration to record the Rust
    half of step (b) as done and to spell out the remainder: copy the fixture into 11 binding
    conformance suites and four sibling `data.json` locations. Explicitly still blocked on the
    ordering ruling **and** the Go decision.
- **CID-doable, unblocked:** "Land the `release.yml` static checks as an executable gate" — one
    `scripts/check_release_workflow.py`, a prek hook scoped to `^\.github/workflows/release\.yml$`
    for checks 1–2, a CI step for check 3 (needs network).
- **Human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`" (tag-vs-SHA reopens a
    repo convention); npm OIDC trusted publishing; the parent dependency issue, now reduced to
    human/major-gated bumps only.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**The `release.yml` static-check script is now the only fully unblocked CID-doable item.** The
Unicode thread has gone as far as it can without Titusz: step (b) requires deriving expected values
for 11 bindings under an ordering rule that may still change wording, and Go additionally needs a
vendor-or-skip decision. Picking it up now means doing the derivation twice. The static-check script
is small, self-contained, and converts invariants that have been hand-retyped into throwaway
heredocs in three consecutive iterations (139, 140, 141) into a re-running gate for the one file
that no CI or CID push ever exercises. Suggested slicing: land checks 1–2 (pure-local, no network)
as a prek hook first; leave check 3 (action-input validation, needs network) as a follow-up CI-only
step.

**Caveat on cadence:** this would be a tooling package again, but the alternative is blocked rather
than merely less attractive, so it is justified this round. If a second consecutive tooling
iteration is proposed after it, treat that as drift and escalate the two human-parked rulings
instead.

**Do not pick:** Unicode step (b) binding propagation (double-blocked), or the
`rubygems/configure-rubygems-credentials` pin (needs the tag-vs-SHA ruling).

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
