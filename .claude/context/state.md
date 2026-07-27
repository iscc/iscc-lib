<!-- assessed-at: 47a87bccb9e514ab70219196cd0cabb4aef7b1c9 -->

# Project State

## Status: IN_PROGRESS

## Phase: Unicode contract complete; CI green on every tracked line, backlog is authorized `[human]` items

Iteration 162 pinned `rubygems/configure-rubygems-credentials` to `@v2.1.0` — a 4-line `release.yml`
edit, the only tracked code that moved since the last assessment. The 785-line `tools/cid.py` /
`tests/test_cid.py` runner rewrite that the previous state.md flagged as never gated has since been
pushed and is **green**, so no tracked code sits outside CI any more.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/` or `.claude/context/specs/` moved since 8c68283 (both diffs empty): 32
    Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no
    `unsafe` outside the FFI crates.
- **All four Unicode criteria MET**: declared 16.0.0 + sentinel freeze (`utils/unicode16.rs`, 731
    ranges), the `Final_Sigma` case freeze, boundary vectors on **11 of 11** surfaces, and the
    fail-closed differential sweep gate.
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; the cut is human-gated and
    HELD. `cargo-semver-checks` runs informational.
- `specs/rust-core.md` L149-157 stays **STALE** (claims the Go `Final_Sigma` defect unfixed, and a
    `str::to_lowercase()` equivalence falsified by measurement at 156); criterion checkboxes
    unchecked though all four hold. Human-owned file, not edited.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64. Nothing under `crates/iscc-py/` moved.
- 432 collected pytest tests, unchanged since 162; the `tests/test_cid.py` growth that drove 399 →
    432 is now covered by a green `python-test` matrix run.
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

**Status**: met — every surface Unicode-gated

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    and `packages/{cpp,dotnet,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    moved this iteration.
- Swift closed the last surface at 161: `Tests/IsccLibTests/UnicodeBoundaryTests.swift` compares
    **`unicodeScalars` arrays, never strings** (Swift's `String ==` folds canonical equivalence,
    which would make the sequence vectors vacuous); fixture registered as
    `.copy("unicode_boundary.json")` in `Package.swift`.
- **Propagation invariant**: `git ls-files -- '*data.json' '*unicode_boundary.json'` = **8** tracked
    paths, exactly matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`. A 13th boundary
    vector costs **12 suites at once** — 8 canonical readers, 2 vendored copies, 1 shared generated
    C header read by both C and C++.
- The go1.27 hazard remains its own tracked issue: go1.27 ships Unicode 17.0 tables, so
    `packages/go` (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with
    the bump. The skip map stays **unconditional** by standing ruling; the red is the intended
    trigger. CI pins Go via `go-version-file: packages/go/go.mod`.

## Documentation

**Status**: met

- Page-list machinery unchanged: **23** documentation pages across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12
    crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no Rust source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `82b559d` (the 162 review commit): check-runs
    API reports **45 runs, 23 distinct names, 0 non-success**. HEAD `47a87bc` is one `cid(log)`
    commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the
    previous iteration's never-gated runner rewrite is now verified. Working tree clean.
- `release.yml`: the `publish-rubygems` job now uses
    `rubygems/configure-rubygems-credentials@v2.1.0` with a two-line rationale comment. The repo has
    **zero** `@main` action refs across all three workflows; 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. The pin's real proof is the first v0.6.0 gem publish — `release.yml`
    is never exercised by CI, so static gates plus actionlint are its whole verification surface.
- Job shape unchanged: **21 job keys → 22 jobs → 23 check names** (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). `ci.yml` and `docs.yml` byte-untouched since
    8c68283.
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — **not shipped**; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- `specs/ci-cd.md` remains drifted (14-row job table vs 21 real keys) — owned by an authorized
    issue. Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can
    red with no code change), docs page-list parity, `unicode-sweep`.

## Open Issues

**8 entries in `issues.md` — 3 `normal`, 5 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
The rubygems pin issue was resolved and deleted at 162.

- **NORMAL:** dependency review/refresh (major bumps, one per step); the exhaustive `specs/ci-cd.md`
    job table; `go1.27` reds the Go boundary suite unless the freeze lands with it.
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

Every target section except CI/CD is met, CI is green over the whole tree, and the Unicode work is
finished — what remains is the authorized `[human]` backlog. The two `normal` items CID may act on
are the exhaustive `specs/ci-cd.md` job table (documentation-only, zero runtime risk) and the
dependency majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit 6.x, and the
riskier `jni` 0.22 / `magnus` 0.8 source rewrites), which the issue requires be taken one per step.
The `go1.27` entry is a standing tripwire, not schedulable work. Sizing and sequencing are
define-next's call.
