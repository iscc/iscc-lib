<!-- assessed-at: 4cd963b4bf50134bba9b466db2d955da0b8f8fdb -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — dependency/lint hygiene while one Unicode spec-wording ruling sits with Titusz

v0.5.0 is released and CI is fully green. Iteration 134 was a pure quality-gate step: ruff's `S`
(security) and `C901` (complexity) rules moved from two pre-push-only hooks into the project lint
selection, so they now run at commit time and in CI, and every stale `# noqa` cleared as a side
effect. The Unicode 16.0.0 work remains half-landed — the freeze rule is in the core, the boundary
vectors are not, and the sweep is blocked behind a human ruling on spec wording.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions and all 10 `gen_*_v0`
    functions pass the vendored `iscc-core/data.json` conformance vectors. Unchanged this iteration
    — no Rust file was touched since the last assessment.
- **Freeze rule (criterion 1) — MET, re-verified.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` /
    `.nfd()` in `text_clean` / `text_collapse`. Generator `scripts/gen_unicode16_unassigned.py` is
    checked in (PEP 723, pins `unicodedata2==16.0.0`).
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer"; the freeze rule makes
    them freely upgradable.
- **Boundary conformance vectors (criterion 3) — NOT met.** Re-verified by grepping `U+1FAE9` /
    `U+113C5` / `U+20C1` across `crates/`, `packages/`, `tests/`, `scripts/`: hits appear only in
    `crates/iscc-lib/src/utils.rs` (inline `#[test]` assertions) and `unicode16.rs`. Zero hits in
    any binding, and no fixture file exists. The criterion asks for a vector set exercised by the
    Rust suite **and every binding's conformance test**.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, 12 GIL-release sites in
    `crates/iscc-py/src/lib.rs`; abi3-py310 wheels including aarch64.
- Pending: the Unicode 16.0 boundary conformance vectors (Rust-core-led). The binding inherits the
    freeze-rule behaviour automatically — only the vector wiring is missing.

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
    2.3+ consumer floor closed in iteration 132 remains documented in all three consumer docs.
- **`packages/go` is the one binding that does not inherit the freeze rule** — it is an independent
    pure-Go implementation on Unicode 15.0 tables (x/text ships 17.0.0 tables only under
    `//go:build go1.27`, ~Aug 2026). It will fail the boundary vectors unless the 15.0→16.0 assigned
    delta is vendored or the vectors are skipped for Go with a tracking note. Both options are
    recorded; neither has been chosen.

## Documentation

**Status**: met

- 12 crate/package READMEs and 12 CLAUDE.md files; **11** `docs/howto/*.md` guides; llms-full
    generation over 22 ordered pages; MkDocs site with branding.
- `docs/development.md` was updated this iteration to state that the pre-commit `ruff check` now
    includes `S` and `C901`, and that the two pre-push hooks are focused re-runs of rules already
    enforced project-wide — accurate as written.
- Observation (not a spec gap, unchanged): the declared Unicode data version appears nowhere in
    user-facing `docs/` or `README.md`, so consumers cannot discover the contract.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
- No benchmark file changed this iteration.

## CI/CD and Publishing

**Status**: partially met — release machinery complete; the ruff 0.16 adoption has slices C and D
left

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `cc838fc`:
    **41 check-runs, 21 distinct check names, 0 non-success.** (Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.)
- `HEAD` `4cd963b` is one unpushed commit (`cid(log): iteration 134`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names. All quality gates
    enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP (`--fail-regression`,
    `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit, `cargo-semver-checks`
    (informational only).
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems.
- **Lint gate strengthened this iteration.** `pyproject.toml` now carries
    `[tool.ruff.lint] extend-select = ["S", "C901"]` — verified present, and verified there is still
    no `select` key (which would have replaced ruff's `E4`/`E7`/`E9`/`F` defaults). `S` and `C901`
    consequently run in pre-commit, `mise run lint`, and CI for the first time; the two pre-push
    `--select` hooks were deliberately kept as command-line redundancy. Two dead `# noqa` directives
    were deleted (`S603` in `tools/metrics.py`, `PLC0415` in `tools/cid.py`).
- **Ruff 0.16 adoption progress, measured live** (`uvx ruff@0.16.0 check .`): **12 findings**, down
    from 26 — `I001` ×8 (7 test modules + `crates/iscc-py/python/iscc_lib/__init__.py`), `RUF022`
    ×1, `RUF007` ×1 (`scripts/gen_unicode16_unassigned.py`), `PLW1510` ×1
    (`scripts/test_install.py`), `EXE001` ×1 (`tools/cid.py`). All `RUF100` cleared. The `ruff<0.16`
    pin remains with a now-accurate `# held:` comment.
- **Open gaps**: ruff slices C (isort cluster + the three one-liners) and D (drop the pin), then the
    magnus 0.8 and jni 0.22 source rewrites. Two human-gated items remain: npm OIDC migration and
    the broken single-registry re-trigger in `release.yml`. There is still no Dependabot or Renovate
    config.

## Open Issues

**7 open — 0 critical, 5 normal, 2 low. One `[review]` entry still carries a HUMAN REVIEW REQUESTED
block, so an item remains parked on Titusz.**

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks (`text_clean("e\u{0378}\u{0301}")` →
    `U+00E9` vs `U+0065 U+0301`, Hangul jamo composition, `Final_Sigma` in `text_collapse`). The
    ruling needed is spec **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0
    tables" is false as literally written, the accepted-divergence paragraph does not cover this
    class, and the upstream `iscc-core` proposal must state pre-normalization removal.
- CID-doable now: ruff slice C (and D once the tree is clean under 0.16).
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Ruff 0.16 sub-slice C — the isort cluster plus the three one-liners.** This is unambiguously
unblocked autonomous work and it continues a slice that has produced three clean, in-budget steps in
a row (26 → 12 findings).

Scope: one real decision — whether to add `[tool.ruff.lint.isort]` with `known-first-party` / `src`
entries so `iscc_lib` sorts as first-party (the package lives under `crates/iscc-py/python/`, not
the repo root) — then let `ruff check --fix --extend-select I,RUF022` apply the mechanical fixes.
Bundle `RUF100` into `extend-select` (green at HEAD today) so stale directives cannot accumulate
unseen again. The three one-liners can ride along or take their own step: `RUF007` in
`scripts/gen_unicode16_unassigned.py` (re-run the generator afterwards and assert
`git status --porcelain` on the generated Rust is empty), `PLW1510` in `scripts/test_install.py`,
and `EXE001` on `tools/cid.py` (note the Windows-bind-mount exec-bit caveat before choosing
`chmod +x`). Never run `ruff@0.16 check --fix .` wholesale — it deletes load-bearing `# noqa`
directives. Slice D (`uv lock --upgrade-package ruff`, drop the pin) unlocks only once
`uvx ruff@0.16.0 check .` exits 0.

**Unresolved tension carried into this iteration, for define-next to settle:** the iteration-133
state.md recommended starting Unicode step (b) with a Rust-only fixture slice, while the
iteration-134 review handoff says explicitly "do not start either (a2) or (b)" until the ruling
lands. Both readings are defensible — the three boundary cases are single code points and therefore
untouched by the sequence question, but the fixture's expected outputs would be re-derived if the
spec wording changes. Recommendation: take slice C now, and if the ruling is still open next
iteration, do the Rust-core-only fixture (format + loader, no binding wiring) rather than idling.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
