<!-- assessed-at: 25ce6f50b28cd8f1c8e5b126ced8697aa759b374 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode freeze rule landed (1 of 4 criteria); a spec-wording ruling is now parked on Titusz

v0.5.0 is released and CI is fully green. Iteration 133 implemented the substantive half of the
Unicode 16.0.0 freeze rule in the Rust core — a vendored 731-range unassigned table, a checked-in
PEP 723 generator, and a pre-normalization filter in both text paths — all verified in the working
tree. It also surfaced a consequence nobody anticipated: stripping before normalization changes
character adjacency, so iscc-lib now diverges from `iscc-core` on multi-code-point sequences. That
is filed as a `[review]` issue with **HUMAN REVIEW REQUESTED**, the first human-parked item in
several iterations.

## Rust Core Crate

**Status**: partially met — 1 of the 4 Unicode criteria now met; 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` now carries **325** `#[test]` functions (was 320 — 5 added
    for the freeze rule) and all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json`
    conformance vectors.
- **Freeze rule (criterion 1) — MET, verified in code.** `crates/iscc-lib/src/utils/unicode16.rs`
    holds `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]` (data-only, "do not edit by hand"
    header); `scripts/gen_unicode16_unassigned.py` is a PEP 723 script pinning
    `unicodedata2==16.0.0` and self-checking the table shape (731 ranges / 819,533 code points /
    first range `0x0378..0x0379`) before writing. `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` and
    `.nfd()` in `text_clean` / `text_collapse` respectively — the ordering the spec mandates. Lookup
    is a binary search with a `cp < 0x378` ASCII fast path.
- **Table dependencies (criterion 2) — MET.** No dependency change was needed;
    `unicode-general-category` 1.1.0 (16.0.0) and `unicode-normalization` 0.1.25 (17.0.0) both
    satisfy "16.0.0 or newer", and the freeze rule makes them freely upgradable.
- **Boundary conformance vectors (criterion 3) — NOT met.** The three spec-named code points now
    appear as inline `#[test]` assertions in `crates/iscc-lib/src/utils.rs` (U+1FAE9 retained,
    U+113C5 retained by `text_clean` / dropped by `text_collapse`, U+20C1 stripped), but the
    criterion asks for a *conformance vector set* exercised by the Rust suite **and every binding's
    conformance test**. A grep across `crates/`, `packages/`, `scripts/` and `tests/` finds those
    code points only in `utils.rs` — zero hits in any binding. No fixture file exists yet.
- **Full-code-space differential sweep (criterion 4) — NOT started, and its acceptance sentence is
    itself under human review** (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, and 12 GIL-release sites in
    `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64.
- Pending: the Unicode 16.0 boundary conformance vectors (Rust-core-led). The binding automatically
    inherits the freeze-rule behaviour — only the vector wiring is missing.

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

**Status**: met, with one cross-cutting criterion pending — Go carries an extra caveat

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. The Kotlin
    2.3+ consumer floor closed in iteration 132 stays documented in all three consumer docs.
- **`packages/go` is the one binding that does not inherit the freeze rule** — it is an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the boundary vectors unless the 15.0→16.0 assigned
    delta is vendored or the vectors are skipped for Go with a tracking note. The decision record
    names both options; neither has been taken.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; **11** `docs/howto/*.md` guides; llms-full
    generation over 22 ordered pages; MkDocs site with branding.
- `crates/iscc-lib/CLAUDE.md` gained an accurate "Text normalization order matters" pitfall entry
    describing the freeze filter and the regeneration command.
- Observation (not a spec gap): the declared Unicode data version is nowhere in user-facing `docs/`
    or `README.md` — `grep -rl "Unicode 16" docs/ README.md` returns nothing. No spec requires it
    today, but consumers currently cannot discover the contract.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate. The freeze filter passed it without a baseline refresh — largest
    increase +1.96%, and the hot text paths measured *faster*.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.

## CI/CD and Publishing

**Status**: partially met — release machinery complete; lint/dep hygiene has open slices

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `d28a94b`:
    **41 check-runs, 21 distinct check names, 0 non-success.** (Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.)
- `HEAD` `25ce6f5` is one unpushed commit (`cid(log): iteration 133`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names. All four quality gates
    enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP (`--fail-regression`
    and `--fail-above 30.0`; baseline now **98** entries after adding `is_unassigned_in_unicode16`
    at CRAP 2.0), `cargo-deny` audit, and `cargo-semver-checks` (informational only).
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems.
- `pyproject.toml` gained a second `[tool.ty.src] exclude` entry for the generator script — a
    documented scope exclusion for a PEP 723 generator-only dependency, mirroring the `conanfile.py`
    precedent, not a gate weakening.
- **Open gaps**: ruff 0.16 refresh slices B/C/D (26 live findings after slice A: RUF100 15, I001 8,
    EXE001/PLW1510/RUF022 1 each), then the magnus 0.8 and jni 0.22 source rewrites. Two human-gated
    items remain: npm OIDC migration and the broken single-registry re-trigger in `release.yml`.
    There is still no Dependabot or Renovate config.

## Open Issues

**7 open — 0 critical, 5 normal, 2 low. One `[review]` entry carries a HUMAN REVIEW REQUESTED block
(new this iteration), so something is parked on Titusz for the first time in several cycles.**

- **HUMAN-PARKED (new):** "Freeze-rule ordering diverges from iscc-core on sequences". Stripping
    unassigned code points before normalization changes adjacency and unblocks contextual transforms
    the reference still blocks — measured: `text_clean("e\u{0378}\u{0301}")` → `U+00E9` vs
    `U+0065 U+0301`, Hangul jamo composition, and `Final_Sigma` in `text_collapse` (`σ` vs `ς`). The
    ruling needed is spec **wording**, not a redesign: criterion 4's "output-equivalent to uniform
    Unicode 16.0.0 tables" is false as literally written, the accepted-divergence paragraph does not
    cover this class, and the upstream `iscc-core` proposal must state pre-normalization removal.
    `decisions.md` records why the code was merged anyway.
- CID-doable: Unicode step (b) boundary vectors, dependency refresh (ruff slices B/C/D → magnus →
    jni).
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Unicode step (b), slice 1: define the boundary-vector fixture format and wire it into the Rust
suite.** This is the only remaining Rust-core criterion that is not blocked, and it is the
prerequisite for the eleven mechanical binding slices that follow.

Scope: a project-owned fixture (e.g. `unicode16_vectors.json`) with its own loader — **not** added
to `crates/iscc-lib/tests/data.json`, which is vendored verbatim from `iscc-core` and would turn
future upstream refreshes into a merge exercise. Cover the three spec-named cases (U+1FAE9 retained,
U+113C5 retained by `text_clean` / dropped by `text_collapse`, U+20C1 stripped) at the `text_clean`
/ `text_collapse` level, and decide the sync mechanism to the bindings up front, since that is what
the later slices replicate.

**Note the one open tension for define-next:** the review handoff argues step (b) can proceed now
because all three boundary cases are *single* code points and therefore untouched by the sequence
question, while the issue text says to settle the ruling before wiring vectors into 11 bindings and
5 `data.json` copies. Both are reconcilable if slice 1 stops at the Rust core plus the fixture
format and the binding slices wait for the ruling — that is the recommended shape.

Cheap unblocked alternative if the loop prefers to stay clear of anything Unicode-adjacent while the
ruling is open: **ruff slice C**. `[tool.ruff.lint]` has no `select` key, so `S` and `C901` run only
in the two pre-push hooks; adding them to `select` clears all 15 RUF100 findings and strengthens the
local loop at the same time.

Two gates will react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
