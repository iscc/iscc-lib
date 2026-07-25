<!-- assessed-at: cc9a3306c3a4fec3e4d08983b88412a727a02c45 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — one line from finishing the ruff 0.16 adoption; one Unicode spec-wording ruling parked on Titusz

v0.5.0 is released and CI is fully green. Iteration 136 cleared the last three ruff-0.16 findings,
so `uvx ruff@0.16.0 check .` exits 0 for the first time — only the `ruff<0.16` pin itself remains.
The Unicode 16.0.0 work is still half-landed: the freeze rule is in the core, the boundary vectors
are not, and the differential sweep is blocked behind a human ruling on spec wording.

## Rust Core Crate

**Status**: partially met — 2 of the 4 Unicode criteria met, 2 unmet; semver/v1.0.0 still held

- 8 crates in the workspace; `iscc-lib` carries **325** `#[test]` functions (re-counted) and all 10
    `gen_*_v0` functions pass the vendored `iscc-core/data.json` conformance vectors.
    `git diff 7c8c613..HEAD -- crates/ packages/` is **empty** — no Rust or binding file moved this
    iteration.
- **Freeze rule (criterion 1) — MET.** `crates/iscc-lib/src/utils/unicode16.rs` holds
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`; `utils.rs:103` and `utils.rs:181` fuse
    `.filter(|&c| !is_unassigned_in_unicode16(c))` into the iterator chain **before** `.nfkc()` /
    `.nfd()`. Generator `scripts/gen_unicode16_unassigned.py` is checked in (PEP 723, pins
    `unicodedata2==16.0.0`); it was edited this iteration (`zip` → `itertools.pairwise`) and review
    confirmed re-running it leaves the generated `.rs` byte-identical.
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

- All 32 Tier 1 symbols exported; `IsccResult`, streaming hashers, **12** `.detach(` GIL-release
    sites in `crates/iscc-py/src/lib.rs` (re-counted); abi3-py310 wheels including aarch64.
- Untouched this iteration — no file under `crates/iscc-py/` appears in the diff.
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
    **21/21** targets OK (live run).
- No file under `docs/` changed this iteration.
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

**Status**: partially met — release machinery complete; the ruff 0.16 adoption is one line from done

- **Latest CI: GREEN.** Verified via the check-runs API on the real `origin/develop` tip `690b1a8`:
    **41 check-runs, 21 distinct check names, 0 non-success.** (Run count is ~2x because PR #44
    `develop` → `main` is open, so each push fires both a `push` and a `pull_request` run.)
- `HEAD` `cc9a330` is one unpushed commit (`cid(log): iteration 136`);
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty, so nothing outside CI coverage
    is unverified. Working tree is clean.
- `.github/workflows/ci.yml`: 19 YAML job entries → 20 jobs → 21 check names, unchanged this
    iteration. All quality gates enforcing and green: iai-callgrind perf (>10% Ir fails), coverage +
    CRAP (`--fail-regression`, `--fail-above 30.0`, 98-entry baseline), `cargo-deny` audit,
    `cargo-semver-checks` (informational only). `specs/ci-cd.md` stands at 44 checked / 8 unchecked
    criteria — unchanged.
- `.github/workflows/release.yml`: 8 registry toggles plus the Swift XCFramework step; OIDC trusted
    publishing for crates.io, PyPI, and RubyGems. Untouched.
- **Ruff 0.16 adoption, measured live:** `uvx ruff@0.16.0 check . --output-format concise` →
    `All checks passed!`, **exit 0** (was 3 findings). Sub-slice D landed the three one-liners:
    `itertools.pairwise` in `scripts/gen_unicode16_unassigned.py`, an explicit `check=False` on the
    `subprocess.run` in `scripts/test_install.py`, and the exec bit on `tools/cid.py` (index mode is
    `100755`, blob hash unchanged — a genuine mode-only change; `core.fileMode=false` on this
    checkout hides it from `git status`).
- **The one remaining lint-gate gap:** `pyproject.toml` line 47 still reads
    `"ruff<0.16", # held: 3 findings left under 0.16 (...)` — a comment that now names three
    findings which no longer exist, so it is actively misleading. Project ruff is still 0.15.22.
- **Other open gaps** (unchanged): after the pin drop, the `magnus` 0.8 and `jni` 0.22 source
    rewrites plus the deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1,
    JUnit 6.x). Two human-gated items remain: npm OIDC migration and the broken single-registry
    re-trigger in `release.yml`. There is still no Dependabot or Renovate config.

## Open Issues

**7 open — 0 critical, 5 normal, 2 low. One `[review]` entry still carries a HUMAN REVIEW REQUESTED
block, so an item remains parked on Titusz.** No issue was opened or closed this iteration; the
dependency-refresh issue was updated in place to record sub-slice D done and rename the pin drop to
sub-slice E.

- **HUMAN-PARKED (carried over from iteration 133):** "Freeze-rule ordering diverges from iscc-core
    on sequences". Stripping unassigned code points before normalization changes adjacency and
    unblocks contextual transforms the reference still blocks (`text_clean("e\u{0378}\u{0301}")` →
    `U+00E9` vs `U+0065 U+0301`, Hangul jamo composition, `Final_Sigma` in `text_collapse`). The
    ruling needed is spec **wording**: criterion 4's "output-equivalent to uniform Unicode 16.0.0
    tables" is false as literally written, the accepted-divergence paragraph does not cover this
    class, and the upstream `iscc-core` proposal must state pre-normalization removal.
- CID-doable now: ruff sub-slice E (drop the pin).
- CID-doable but contested: Unicode step (b) boundary vectors — see Next Milestone.
- Human-gated: npm OIDC trusted publishing, `release.yml` single-registry re-trigger.
- Low / CID skips: v1.0.0 stability commitment (HELD), docs language logos.

## Next Milestone

**Ruff sub-slice E — drop the `ruff<0.16` pin.** It is the last piece of an eight-part slice, it is
now unblocked (the tree is provably clean under 0.16), and every day it waits the stale `# held:`
comment misleads the next reader:

1. `uv lock --upgrade-package ruff` (touches `uv.lock` only), then replace the pinned
    `"ruff<0.16", # held: ...` line in `pyproject.toml` with a plain `ruff` dev-group entry.
2. Re-run the full pre-push set with 0.16 as the *project* formatter and linter. The real risk is
    **`ruff format` drift** — 0.15.22 reports "25 files already formatted" and 0.16 may reflow some
    of them. Reformat churn belongs in this step (it is the cost of the upgrade) but must be called
    out explicitly in the handoff so it does not read as unrelated noise.
3. Never blanket `--fix`. `uvx ruff@0.16.0 check .` already exits 0, so nothing needs fixing; the
    load-bearing `# noqa: S603/S607` directives would be deleted by a wholesale `--fix`.

After E the dependency-refresh issue has only human/major-gated remainders left, and the next
CID-shaped work is the Unicode question.

**Unresolved tension, still unsettled:** the human ruling on freeze-rule sequence ordering has not
landed, and reviews from iterations 134-136 all repeat that boundary vectors should not be wired
into 11 bindings before it does. If the ruling is still open after E, the defensible middle path is
a Rust-core-only fixture (format + loader, no binding wiring) for the three single-code-point
boundary cases — they are unaffected by the sequence question — rather than idling.

Two gates react to any change on the text hot path and must be handled in the same commit: the
iai-callgrind perf gate (10% Ir budget) and the CI-only CRAP `--fail-regression` check (a new branch
in a covered function fails CI even when `mise run check` is green locally).
