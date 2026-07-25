<!-- assessed-at: 7c8c613406c5df9730a4fdd53b758f956695ebba -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — closing out the ruff 0.16 adoption while one Unicode spec-wording ruling sits with Titusz

v0.5.0 is released and CI is fully green. Iteration 135 landed ruff slice C: isort is configured for
the repo's split Python layout and `I` / `RUF022` / `RUF100` joined the enforced rule set, taking
the ruff-0.16 baseline from 12 findings to exactly 3. The Unicode 16.0.0 work remains half-landed —
the freeze rule is in the core, the boundary vectors are not, and the sweep is blocked behind a
human ruling on spec wording.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted) and all 10
    `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors. No Rust file
    was touched since the last assessment — `git diff 4cd963b..HEAD` contains zero `.rs` changes.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` /
    `.nfd()`. Generator `scripts/gen_unicode16_unassigned.py` is checked in (PEP 723, pins
    `unicodedata2==16.0.0`).
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer"; the freeze rule makes
    them freely upgradable.
- **Boundary conformance vectors (criterion 3) — NOT met.** Re-verified by grepping `1FAE9` /
    `113C5` / `20C1` across `crates/`, `packages/`, `tests/`, `scripts/`: hits appear only in
    `crates/iscc-lib/src/utils.rs` (inline `#[test]` assertions) and `utils/unicode16.rs`. Zero hits
    in any binding, and no fixture file exists. The criterion asks for a vector set exercised by the
    Rust suite **and every binding's conformance test**.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.
- `specs/` and `target.md` are byte-identical to the previous assessment, so no criterion moved
    underneath the code this iteration.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites
    in `crates/iscc-py/src/lib.rs` (re-counted); abi3-py310 wheels including aarch64.
- `crates/iscc-py/python/iscc_lib/__init__.py` changed this iteration, but the change is purely
    cosmetic: one blank line removed and `__all__` re-sorted for `RUF022`. Review diffed `__all__`
    as a *set* across the commit (`ast.parse`) — nothing added, nothing removed. Export surface
    intact.
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

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs (re-counted); cbindgen
    header plus csbindgen C# generation run from `build.rs` on every `cargo build`.
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
    **21/21** targets OK.
- `docs/development.md` was updated this iteration to list the newly enforced `I`, `RUF022` and
    `RUF100` rules and to warn that `ruff check --fix` now auto-sorts imports at commit time —
    accurate against the live `pyproject.toml`.
- Observation (not a spec gap, unchanged): the declared Unicode data version appears nowhere in
    user-facing `docs/` or `README.md`, so consumers cannot discover the contract.
- The only open docs item is `Add programming language logos to docs site` (`low`, cosmetic).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7).
- `crates/iscc-lib/benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases, wired to
    the enforcing regression gate.
- 18 pytest-benchmark fixtures; documented speedups of 1.3x–158x over the Python reference.
- `benchmarks/python/bench_iscc_lib.py` gained one line this iteration — an isort reorder only, no
    fixture added or removed.

## CI/CD and Publishing

**Status**: partially met — release machinery complete; the ruff 0.16 adoption has 3 one-liners plus
slice D left

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `7626f7e`:
    **41 check-runs, 21 distinct check names, 0 non-success.** (Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.)
- `HEAD` `7c8c613` is one unpushed commit (`cid(log): iteration 135`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names. All quality gates
    enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP (`--fail-regression`,
    `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit, `cargo-semver-checks`
    (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked criteria — unchanged.
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems.
- **Lint gate strengthened again this iteration (ruff slice C).** Verified live from
    `pyproject.toml` via tomllib: `[tool.ruff] src = [".", "crates/iscc-py/python"]`,
    `[tool.ruff.lint.isort] combine-as-imports = true`, and
    `extend-select = ["S", "C901", "I", "RUF022", "RUF100"]` — with **still no `select` key**, which
    is load-bearing (a bare `select` would drop ruff's `E4`/`E7`/`E9`/`F` defaults). Both isort
    settings are documented inline as load-bearing: without `src`, `iscc_lib` sorts as third-party;
    without `combine-as-imports`, the `_lowlevel` re-export block shatters into ~60 statements.
- **Ruff 0.16 adoption progress, measured live**
    (`uvx ruff@0.16.0 check . --output-format concise`): **exactly 3 findings**, down from 12 —
    `RUF007` (`scripts/gen_unicode16_unassigned.py:83`), `PLW1510` (`scripts/test_install.py:53`),
    `EXE001` (`tools/cid.py:1`). All `I001` and `RUF022` cleared. The `ruff<0.16` pin remains, with
    its `# held:` comment refreshed to name exactly these three (it has now gone stale twice — slice
    D retires it entirely). Project ruff is 0.15.22.
- **Open gaps**: the 3 ruff one-liners, then slice D (drop the pin), then the magnus 0.8 and jni
    0.22 source rewrites. Two human-gated items remain: npm OIDC migration and the broken
    single-registry re-trigger in `release.yml`. There is still no Dependabot or Renovate config.

## Open Issues

**7 open — 0 critical, 5 normal, 2 low. One `[review]` entry still carries a HUMAN REVIEW REQUESTED
block, so an item remains parked on Titusz.** No issue was opened or closed this iteration.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks (`text_clean("e\u{0378}\u{0301}")` →
    `U+00E9` vs `U+0065 U+0301`, Hangul jamo composition, `Final_Sigma` in `text_collapse`). The
    ruling needed is spec **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0
    tables" is false as literally written, the accepted-divergence paragraph does not cover this
    class, and the upstream `iscc-core` proposal must state pre-normalization removal.
- CID-doable now: the 3 ruff one-liners, then slice D.
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**The three remaining ruff-0.16 one-liners, in one step.** All three live in `scripts/`/`tools/`
(non-test, non-doc), so they fit a single small package, and they are the last thing standing
between the repo and slice D:

1. `RUF007` in `scripts/gen_unicode16_unassigned.py:83` → `itertools.pairwise`. This file generates
    the vendored Unicode freeze table, so after the edit re-run
    `uv run --script scripts/gen_unicode16_unassigned.py` and assert `git status --porcelain` on
    the generated `.rs` is empty.
2. `PLW1510` in `scripts/test_install.py:53` → explicit `check=` (confirm the call site really
    tolerates a non-zero exit before choosing `False` over `True`).
3. `EXE001` on `tools/cid.py:1` — shebang without exec bit. Check the Windows bind-mount exec-bit
    caveat (`core.fileMode=false` hides dropped exec bits) before reaching for `chmod +x`; deleting
    the shebang is the alternative, but confirm nothing invokes the file directly first.

Then **slice D**: `uv lock --upgrade-package ruff`, drop the `ruff<0.16` pin and its `# held:`
comment once `uvx ruff@0.16.0 check .` exits 0, and re-run the full pre-push set. Never run
`ruff@0.16 check --fix .` wholesale — it deletes load-bearing `# noqa` directives; both prior slices
used an explicit `--select`.

**Unresolved tension, still unsettled:** the human ruling on freeze-rule sequence ordering has not
landed, and the iteration-135 review repeats that boundary vectors should not be wired into 11
bindings before it does. If the ruling is still open after slice D, the defensible middle path is a
Rust-core-only fixture (format + loader, no binding wiring) for the three single-code-point boundary
cases — they are unaffected by the sequence question — rather than idling.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
