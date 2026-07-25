<!-- assessed-at: 544750c8dcb3205961875b83795a69c74c8063da -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — local/CI lint parity closed and release re-trigger path repaired; the parked Unicode ruling still blocks the only remaining library work

Iteration 139 closed two silent no-op gates in one package: all 28 non-`prepare-release` jobs in
`release.yml` now carry `!cancelled() && !failure()` guards so `-f <registry>=true` re-triggers
actually reach their publish jobs, and both local ruff hooks gained the `pyi` type tag. CI is green
on the `origin/develop` tip. No Rust or binding source has moved since iteration 133, and it cannot
move on the Unicode track until a human spec-wording ruling lands.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted at HEAD) and
    all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors.
- `git diff 862c128..HEAD -- . ':!.claude'` touches only `.github/workflows/release.yml`,
    `.pre-commit-config.yaml`, `CLAUDE.md` and `docs/development.md`. No Rust or binding source
    moved, and `.claude/context/specs/` plus `target.md` are byte-identical to the previous
    assessment — no criterion moved underneath the code.
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

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, **12** `.detach(` GIL-release
    sites in `crates/iscc-py/src/lib.rs` (re-counted); abi3-py310 wheels including aarch64.
- No file under `crates/iscc-py/` appears in this iteration's diff, but the published stub
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (the workspace's only tracked `.pyi`) is now
    covered by both local ruff hooks — probed at HEAD: `prek run ruff-check` and
    `prek run ruff-format --files …/_lowlevel.pyi` both report `Passed`, neither `Skipped`.
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
    header plus csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched this
    iteration.
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
- `CLAUDE.md` and `docs/development.md` were updated this iteration so the documented hook surface
    matches the shipped config: both now state that the two ruff hooks also cover `.pyi` type stubs
    and that `ruff format` additionally covers Python code blocks inside Markdown.
- `.claude/skills/release/SKILL.md` no longer describes the single-registry re-trigger as a known
    bug; it now says the guards are verified **statically only**, with first real-world confirmation
    at the next release run. That honesty caveat is correct and should not be dropped.
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

**Status**: partially met — both silent no-op gates closed; the stale GHA `uses:` refs in
`release.yml` are now the last CID-doable item

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `7b816da`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    (`develop` → `main`, titled "Release 0.6.0") is open, so each push fires both a `push` and a
    `pull_request` run.
- `HEAD` `544750c` is one unpushed commit (`cid(log): iteration 139`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified. Working tree is clean.
- **Single-registry re-trigger — FIXED.** Independently re-verified by parsing `release.yml`: **29
    jobs, 28 guarded** with `!cancelled() && !failure() && (…)`; the only unguarded job is
    `prepare-release` (bare `inputs.version != ''`), which is correct — it is the job whose skip was
    being inherited. All 8 registry toggles (`crates-io`, `pypi`, `npm`, `maven`, `ffi`, `rubygems`,
    `nuget`, `maven-kotlin`) plus `version` are intact and every registry condition is preserved
    verbatim inside the new parentheses. **Caveat: statically verified only** — no CI or CID push
    exercises this file; first real proof is the next release run.
- **`.pyi` hook hole — CLOSED.** `.pre-commit-config.yaml` now has `types_or: [python, pyi]` on
    `ruff-check` and `types_or: [python, pyi, markdown]` on `ruff-format`. Probed at HEAD against
    the real stub: both hooks run and pass rather than reporting `no files to check`. The local prek
    surface is now a strict superset of CI's recursive discovery. Both changes strictly *widen*
    gates; no exclusion or suppression was added.
- **Residual doc nit (not filed):** the comment above `ruff-format` in `.pre-commit-config.yaml`
    still says it "covers the same surface as the CI `ruff format --check` step". With `pyi` added
    it is a superset. Harmless; fold into a future edit of that file.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked
    criteria — unchanged despite the re-trigger fix.
- **Last CID-doable CI gap:** `release.yml`'s **97** `uses:` refs still lag `ci.yml` — re-counted at
    HEAD: `actions/checkout@v4` ×23, `actions/upload-artifact@v4` ×11,
    `actions/download-artifact@v4` ×20 — and there is still **no** `setup-uv` step
    (`grep -c setup-uv` → 0, so the old issue text claiming one needs a bump was wrong).
- **Other open gaps** (unchanged): the `magnus` 0.8 and `jni` 0.22 source rewrites plus the deferred
    majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x). One human-gated
    item remains: npm OIDC trusted publishing. There is still no Dependabot or Renovate config.

## Open Issues

**6 open — 0 critical, 4 normal, 2 low** (down from 8: the re-trigger and `.pyi` issues were closed
this iteration). **One `[review]` entry still carries a HUMAN REVIEW REQUESTED block**, so an item
remains parked on Titusz.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks. The ruling needed is spec
    **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as
    literally written, the accepted-divergence paragraph does not cover this class, and the upstream
    `iscc-core` proposal must state pre-normalization removal.
- CID-doable: the `release.yml` GHA `uses:` refresh (static verification only). Its cross-reference
    to the retired `if:`-guard issue was repaired this iteration.
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Also open: "Declare and gate a Unicode data version (DECIDED)" (the two unmet criteria above).
- Human-gated: npm OIDC trusted publishing.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Preferred: the 97-ref GHA `uses:` refresh in `release.yml`.** This is now a purely mechanical bump
— the behavioural change is already isolated in its own commit. Move `upload-artifact` ↔
`download-artifact` together, bump `actions/checkout` v4→v7 (23 sites), plus
setup-java/node/dotnet/python; if a `setup-uv` step is introduced it must be pinned to the exact
`@v9.0.0` (no floating major past v7). Verify statically the same way the guard fix was: YAML parse,
`actionlint@v1.7.7`, artifact-name consistency across build → test → publish, and a `grep` proving
no `if:` guard was disturbed (28/29 guarded, `prepare-release` bare).

**Worth bundling:** landing the guard-shape check as a small `scripts/` file wired into a prek or CI
step. The `yaml.safe_load` + shape-regex logic is already written and green at HEAD, and it is the
only way to make silent guard regressions impossible in the one file no CID push exercises. It
changes gate config, so it needs an explicit scope statement, not a drive-by.

**Cadence warning, unchanged and now sharper:** iterations 134–139 have all been tooling/CI
configuration. After the GHA refresh the tooling thread is genuinely exhausted, and the next
iteration must either move to library work or stop. Do not open a fourth tooling thread.

**Unresolved tension:** the human ruling on freeze-rule sequence ordering has not landed, and every
review since iteration 133 has said boundary vectors must not be wired into 11 bindings before it
does. Once the GHA refresh is in, the defensible middle path is a Rust-core-only fixture (format
plus loader, no binding wiring) for the three single-code-point boundary cases, which the
sequence-ordering question does not affect.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
