# Handoff

## 2026-07-26 — Propagate the Unicode boundary fixture to the napi and JNI/Java test suites (slice 3)

**Done:** Added Unicode 16.0.0 boundary conformance tests to the Node.js (napi) and Java (JNI)
suites. Both read the canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` in place by
relative path — no vendored copy was added, and `VENDORED_COPIES` in
`tests/test_vendored_fixtures.py` is untouched. Both local native artifacts were rebuilt first and
freshness-probed before trusting any result.

**Files changed:**

- `crates/iscc-napi/__tests__/unicode_boundary.test.mjs`: new — metadata guard (version `16.0.0`, 7
    \+ 5 case counts) plus one `it` per fixture case across two `describe` sections, driven by
    `Object.entries`, asserting `strictEqual(text_clean/text_collapse(...inputs), outputs.result)`.
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/UnicodeBoundaryTest.java`: new — gson
    `@BeforeAll` loader, one `@Test` metadata guard, two `@TestFactory` `DynamicTest` sections
    calling `IsccLib.textClean` / `IsccLib.textCollapse`, mirroring `IsccLibTest`'s pattern.
- `docs/unicode.md`: the "Both vector families are checked into the repository…" paragraph now names
    the Node.js and Java suites in the canonical-fixture list (pure-Go clause intact).

**Verification:** (all commands run this session, outputs observed)

- `npx napi build --platform` (from `crates/iscc-napi`) exit 0; freshness probe
    `text_clean('a'+U+A7F1+'b')` printed `ab` (sentinel rule present in the rebuilt addon)
- `node --test __tests__/unicode_boundary.test.mjs` — **13 pass, 0 fail, 0 skipped** (≥ 13 required)
- `npm test` (full napi suite) — **152 pass, 0 fail, 0 skipped**
- `cargo build -p iscc-jni` exit 0 (`target/debug/libiscc_jni.so` rebuilt, Jul 26 21:09);
    `mvn -o -B test -f crates/iscc-jni/java/pom.xml` — BUILD SUCCESS, **82 run, 0 failures, 0
    errors, 0 skipped**; surefire report `io.iscc.iscc_lib.UnicodeBoundaryTest.txt` shows **Tests
    run: 13**
- `git ls-files -- '*unicode_boundary.json'` — exactly 2 paths (canonical +
    `packages/go/testdata/`); `git diff --stat -- tests/test_vendored_fixtures.py` — empty
- `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed** (drift gate green)
- `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json` — empty; full
    `git status --porcelain` shows only the two new text test files + `docs/unicode.md` (napi
    `.node`/`index.js`/`index.d.ts` and `java/target/` are gitignored — no binary artifacts to
    stage)
- `uv run scripts/check_docs_nav.py` — OK, 23 pages consistent; the edited paragraph
    (docs/unicode.md:100) names Node.js and Java
- `mise run check` — all hooks Passed (mdformat kept the docs paragraph as written)
- `cargo test -p iscc-lib` — 281 + 28 + 22 + 4 + 1 passed, 0 failed (unchanged core, sanity)

**Next:** Criterion 3 is now 6 of 11 surfaces. Remaining slices per state.md's cost survey: the
`packages/{dotnet,kotlin,swift}` slice (adds tracked vendored copies — registering each in
`VENDORED_COPIES` of `tests/test_vendored_fixtures.py` is a required part of that scope; the gate
reddening on an unregistered copy is it working), then C FFI (needs a JSON reader or a generated
vector table — `tests/test_iscc.c` has no text-function coverage today), C++ deferred (no `cmake` in
this container). Criterion 4 (full-code-space differential sweep) is still a separate step.

**Notes:**

- Followed next.md's stated deviation from the previous handoff's "Next" (napi + JNI instead of C
    FFI + JNI); no further deviation was needed.
- The napi rebuild left zero tree diff as predicted (`*.node`, `index.js`, `index.d.ts` all
    gitignored). The freshness probe matters: the pre-rebuild addon dated from Jun 17 and would have
    returned `aSb` for the U+A7F1 rows while passing the U+0378 rows.
- `mvn -o` (offline) resolved everything from the cached `~/.m2` in ~5 s, as next.md's environment
    facts predicted.
- The runner's `.claude/context/iterations.jsonl` modification is left unstaged as usual.
- No API, hot path, baseline, dependency manifest, or CI file was touched — CRAP, iai, semver and
    audit gates cannot react to this diff.
