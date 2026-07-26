# Handoff

## 2026-07-26 — Review of: Propagate the Unicode boundary fixture to the napi and JNI/Java test suites (slice 3)

**Verdict:** PASS

**Summary:** Two new test files (napi `unicode_boundary.test.mjs`, JNI `UnicodeBoundaryTest.java`)
run all 12 canonical Unicode 16.0.0 boundary vectors plus a metadata guard against freshly built
native artifacts — 13 tests each, zero skips, zero failures. Both read the canonical fixture in
place; no vendored copy was added and the iteration-152 drift gate is untouched. Scope is exactly
what next.md asked for: 2 test files + 1 documentation paragraph, zero non-test/non-doc source
files.

**Verification:** (every command re-run independently this review)

- [x] `npx napi build --platform` + freshness probe — the addon in the tree is dated Jul 26 21:09
    and `text_clean('a'+U+A7F1+'b')` prints `ab` (sentinel rule present; a pre-sentinel build prints
    `aSb`)
- [x] `node --test crates/iscc-napi/__tests__/unicode_boundary.test.mjs` — **13 pass, 0 fail, 0
    skipped** (1 metadata guard + 7 `text_clean` + 5 `text_collapse`)
- [x] `npm --prefix crates/iscc-napi test` — **152 pass, 0 fail, 0 skipped** (35 suites)
- [x] `cargo build -p iscc-jni` exit 0 in 1.11 s with no recompilation (so `libiscc_jni.so` is
    current w.r.t. HEAD sources), then `mvn -o -B test -f crates/iscc-jni/java/pom.xml` — BUILD
    SUCCESS, **82 run, 0 failures, 0 errors, 0 skipped**; `UnicodeBoundaryTest` reports **13 run**
- [x] `git ls-files -- '*unicode_boundary.json'` — exactly **2** paths (canonical +
    `packages/go/testdata/`); `git diff HEAD~1..HEAD -- tests/test_vendored_fixtures.py` empty
- [x] `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed** (drift gate green)
- [x] `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json` — empty; the
    CRAP, iai and semver gates cannot react to this diff
- [x] `git status --porcelain` — only the runner's unstaged `iterations.jsonl`; no binary artifact
    staged (`.node`, `index.js`, `index.d.ts`, `java/target/` all gitignored)
- [x] `uv run scripts/check_docs_nav.py` — `OK: 23 documentation pages consistent`; the edited
    `docs/unicode.md` paragraph now reads "Python, Node.js, WASM, Java, and Ruby binding suites
    (which all read the canonical fixture directly)" with the pure-Go clause intact
- [x] `mise run check` — every hook Passed, tree clean afterwards
- [x] `cargo test -p iscc-lib` — 281 + 28 + 22 + 4 + 1 passed, 0 failed;
    `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] Gate-integrity scan over the full unpushed range `@{upstream}..HEAD` (4 commits) — no
    suppression, skip marker, threshold change, `@Disabled`/`assumeTrue`/`it.skip`, or hook
    weakening

**Independent probes run by review (beyond next.md's criteria):**

- **Mutation A — delete-filter regression.** Set
    `text_clean/test_0006_seq_ua7f1_no_decomposition_leak` expected to the delete-filter value
    `U+00E9`: napi reds (12 pass / 1 fail in the text_clean boundary suite) and Java reds naming the
    exact case (`textCleanBoundary()[7] … expected: <é> but was: <é>`). Plain equality against
    `outputs.result` is sufficient — no oracle column needed, confirming next.md's Not-In-Scope
    call.
- **Mutation B — dropped case.** Deleting `text_collapse/test_0004_seq_u0378_preserves_final_sigma`
    reds the metadata guard in both suites (`expected: <5> but was: <4>`), so neither file can
    silently degrade to a shorter loop.
- **CI reachability.** `ci.yml` job `node` runs `npm install` → `npx napi build --platform` →
    `npm test` (script is `node --test __tests__/*.test.mjs`, so the new file is globbed); job
    `java` runs `cargo build -p iscc-jni` → `mvn test -f crates/iscc-jni/java/pom.xml` (surefire
    auto-discovers `*Test.java`, and its working directory is the pom basedir so the `../../`
    fixture path resolves). Both jobs rebuild the native first — the local stale-artifact hazard
    cannot reach CI. No CI edit was needed, as next.md predicted.
- **Mechanism, not coincidence.** Unlike `packages/go` (which passes two vectors for the wrong
    reason), both surfaces execute the Rust core's sentinel map: the `U+A7F1` sequence vector is the
    discriminator and it passes on a build cargo considers current.

**Issues found:** (none)

**Codex review:** No findings. Verdict: "The new Node.js and Java suites correctly load the
canonical fixture, exercise all 12 vectors, and are discovered by existing test commands. Both full
binding test suites pass without failures or skips." Its claims about suite results match my own
independent runs (152 napi / 82 mvn), so this is a corroborated clean report rather than an
unverified assertion.

**Next:** Criterion 3 is now **6 of 11** native surfaces (Rust, Python, WASM, Ruby, napi, Java) plus
the pure-Go port. Two candidates, in cost order:

1. **`packages/{dotnet,kotlin,swift}` slice** — the first one that adds *tracked* vendored copies,
    so each new copy must be registered in `VENDORED_COPIES` of `tests/test_vendored_fixtures.py`
    in the same step (the gate redding on an unregistered copy is it working, not a bug), and
    copies must keep the canonical basename. Kotlin/Swift go through UniFFI; check
    `packages/kotlin`'s Gradle flakiness on this bind mount before committing to it in one step.
2. **Criterion 4 — the differential sweep wired in as a runnable check** (1,112,064 scalars *and*
    sequence classes). This is the only remaining piece with no partial credit and it protects
    every future table bump; a case can be made for doing it before the last binding surfaces.

C FFI stays expensive (`tests/test_iscc.c` has no JSON reader and no text-function coverage — needs
a generated vector table or a hand-rolled reader) and C++ is not buildable in this container (no
`cmake`).

**Notes:**

- `git ls-files | grep unicode_boundary` **no longer tracks the surface tally** — it now returns 8
    paths, not 9, because the Java test file is `UnicodeBoundaryTest.java` (CamelCase). update-state
    used that grep as an anchor at 152; use a case-insensitive or per-surface check instead.
- `crates/iscc-jni/java/pom.xml` does not set `project.build.sourceEncoding`, so javac uses the
    platform charset. The new file's only non-ASCII is two decorative `──` comment separators, and
    `IsccLibTest.java` / `IsccLib.java` already carry more non-ASCII, so this is pre-existing and CI
    is green — not filed as an issue, but worth knowing before adding non-ASCII *string literals* to
    a Java source here.
- `mvn -o -B test -f crates/iscc-jni/java/pom.xml` resolves fully offline from the cached `~/.m2` in
    ~4 s; use `-o` if online resolution stalls.
- No API, hot path, baseline, dependency manifest or CI file was touched, so the semver, iai, CRAP
    and audit gates are all inert for this diff.
