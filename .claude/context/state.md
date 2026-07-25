<!-- assessed-at: 2cd0d6d3ea8fa0025919b802b19e07cc02a8861a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the v0.6.0 dependency refresh is fully closed (all 9 slices); the only remaining CID-doable work is library-side, and it is parked on a human spec ruling

Iteration 140 bumped the last nine stale GitHub Actions refs in `release.yml`, closing slice 9 and
with it every locally-verifiable ecosystem slice of the v0.6.0 dependency refresh. CI is green on
the `origin/develop` tip. No Rust or binding source has moved since iteration 133, seven consecutive
iterations of tooling/CI configuration have now landed, and the tooling thread is exhausted.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted at HEAD) and
    all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors.
- `git diff 544750c..HEAD -- . ':!.claude'` touches exactly one file:
    `.github/workflows/release.yml` (73 +/73 −, all `uses:` lines). `.claude/context/specs/` and
    `target.md` are byte-identical to the previous assessment, so no criterion moved underneath the
    code. All binding, benchmark and docs findings below are carried forward with spot re-checks.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` /
    `.nfd()`, with the checked-in PEP 723 generator `scripts/gen_unicode16_unassigned.py`.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer"; the freeze rule makes
    them freely upgradable.
- **Boundary conformance vectors (criterion 3) — NOT met.** Re-verified at HEAD: `1FAE9` / `113C5` /
    `20C1` appear only in `crates/iscc-lib/src/utils.rs` (inline test assertions) and
    `utils/unicode16.rs`. Zero hits in any binding, no fixture file. The criterion asks for a vector
    set exercised by the Rust suite **and** every binding's conformance test.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.
    Workspace version is still **0.5.0** — PR #44 is titled "Release 0.6.0" but is only the open
    `develop` → `main` PR, not a shipped release.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, **12** `.detach(` GIL-release
    sites in `crates/iscc-py/src/lib.rs` (re-counted at HEAD); abi3-py310 wheels including aarch64.
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (the workspace's only tracked `.pyi`) remains
    covered by both local ruff hooks since iteration 139; nothing under `crates/iscc-py/` moved this
    iteration.
- Pending: the Unicode 16.0 boundary conformance vectors (Rust-core-led). The binding inherits the
    freeze-rule behaviour automatically — only the vector wiring is missing.

## Node.js Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes. Untouched this iteration.
- Pending: the same Unicode 16.0 boundary conformance vectors.

## WASM Bindings

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3` `wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched this iteration.
- Pending: the same Unicode 16.0 boundary conformance vectors.

## C FFI

**Status**: met, with one cross-cutting criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs (re-counted at HEAD);
    cbindgen header plus csbindgen C# generation run from `build.rs` on every `cargo build`.
    Untouched this iteration.
- Pending: the same Unicode 16.0 boundary conformance vectors.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met, with one cross-cutting criterion pending — Go carries an extra caveat

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` all carry the full 32-symbol surface. The Kotlin
    2.3+ consumer floor closed in iteration 132 remains documented in all three consumer docs.
- **`packages/go` is the one binding that does not inherit the freeze rule** — it is an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the boundary vectors unless the 15.0→16.0 assigned
    delta is vendored or the vectors are skipped for Go with a tracking note. Both options are
    recorded; neither has been chosen.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; **11** `docs/howto/*.md` guides; llms-full
    generation over 22 ordered pages; MkDocs site with branding. `scripts/version_sync.py` reports
    **21/21** targets OK (live run at HEAD).
- No documentation file changed this iteration. The iteration-139 hook-surface corrections in
    `CLAUDE.md`, `docs/development.md` and `.claude/skills/release/SKILL.md` are intact, including
    the honest "statically verified only" caveat on the release re-trigger path — that caveat now
    also covers the action-major bump and should not be dropped.
- Observation (not a spec gap, unchanged): the declared Unicode data version appears nowhere in
    user-facing `docs/` or `README.md`, so consumers cannot discover the contract.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
    Untouched this iteration.

## CI/CD and Publishing

**Status**: partially met — the dependency-freshness gap is closed; what remains is human-gated or
newly filed gate work

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `b8aa174`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`) is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `2cd0d6d` is three unpushed commits (two `cid(log)` + one `cid(audit)` metrics snapshot);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified. Working tree is clean.
- **GHA `uses:` refresh — DONE (slice 9).** Independently re-verified at HEAD: still exactly **97**
    `uses:` lines with the histogram `checkout@v7` ×23, `download-artifact@v8` ×20,
    `upload-artifact@v7` ×11, `setup-java@v5` ×6, `setup-node@v7` ×5, `setup-dotnet@v6` ×3,
    `action-gh-release@v3` ×2, `setup-python@v7` ×2, `cache@v6` ×1. `grep -c setup-uv` → **0** — the
    old issue text claiming this file needed a `setup-uv` bump was wrong and has been corrected in
    `issues.md`. `release.yml` no longer lags `ci.yml`.
- **Iteration-139 guard invariant intact.** Re-parsed with `yaml.safe_load` at HEAD: **29 jobs, 28
    guarded** with `!cancelled() && !failure()`, exactly one bare (`prepare-release`), zero other
    shapes. All 8 registry toggles (`crates-io`, `pypi`, `npm`, `maven`, `ffi`, `rubygems`, `nuget`,
    `maven-kotlin`) plus `version` present.
- **Caveat that now covers two consecutive changes:** `release.yml` is `workflow_dispatch`-only, so
    neither the guards nor the action majors have ever executed. Both are recorded accepted risks in
    `decisions.md` (2026-07-25). This is exactly why a new `[review]` issue asks for the
    hand-retyped static checks to become an executable gate.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked
    criteria — unchanged.
- **Open gaps:** one floating-branch action ref (`rubygems/configure-rubygems-credentials@main`,
    newly filed, needs a human tag-vs-SHA ruling); the `magnus` 0.8 and `jni` 0.22 source rewrites
    plus the deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x);
    npm OIDC trusted publishing (human-gated). There is still no Dependabot or Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (up from 6: the review agent filed two new `[review]`
entries while closing slice 9). **One `[review]` entry still carries a HUMAN REVIEW REQUESTED
block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks. The ruling needed is spec
    **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as
    literally written, the accepted-divergence paragraph does not cover this class, and the upstream
    `iscc-core` proposal must state pre-normalization removal.
- **NEW, CID-doable:** "Land the `release.yml` static checks as an executable gate" — the
    guard-shape, artifact-wiring and action-input checks have been hand-retyped into heredocs in two
    consecutive iterations. Scope: one `scripts/check_release_workflow.py`, a prek hook scoped to
    `^\.github/workflows/release\.yml$` for checks 1–2, a CI step for check 3 (needs network).
- **NEW, human-gated:** "Pin `rubygems/configure-rubygems-credentials` off `@main`" — the only
    unpinned `uses:` in the repo, running with `id-token: write` and publish authority. Facts are
    gathered; the tag-vs-SHA choice reopens a repo convention, so CID must not act unilaterally.
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Also open: "Declare and gate a Unicode data version (DECIDED)" (the two unmet criteria above), and
    the parent dependency issue, now reduced to human/major-gated bumps only.
- Human-gated: npm OIDC trusted publishing. Low / CID skips: v1.0.0 stability commitment (HELD),
    docs language logos.

## Next Milestone

**Preferred: the Rust-core-only Unicode boundary fixture.** Seven consecutive iterations (134–140)
have been tooling/CI configuration and the tooling thread is now genuinely exhausted — the next step
must move library code or stop. The defensible middle path, unchanged from the last two assessments
and now endorsed by the iteration-140 review: land a fixture file plus loader in `crates/iscc-lib`
covering only the **three single-code-point** boundary cases (a 15.1→16 emoji that must be retained,
a Unicode-16 code point with a canonical decomposition, a post-16.0 code point the freeze rule must
strip). No binding wiring, no `data.json` copies, no `specs/rust-core.md` edit — the parked HUMAN
REVIEW question concerns *sequence* adjacency only, which single-code-point vectors cannot express.
This converts three inline assertions into a reusable fixture that step (b) can propagate unchanged
once the ruling lands.

**Acceptable alternative if that is judged too large:** the newly filed `release.yml` static-check
script. It is small, self-contained, and converts two iterations of hand-retyped heredocs into a
re-running gate for the one file no CI or CID push exercises. But it is a fourth tooling package in
a row — pick it only with an explicit justification, not by default.

**Do not pick** the `rubygems/configure-rubygems-credentials` pin: it needs Titusz's tag-vs-SHA
ruling first.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
