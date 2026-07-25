<!-- assessed-at: f18c5d578ec38cff34308f460885ac8dc8d50211 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI green; Kotlin floor closed, Unicode freeze rule is the open headline

v0.5.0 is released across all registries and CI is fully green (41 check-runs, 21 distinct names, 0
non-success). Iteration 132 closed the cheapest of the four criteria the human decisions added last
cycle — the Kotlin 2.3+ consumer floor is now documented in all three consumer-facing docs and its
issue is gone from `issues.md` (7 → 6 open). What remains is the substantial part: the three Unicode
16.0.0 freeze-rule criteria, which are fully specified and entirely unimplemented.

## Rust Core Crate

**Status**: partially met — 3 unmet Unicode criteria plus the held semver/v1.0.0 criterion

- 8 crates in the workspace; `iscc-lib` carries **320** `#[test]` functions and all 10 `gen_*_v0`
    functions pass the vendored `iscc-core/data.json` conformance vectors.
- **Unicode 16.0.0 freeze rule — NOT started.** Verified directly in `crates/iscc-lib/src/utils.rs`:
    `is_c_category` / `is_cmp_category` classify via
    `unicode_general_category::get_general_category` and match `GeneralCategory::Unassigned` at
    lookup time. That uses whatever table version the *live* crate ships (currently 16.0.0), so
    today's output happens to coincide with the declared version — but the freeze *guarantee* does
    not exist. There is no vendored range table and no generator script (`scripts/` holds only
    `build_xcframework.sh`, `gen_llms_full.py`, `iai_regression.py`, `test_install.py`,
    `version_sync.py`).
- **Boundary conformance vectors — absent.** Grepped `crates/`, `packages/`, and `scripts/` for the
    three spec-named boundary code points (`U+1FAE9`, `U+113C5`, `U+20C1`): zero hits anywhere.
- **Full-code-space differential sweep — absent.**
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, and 12 GIL-release sites in
    `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64.
- Pending: the Unicode 16.0 boundary conformance vectors, which the spec requires in *every*
    binding's conformance test. This is a Rust-core-led change; the binding is otherwise complete.

## Node.js Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes.
- Pending: the same Unicode 16.0 boundary conformance vectors.

## WASM Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3` `wasm32_simd`
    feature dep is intentional feature-unification (do not prune).
- Pending: the same Unicode 16.0 boundary conformance vectors.

## C FFI

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`.
- Pending: the same Unicode 16.0 boundary conformance vectors.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met — the Kotlin floor gap closed this iteration

- **Kotlin consumer floor — now MET.** Verified all three required locations state it: root
    `README.md:155` ("Requires **Kotlin 2.3 or newer**"), `docs/howto/kotlin.md:29` (admonition),
    and `packages/kotlin/README.md:23`. All correctly attribute the floor to the
    `kotlin("jvm") 2.4.10` compiler's metadata version, matching the spec. Note
    `specs/kotlin-bindings.md` still shows 0/14 boxes checked — that file's checkboxes are
    unmaintained and are not a progress signal.
- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface.
- `packages/go/` is pure Go (no CGO/WASM); it is exposed to the Unicode drift until go1.27 ships
    x/text 17.0.0 tables, which the freeze rule is designed to neutralize.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; **11** `docs/howto/*.md` guides; llms-full
    generation over 22 ordered pages; MkDocs site with branding.
- Kotlin howto gained the version-floor admonition this iteration.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.

## CI/CD and Publishing

**Status**: partially met — the release machinery is complete; lint/dep hygiene has open slices

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `bb618a1`:
    41 check-runs, 21 distinct check names, **0 non-success**. (Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.)
- `HEAD` `f18c5d5` is one unpushed commit; `git diff --stat origin/develop..HEAD -- . ':!.claude'`
    is empty, so nothing outside CI coverage is unverified.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names. All four quality gates
    enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP (`--fail-regression`
    and `--fail-above 30.0`), `cargo-deny` audit, and `cargo-semver-checks` (informational only).
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems.
- **Open gaps**: ruff 0.16 dependency-refresh slices B/C/D (26 live findings remain after slice A:
    RUF100 15, I001 8, EXE001/PLW1510/RUF022 1 each), then the magnus 0.8 and jni 0.22 source
    rewrites. Two human-gated items remain: npm OIDC migration and the broken single-registry
    re-trigger in `release.yml`. There is still no Dependabot or Renovate config.

## Open Issues

6 open — 0 critical, 4 normal, 2 low. Zero `[review]` entries and zero HUMAN REVIEW REQUESTED
blocks, so nothing is parked on Titusz.

- CID-doable: Unicode data version freeze rule (DECIDED, fully specified), dependency refresh (ruff
    slices B/C/D → magnus → jni).
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Implement the Unicode 16.0.0 freeze rule in the Rust core — slice 1 of 3.** It is now the only
substantive work keeping any section from `met`, it is fully specified in `specs/rust-core.md`, and
it blocks the other two Unicode criteria.

Slice 1 scope: add a checked-in generator script that emits a vendored range table of code points
unassigned in Unicode 16.0.0 (via `unicodedata2==16.0.0`), then filter those code points in
`crates/iscc-lib/src/utils.rs` **before** normalization and category lookup, rather than relying on
the live crate's `GeneralCategory::Unassigned`. Slices 2 and 3 (boundary conformance vectors across
Rust plus all 12 bindings, and the full-code-space differential sweep) follow.

Two gates will react to slice 1 and must be handled in the same change: the iai-callgrind perf gate
(a new per-character range lookup on the hot text path can exceed the 10% Ir budget — refresh
`.iai-baseline.json` only with justification) and the CI-only CRAP `--fail-regression` check (a new
branch in a covered function fails CI even when `mise run check` is green locally — refresh that
entry in `.crap-baseline.json` in the same commit).

If a smaller unit is preferred first, ruff slice C is the cheap alternative: `[tool.ruff.lint]` has
no `select` key, so the `S` and `C901` rules run only in the two pre-push hooks; adding them to
`select` clears all 15 RUF100 findings and strengthens the local loop at the same time.
