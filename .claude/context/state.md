<!-- assessed-at: 862975245aee31c74e71a04456c2035fffa0e4f6 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — ruff 0.16 fully adopted; a fresh local/CI gate-parity gap and the parked Unicode ruling are what remain

Iteration 137 closed the eight-part dependency-refresh slice: the `ruff<0.16` pin is gone and the
project now runs ruff 0.16.0 as linter and formatter, with zero findings and zero reformats at the
flip. The upgrade widened `ruff format` to Python code blocks inside Markdown, which no local hook
covers — filed as a new issue. CI is green; the Unicode boundary-vector work is still blocked behind
a human spec-wording ruling.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted at HEAD) and
    all 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors.
- `git diff cc9a330..HEAD -- . ':!.claude'` touches **only** `pyproject.toml` and `uv.lock`. No Rust
    or binding source moved this iteration, and `specs/` plus `target.md` are byte-identical to the
    previous assessment — no criterion moved underneath the code.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` /
    `.nfd()`. The checked-in PEP 723 generator `scripts/gen_unicode16_unassigned.py` was re-run in
    review this iteration and left the generated `.rs` byte-identical.
- **Table dependencies (criterion 2) — MET.** `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both satisfy "16.0.0 or newer"; the freeze rule makes
    them freely upgradable.
- **Boundary conformance vectors (criterion 3) — NOT met.** Unchanged: `1FAE9` / `113C5` / `20C1`
    appear only in `crates/iscc-lib/src/utils.rs` (inline test assertions) and `utils/unicode16.rs`.
    Zero hits in any binding, no fixture file. The criterion asks for a vector set exercised by the
    Rust suite **and** every binding's conformance test.
- **Full-code-space differential sweep (criterion 4) — NOT started**, and its acceptance sentence is
    itself under human review (see Open Issues).
- The `cargo-semver-checks` criterion stays unmet by design: the job is `continue-on-error: true`
    (informational until v1.0.0) and the v1.0.0 stability commitment is a HELD `low` issue.

## Python Bindings

**Status**: met, with one cross-cutting criterion pending

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, **12** `.detach(` GIL-release
    sites in `crates/iscc-py/src/lib.rs` (re-counted); abi3-py310 wheels including aarch64.
- No file under `crates/iscc-py/` appears in this iteration's diff. The dev-tooling change
    (`pyproject.toml`, `uv.lock`) does not reach the binding surface; `pytest` reports 314 passing.
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
- No file under `docs/` changed this iteration. All **129** tracked `.md` files are clean under ruff
    0.16's new Markdown code-fence formatting (measured in review).
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

**Status**: partially met — release machinery complete; ruff 0.16 adoption **done**, one new
gate-parity gap opened

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `3063fc6`:
    **41 check-runs, 21 distinct check names, 0 non-success.** That tip already contains the ruff
    0.16 bump, so the widened `uv run ruff format --check` (153 files) is proven green on both the
    3.10 and 3.14 matrix legs. Run count is ~2x because PR #44 `develop` → `main` is open, so each
    push fires both a `push` and a `pull_request` run.
- `HEAD` `8629752` is one unpushed commit (`cid(log): iteration 137`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified. Working tree is clean.
- **Ruff 0.16 adoption — COMPLETE.** `pyproject.toml` line 48 is now a plain `"ruff"` dev entry; the
    `ruff<0.16` pin and its stale `# held:` comment are gone (`grep -c 'ruff<0.16'` → 0), `uv.lock`
    resolves ruff `0.16.0`, and `uv run ruff --version` confirms 0.16.0. The `extend-select` comment
    was reworded to an evergreen rationale. Slice 8 of the dependency-refresh issue is CLOSED — all
    eight ecosystem/tooling slices are now done.
- **New gap the upgrade exposed (filed as a `normal` `[review]` issue):** ruff 0.16 formats Python
    code blocks inside Markdown, so the bare `uv run ruff format --check` used by `ci.yml` (step at
    line 71) and `mise run lint` covers 153 files, while the prek `ruff-check` / `ruff-format` hooks
    declare `types: [python]` (lines 42 and 48 of `.pre-commit-config.yaml`) and the pre-push stage
    never runs `ruff format` at all. A mis-formatted snippet in a `.md` therefore passes
    `mise run format`, `mise run check` and `git push`, then reds CI. Nothing is red today; the
    issue carries two candidate fixes.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged. All quality
    gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage + CRAP
    (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked
    criteria — unchanged.
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems. Untouched — its 97 `uses:` refs still lag ci.yml
    and it still has no `setup-uv` step.
- **Other open gaps** (unchanged): the `magnus` 0.8 and `jni` 0.22 source rewrites plus the deferred
    majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x). Two human-gated
    items remain: npm OIDC migration and the broken single-registry re-trigger in `release.yml`.
    There is still no Dependabot or Renovate config.

## Open Issues

**8 open — 0 critical, 6 normal, 2 low** (one more than last iteration: review filed the ruff /
Markdown gate-parity issue; none were closed). **One `[review]` entry still carries a HUMAN REVIEW
REQUESTED block**, so an item remains parked on Titusz.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks. The ruling needed is spec
    **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as
    literally written, the accepted-divergence paragraph does not cover this class, and the upstream
    `iscc-core` proposal must state pre-normalization removal.
- **NEW this iteration:** "`ruff format` covers Markdown in CI but no local hook does" — local and
    CI lint surfaces disagree (see CI/CD above). CID-doable, small, fully locally verifiable.
- CID-doable: the release.yml GHA-refs bundle with the `if:`-guard fix (static verification only).
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Close the ruff / Markdown gate-parity gap.** It is the direct consequence of the change just
landed, it is one config file, and it protects every future docs iteration from a surprise CI red:

1. Pick one of the two recorded fixes — widen the prek `ruff-check` / `ruff-format` hooks to
    `types_or: [python, markdown]` (auto-fixes, but must be checked against mdformat fighting over
    the same fences and against `ruff check` behaviour on files with no Python), or add a bare
    `ruff format --check` pre-push hook with `pass_filenames: false` that mirrors the CI command
    exactly (detects only, no auto-fix).
2. Verify with the same probe review used: a `.md` containing an unformatted `python` fence must now
    be caught locally, and the local surface must be stated to equal the CI surface (153 files).
3. Do not add a `[tool.ruff.format]` exclude — `decisions.md` (2026-07-25) already rejected that as
    scope exclusion.

**Second candidate if the first is taken:** bundle the `release.yml` GHA-ref refresh with the
`if:`-guard fix — same file, neither exercised by a CID push, so YAML parse plus
matrix/artifact-name consistency plus guard presence on every `test-*` / `publish-*` job is the
whole gate.

**Unresolved tension, still unsettled:** the human ruling on freeze-rule sequence ordering has not
landed, and reviews from iterations 134-137 all repeat that boundary vectors must not be wired into
11 bindings before it does. If the ruling is still open after the parity fix, the defensible middle
path is a Rust-core-only fixture (format plus loader, no binding wiring) for the three
single-code-point boundary cases, which are unaffected by the sequence question.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
