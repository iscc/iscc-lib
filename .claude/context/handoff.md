# Handoff

## 2026-07-26 — Review of: Propagate the Unicode boundary fixture to the C# and Kotlin test suites (slice 4)

**Verdict:** PASS_WITH_NOTES

**Summary:** The C# and Kotlin boundary suites are correct, idiomatic, in scope, and genuinely
gating — I mutation-probed both and each reds the *named* vector on a delete-filter-shaped expected
value and reds the metadata guard on a dropped case. Criterion 3 goes 6 → **8 of 11** native
surfaces with no new tracked fixture copy. One real hole, found by the independent Codex review and
confirmed by a three-run probe: the Kotlin `Test` task did not treat the out-of-tree fixture as an
input, so `./gradlew test` reported `UP-TO-DATE` and silently skipped all 13 boundary tests after a
fixture edit. Fixed in this review with one additive, gate-strengthening line (CI was never
affected).

**Verification:** (every criterion from next.md, all commands re-run this session)

- [x] `cargo build -p iscc-ffi` exit 0 →
    `LD_LIBRARY_PATH=$PWD/target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/` — **104 passed,
    0 failed, 0 skipped** (≥ 104 met exactly)
- [x] `--filter FullyQualifiedName~UnicodeBoundary` — **13 passed, 0 failed, 0 skipped**
- [x] `cargo build -p iscc-uniffi` exit 0 → `gradlew -p packages/kotlin cleanTest test --offline`
    exit 0; `TEST-uniffi.iscc_uniffi.UnicodeBoundaryTest.xml` written at 22:23 (newer than the
    source) with `tests="13" skipped="0" failures="0" errors="0"` and all 12 vector names +
    `boundaryFixtureMetadata()` as individual `testcase` entries
- [x] `git ls-files -- '*unicode_boundary.json'` = **2** paths; `git ls-files -- '*data.json'` =
    **5** — no tracked copy added
- [x] `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed** (`VENDORED_COPIES`
    untouched)
- [x] `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json packages/dotnet/Iscc.Lib/NativeMethods.g.cs packages/kotlin/src/main`
    — **empty** (CRAP / iai / semver gates inert for this diff)
- [x] No build artifact untracked-and-stageable — `git status --porcelain` showed only
    `iterations.jsonl` (runner-owned) before my fix
- [x] `uv run scripts/check_docs_nav.py` — `OK: 23 documentation pages consistent`
- [x] `docs/unicode.md` canonical-fixture sentence names **C#** and **Kotlin** alongside Python,
    Node.js, WASM, Java, Ruby; the pure-Go vendored-copy clause is intact
- [x] `mise run check` exit 0 (all prek hooks Passed) and the tree clean afterwards; pre-push stage
    (`prek run --hook-stage pre-push --all-files`) also all Passed
- [x] `cargo test -p iscc-lib` — 281 + 28 + 22 + 4 + 1 passed, 0 failed;
    `cargo clippy --workspace --all-targets -- -D warnings` clean

**Review-added probes (beyond next.md):**

- **Mutation A** — `text_clean/test_0006_seq_ua7f1_no_decomposition_leak` expected value set to the
    delete-filter result `é`: C# **12 passed / 1 failed** naming
    `TextCleanBoundary(_: "test_0006_seq_ua7f1_no_decomposition_leak", …)`; Kotlin **1 failed**
    naming `textCleanBoundary() > test_0006_seq_ua7f1_no_decomposition_leak`. The sentinel-vs-delete
    distinction is genuinely gated on both surfaces without an oracle column.
- **Mutation B** — one `text_clean` case deleted: both metadata guards red
    (`BoundaryFixtureMetadata` / `boundaryFixtureMetadata()`), so a truncated fixture cannot
    silently shrink the run.
- **Freshness** — both natives rebuilt from source before every run; `cargo build -p iscc-uniffi`
    recompiled (expected: the advance agent's `cargo clippy --all-targets` had replaced the
    artifacts), so the suites ran against a freshly linked core, not the advance agent's binaries.
- **Gate-circumvention scan** over the whole unpushed batch (`@{upstream}..HEAD`, 4 commits): zero
    suppressions, skips, threshold changes or hook weakenings. No CI, lint-config or manifest file
    is touched.

**Issues found:**

- **(fixed in this review)** Kotlin `Test` tasks track only their own project tree, so
    `crates/iscc-lib/tests/unicode_boundary.json` was not a declared input. Probe: `cleanTest test`
    (green) → `test` (`UP-TO-DATE`, correct) → **mutate fixture** → `test` still `UP-TO-DATE` and
    all 13 boundary tests silently did not run. Fixed with
    `inputs.file("$fixtureDir/unicode_boundary.json").withPathSensitivity(PathSensitivity.NONE)` in
    the existing `tasks.withType<Test>` block, plus a comment. Re-probed: no change → `UP-TO-DATE`;
    fixture edit → task re-executes and reds; restore → greens. **CI was never affected** (the
    `kotlin` job is checkout + `cargo build` + `./gradlew test` with no build-dir cache), so this is
    a local/CID verification hole, not a shipped defect. `PathSensitivity.NONE` is deliberate: the
    absolute path differs per checkout and must not by itself invalidate the task.
- No scope violations. Two non-test/non-doc files modified (`Iscc.Lib.Tests.csproj`,
    `build.gradle.kts`) — within the 3-file budget. Every `## Not In Scope` item respected: no
    vendored copy, no `delete_filter_output` oracle, no `crates/*/src` change, no baseline refresh,
    no regenerated binding, no dependency edit, no CI job, no spec edit, and the issue was left for
    review to update.

**Codex review:** one P2, and it was right. `packages/kotlin/build.gradle.kts:37` — "this property
tracks only the unchanged directory string, not the fixture contents; Gradle therefore reports
`:test UP-TO-DATE` and skips all 13 boundary tests even if the new vectors would fail." I reproduced
it exactly as described and applied its recommended `inputs.file(...)` fix (with
`PathSensitivity.NONE` added). Codex found nothing else and explicitly cleared the vector logic.
Worth noting for the loop: every surface-level gate was green (13/13 on both suites, plus my own
mutation probe — which only worked because I used `cleanTest`), so this hazard was invisible to the
entire verification grid. Rationale recorded in `decisions.md` (2026-07-26, "Binding boundary suites
link the canonical fixture instead of vendoring a copy").

**Next:** Two reasonable candidates, in this order of value:

1. **Criterion 4 — the differential sweep, wired in as a runnable check** (`issues.md` → "Declare
    and gate a Unicode data version", item (a2)). It is the only remaining piece with no partial
    credit, it protects every future table bump on every surface at once, and it is fully local (no
    missing toolchain). It **must** cover both the 1,112,064 Unicode scalar values *and* sequence
    classes — a per-code-point sweep scored the superseded pre-filter 0 failures while it failed 42
    of 140 sequence cases. Harness sketch in `decisions.md` 2026-07-26 ("Sentinel conformance
    accepted on sequence evidence"). Treat it as a **new gate script**: define-next should scope
    both the sweep and its blind-spot properties (is it fail-open on a missing table? does a
    zero-case run read as green? does it distinguish "skipped" from "passed"?).
2. **C FFI slice 5** — the cheapest remaining surface that is buildable here.
    `crates/iscc-ffi/tests/test_iscc.c` has neither a JSON reader nor any text-function coverage,
    so it needs either a generated C vector table (build-script or checked-in, generated from the
    canonical fixture) or a hand-rolled reader. C++ and Swift stay blocked — no `cmake`, no `swift`
    toolchain in this container; Swift is also the one surface that would add a *tracked* vendored
    copy and must register it in `VENDORED_COPIES`.

Whichever is picked, carry forward the question this iteration surfaced: **is the fixture a declared
input of that surface's build system?** SwiftPM resource bundling, CMake and a generated C table
each answer it differently, and a green run does not.

**Notes:**

- `packages/dotnet` and `packages/kotlin` now handle `data.json` (tracked vendored copy) and
    `unicode_boundary.json` (linked to the canonical file) asymmetrically. That is deliberate — see
    `decisions.md` — not an oversight to "clean up".
- The Kotlin suite's `@TestFactory`/`DynamicTest` shape is load-bearing: a plain `@Test` loop
    reports 3 `testcase` entries in the Gradle XML, which would make `tests="13"` unmeetable and
    hide per-vector failure names. Same reasoning as the iteration-153 Java twin.
- `dotnet test --list-tests` shows theories at *method* level only (3 names for this class). That is
    pre-discovery, not a missing test — the runtime failure line does carry the case name.
- Gradle emits a pre-existing "Deprecated Gradle Version" warning (wrapper 8.12.1 vs Kotlin plugin
    2.4.10). The wrapper bump is one of the separately-authorized majors under the dependency issue,
    not a regression from this step.
- `learnings.md` was over its 200-line budget; the Windows-`pwsh` entry was archived and two closed
    gate entries condensed. Review memory `MEMORY.md` was compacted 161 → 139 lines with the detail
    pushed into `binding-reviews.md` and `codex-integration.md`.
