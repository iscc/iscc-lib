<!-- assessed-at: 8c682837b64c9ef33c600a0d2180e6cd5189dec0 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode contract gated; propagation at 10 of 11 binding surfaces

Iteration 160 gated the C++ wrapper test on the 12 Unicode 16.0.0 boundary vectors by **reusing the
same generated header** the C FFI test already consumes — no second artifact, no vendored copy, no
public-interface leak. I reproduced the slice by a route the review did not use (a plain
`g++ -Wall -Wextra -Wpedantic` compile with no CMake at all) and cross-checked the passing assertion
names against the canonical JSON fixture. **Swift is the last ungated surface.**

## Rust Core Crate

**Status**: partially met — criteria 1, 2, 4 met; criterion 3 at 10 of 11 surfaces

- **Incremental scope**: `git diff 98e2964..HEAD --stat -- . ':!.claude'` = **4 files**, all C++
    propagation: `packages/cpp/tests/test_iscc.cpp` (+27), `packages/cpp/tests/CMakeLists.txt` (+4),
    `packages/cpp/CLAUDE.md` (+5), `docs/unicode.md` (+4/−3). **Zero Rust source files, zero
    baselines, zero workflow files, zero generated artifacts.** `.claude/context/specs/` diff is
    **empty** — no spec moved under me. `target.md` unchanged.
- Tier 1 surface unchanged: **32** symbols re-exported from `crates/iscc-lib/src/lib.rs`, all 10
    `gen_*_v0` conformant against `crates/iscc-lib/tests/data.json`. `#[test]` count re-counted at
    **342** (unchanged, corroborated by the audit metrics snapshot). No `unsafe` outside the FFI
    crates.
- **Criterion 1 (declared version 16.0.0 + sentinel freeze) — MET.** `utils/unicode16.rs` (731
    unassigned ranges) + `utils/unicode16_case.rs` (152 `Cased`, 452 `Case_Ignorable`).
- **Criterion 2 (`Final_Sigma` case freeze, landed 156) — MET.** Untouched.
- **Criterion 4 (differential sweep as a fail-closed gate, 157, hardened 158) — MET.** Untouched;
    the `unicode-sweep` CI job is green on the tip.
- **Criterion 3 (boundary vectors on every surface) — 10 of 11.** Gated: Python, Node.js, WASM,
    Java, Ruby, Kotlin, C#, Go, C FFI, **C++ (new)**. Ungated: **Swift only**.
- `specs/rust-core.md` L149-157 remains **STALE** (claims Go carries the `Final_Sigma` defect —
    fixed at 147 — and that `str::to_lowercase()` matches the reference, falsified by measurement at
    156); criterion boxes 1/3/4 still unchecked though 1, 2 and 4 are met. Human-owned file; not
    edited.

## Python Bindings

**Status**: met — Unicode-gated since 150; carries the generated-header drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- `uv run pytest --collect-only -q` → **399** tests, **unchanged** — correct, since the C++ slice
    added no Python. `tests/test_unicode_sweep.py` still 14;
    `tests/test_gen_ffi_boundary_vectors.py` still 6.
- **The drift gate now protects two consumers, not one.** I ran
    `uv run --script scripts/gen_ffi_boundary_vectors.py` directly (rather than the pytest anchor
    the review used): it rewrote the header and left `git status --porcelain` **empty** — the
    tracked header is a byte-exact regeneration of the canonical fixture, so both the C and the C++
    test see the current vectors.
- The full 17.8M-comparison sweep stays out of `pytest`, `mise run test` and the pre-push hooks —
    **do not wire it in**. Its one `@pytest.mark.skipif` (rebuild-required guard) is intentional and
    pre-existing.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips. The
    local `.node` addon is gitignored — the recurring "checked-in stale artifact" claim stays
    refuted.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    only under `wasm-pack test --node` (the CI `wasm` job).

## C FFI

**Status**: met — Unicode-gated at 159; now also the *source* of the C++ vectors

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs (unchanged). No FFI
    source moved this iteration.
- `tests/unicode_boundary_vectors.h` (65 lines, tracked, generated) is now included by **two**
    tests: `crates/iscc-ffi/tests/test_iscc.c` (quoted `#include`, resolves beside the includer, no
    `-I` in the CI gcc line) and `packages/cpp/tests/test_iscc.cpp` (one `PRIVATE` include dir).
    Non-ASCII UTF-8 is 3-digit octal, deliberately never `\x`.
- Verified at 159 by header-decode + `ctypes` and re-confirmed no-op-regenerable here.

## C++ Wrapper

**Status**: met — Unicode-gated this iteration (surface 10 of 11)

- `packages/cpp/tests/test_iscc.cpp` gained a `run_unicode_boundary_section()` helper (taking
    `std::string (*)(const std::string&)`) and block 36: 3 metadata guards + both vector sections,
    driven through the **wrapper** functions `iscc::text_clean` / `iscc::text_collapse`.
- **Independently reproduced by a different route than the review's.** I compiled the test with a
    bare `g++ -std=c++17 -Wall -Wextra -Wpedantic` (no CMake, manual `-I`) against
    `target/debug/libiscc_ffi.so`: **zero diagnostics**, and the binary printed **69 passed, 0
    failed**, exit 0, with **12** `PASS: unicode_boundary/…` lines and all 3 metadata guards passing
    (`16.0.0`, 7, 5), `FAIL` count 0.
- Stronger than a count check: I parsed the emitted PASS names and diffed them against the canonical
    `crates/iscc-lib/tests/unicode_boundary.json` — the sets are **identical** (7 `text_clean` + 5
    `text_collapse`, no missing, no extra). The C++ suite covers every canonical vector, not a
    subset.
- **No public-interface leak.** `packages/cpp/CMakeLists.txt` still exposes only `include/` and
    `crates/iscc-ffi/include`; the new include dir is `PRIVATE` on the `test_iscc` target only, and
    its relative path `…/tests/../../../crates/iscc-ffi/tests` resolves correctly (verified with
    `realpath`). The review additionally proved the leak-freedom with a throwaway
    `add_subdirectory()` consumer that fails to compile on the header.
- **Correction to the previous state.md:** it claimed C++ "is not buildable in this container". That
    was wrong on two counts — `uv run --with cmake cmake …` supplies cmake 4.4.0 from PyPI (review,
    160), and the test program needs no cmake at all for a local proof (my g++ route). Only the
    CMake *wiring* needs cmake. **The Swift half of that sentence still stands.**
- Review's mutation probes (wrong expected value, dropped case + decremented count, drifted version
    macro) each red the suite, and a bare `cmake --build` recompiles after a header edit — CMake
    depfiles treat the cross-package header as a build input, so there is **no Gradle-style
    stale-green** here (the class of bug found at 154).
- Housekeeping only: `packages/cpp/` now carries **five** stale gitignored build dirs (`build`,
    `build-asan`, `build-ci`, `build-review`, `build-uv`); `build/` holds an incompatible cmake 3.25
    cache. Zero tree diff — clutter, not an issue. Configure into a fresh dir.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#)

**Status**: met as bindings; all Unicode-gated except **Swift**

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    moved this iteration.
- C# (154, 13 tests), Kotlin (154, 13 tests, fixture registered as a Gradle task **input**), Java
    (153, surefire CWD = pom basedir), Ruby (151, 12 vectors) — all zero-skip.
- `packages/go` re-counted at **177** `func Test`; its vendored `testdata/unicode_boundary.json` is
    byte-identical to the canonical file. **The go1.27 checklist item stands and is recorded nowhere
    but here:** go1.27 ships Unicode 17.0 tables, so Go reacquires the `Final_Sigma` defect the core
    shed at 156 unless an equivalent freeze lands there. Standing ruling (`decisions.md`
    2026-07-26): the skip map stays **unconditional**; the red is the intended trigger. CI pins Go
    via `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- **Swift is the last surface and is structurally different from C/C++.**
    `Tests/IsccLibTests/ConformanceTests.swift` (215 lines) parses JSON with `JSONSerialization`
    against a **tracked vendored** `Tests/IsccLibTests/data.json`, declared in `Package.swift:27` as
    `resources: [.copy("data.json")]` and registered in `VENDORED_COPIES`. The boundary slice
    therefore needs a *second* vendored copy (`unicode_boundary.json`), a second `.copy(…)` entry, a
    `Bundle.module` read, and a `VENDORED_COPIES` registration — the reverse of the C/C++ pattern.
- **Swift is genuinely not locally verifiable:** `swift`, `swiftc` absent from `$PATH`, no
    `/usr/share/swift` or `/opt/swift`, and no PyPI trick (`uv --with swift` installs an unrelated
    OpenStack package — review confirmed). CI-proof-only.

## Documentation

**Status**: met

- `docs/unicode.md` now states that the C FFI test **and the C++ wrapper test** consume a *single*
    generated header, explicitly noting there is no second artifact. Accurate against the code.
- `packages/cpp/CLAUDE.md` gained the header path, the regeneration command, the do-not-hand-edit
    rule, and the test-only-include-dir invariant.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 documentation pages
    consistent** across `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt`. 12 crate/package
    READMEs; 12 crate/package CLAUDE.md files; 11 `docs/howto/*.md`;
    `scripts/version_sync.py --check` **21/21** targets at **0.5.0**.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
- `.iai-baseline.json` and `.crap-baseline.json` are **byte-untouched** (CRAP re-counted at **105**
    entries; `git status --porcelain` on both is empty) — correct, since no Rust or Python source
    moved.

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `bd50c27` (= `origin/develop`): **45 check-runs, 23
    distinct check names, 0 non-success.** That tip is the iteration-160 review commit, i.e. it sits
    **above** the C++ advance commit, so the green run genuinely covers the new slice.
- HEAD is `8c68283`, **three** commits ahead of origin — two `cid(log)` plus the audit's metrics
    snapshot — and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty**, so there is
    no uncovered code. Working tree clean.
- Job shape unchanged: **21 job keys → 22 jobs** (`python-test` is a `[3.10, 3.14]` matrix; `python`
    is an `if: always()` aggregator) **→ 23 check names**, matching the API exactly. The `cpp` job
    (`C++ (cmake, ASAN, test)`) is `apt-get install cmake` → `cargo build -p iscc-ffi` → configure
    (`-DSANITIZE_ADDRESS=ON`) → build → run; it picked up the new vectors with no workflow edit.
- PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — **not shipped**; version is still
    **0.5.0**. (This is why runs appear 2×.)
- Iteration 160 completed cleanly: `iterations.jsonl` shows update-state / define-next / advance /
    review all `OK`, `iteration_summary` verdict **PASS** (200 turns, 2,433 s), plus the cadence
    `audit` role `OK` (8 turns) which filed **no** new issues and committed only `metrics.jsonl`.
- `release.yml` / `docs.yml` byte-unchanged: 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only; `rubygems/configure-rubygems-credentials@main` still unpinned at
    `release.yml:895`. **`specs/ci-cd.md` remains drifted** — job table still 14 rows against 21
    real job keys. Owned by the existing human-authorized issue.
- Enforcing gates green: iai-callgrind perf (>10% Ir, baseline untouched), coverage + CRAP (105
    entries; `--fail-regression` is **CI-only**, so a green `mise run check` proves nothing),
    `cargo-deny` (live advisory DB — can red with no code change), docs page-list parity, and the
    Unicode sweep. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment; major dependency bumps **one per step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper,
    JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source rewrites, one crate per step); make the
    `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
None opened and none closed this iteration (the audit filed nothing). Nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)".** (a1) and (a2) ✅; the remainder
    is **(b)** propagation, updated in place to **10 of 11** with Swift outstanding.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Propagation slice 7 — Swift, the final boundary-vector surface.** It closes criterion 3 of the
Unicode issue outright, but it is the *hardest* of the eleven and the only one that cannot be proven
locally: no `swift`/`swiftc` on `$PATH`, no toolchain under `/usr/share/swift` or `/opt/swift`, and
no PyPI substitute (unlike the cmake-from-PyPI discovery that unblocked C++). **The advance agent
must scope it as CI-proof-only and say so explicitly — do not claim a local run.**

Unlike C and C++, Swift takes the fixture as a *tracked vendored copy*, mirroring what
`ConformanceTests.swift` already does with `data.json`:

1. `cp crates/iscc-lib/tests/unicode_boundary.json packages/swift/Tests/IsccLibTests/` — **`cp`
    only**, never the Write/Edit tools (the `\uXXXX` escapes would decode to literal UTF-8 and
    break byte-identity).
2. Add `.copy("unicode_boundary.json")` to the existing `resources:` array at `Package.swift:27`.
3. Register the new path under the `crates/iscc-lib/tests/unicode_boundary.json` key in
    `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` — the byte-identity gate discovers
    copies by basename and **will red** on an unregistered one.
4. Read it via `Bundle.module` + `JSONSerialization` (same shape as the `data.json` loader) and
    assert the metadata guards (`16.0.0`, 7, 5) plus all 12 vectors, zero skips.

Lighter human-authorized alternatives if a smaller step is wanted: pin
`rubygems/configure-rubygems-credentials` to `@v2.1.0` + `# exact tag:` comment (one line, still
`@main` at `release.yml:895`; verify with the committed `scripts/check_release_workflow.py` gates,
never a retyped heredoc, and require zero `warning: skipped` lines); or make the `specs/ci-cd.md`
job table exhaustive (14 rows vs 21 job keys — re-derive from `ci.yml`, not from prose).

Standing hazards for any fixture work: keep any generated or vendored file pure ASCII, LF-only, one
trailing newline, or the prek hygiene hooks rewrite it and red the regeneration gate; do not relabel
the `e U+A7F1 U+0301` row's oracle back to `e U+015A`. Adding a *vector* to the canonical fixture
remains a separate, deliberate slice — **eleven** consumers now assert exactly 7 `text_clean` + 5
`text_collapse`, so a new row reds them all at once. Propagation invariant re-verified this
iteration: `git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** tracked paths, exactly
matching `VENDORED_COPIES`; a Swift slice takes it to 8.
