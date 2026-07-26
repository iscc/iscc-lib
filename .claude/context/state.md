<!-- assessed-at: 672d109c2c87e92f2d08cbbf9c2267577c2f0688 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — vendored-fixture drift gate landed; Unicode boundary propagation still at 4 of 11 native surfaces

Iteration 152 landed the byte-identity drift gate for vendored conformance fixtures
(`tests/test_vendored_fixtures.py`) — one new test file plus one doc line, zero source changes. CI
is green on the pushed tip and covers every line of working-tree code. Unicode criterion 3 did not
move this iteration: 4 of 11 native surfaces are gated (plus the pure-Go port), and criterion 4 (the
differential sweep) has not started.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 stalled at 4 of 11 surfaces; criterion
4 not started

- Incremental scope: `git diff c548399..HEAD --stat` = 16 files, **2 outside `.claude/`**:
    `tests/test_vendored_fixtures.py` (new, 99 lines) and `crates/iscc-lib/CLAUDE.md` (+2 lines
    pointing at the new gate). `git diff c548399..HEAD -- .claude/context/specs/` is **empty** and
    `-- .github/` is **empty** — no spec text and no workflow moved, so no met→unmet flip from
    either side. **No file under any `crates/*/src/` changed**, and neither `.crap-baseline.json`
    nor `.iai-baseline.json` was touched.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions (unchanged — the new gate lives in the
    root-level pytest tree, not in a crate). All 10 `gen_*_v0` functions still pass the vendored
    `iscc-core/data.json` vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — MET**, unchanged at HEAD (`UNASSIGNED_SENTINEL: char = '\u{FFFF}'`
    in `utils.rs`, mapped at both call sites inside the fused iterator; vendored 731-range /
    819,533-code-point table untouched).
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0), both freely upgradable under the freeze rule.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 4 of 11 native surfaces, no
    change this iteration.**
    - Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` unchanged (SHA-256 `3ccc4418…`):
        **7 `text_clean` + 5 `text_collapse`** cases including the four sequence vectors that
        discriminate the ruled sentinel map from the superseded delete filter.
    - **Tally definition** (anchored in `docs/unicode.md:105`): the 11 native surfaces are Python,
        Node.js, WASM, C FFI, Java, Ruby, C#, C++, Swift, Kotlin **and the Rust crate itself**.
        `packages/go` is a separate pure-Go reimplementation (gated, not one of the 11); UniFFI is the
        shared mechanism behind Kotlin/Swift, not an independent surface.
    - **Gated: Rust (149), Python (150), WASM (151), Ruby (151)** + the **pure-Go port** (150, 9 of 12
        vectors, exactly 3 ruled skips with a stale-key guard). `git ls-files | grep unicode_boundary`
        returns exactly 7 paths — 1 canonical fixture, 1 vendored Go copy, 5 test files — matching
        that tally with no surprises.
    - **Remaining gap — 7 native surfaces + 4 sibling `data.json` copies.** Ungated: napi, C FFI,
        JNI/Java, Kotlin, Swift, C#, C++; `packages/{dotnet,kotlin,swift}` and `packages/go` carry a
        `data.json` copy but no `unicode_boundary.json`.
    - **Slice-cost survey redone this iteration on TWO axes (corrects both the previous state.md and
        the 152 handoff).** Cost is driven by fixture-reading plumbing *and* text-function coverage,
        not by text coverage alone:
        - **Both axes present, canonical path, no vendored copy:** **napi** (`__tests__` has 13
            snake_case `text_clean`/`text_collapse` hits and `conformance.test.mjs` already reads
            `../../iscc-lib/tests/data.json`) — cheapest shape, but its local gitignored `.node` addon
            must be rebuilt first. **JNI/Java** has gson plus a `Files.readString` of the canonical
            `../../iscc-lib/tests/data.json` (91 json refs) but **zero** text-function tests — a loader
            is reusable, the vectors are new.
        - **JSON plumbing but no text tests, and a vendored-copy pattern:** **Kotlin** (classloader
            resource), **Swift** (`Bundle.module`), **C#** (`AppContext.BaseDirectory` plus a
            `testdata/` content copy, 4 text hits in `SmokeTests.cs`). Each of these adds a *new tracked
            copy* that must be registered in `VENDORED_COPIES`.
        - **No JSON parsing at all — most expensive:** **C FFI** (`tests/test_iscc.c`, its 3 "json" hits
            are the `iscc_json_to_data_url` case, not a parser) and **C++**
            (`packages/cpp/tests/test_iscc.cpp` is a 59-assertion hand-written smoke suite with 6 text
            hits and no vector file anywhere). **C++ additionally cannot be built in this container —
            `cmake` is not installed** (`dotnet`, `mvn`, `gcc`, `java`, `go`, `ruby`, `node`,
            `wasm-pack` all are).
- **Criterion 4 (differential sweep) — NOT started.** `scripts/` still has no sweep harness. The
    requirement is **zero** divergence over both the **1,112,064 Unicode scalar values** and the
    sequence classes; a per-code-point-only sweep is explicitly forbidden as false assurance.
- **Correction to inherited text, re-verified at HEAD (fourth iteration running):** the 152 handoff
    again calls `crates/iscc-napi/iscc-lib.linux-x64-gnu.node` a *checked-in* stale artifact. It is
    **not in git** — `git check-ignore -v` resolves it to `crates/iscc-napi/.gitignore:4:*.node`,
    and `git ls-files crates/iscc-napi/` lists 9 tracked files, none of them `.node`. The local file
    dates from Jun 17, so the napi slice does need a local `napi build` before its tests mean
    anything, but there is no stale file in the repository and CI rebuilds the addon every run.
- **Stale spec text, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense although the fix landed at iteration
    147; the criterion-1 and criterion-3 **Verified when** boxes remain unchecked despite being
    met/partially met. Spec checkboxes are not a progress signal here.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — gated by the Unicode boundary fixture since 150; now also host to the drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- `tests/test_unicode_boundary.py` reads the canonical fixture by relative path (12 vectors + a
    metadata guard).
- **New: `tests/test_vendored_fixtures.py` — verified independently this iteration.** 8 collected
    tests: 5 parametrized byte-identity cases (one per vendored copy), a non-vacuity floor
    (`len(PAIRS) >= 5` + canonical-existence), a `git ls-files` set-equality check that catches an
    unregistered *or deleted* tracked copy, and an ASCII guard on the boundary fixture. Re-verified
    at HEAD: `git ls-files -- '*data.json' '*unicode_boundary.json'` returns exactly the **7** paths
    the table registers; all five `data.json` share md5 `4f17639ab1dd…` and both
    `unicode_boundary.json` share SHA-256 `3ccc4418…`. The gate reads the git **index**, so a staged
    copy is caught and untracked build outputs are correctly invisible.
- Known blind spot, recorded not fixed: discovery keys on the two basenames, so a copy vendored
    under a different filename escapes the set check. Mitigation is to keep canonical basenames when
    propagating — noted in learnings.md and in the Unicode issue.
- Both files are collected by `testpaths = ["tests"]` and therefore run in the `python-test`
    3.10/3.14 CI matrix and in the `always_run` pre-push pytest hook.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- Boundary fixture not wired, but this is the **cheapest remaining slice by test shape**: the suite
    already loads the canonical `data.json` by relative path and already calls
    `text_clean`/`text_collapse` (exports are **snake_case**). The real cost is rebuilding the local
    gitignored `.node` addon, which still shows pre-sentinel behaviour (`aSb` for
    `text_clean("a" U+A7F1 "b")`).

## WASM Bindings

**Status**: met — gated by the Unicode boundary fixture since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched this iteration.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns over the canonical fixture via
    `include_str!`, zero skips, ungated by the `conformance` feature; runs only under
    `wasm-pack test --node` (the CI `WASM` job).

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- **Most expensive propagation target, jointly with C++**: `tests/test_iscc.c` has neither a JSON
    parser nor any text-function coverage, so the fixture needs a generated C table or a hand-rolled
    reader. The 152 handoff's "C FFI + JNI/Java is the cheapest in-container pair" holds for the JNI
    half only — scope it deliberately if picked.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Ruby Unicode-gated, the rest pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. Kotlin 2.3+
    consumer floor stands. No binding source changed this iteration.
- **Ruby — gated since 151**: `crates/iscc-rb/test/test_unicode_boundary.rb` reads the canonical
    fixture via `File.expand_path`, 12 vectors, zero skips.
- Go: `Final_Sigma` fix holds (`packages/go/utils.go:125` uses `cases.Lower(language.Und)`; the
    per-call `cases.Caser` is deliberate — do not hoist it). `packages/go` sits at **177**
    `func Test` and its vendored `testdata/unicode_boundary.json` is byte-identical to the canonical
    file — now machine-enforced rather than convention.
- **Standing ruling (decisions.md 2026-07-26):** the Go skip map stays **unconditional** — do not
    build-tag it for go1.27. Go passes the two post-16.0 vectors today *by accident* (its 15.0
    tables call `U+20C1` / `U+A7F1` `Cn`), so a go1.27 bump will red 5 cases; that red is the
    intended trigger to land the 731-range freeze table in `packages/go/utils.go` in the **same
    commit** as the toolchain bump. CI pins Go via `go-version-file: packages/go/go.mod`
    (`go 1.26.1`).
- JNI-Java, Kotlin and Swift suites still contain no text-function tests at all (verified
    case-insensitively); C# and C++ do.

## Documentation

**Status**: met

- `crates/iscc-lib/CLAUDE.md` gained the two-line pointer to `tests/test_vendored_fixtures.py` — the
    only doc change this iteration, and the right place for it (the fixture-file section).
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt`. 12 crate/package READMEs, 12 crate/package CLAUDE.md files
    (+ root), 11 `docs/howto/*.md`, `scripts/version_sync.py` covers **21** targets (all OK).
- `docs/unicode.md` unchanged and still accurate: it names the Rust, Python, WASM and Ruby suites as
    reading the canonical fixture, with the pure-Go vendored copy called out separately.
- Known incompleteness, deliberately left: the pure-Go admonition explains the *skips* but not that
    go1.27 will also red 5 currently-green cases. Worth widening when the Go freeze table lands.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
    Untouched — no perf surface moved, so neither the iai nor the CRAP gate could react.

## CI/CD and Publishing

**Status**: partially met — green and covering all working-tree code

- **Latest CI: GREEN.** check-runs API on `cec0dc4` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** HEAD is `672d109`, one commit ahead (the
    `cid(log)` commit for iteration 152), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the green run covers
    every line of code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged since the last assessment: 22 check names,
    matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- The new drift gate needs **no new CI job** — it rides `python-test` (ubuntu, 3.10 + 3.14) in an
    `actions/checkout` git tree, and `.gitattributes` pins `*.json` to `eol=lf` so byte-identity
    cannot be broken by checkout normalization. All five gated suites now ride existing jobs
    (`python-test`, `Go`, `WASM`, `ruby`).
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny`, docs page-list parity.
    `cargo-semver-checks` informational.
- Iteration 152 ran clean end to end: four roles, all `"status":"OK"`, verdict `PASS`, no crash and
    no timeout in `iterations.jsonl`. One environment flake was recorded and correctly classified as
    infrastructure, not a gate rejection: three parallel `clippy-driver` processes died with SIGBUS
    during a pre-push hook; re-running `cargo clippy --workspace --all-targets -- -D warnings` exits
    cleanly. Treat a recurrence as a container/toolchain issue, not a work package.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source
    rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
One closed this iteration (the drift gate), none opened; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Remainders: **(b)** propagate the fixture to the 7 remaining native surfaces and the 4 sibling
    `data.json` locations, and **(a2)** the criterion-4 differential sweep as a runnable check. The
    entry now also carries the `VENDORED_COPIES` registration requirement and the go1.27 bump
    checklist. Its `**Upstream:**` section holds a human-owned task (update `iscc-core#137` to
    propose the sentinel mechanism) — explicitly *not* CID's.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Resume Unicode boundary propagation with slice 3 — and pick the pair on the two-axis cost survey
above, not on the single "already calls text functions" axis that misled the last two iterations.**
Two candidate pairs, both defensible:

- **napi + JNI/Java** — napi is the cheapest test shape in the repo (canonical `data.json` already
    read by relative path, `text_clean`/`text_collapse` already exercised, snake_case exports) and
    its only cost is a local `napi build` of the gitignored addon; Java has gson plus the same
    canonical-relative-path loader, so it needs a new vector loop but no new plumbing and no
    vendored copy. Both keep `VENDORED_COPIES` untouched.
- **JNI/Java + Kotlin** — same UniFFI-adjacent JVM toolchain in one step, but Kotlin loads fixtures
    as a classloader resource, so it *adds* a tracked copy that must be registered in
    `VENDORED_COPIES` (`test_no_unregistered_tracked_copy` reddening is the gate working, not a
    bug).

Avoid **C FFI** and **C++** for now: neither suite parses JSON at all, so each needs a generated
table or a hand-rolled reader, and **C++ cannot even be built in this container (`cmake` is
missing)** — verification would rest on CI alone.

Reuse the pattern proven at 150–152 verbatim: read the canonical fixture in place wherever the
language can; assert `unicode_data_version` plus per-section counts (7 / 5) as a metadata guard so a
truncated fixture cannot degrade to a zero-iteration loop; never copy the `delete_filter_output`
oracle (plain equality against `outputs.result` already reds a delete-filter regression); give any
skip list a stale-key guard; and keep the canonical basenames `data.json` / `unicode_boundary.json`
for any new copy — the drift gate discovers copies by basename, so a renamed copy is invisible to
it. Two standing hazards: never write the fixture through the Write/Edit tools (the `\uXXXX` escapes
decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`), and do not relabel the
`e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the category-override design's output,
not a delete filter's).

After propagation, the last Unicode item is **(a2)**, the criterion-4 differential sweep covering
both the 1,112,064 scalar values and the sequence classes. Neither task touches a hot path, so no
perf or CRAP gate should react.
