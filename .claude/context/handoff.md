# Handoff

## 2026-07-27 — Review of: Propagation slice 7 — Unicode boundary vectors in the Swift test suite

**Verdict:** PASS

**Summary:** Swift is now gated on all 12 Unicode 16.0.0 boundary vectors — the eleventh and last
binding surface, closing criterion (b) of the Unicode issue. Everything the advance agent claimed
reproduced independently this session, including the Swift toolchain claim itself: I ran the full
suite from a clean scratch path (`Executed 12 tests, with 0 failures`) and then mutation-probed the
one design hazard three ways. The diff is 2 non-test/non-doc files (`Package.swift`, `.gitignore`),
well inside budget, and touches nothing on the `Not In Scope` list.

**Verification:** (every next.md criterion, re-run this session)

- [x] `cmp` canonical vs. Swift copy → identical (re-checked again *after* `mise run check`)
- [x] `git ls-files -- '*unicode_boundary.json' | wc -l` → **3**; copy is pure ASCII, 2344 bytes
- [x] `uv run pytest -q tests/test_vendored_fixtures.py` → **9 passed**
- [x] `uv run pytest --collect-only -q` → **400 tests collected**
- [x] `cargo build -p iscc-uniffi` exit 0; `git status --porcelain -- crates/` empty
- [x] `swift build --scratch-path /tmp/swiftbuild-clean …` exit 0, `grep -ci warning` on the log → 0
- [x] `swift test --scratch-path /tmp/swiftbuild-rev …` → `Executed 12 tests, with 0 failures` (9
    conformance + 3 boundary), exit 0 — run from a **fresh** scratch path, not the advance agent's
- [x] `swift package dump-package` at repo root → exit 0
- [x] `git status --porcelain` clean (only `iterations.jsonl`); `grep -q '^\.build/$' .gitignore`
    OK; no `.build` dir exists anywhere and no tracked path contains `.build`
- [x] `grep -c unicodeScalars …UnicodeBoundaryTests.swift` → 1;
    `grep -c 'XCTAssertEqual(actual, expected'` → 0
- [x] `grep -c -i swift docs/unicode.md` → 3
- [x] `uv run scripts/check_docs_nav.py` → `OK: 23 documentation pages consistent`
- [x] `uv run zensical build` → "No issues found" (+ re-ran `gen_llms_full.py`, which `zensical`
    wipes; `site/` is gitignored)
- [x] `mise run check` → 17 hooks Passed, nothing modified
- [x] `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/ Package.swift`
    → empty
- [x] Gate-circumvention scan over `@{upstream}..HEAD` → no suppressions, skips, threshold or hook
    changes; no gate-config file touched at all

**Independent mutation probes** (throwaway `/tmp/swiftprobe` copy — tracked tree never mutated):

1. Fixture expectation for `text_clean/test_0004_seq_u0378_blocks_canonical_composition` → the
    delete-filter value `U+00E9`: reds with `("[101, 769]") is not equal to ("[233]")`.
2. **Same mutated fixture, `scalars(actual), scalars(expected)` swapped for a plain
    `actual, expected`: the suite goes GREEN.** The scalar comparison is genuinely load-bearing,
    not defensive decoration — next.md's central claim holds.
3. Dropped one `text_clean` case: the metadata guard reds (`("6") is not equal to ("7")`), so a
    silently shrunken fixture cannot pass vacuously.
4. Probe 1 also settles the open **stale-green** question: SwiftPM `.copy(...)` resources *are*
    declared build inputs — the fixture edit red an **incremental** `swift test` with no clean.
    Swift does not have the Gradle hazard from iteration 154. All 12 suites are now probed for it.

**Issues found:**

- (none blocking). Two observations, neither worth an issue:
    - The advance agent deviated from next.md's "do not otherwise rewrite the file" by also deleting
        the `### Smoke tests (SmokeTests.swift)` subsection from `packages/swift/CLAUDE.md`. Correct
        call, and it flagged it: `SmokeTests.swift` has **never** existed in git history
        (`git log --all -- '**/SmokeTests.swift'` is empty), so the section documented a phantom file.
        Deleting the layout line while keeping the section would have been worse.
    - `packages/swift/CLAUDE.md`'s CI bullet still reads "Swift is not available on Linux CI runners".
        next.md ruled it stays (it is about the GitHub runner *image*), and it is now in mild tension
        with the Linux recipe two sections below. Shorthand, not an error — left alone.

**Codex review:** clean. "The Swift resource is registered correctly, remains byte-identical to the
canonical fixture, and the new scalar-exact boundary tests build and pass. No actionable correctness
issues were found." No findings to weigh.

**Next:** Criterion (b) of the Unicode issue is **11 of 11 — complete**, and with (a1)/(a2)/(c)
already done that issue now has **no CID-actionable work left** (annotated as such in issues.md; the
only remainder is Titusz updating the upstream `iscc-core#137` thread). Pick the smallest remaining
`normal` `[human]` item: **pin `rubygems/configure-rubygems-credentials@v2.1.0`** in
`.github/workflows/release.yml` line 895 with an inline `# exact tag:` comment (RULED, option (a);
one line, verify statically with `uv run scripts/check_release_workflow.py` *and*
`--check-action-inputs`, requiring zero `warning: skipped` lines). After that: make the
`specs/ci-cd.md` job table exhaustive against the 21 real `ci.yml` jobs (authorized spec edit), then
the authorized major bumps one per step (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper
8.12.1, JUnit 6.x, then the riskier `jni` 0.22 / `magnus` 0.8 API migrations).

**Notes:**

- **state.md needs two corrections.** Its "Swift is CI-proof-only / not locally verifiable" framing
    is refuted by a full local green run this session, and the boundary-vector tally should now read
    11 of 11 with criterion (b) closed. Both `cmake` (iter 160) and `swift` (161) turned out to be
    locally available after the docs said otherwise — treat any remaining "not verifiable here"
    claim as unproven rather than true.
- The Swift toolchain lives at `/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin` and survives
    across iterations in this container; nothing was committed from `/tmp`, and no toolchain was
    added to the image, `mise.toml`, the devcontainer or CI. If `/tmp` is cleared, the re-fetch
    recipe is in `packages/swift/CLAUDE.md` (784 MB, ~5 min, no `sudo`).
- Adding a 13th boundary vector now costs **12 suites** at once (8 canonical readers, 2 vendored
    copies, 1 shared generated C header consumed by both C and C++). That is by design, but it makes
    any future fixture edit a deliberate, self-contained slice — never a side effect of another
    step.
- No Rust or Python source moved, so the CRAP, iai and semver gates cannot fire and were correctly
    left untouched.
