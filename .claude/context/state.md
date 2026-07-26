<!-- assessed-at: 19eecc8eadb74a4cbb25624e7b2ba5e221a7b46b -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode boundary-fixture propagation is underway: 2 of 11 binding surfaces gated, 9 to go, then the differential sweep

Iteration 150 landed propagation slice 1: the canonical Unicode 16.0.0 boundary fixture now gates
the Python binding (reads the canonical file directly, 12 vectors + a metadata guard) and the
pure-Go package (byte-identical vendored copy, 9 of 12 vectors live, exactly 3 ruled skips with a
stale-skip guard). CI is green on the pushed tip and covers every line of working-tree code.
Rust-core criterion 3 remains partially met — 9 binding surfaces and 3 sibling `data.json` locations
are still ungated — and criterion 4 (differential sweep) has not started.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 advancing (Rust done, 2/11 bindings);
criterion 4 not started

- Incremental scope: `git diff 8183ffb..HEAD --stat` = 22 files, **5 outside `.claude/`**:
    `tests/test_unicode_boundary.py`, `packages/go/unicode_boundary_test.go`,
    `packages/go/testdata/unicode_boundary.json`, `packages/go/CLAUDE.md`, `docs/unicode.md`.
    `git diff 8183ffb..HEAD -- .claude/context/specs/` is **empty** and `-- .github/` is **empty** —
    no spec text and no workflow moved, so no met→unmet flip from either side. **No file under any
    `crates/*/src/` changed**: this was a test/docs-only iteration, and both baselines
    (`.crap-baseline.json`, `.iai-baseline.json`) were correctly left untouched.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions (unchanged — the new tests live in `tests/`
    and `packages/go/`). All 10 `gen_*_v0` functions still pass the vendored `iscc-core/data.json`
    vectors. Version **0.5.0**.
- **Criterion 1 (freeze rule) — MET**, unchanged at HEAD: `utils.rs` defines
    `const UNASSIGNED_SENTINEL: char = '\u{FFFF}'`, both call sites map to it inside the fused
    iterator, the superseded pre-normalization delete filter is gone, and the category-`C`/`CMP`
    filters are the reference's. Vendored 731-range / 819,533-code-point table untouched.
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0) both qualify and are explicitly freely upgradable.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, now 2 of 11 surfaces.**
    - Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` (2,344 bytes, pure ASCII): **7
        `text_clean` + 5 `text_collapse`** cases. Re-verified this iteration by decoding to numeric
        code points, not by reading glyphs — the four sequence vectors are intact (`e U+0378 U+0301` →
        `e U+0301`; `U+1100 U+0378 U+1161` → `U+1100 U+1161`; `e U+A7F1 U+0301` → `e U+0301`;
        `U+0391 U+03A3 U+0378 U+0392` → `U+03B1 U+03C2 U+03B2`), so the fixture still discriminates
        the ruled sentinel map from the superseded delete filter.
    - **Python — gated.** `tests/test_unicode_boundary.py` loads the canonical fixture by relative
        path and parametrizes both sections (12 vectors) plus `test_boundary_fixture_metadata`
        (version string + per-section counts 7/5). Collected by `testpaths = ["tests"]`, so it runs in
        the `python-test` matrix job.
    - **Go — gated with ruled skips.** `packages/go/testdata/unicode_boundary.json` is
        **byte-identical** to the canonical file (verified independently: same SHA-256, same 2,344
        bytes, both `isascii()`), embedded via `//go:embed`. `packages/go/unicode_boundary_test.go`
        runs both sections through the shared `parseConformanceData` loader and skips exactly the 3
        table-dependent cases named in the 2026-07-26 ruling (`text_clean` U+1FAE9 + U+113C5,
        `text_collapse` U+1FAE9), each with an inline reason citing `decisions.md`. A metadata test
        guards version, counts **and stale skip keys** (a renamed vector makes the skip key fail
        loudly instead of silently masking). All four sequence vectors run live in Go. `packages/go`
        is now at **177** `func Test` (was 174).
    - **Remaining gap — 9 surfaces + 3 sibling files.** Ungated: napi, WASM, Ruby, C FFI, JNI/Java,
        UniFFI, Kotlin, Swift, C#, C++; and `packages/{dotnet,kotlin,swift}` plus the shared
        `crates/iscc-lib/tests/data.json` siblings carry no `unicode_boundary.json`. Scoping detail
        worth carrying: `crates/iscc-wasm/tests/unit.rs` already exercises `text_clean`/
        `text_collapse`, but the **C FFI, JNI-Java, Kotlin and Swift test suites contain no text
        function tests at all** — the boundary fixture would be their first, so those slices need new
        plumbing, not a copied loop.
- **Criterion 4 (differential sweep) — NOT started.** `scripts/` still has no sweep harness. The
    requirement is **zero** divergence over both the **1,112,064 Unicode scalar values** and the
    sequence classes; a per-code-point-only sweep is explicitly forbidden as false assurance.
- **Correction to inherited text (verify before acting):** handoff.md and issues.md both call
    `crates/iscc-napi/iscc-lib.linux-x64-gnu.node` a *checked-in* stale artifact. It is **not in
    git** — `crates/iscc-napi/.gitignore:4` ignores `*.node`, and the local file dates from Jun 17.
    The napi slice therefore needs a local `napi build` before its tests mean anything, but there is
    no stale file in the repository and CI rebuilds the addon every run.
- **Stale spec text, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense although the fix landed at iteration
    147 and has passed CI since; the criterion-1 and criterion-3 **Verified when** boxes are
    likewise unchecked despite being met/partially met. Spec checkboxes are not a progress signal
    here.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — and now the first binding gated by the Unicode boundary fixture

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/src/` moved.
- `tests/test_unicode_boundary.py` added (13 tests). Review reported the full Python suite at **371
    passed**, no regressions.

## Node.js Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- Boundary fixture not wired. This is the **most expensive remaining slice** — the local `.node`
    addon must be rebuilt first (it currently returns `aSb` for `text_clean("a" U+A7F1 "b")`, i.e.
    pre-sentinel behaviour).

## WASM Bindings

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unit.rs` already covers `text_clean`/`text_collapse`, so adding the fixture is an extension
    of an existing pattern — one of the cheaper next slices.

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- `crates/iscc-ffi/tests/test_iscc.c` has **no** text-function coverage today; the boundary fixture
    would need a JSON reader or a generated C table — scope this deliberately.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. Kotlin 2.3+
    consumer floor stands.
- Go: `Final_Sigma` fix holds (`packages/go/utils.go:125` uses `cases.Lower(language.Und)`; the
    per-call `cases.Caser` is deliberate — do not hoist it), and the `Final_Sigma` sequence vector
    now runs green in Go. `packages/go/CLAUDE.md` was updated to document the fixture and skip map.
- **New ruling to respect (decisions.md 2026-07-26):** the Go skip map stays **unconditional** — do
    not build-tag it for go1.27. Go passes the two post-16.0 vectors today *by accident* (its 15.0
    tables call `U+20C1` / `U+A7F1` `Cn`), so a go1.27 bump will red 5 cases; that red is the
    intended trigger to land the 731-range freeze table in `packages/go/utils.go` in the **same
    commit** as the toolchain bump. CI pins Go via `go-version-file: packages/go/go.mod`
    (`go 1.26.1`), so nothing moves unintentionally.
- Ruby, C#, C++ test suites do exercise text functions today; JNI-Java, Kotlin and Swift do not.

## Documentation

**Status**: met

- `docs/unicode.md` updated in step with the code: the vector paragraph now names all three gated
    suites (Rust, Python, vendored Go copy), and the pure-Go admonition states that Go runs 9 of 12
    vectors including all four sequence vectors and skips exactly three until go1.27.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt` (no page added). 12 crate/package READMEs, 12 crate/package
    CLAUDE.md files (+ root), 11 `docs/howto/*.md`, `scripts/version_sync.py` covers **21** targets
    (all OK).
- Known incompleteness, deliberately left: the pure-Go admonition explains the *skips* but not that
    go1.27 will also red 5 currently-green cases. Worth widening when the Go freeze table lands.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures; documented speedups 1.3x–158x. Untouched — no perf surface moved.

## CI/CD and Publishing

**Status**: partially met — green and covering all working-tree code

- **Latest CI: GREEN.** check-runs API on `a09f1d0` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** HEAD is `19eecc8`, three commits ahead
    (`cid(log)` ×2 + `cid(audit): metrics snapshot`), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the green run covers
    every line of code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged: 20 YAML jobs → 21 jobs → 22 check names,
    matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Both new suites run in existing jobs — no new job needed: pytest via `testpaths = ["tests"]` in
    the `python-test` 3.10/3.14 matrix, Go via `CGO_ENABLED=0 go test -v -count=1 ./...` in the `Go`
    job.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**), `cargo-deny`, docs page-list parity. `cargo-semver-checks` informational.
- The iteration-150 audit ran on cadence (every 10th iteration), status `OK`, and filed **zero**
    issues — commit `e52116a` contains only the metrics snapshot.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source
    rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**9 entries in `issues.md` — 5 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
One net new entry this iteration; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Remainders: **(b)** propagate the fixture to the 9 remaining bindings and the 3 remaining
    sibling `data.json` locations (Python + Go now ✅ with mutation evidence), and **(a2)** the
    criterion-4 differential sweep as a runnable check. The entry now also carries the **go1.27 bump
    checklist**. Its `**Upstream:**` section holds a human-owned task (update `iscc-core#137` to
    propose the sentinel mechanism) — explicitly *not* CID's.
- **NORMAL — new, `[review]`: "Gate byte-identity of the vendored test-vector copies".** Six
    vendored copies (5 × `data.json`, 1 × `unicode_boundary.json`) match by convention only, with no
    gate. Green at HEAD (all five `data.json` share one md5; the boundary copy is SHA-identical —
    both re-verified this iteration), so the step is small and its value compounds with every
    remaining propagation slice.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Continue Unicode fixture propagation with slice 2, and land the byte-identity drift gate with it
or just before it.** The drift gate is the higher-leverage half-step: it is a roughly one-file
pytest anchor asserting each vendored copy equals its canonical source, it is green at HEAD, and
every remaining slice adds another copy that would otherwise be unguarded — landing it now means the
rest of the propagation is drift-proof by construction rather than by convention. Prefer a pytest
anchor over a prek hook as the primary (a `files:`-scoped hook never sees deletions).

For the binding half, pick by build cost, not by alphabet: **WASM + Ruby** or **C FFI + JNI** are
runnable in-container; **napi is the most expensive** because its local `.node` addon must be
rebuilt first (it still shows pre-sentinel behaviour). Note that C FFI, JNI-Java, Kotlin and Swift
have **no text-function tests at all** today, so those slices are new plumbing rather than a copied
loop — scope one language group per step.

Reuse the pattern proven at 150 verbatim: read the canonical fixture by relative path where the
language can, otherwise vendor with `cp` and verify with `cmp`; assert `unicode_data_version` plus
per-section counts (7 / 5); never copy the `delete_filter_output` oracle (plain equality against
`outputs.result` already reds a delete-filter regression); and give any skip list a stale-key guard.
Two standing hazards: never write the fixture through the Write/Edit tools (the `\uXXXX` escapes
decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`, and verify with `ord()`),
and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the
category-override design's output, not a delete filter's).

After propagation, the last Unicode item is **(a2)**, the criterion-4 differential sweep covering
both the 1,112,064 scalar values and the sequence classes. Neither task touches a hot path, so no
perf or CRAP gate should react.
