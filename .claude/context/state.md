<!-- assessed-at: 862c12862557ef2f4c9a0315c2ba88fb82f2c2d7 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — local/CI lint parity nearly closed; the parked Unicode ruling still blocks the only remaining library work

Iteration 138 widened the prek `ruff-format` hook to `types_or: [python, markdown]`, so a
mis-formatted Python fence in a `.md` is now caught at commit time instead of in CI. Reviewing the
surface arithmetic exposed a residual hole of the same class — `.pyi` files are skipped by both
local ruff hooks — which was filed rather than fixed. CI is green; the Unicode boundary-vector work
remains blocked behind a human spec-wording ruling.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted at HEAD) and
    all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors.
- `git diff 8629752..HEAD -- . ':!.claude'` touches **only** `.pre-commit-config.yaml`, `CLAUDE.md`
    and `docs/development.md`. No Rust or binding source moved, and `specs/` plus `target.md` are
    byte-identical to the previous assessment — no criterion moved underneath the code.
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
- No file under `crates/iscc-py/` appears in this iteration's diff. The published stub
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (the workspace's only tracked `.pyi`) is clean
    under bare `ruff check` / `ruff format`, but is invisible to both local hooks — see CI/CD.
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
- `docs/development.md` and root `CLAUDE.md` were updated this iteration to state that `ruff format`
    also covers Python code blocks inside Markdown — the hook lists now match the shipped config.
- All **129** tracked `.md` files are clean: `uv run ruff format --check` reports
    `153 files already formatted`, exit 0.
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

**Status**: partially met — release machinery complete; Markdown gate parity **closed**, a `.pyi`
hole of the same class remains open

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `2b7ec6f`:
    **41 check-runs, 21 distinct check names, 0 non-success.** Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.
- `HEAD` `862c128` is one unpushed commit (`cid(log): iteration 138`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified. Working tree is clean.
- **Markdown gate parity — CLOSED.** `.pre-commit-config.yaml:50` now reads
    `types_or: [python, markdown]` for `ruff-format`, with an evergreen comment above it.
    `ruff-check` deliberately stays `types: [python]` (ruff 0.16 formats Markdown fences but does
    not lint them). The change strictly *widens* a gate — no suppression or exclusion.
    `decisions.md` (2026-07-25) records both the "accept the reach, do not exclude" and "widen the
    hook, do not add a duplicate pre-push check" rulings.
- **Residual hole, filed as a `normal` `[review]` issue:** prek classifies `.pyi` as the `pyi` type,
    so **both** ruff hooks skip `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`, which CI does check
    via recursive discovery. Confirmed at HEAD: 154 tracked candidates (129 `.md` + 24 `.py` + 1
    `.pyi`) vs 153 seen by CI vs a *different* 153 seen by the hook at `--all-files`. Local is
    neither a subset nor a superset of CI. Nothing is red today; the stub is clean under both bare
    commands. Fix is two tokens, already probed green.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked
    criteria — unchanged.
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems. Untouched — its **97** `uses:` refs still lag
    ci.yml and it still has **no** `setup-uv` step (re-verified: `grep -c setup-uv` → 0).
- **Other open gaps** (unchanged): the `magnus` 0.8 and `jni` 0.22 source rewrites plus the deferred
    majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x). Two human-gated
    items remain: npm OIDC migration and the broken single-registry re-trigger in `release.yml`.
    There is still no Dependabot or Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (count unchanged: the resolved Markdown gate-parity issue
was deleted and the `.pyi` issue took its place). **One `[review]` entry still carries a HUMAN
REVIEW REQUESTED block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks. The ruling needed is spec
    **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as
    literally written, the accepted-divergence paragraph does not cover this class, and the upstream
    `iscc-core` proposal must state pre-normalization removal.
- **NEW this iteration:** "Local ruff hooks skip `.pyi` files" — the same trap class just closed for
    Markdown, on a consumer-facing file that was hand-edited as recently as iteration 131.
    CID-doable, two tokens, fully locally verifiable.
- CID-doable: the release.yml GHA-refs bundle with the `if:`-guard fix (static verification only).
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Honest framing first:** iterations 134–138 have all been Python dev-tooling configuration. Real
library work has not moved since iteration 133, and it cannot move on the Unicode track until the
human ruling lands. The next step should therefore either finish the tooling thread cleanly or move
to substantive release machinery — not open a third tooling thread.

**Preferred: close the `.pyi` hook gap, then move on.** It is the direct, unfinished half of the
change just landed, two tokens (`ruff-check` → `types_or: [python, pyi]`, `ruff-format` →
`types_or: [python, pyi, markdown]`), and it makes the local surface a genuine strict superset of
CI's. Verify by probing the hook itself (`uv run prek run <hook> --files <staged probe>`), never by
`git ls-files` arithmetic — that is exactly the measurement error the last handoff made. Leaving the
gap open means the same failure mode ships on the file that documents the public Python API.

**If bundled or taken already: the release.yml pair.** Merge the GHA `uses:` refresh (97 refs;
`upload-artifact@v4` ↔ `download-artifact@v4` move together, `setup-uv` needs the exact `@v9.0.0`)
with the broken single-registry re-trigger fix (`!cancelled() && !failure()` guards on every
`test-*` / `publish-*` job, matching `publish-crates-io`). Same file, neither exercised by a CID
push, so the whole gate is static: YAML parse, matrix-entry presence, artifact-name consistency,
guard presence on each job.

**Unresolved tension, still unsettled:** the human ruling on freeze-rule sequence ordering has not
landed, and reviews from iterations 134–138 all repeat that boundary vectors must not be wired into
11 bindings before it does. If the ruling is still open after both items above, the defensible
middle path is a Rust-core-only fixture (format plus loader, no binding wiring) for the three
single-code-point boundary cases, which are unaffected by the sequence question.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
