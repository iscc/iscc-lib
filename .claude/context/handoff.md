# Handoff

## 2026-07-27 — Propagation slice 7 — Unicode boundary vectors in the Swift test suite

**Done:** Gated `packages/swift` on all 12 Unicode 16.0.0 boundary vectors — the 11th and last
binding surface. Added `UnicodeBoundaryTests.swift` (3 test methods: metadata guard + 7 `text_clean`
\+ 5 `text_collapse` vectors) comparing **Unicode scalar arrays**, never `String ==` (Swift string
equality folds canonical equivalence, which would make three of the four sequence vectors vacuous).
Vendored a byte-identical fixture copy (made with `cp`, registered in `VENDORED_COPIES`), and ran
the whole suite locally via the swift.org Debian 12 toolchain — Swift is locally verifiable in this
container, and the docs now say so.

**Files changed:**

- `packages/swift/Tests/IsccLibTests/UnicodeBoundaryTests.swift`: new boundary suite (metadata guard
    pinning `16.0.0`/7/5; `scalars()` helper with a comment explaining why `String ==` is forbidden;
    shared `assertVectors` helper with `file:`/`line:` forwarding)
- `packages/swift/Tests/IsccLibTests/unicode_boundary.json`: vendored copy, created with `cp` only
- `packages/swift/Package.swift`: added `.copy("unicode_boundary.json")` to the test target's
    `resources:` array (only non-test/non-doc file beyond `.gitignore`)
- `.gitignore`: added `.build/` (SwiftPM default scratch dir; `build/`/`build-*/` did not match it)
- `tests/test_vendored_fixtures.py`: registered the Swift copy under the
    `crates/iscc-lib/tests/unicode_boundary.json` key
- `docs/unicode.md`: propagation paragraph now names the Swift test suite and its vendored copy
    alongside the pure-Go package
- `packages/swift/CLAUDE.md`: added `UnicodeBoundaryTests.swift`/`unicode_boundary.json`/
    `Constants.swift` to the layout, dropped the stale `SmokeTests.swift` line, added a "Boundary
    tests" section, replaced the false "Cannot run Swift tests in the Linux devcontainer" and
    "macOS-only testing" claims with the swift.org Debian 12 + `--scratch-path` recipe

**Verification:** (every next.md criterion reproduced this session)

- `cmp` canonical vs. Swift copy → identical (re-checked *after* `mise run check` hooks ran)
- `git ls-files -- '*unicode_boundary.json' | wc -l` → **3**
- `uv run pytest -q tests/test_vendored_fixtures.py` → **9 passed** (was 8); full `uv run pytest -q`
    → **400 passed** (collect-only also 400, was 399)
- `cargo build -p iscc-uniffi` exit 0; `git status --porcelain -- crates/` empty
- `swift build --scratch-path /tmp/swiftbuild …` exit 0, `grep -ci warning` on the log → 0
- `swift test --scratch-path /tmp/swiftbuild -Xlinker -L… -rpath …` →
    **`Executed 12 tests, with 0 failures`** (9 conformance + 3 boundary), exit 0, no warnings
- **Mutation probe** (in a `/tmp` copy, tracked tree never mutated): fixture expectation for
    `text_clean/test_0004_seq_u0378_blocks_canonical_composition` changed to the delete-filter value
    U+00E9 → suite reds with `("[101, 769]") is not equal to ("[233]")`, `12 tests, 1   failure`.
    The scalar comparison is load-bearing exactly as next.md predicted.
- `swift package dump-package` at repo root → exit 0 (root manifest untouched)
- `grep -q '^\.build/$' .gitignore` → OK; `git status --porcelain` shows no `.build/` anywhere
- `grep -c unicodeScalars UnicodeBoundaryTests.swift` → 1;
    `grep -c 'XCTAssertEqual(actual, expected'` → 0
- `grep -c -i swift docs/unicode.md` → 3 (≥ 2)
- `uv run scripts/check_docs_nav.py` → `OK: 23 documentation pages consistent`;
    `uv run zensical build` → "No issues found"
- `mise run check` → all 17 hooks Passed (mdformat reflowed `packages/swift/CLAUDE.md` on the first
    pass; second pass clean)
- `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/ Package.swift`
    → empty

**Next:** Criterion 3 of the Unicode issue is now **11 of 11** binding surfaces. The two remaining
cheap `normal` `[human]` items from the previous review stand: (1) pin
`rubygems/configure-rubygems-credentials@v2.1.0` in `release.yml` (one line + `# exact tag:`
comment, verify with the committed release-workflow gates), (2) make the `specs/ci-cd.md` job table
exhaustive against the 21 real `ci.yml` jobs (authorized spec edit).

**Notes:**

- **One deliberate deviation from next.md's letter:** next.md said to drop the stale
    `SmokeTests.swift` line from the *layout block* and "not otherwise rewrite the file" — but
    `packages/swift/CLAUDE.md` also carried a whole "### Smoke tests (`SmokeTests.swift`)"
    subsection under *Test Patterns* describing the same nonexistent file (the scoping run's
    "Executed 9 tests" confirms only ConformanceTests runs). I removed that subsection too: leaving
    a section documenting tests that don't exist while deleting its layout line would have made the
    doc self-contradictory, and doc updates must match the actual implementation. No other section
    was touched beyond what next.md lists.
- The gate sequencing is worth knowing: `test_no_unregistered_tracked_copy` discovers copies via
    `git ls-files`, so it *fails* until the new fixture copy is `git add`ed (index = tracked). Red
    before staging, green after — the gate works exactly as designed.
- The swift.org toolchain from the scoping run was still unpacked in `/tmp/swifttc` (no re-download
    needed). Nothing from `/tmp` is committed; no toolchain was added to mise/CI/devcontainer.
- `state.md`'s "Swift is CI-proof-only / not locally verifiable" framing (also in the previous
    handoff) is now disproven with a full local green run — update-state should drop it.
- mdformat reflowed the new CLAUDE.md bullets (line-wrap only); the committed file is the post-hook
    version.
