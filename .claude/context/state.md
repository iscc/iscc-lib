<!-- assessed-at: 312524a66e30f64df9de0ab2e9d9dab021d6075e -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-dependency-refresh cleanup — JNI debt closed, one static-evidence dep slice and one doc fix left for CID

Iteration 169 closed both `normal` issues the 168 review had filed against `crates/iscc-jni`: the
hand-rolled byte-array helper is gone and all 33 natives now have JUnit callers. Open issues fell 10
→ 8 (3 `normal`, 5 `low`). CI is green over the whole tree; the every-10th `audit` role is due this
iteration.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/iscc-lib/`, `benches/` or `.claude/context/specs/` moved since 8ea3e02: 32
    Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant, no `unsafe` outside the FFI crates.
- All four Unicode criteria remain MET: declared 16.0.0 + sentinel freeze, the `Final_Sigma` case
    freeze, boundary vectors on 11 of 11 surfaces, and the fail-closed differential sweep gate.
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; the cut is human-gated and
    HELD. `cargo-semver-checks` runs informational.
- `specs/rust-core.md` L149-157 stays STALE (claims the Go `Final_Sigma` defect unfixed, and a
    `str::to_lowercase()` equivalence falsified by measurement at 156). Human-owned, not edited.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64; 441 collected pytest tests. Nothing under `crates/iscc-py/` or `tests/` moved.
- The 17.8M-comparison Unicode sweep deliberately stays out of `pytest` / `mise run test` / pre-push
    — do not wire it in.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips.

## C FFI

**Status**: met — Unicode-gated at 159

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched.
- `tests/unicode_boundary_vectors.h` (tracked, generated) is consumed by both
    `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` from one artifact.

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: met — every surface Unicode-gated; the Java bridge was the only source to move

- **JNI is the sole changed source.** `grep -rn "build_byte_array\|Vec<i8>" crates/iscc-jni/`
    returns nothing — the 168 allocation regression is gone, replaced by **5**
    `env.byte_array_from_slice` sites. `src/lib.rs` is **1152 lines**, 33 `Java_*` natives, still
    under the ~1500-line split threshold its `CLAUDE.md` sets.
- `IsccLibTest.java` grew 29 → **40** `@Test` methods (93 JVM tests total with
    `UnicodeBoundaryTest`); the seven previously untested natives now have Java callers, so the
    JUnit surface covers all 33. Both JNI issues filed at 168 are resolved and deleted.
- Root `Cargo.toml` pins `jni = "0.22"` (lock 0.22.4); **2 inline `# held:` pins left** (criterion
    0.8, uniffi 0.32).
- `crates/iscc-rb` (magnus 0.8.2 since 167), `crates/iscc-uniffi` (32 exports, 21 tests,
    `publish=false`), `packages/dotnet` (xunit.v3) and `packages/{cpp,go,kotlin,swift}` unchanged;
    Kotlin's 9 + 3 sources and the Java fixture plumbing untouched.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- go1.27 remains a standing tripwire issue (not schedulable until go1.27 ships, ~Aug 2026): the bump
    reds 5 Go boundary cases unless the 731-range freeze table lands in the same step.

## Documentation

**Status**: met

- Nothing under `docs/` moved since 2840c7f (iteration 161): 23 pages across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12
    crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- `crates/iscc-jni/CLAUDE.md` was corrected in the same step as the code (type table + "don't" list
    now teach `env.byte_array_from_slice()`).
- Two surfaces still teach removed APIs (verified stale, one open `normal` issue):
    `crates/iscc-rb/CLAUDE.md:108` (`RString::from_slice`) and `specs/java-bindings.md` ("`jni`
    crate (v0.21)" L22, "~1060 lines" L13 and L32 — actual 1152). The spec half carries the only
    open **HUMAN REVIEW REQUESTED** marker; the Ruby half needs no authorization.
- Remaining cosmetic item is human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `d06e024` (the 169 review commit carrying the
    JNI change): check-runs API reports **45 runs, 23 distinct names, 0 non-success**. HEAD
    `312524a` is one `cid(log)` commit ahead, and its diff against the remote tip outside `.claude/`
    is empty. Working tree is clean.
- Job shape unchanged: 21 job keys → 22 jobs → 23 check names (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). No workflow file changed since 21bb004
    (iteration 162): zero `@main` action refs, 97 `uses:` in `release.yml`, 8 registry toggles,
    `workflow_dispatch`-only.
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — not shipped; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Dependency freshness: zero Rust majors left to schedule. The issue body lists three items —
    `uniffi` 0.32 and `criterion` 0.8 (human-gated: binding regeneration / MSRV policy) and the
    `release.yml` action refresh, which is CID-doable but on static evidence only, since that
    workflow never runs in CI.
- Reproducibility gap, observed by two roles and still unfiled: `packages/dotnet` is the only
    ecosystem with no lockfile, its two test packages floating on `3.*` / `18.*`. The 168 review
    parked it for the audit pass; audits have run at iterations 130/140/150/160, so 170 is due.

## Open Issues

**8 entries in `issues.md` — 3 `normal`, 5 `low`, zero `critical`, one HUMAN REVIEW REQUESTED
marker** (inside the doc-drift issue). Down from 10: the 169 review deleted both JNI entries.

- **NORMAL:** dependency refresh (3 items, above); binding docs teaching removed APIs; the go1.27
    tripwire.
- **LOW / CID skips:** upstream `iscc-core#137` thread (human-only), the three gate-script
    remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), npm OIDC (ruled out for v0.6.0),
    docs language logos.

## Next Milestone

Two `normal` items are CID-actionable: the `release.yml` action-freshness pass — the last
schedulable slice of the v0.6.0 dependency issue, verifiable only through
`scripts/check_release_workflow.py` and tag existence, never through a CI run — and the Ruby half of
the doc-drift issue. Beyond those, the backlog is human-gated (v1.0.0 cut, npm OIDC, docs logos, the
upstream thread, the `specs/java-bindings.md` edit), and the unfiled `packages/dotnet` lockfile gap
is waiting on the audit due this iteration.
