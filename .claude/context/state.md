<!-- assessed-at: c548399591aacd5f4abbe05eceb268f9cbf56afb -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode boundary-fixture propagation: 4 of 11 native surfaces gated (plus the pure-Go port), 7 to go, then the differential sweep

Iteration 151 landed propagation slice 2: the canonical Unicode 16.0.0 boundary fixture now gates
the WASM and Ruby binding suites, both reading the canonical file in place with **zero skips** and
**zero new vendored copies**. CI is green on the pushed tip and covers every line of working-tree
code. Rust-core criterion 3 remains partially met — 7 native surfaces and 4 sibling `data.json`
locations are still ungated — and criterion 4 (differential sweep) has not started.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 advancing (4 of 11 surfaces);
criterion 4 not started

- Incremental scope: `git diff 19eecc8..HEAD --stat` = 18 files, **5 outside `.claude/`**:
    `crates/iscc-wasm/tests/unicode_boundary.rs` (new),
    `crates/iscc-rb/test/test_unicode_boundary.rb` (new), `crates/iscc-wasm/CLAUDE.md`,
    `crates/iscc-rb/CLAUDE.md`, `docs/unicode.md`. `git diff 19eecc8..HEAD -- .claude/context/specs/`
    is **empty** and `-- .github/` is **empty** — no spec text and no workflow moved, so no
    met→unmet flip from either side. **No file under any `crates/*/src/` changed**: test/docs-only
    iteration; both baselines (`.crap-baseline.json`, `.iai-baseline.json`) correctly untouched.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions (unchanged — the new tests are integration
    tests in binding crates). All 10 `gen_*_v0` functions still pass the vendored
    `iscc-core/data.json` vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — MET**, unchanged at HEAD (`UNASSIGNED_SENTINEL: char = '\u{FFFF}'`
    in `utils.rs`, mapped at both call sites inside the fused iterator; vendored 731-range /
    819,533-code-point table untouched).
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0), both freely upgradable under the freeze rule.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 4 of 11 native surfaces.**
    - Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` unchanged this iteration (2,344
        bytes, SHA-256 `3ccc4418…`): **7 `text_clean` + 5 `text_collapse`** cases including the four
        sequence vectors that discriminate the ruled sentinel map from the superseded delete filter.
    - **Tally definition (reconciled at 151, anchored in `docs/unicode.md:105`):** the 11 native
        surfaces are Python, Node.js, WASM, C FFI, Java, Ruby, C#, C++, Swift, Kotlin **and the Rust
        crate itself**. `packages/go` is a separate pure-Go reimplementation (gated, but not one of
        the 11); UniFFI is the shared mechanism behind Kotlin/Swift, not an independent surface.
        Earlier state.md used a different denominator — this one matches the docs and issues.md.
    - **Gated: Rust (149), Python (150), WASM (151), Ruby (151)** + the **pure-Go port** (150, 9 of 12
        vectors, exactly 3 ruled skips with a stale-key guard).
    - **WASM — verified this iteration.** `crates/iscc-wasm/tests/unicode_boundary.rs`:
        `include_str!("../../iscc-lib/tests/unicode_boundary.json")` — canonical file, no copy
        (`find . -name unicode_boundary.json` returns exactly **2** paths, unchanged from 150). Three
        `#[wasm_bindgen_test]` fns: a metadata guard (version `16.0.0` + counts 7/5) and one loop per
        section, all **ungated by the `conformance` feature**. Runs only under `wasm-pack test --node`
        (the CI `WASM` job); `cargo test -p iscc-wasm` reports 0 tests by design.
    - **Ruby — verified this iteration.** `crates/iscc-rb/test/test_unicode_boundary.rb`:
        `File.expand_path("../../iscc-lib/tests/unicode_boundary.json", __dir__)` — canonical file, no
        copy. Metadata guard + `define_method` per case (12 vectors), zero skips. CI's `ruby` job runs
        `rake compile` before `rake test`, so the stale-`.so` hazard cannot reach CI.
    - **Remaining gap — 7 native surfaces + 4 sibling `data.json` copies.** Ungated: napi, C FFI,
        JNI/Java, Kotlin, Swift, C#, C++; `packages/{dotnet,kotlin,swift}` and `packages/go` carry a
        `data.json` copy but no `unicode_boundary.json`.
    - **Slice-cost survey (re-verified at 151, corrects inherited text):** suites that **already test
        text functions** — and where the fixture is an extension of an existing pattern — are **napi**
        (`__tests__/functions.test.mjs`, snake_case exports `text_clean`/`text_collapse`, 5 cases),
        **C#** (`SmokeTests.cs`, 4 hits) and **C++** (`tests/test_iscc.cpp`, 6 hits). Suites with
        **zero** text-function coverage today, where the fixture is new plumbing: **C FFI**
        (`tests/test_iscc.c`), **JNI/Java** (`IsccLibTest.java`), **Kotlin** (`ConformanceTest.kt`),
        **Swift** (`ConformanceTests.swift`) — all four grep 0 case-insensitively.
- **Criterion 4 (differential sweep) — NOT started.** `scripts/` still has no sweep harness. The
    requirement is **zero** divergence over both the **1,112,064 Unicode scalar values** and the
    sequence classes; a per-code-point-only sweep is explicitly forbidden as false assurance.
- **Correction to inherited text, re-verified at 151 (third iteration running):** handoff.md and
    issues.md both call `crates/iscc-napi/iscc-lib.linux-x64-gnu.node` a *checked-in* stale
    artifact. It is **not in git** — `git check-ignore -v` resolves it to
    `crates/iscc-napi/.gitignore:4:*.node` and `git ls-files crates/iscc-napi/` lists 9 tracked
    files, none of them `.node` (`index.js`, `index.d.ts`, `package-lock.json` are untracked too).
    The local file dates from Jun 17, so the napi slice does need a local `napi build` before its
    tests mean anything — but there is no stale file in the repository and CI rebuilds the addon
    every run.
- **Stale spec text, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense although the fix landed at iteration
    147; the criterion-1 and criterion-3 **Verified when** boxes remain unchecked despite being
    met/partially met. Spec checkboxes are not a progress signal here.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — gated by the Unicode boundary fixture since 150

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/src/` or `tests/` moved this
    iteration.
- `tests/test_unicode_boundary.py` reads the canonical fixture by relative path (12 vectors + a
    metadata guard), collected by `testpaths = ["tests"]` in the `python-test` 3.10/3.14 matrix.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- Boundary fixture not wired. `__tests__/functions.test.mjs` already exercises
    `text_clean`/`text_collapse`, so the test shape is a copied loop — the real cost is rebuilding
    the local (gitignored) `.node` addon first, which still shows pre-sentinel behaviour (`aSb` for
    `text_clean("a" U+A7F1 "b")`).

## WASM Bindings

**Status**: met — gated by the Unicode boundary fixture (new at 151)

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). No `src/` change.
- `tests/unicode_boundary.rs` added: 3 `#[wasm_bindgen_test]` fns over the canonical fixture, zero
    skips, ungated by the `conformance` feature. `tests/unit.rs` (78 tests) unchanged.
- `CLAUDE.md` module-layout table updated; review also corrected its `conformance.rs` line, which
    had described the canonical `data.json` as "vendored".

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- `crates/iscc-ffi/tests/test_iscc.c` has **zero** text-function coverage, so the boundary fixture
    would be its first — it needs a JSON reader or a generated C table. Scope this deliberately.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Ruby now Unicode-gated, the rest pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. Kotlin 2.3+
    consumer floor stands.
- **Ruby — gated at 151** (see Rust Core). No `crates/iscc-rb/src/` change; `CLAUDE.md` key-files
    table lists the new test.
- Go: `Final_Sigma` fix holds (`packages/go/utils.go:125` uses `cases.Lower(language.Und)`; the
    per-call `cases.Caser` is deliberate — do not hoist it). `packages/go` sits at **177**
    `func Test` and its vendored `testdata/unicode_boundary.json` is still **SHA-256-identical** to
    the canonical file (`3ccc4418…`, verified this iteration).
- **Standing ruling (decisions.md 2026-07-26):** the Go skip map stays **unconditional** — do not
    build-tag it for go1.27. Go passes the two post-16.0 vectors today *by accident* (its 15.0
    tables call `U+20C1` / `U+A7F1` `Cn`), so a go1.27 bump will red 5 cases; that red is the
    intended trigger to land the 731-range freeze table in `packages/go/utils.go` in the **same
    commit** as the toolchain bump. CI pins Go via `go-version-file: packages/go/go.mod`
    (`go 1.26.1`).
- JNI-Java, Kotlin and Swift suites still contain no text-function tests at all; C# and C++ do.

## Documentation

**Status**: met

- `docs/unicode.md` updated in step with the code: the vector paragraph now names the Rust,
    **Python, WASM and Ruby** suites as reading the canonical fixture directly, with the pure-Go
    vendored copy called out separately. Page count unchanged.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt`. 12 crate/package READMEs, 12 crate/package CLAUDE.md files
    (+ root), 11 `docs/howto/*.md`, `scripts/version_sync.py` covers **21** targets (all OK).
- Known incompleteness, deliberately left: the pure-Go admonition explains the *skips* but not that
    go1.27 will also red 5 currently-green cases. Worth widening when the Go freeze table lands.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 54
    `benchmark` references in `tests/test_benchmarks.py` (18 fixtures); documented speedups
    1.3x–158x. Untouched — no perf surface moved.

## CI/CD and Publishing

**Status**: partially met — green and covering all working-tree code

- **Latest CI: GREEN.** check-runs API on `b4e4f1f` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** HEAD is `c548399`, one commit ahead (the
    `cid(log)` commit for iteration 151), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the green run covers
    every line of code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged since the last assessment: 22 check names,
    matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Both new suites run in existing jobs — no new job needed and none should be added: WASM via
    `wasm-pack test --node crates/iscc-wasm --features conformance` in the `WASM` job, Ruby via
    `rake compile` + `rake test` in the `ruby` job.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny`, docs page-list parity.
    `cargo-semver-checks` informational.
- Iteration 151 ran clean end to end: four roles, all `status":"OK"`, verdict `PASS`, no crash and
    no timeout in `iterations.jsonl`.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source
    rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**9 entries in `issues.md` — 5 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
No entry opened or closed this iteration; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Remainders: **(b)** propagate the fixture to the 7 remaining native surfaces and the 4 sibling
    `data.json` locations, and **(a2)** the criterion-4 differential sweep as a runnable check. The
    entry also carries the **go1.27 bump checklist**. Its `**Upstream:**` section holds a
    human-owned task (update `iscc-core#137` to propose the sentinel mechanism) — explicitly *not*
    CID's.
- **NORMAL — `[review]`: "Gate byte-identity of the vendored test-vector copies".** Six vendored
    copies (5 × `data.json`, 1 × `unicode_boundary.json`) match by convention only, with no gate.
    Re-verified green at HEAD this iteration: all five `data.json` share md5 `4f17639ab1dd…` and the
    boundary copy is SHA-256-identical. Not yet implemented — `tests/` contains no such anchor.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Land the vendored-copy byte-identity drift gate, then continue Unicode fixture propagation with
slice 3.** The drift gate is the higher-leverage half-step and is now explicitly sequenced first by
the review handoff: it is a roughly one-file pytest anchor over an explicit `(canonical, copy)`
table asserting byte equality, it is green at HEAD, and the next propagation slice that touches
`packages/{dotnet,kotlin,swift}` adds three more copies that would otherwise stay unguarded. Give
the table a count floor so the assertion cannot pass vacuously on an empty list, and prefer a pytest
anchor over a prek hook as the primary (a `files:`-scoped hook never sees deletions).

For the binding half, pick by **existing text-test shape**, not by alphabet. Three suites already
exercise `text_clean`/`text_collapse` and take a copied loop: **napi** (cheapest test shape, but its
local gitignored `.node` addon must be rebuilt first — it still shows pre-sentinel behaviour),
**C#** and **C++**. Four suites — **C FFI, JNI/Java, Kotlin, Swift** — have no text-function tests
at all, so those are new plumbing; scope one language group per step. C# + C++ is the cheapest
in-container pair that needs no addon rebuild.

Reuse the pattern proven at 150–151 verbatim: read the canonical fixture in place wherever the
language can (Python, WASM and Ruby all do — only Go needed a vendored copy); assert
`unicode_data_version` plus per-section counts (7 / 5) as a metadata guard so a truncated fixture
cannot degrade to a zero-iteration loop; never copy the `delete_filter_output` oracle (plain
equality against `outputs.result` already reds a delete-filter regression); and give any skip list a
stale-key guard. Two standing hazards: never write the fixture through the Write/Edit tools (the
`\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`, and verify
with `ord()`), and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the
category-override design's output, not a delete filter's).

After propagation, the last Unicode item is **(a2)**, the criterion-4 differential sweep covering
both the 1,112,064 scalar values and the sequence classes. Neither task touches a hot path, so no
perf or CRAP gate should react.
