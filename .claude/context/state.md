<!-- assessed-at: c59749662860392747199d62a3bf74bd9f5fdb6a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the Unicode freeze rule is now the ruled `U+FFFF` sentinel map, verified conformant and green in CI; the remaining Unicode work is propagation (sequence vectors, bindings) and the differential sweep

Iteration 148 landed the sentinel conversion in one commit and its review PASSED with an independent
1,270-case sequence differential (0 mismatches). Everything is pushed — HEAD **is** `origin/develop`
— and CI on that exact commit is **fully green** (43 check-runs, 22 distinct names, 0 non-success),
so for the first time in three iterations the green run actually covers HEAD. Rust-core criterion 1
flips from unmet to **met**; criteria 3 (partial) and 4 (not started) are what remain.

## Rust Core Crate

**Status**: partially met — criterion 1 **now met**; criterion 3 partial; criterion 4 not started

- Incremental scope: `git diff c2aabf5..HEAD --stat` = 26 files, **8 outside `.claude/`**:
    `crates/iscc-lib/src/utils.rs`, `crates/iscc-lib/CLAUDE.md`,
    `crates/iscc-lib/tests/test_unicode_boundary.rs`, `docs/unicode.md`,
    `scripts/gen_unicode16_unassigned.py` (docstring only), `.crap-baseline.json`, plus two human
    `fix(cid)` commits touching `tools/cid.py` / `tests/test_cid.py`.
    `git diff c2aabf5..HEAD -- .claude/context/specs/` is **empty** — no spec moved, so no met→unmet
    flip from the spec side this time.
- 8 crates; `iscc-lib` now carries **334** `#[test]` functions (was 328 — six new sentinel
    regression tests). All 10 `gen_*_v0` functions still pass the vendored `iscc-core/data.json`
    vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — MET.** Verified in the source at HEAD, not from the handoff:
    `utils.rs:35` defines `const UNASSIGNED_SENTINEL: char = '\u{FFFF}'` and **both** call sites
    (`text_clean` L118-126, `text_collapse` L209-217) now use
    `.map(|c| if is_unassigned_in_unicode16(c) { UNASSIGNED_SENTINEL } else { c })` inside the same
    fused iterator. **Zero `.filter(|&c| !is_unassigned_in_unicode16` matches remain.** The
    category-`C`/`CMP` filters are byte-unchanged from the reference, and the vendored 731-range /
    819,533-code-point table is untouched (`test_unicode16_table_invariants` asserts both numbers).
- Every **Verified when** box in the spec's freeze-rule cluster is backed by a named test at HEAD:
    `test_sentinel_blocks_canonical_composition` (`e\u{0378}\u{0301}` → `e\u{0301}`, not `é`),
    `test_sentinel_blocks_hangul_jamo_composition` (no `U+AC00`),
    `test_sentinel_preserves_final_sigma_context` (`ς`, not `σ`),
    `test_sentinel_prevents_decomposition_leak` (`e\u{A7F1}\u{0301}` → `e\u{0301}`, not `eŚ` — the
    case the superseded category-override design gets wrong),
    `test_normalizer_passes_sentinel_through` (the design's one assumption, asserted for NFKC *and*
    NFD) and `test_literal_sentinel_input_removed`.
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0) both qualify and are explicitly freely-upgradable.
- **Criterion 3 (boundary vectors) — PARTIALLY met.** `tests/unicode_boundary.json` still holds only
    the **4 single-code-point** vectors; `tests/test_unicode_boundary.rs` moved this iteration but
    only in doc-comments. Two gaps: (a) the **four sequence vectors are absent**, and the existing
    single-code-point vectors wrap each code point in ASCII so they are structurally
    *deletion-vs-sentinel agnostic* — nothing in the fixture can currently distinguish the ruled
    design from the superseded one; (b) `grep -rn unicode_boundary` finds **no** reference outside
    `crates/iscc-lib/`, so zero of the 11 bindings and none of the 4 sibling `data.json` copies have
    the fixture. Both former blockers (Go table ruling, Go `Final_Sigma`) are cleared.
- **Criterion 4 (differential sweep) — NOT started.** `ls scripts/` confirms no sweep harness is
    checked in. Review's 1,270-case probe at 148 was ad hoc and is gone with the session. The
    requirement is **zero** divergence over both the **1,112,064 Unicode scalar values** (surrogates
    excluded — `&str` cannot carry them) **and** sequence classes; a per-code-point-only sweep is
    explicitly forbidden as false assurance (it scored the non-conformant pre-filter 0).
- **Stale spec text worth a human's attention (not CID's to edit):** `specs/rust-core.md` still
    describes the Go `Final_Sigma` defect in the present tense ("`packages/go` currently has one:
    `TextCollapse` uses `strings.ToLower`") and leaves its **Verified when** box unchecked, although
    the fix landed in iteration 147 and is verified below.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, **12** `.detach(` GIL-release sites
    (re-counted); abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved. Inherits
    the sentinel fix automatically; only boundary-vector fixture wiring is missing.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched this iteration.

## WASM Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs (re-counted); cbindgen
    header plus csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met — the Go `Final_Sigma` fix has now passed CI

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. Nothing moved
    this iteration. Kotlin 2.3+ consumer floor stands.
- **Go `Final_Sigma`: fixed, and now CI-covered.** `packages/go/utils.go:125` uses
    `cases.Lower(language.Und).String(norm.NFD.String(text))`; the only remaining `strings.ToLower`
    is the test-only `codec_test.go:390`. **174** named `func Test`. The iteration-147 code finally
    ran through the `go-test` job on this push and passed — previously it had only my own local
    reproduction behind it.
- The per-call `cases.Caser` is deliberate (x/text documents `Caser` as possibly stateful / not
    goroutine-safe) with an in-source comment forbidding hoisting. Do not "optimize" it.
- Go's separate, *accepted* divergence (Unicode 15.0 stdlib tables) stands as ruled: Go **skips**
    the two table-dependent boundary vectors until go1.27. The `Final_Sigma` sequence vector is
    table-independent and Go **must** take it once the fixture gains it.

## Documentation

**Status**: met

- **`docs/unicode.md` now matches the ruling** — the previously-stale page was rewritten in the same
    commit as the code. Verified at HEAD: the freeze rule is described as "replaced by the
    noncharacter sentinel `U+FFFF`" with the category filter left "exactly as the reference
    implementation defines it"; the two load-bearing properties (removal happens where the reference
    performs it; `U+FFFF` is permanently `Cn` so tables may be upgraded freely) are both stated; the
    banned "removed … before any normalization" framing is gone. The two other specified edits also
    landed: a **"How much does this matter?"** section (line 27) and the widened CPython-3.14
    sentence ("CPython 3.14 ships Unicode 16.0.0 tables and agrees with iscc-lib").
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt`. 12 crate/package READMEs, 12 CLAUDE.md files, 11
    `docs/howto/*.md`, `scripts/version_sync.py` covers 21 targets. `crates/iscc-lib/CLAUDE.md` was
    updated in step with the sentinel design.
- `docs/howto/go.md:292` keeps only the Unicode-tables note, which remains accurate.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`,
    `[human]`). One nit flagged by review is **not** CID-actionable: the page calls Todhri a "living
    script", verbatim from `specs/rust-core.md`; changing it needs a human and does not affect the
    argument.

## Benchmarks

**Status**: met

- `benches/benchmarks.rs`: 12 criterion benches (criterion 0.7). `benches/iai_benches.rs`:
    iai-callgrind 0.16, 11 functions / 16 cases. 18 pytest-benchmark fixtures; documented speedups
    of 1.3x–158x. Untouched. The sentinel conversion moved Ir **down** slightly
    (`bench_text_code.chars_1000` −3.89%) — well inside the 10% budget, and `.iai-baseline.json` was
    correctly left untouched.

## CI/CD and Publishing

**Status**: partially met — but green and, for once, covering HEAD

- **Latest CI: GREEN on HEAD.** check-runs API on `c597496` (= `origin/develop` = HEAD): **43
    check-runs, 22 distinct check names, 0 non-success, none in progress.** `origin/develop..HEAD`
    is empty — nothing unpushed, so this run genuinely covers all working-tree code. (~2x runs
    because PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — not shipped; version is
    still **0.5.0**.)
- ci.yml / `release.yml` / `docs.yml` byte-unchanged: 22 raw job entries → 20 YAML jobs → 21 jobs →
    22 check names, matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry
    toggles, `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression`,
    `--fail-above` with `.cargo-crap.toml`'s 30.0 threshold; baseline refreshed to **100** entries
    in the same commit, which is the correct handling), `cargo-deny`. `cargo-semver-checks`
    informational.
- **The iteration-148 review timeout was a loop defect and has been fixed by a human.** The log
    records `"role":"review","status":"TIMEOUT","turns":0,"duration_s":3000.1` even though review
    *did* commit (`b9a600c`, `82ec3ac`). Two `fix(cid)` commits followed: `4739a4b` adds
    `_role_commit_landed()` so a role that commits then overruns no longer fails the iteration (with
    94 lines of new `tests/test_cid.py` coverage), and `c597496` raises the review timeout from
    3000s to 3600s to match advance. Both are covered by the green `python-test` job. This closes
    the process gap flagged at 148 — do not re-raise it.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0` +
    inline comment (still `@main` at `release.yml:895` — verified unchanged); major dependency bumps
    **one per step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 /
    `magnus` 0.8 source rewrites, one crate per step); make the `specs/ci-cd.md` job table
    exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
Review correctly deleted both resolved entries (the Go `Final_Sigma` critical and the sentinel
conversion) and folded their residuals into the umbrella Unicode issue rather than dropping them.
Nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", now the only correctness item.**
    Two tracked remainders: **(a2)** the criterion-4 differential sweep wired in as a runnable check
    (scalar values *and* sequence classes), and **(b)** the four sequence vectors plus propagation
    to 11 bindings and 4 sibling `data.json` copies. Its `**Upstream:**` section carries a
    human-owned task (update `iscc-core#137` to propose the sentinel mechanism) — explicitly *not*
    CID's.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (all trigger-contingent — do
    not act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Add the four sequence vectors to `crates/iscc-lib/tests/unicode_boundary.json`** and extend the
ungated content guard in `tests/test_unicode_boundary.rs` to assert them by code-point set rather
than by shape. This is the right next step for a specific reason: the fixture is the **propagation
source** for all 11 bindings, and today it cannot gate the distinction that iteration 148 just fixed
— every existing vector wraps a single code point in ASCII, so it scores the deleted pre-filter and
the sentinel map identically. Until the sequence vectors land, the sentinel behaviour is guarded by
unit tests in one crate only.

The four expected outputs are already pinned by the six `utils.rs` tests and tabulated as ASCII
escapes in the issues.md umbrella entry (`e͸́` → `é`; `ᄀ͸ᅡ` → `가`; `e꟱́` → `é`; `ΑΣ͸Β` → `αςβ`) —
**do not re-derive them.** The fixture is ASCII-escaped, and composed vs. decomposed forms render
identically, so escapes are mandatory.

After that, two roughly equal-sized follow-ups in either order: propagate the fixture to the 11
bindings and the 4 sibling `data.json` locations (Go skips the two table-dependent vectors per the
ruling but **must** take the `Final_Sigma` one — its blocker is gone), and wire the criterion-4
differential sweep in as a runnable check covering both the 1,112,064 scalar values and the sequence
classes. Neither touches a hot path, so no perf or CRAP gate should react; expect a
docs-nav-neutral, fixture-only diff.
