<!-- assessed-at: e38c17e7dba17811bf6fcc68d0789579f3113f84 -->

# Project State

## Status: IN_PROGRESS

## Phase: Unicode contract complete on all 11 surfaces; loop infrastructure ahead of CI

Iteration 161 gated the Swift suite on the 12 Unicode 16.0.0 boundary vectors — the eleventh and
last binding surface — and the Unicode umbrella issue is gone from `issues.md`. Since then an
out-of-loop commit (`e38c17e`, no `iterations.jsonl` entry) rewrote `tools/cid.py`,
`tests/test_cid.py` and `CLAUDE.md`: 785 lines of tracked code that CI has never seen, because HEAD
is two commits ahead of `origin/develop`.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/` moved since 8c68283 (`git diff --stat 8c68283..HEAD -- crates/` empty): 32
    Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no
    `unsafe` outside the FFI crates. `.claude/context/specs/` diff is **empty** — no spec moved.
- **All four Unicode criteria MET**: (1) declared 16.0.0 + sentinel freeze (`utils/unicode16.rs`,
    731 ranges), (2) the `Final_Sigma` case freeze (156), (3) boundary vectors on **11 of 11**
    surfaces (Swift closed it), (4) the fail-closed differential sweep gate.
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; the cut is human-gated and
    HELD. `cargo-semver-checks` runs informational.
- `specs/rust-core.md` L149-157 stays **STALE** (claims the Go `Final_Sigma` defect fixed at 147,
    and a `str::to_lowercase()` equivalence falsified by measurement at 156); criterion checkboxes
    unchecked though all four hold. Human-owned file, not edited.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64. Nothing under `crates/iscc-py/` moved.
- `uv run pytest --collect-only -q` → **432** tests (was 399 at iteration 160): +1 in
    `tests/test_vendored_fixtures.py` for the Swift copy, +32 in `tests/test_cid.py` (46 → 78) from
    the out-of-loop commit. Collection is clean, so the new loop code at least imports.
- The 17.8M-comparison sweep deliberately stays out of `pytest` / `mise run test` / pre-push — do
    not wire it in.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    under `wasm-pack test --node` in the CI `wasm` job.

## C FFI

**Status**: met — Unicode-gated at 159

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched.
- `tests/unicode_boundary_vectors.h` (tracked, generated, non-ASCII as 3-digit octal) is consumed by
    **two** tests — `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` —
    from a single artifact, with no vendored copy and no public-interface leak.

## Other Bindings (C++, Java, Kotlin, Go, Ruby, C#, Swift)

**Status**: met — every surface now Unicode-gated

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    and `packages/{cpp,dotnet,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    moved except Swift.
- **Swift (new, 161)**: `Tests/IsccLibTests/UnicodeBoundaryTests.swift` (88 lines) — metadata guard
    (`16.0.0`, 7, 5) plus both vector sections through `textClean` / `textCollapse`, zero skips. It
    compares **`unicodeScalars` arrays, never strings**: Swift's `String ==` folds canonical
    equivalence, which would make the sequence vectors vacuous (the review proved this load-bearing
    by mutation). Fixture registered as `.copy("unicode_boundary.json")` in `Package.swift:27`;
    `.gitignore` gained `.build/`.
- **Propagation invariant re-verified**: `git ls-files -- '*data.json' '*unicode_boundary.json'` =
    **8** tracked paths, exactly matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`;
    the Swift copy is byte-identical to the canonical fixture (`cmp` clean).
- **A 13th boundary vector now costs 12 suites at once** — 8 canonical readers, 2 vendored copies, 1
    shared generated C header read by both C and C++. Any fixture edit is a deliberate,
    self-contained slice.
- The go1.27 hazard is now its own tracked issue: go1.27 ships Unicode 17.0 tables, so `packages/go`
    (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with the bump. The
    skip map stays **unconditional** by standing ruling; the red is the intended trigger. CI pins Go
    via `go-version-file: packages/go/go.mod`.
- **Correction carried forward**: the "Swift is not locally verifiable" claim in the previous
    state.md is refuted. A Swift 6.1.2 toolchain sits at
    `/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin` (present now, absent from `$PATH`, not in
    the image, `mise.toml`, devcontainer or CI). Both `cmake` (160) and `swift` (161) turned out
    available after docs said otherwise — treat any "not verifiable here" claim as unproven.

## Documentation

**Status**: met

- `docs/unicode.md` now names Swift among the gated surfaces. Page-list machinery unchanged: **23**
    documentation pages across `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt`; 11
    `docs/howto/*.md`; 12 crate/package READMEs; 12 crate/package `CLAUDE.md`.
- `packages/swift/CLAUDE.md` gained the fixture/vendoring rules and a Linux toolchain fetch recipe;
    it also dropped a section documenting a `SmokeTests.swift` that never existed in git history.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x–158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no Rust source moved.

## CI/CD and Publishing

**Status**: partially met — green on the pushed tip, but HEAD carries code CI has not seen

- **CI GREEN on `origin/develop` = `b44c72e`** (the iteration-161 review commit): check-runs API
    reports **45 runs, 23 distinct names, 0 non-success**. That tip is above the Swift advance
    commit, so the green run genuinely covers the Swift slice.
- **It does NOT cover HEAD.** `e38c17e` + `acf178a` are unpushed and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **non-empty**: `tools/cid.py` (389
    changed lines), `tests/test_cid.py` (421), `CLAUDE.md` (13). Those paths are gated by
    `python-test`, `ruff` and `ty` — all unverified until the push lands. Working tree is clean.
- Job shape unchanged: **21 job keys → 22 jobs → 23 check names** (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). `.github/` byte-untouched since 8c68283.
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — **not shipped**; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- `release.yml` unchanged: 97 `uses:` refs, 8 registry toggles, `workflow_dispatch`-only,
    `rubygems/configure-rubygems-credentials@main` still unpinned at line 895. `specs/ci-cd.md`
    remains drifted (14-row job table vs 21 real keys) — owned by an authorized issue.
- Enforcing gates: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is **CI-only**, so a
    green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`.
- **Loop infrastructure changed under the roles**: `ARTIFACT_BUDGETS` in `tools/cid.py` now caps
    next/state/handoff/issues/learnings/decisions (state.md = 200 lines) and reports overruns to the
    owning role; `decisions.md` rotates into `decisions-archive.md`; each role's context arrives via
    a `.claude/skills/cid-ctx-<role>/SKILL.md` pack. Six new packs are committed.

## Open Issues

**9 entries in `issues.md` (rebuilt 597 → 167 lines) — 4 `normal`, 5 `low`, zero `critical`, zero
`HUMAN REVIEW REQUESTED`.** The Unicode umbrella issue is **closed**: its remaining work was
propagation, now 11 of 11.

- **NORMAL:** dependency review/refresh (major bumps, one per step); `go1.27` reds the Go boundary
    suite unless the freeze lands with it (extracted from the umbrella issue); the rubygems
    `@v2.1.0` pin (RULED); the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

**Get `e38c17e` under CI.** It is the only code in the tree no gate has ever run against, and
`tools/cid.py` is the runner every subsequent iteration depends on — a red there stops the loop
itself. Nothing else should be built on top of an unverified runner.

After that, the Unicode work is finished and the backlog is the authorized `[human]` items: the
one-line rubygems `@v2.1.0` pin, the exhaustive `specs/ci-cd.md` job table, then the major
dependency bumps one per step (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit 6.x,
and the riskier `jni` 0.22 / `magnus` 0.8 API migrations). Sizing and sequencing are define-next's
call.
