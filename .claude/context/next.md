# Next Work Package

## Step: Propagate the Unicode boundary fixture to the napi and JNI/Java test suites (slice 3)

## Goal

Advance the `normal` issue "Declare and gate a Unicode data version (DECIDED)", remainder **(b)**,
by gating the Unicode 16.0.0 freeze rule on two more native surfaces — **Node.js (napi)** and **Java
(JNI)** — taking criterion 3 from 4 of 11 to 6 of 11. Both read the *canonical* fixture in place, so
no vendored copy is added and the iteration-152 drift-gate table stays untouched.

**Deviation from the handoff's "Next" (C FFI + JNI/Java):** the handoff's cost claim rests on two
facts that no longer hold — it calls `crates/iscc-napi/iscc-lib.linux-x64-gnu.node` a *checked-in*
stale artifact (it is untracked, `crates/iscc-napi/.gitignore:4`, and CI rebuilds it every run), and
it treats C FFI as cheap. `tests/test_iscc.c` has **no JSON parser and no text-function coverage**
at all, so C FFI needs a generated table or a hand-rolled reader. state.md's two-axis survey
(fixture plumbing × text-function coverage) puts napi first; this step follows that survey. C FFI
and C++ stay deferred (C++ additionally cannot be built here — no `cmake`).

## Scope

- **Create**:
    - `crates/iscc-napi/__tests__/unicode_boundary.test.mjs`
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/UnicodeBoundaryTest.java`
- **Modify**: `docs/unicode.md` (docs — the "Both vector families are checked into the repository…"
    paragraph, ~line 98, currently names only the Rust, Python, WASM and Ruby suites plus the
    pure-Go copy; add the Node.js and Java suites)
- **Reference**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — the canonical fixture (read it, never rewrite
        it)
    - `crates/iscc-rb/test/test_unicode_boundary.rb` — the proven loader shape (metadata guard + one
        test per case per section); `tests/test_unicode_boundary.py` for the Python variant
    - `crates/iscc-napi/__tests__/conformance.test.mjs` — existing canonical-path loader
        (`join(__dirname, '..', '..', 'iscc-lib', 'tests', 'data.json')`) and `node:test` style
    - `crates/iscc-napi/__tests__/functions.test.mjs` — existing `text_clean` / `text_collapse` calls
        (exports are **snake_case**)
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — gson +
        `Files.readString(Path.of("../../iscc-lib/tests/data.json"))` + `@TestFactory` / `DynamicTest`
        pattern to mirror
    - `crates/iscc-jni/java/pom.xml` — surefire `argLine` already sets
        `-Djava.library.path=${project.basedir}/../../../target/debug`
    - `.claude/context/issues.md` → "Declare and gate a Unicode data version (DECIDED)"
    - `tests/test_vendored_fixtures.py` — the drift gate; read it to confirm why this slice must
        **not** touch `VENDORED_COPIES`

## Not In Scope

- **Do not vendor a copy of the fixture.** Both surfaces read
    `crates/iscc-lib/tests/unicode_boundary.json` by relative path. `VENDORED_COPIES` in
    `tests/test_vendored_fixtures.py` must stay unchanged and `git ls-files | grep unicode_boundary`
    must still return exactly 2 paths (canonical + the pure-Go copy).
- No other surface this step: C FFI, C++, C#, Kotlin, Swift are later slices (the
    `packages/{dotnet,kotlin,swift}` slice is the one that adds tracked copies).
- Do **not** copy the `delete_filter_output` oracles into the binding tests — plain equality against
    `outputs.result` already reds a delete-filter regression (mutation-verified at iters 150/151),
    and copying oracles re-opens the iter-149 mislabel hazard.
- Do not start criterion 4 (the 1,112,064-scalar + sequence-class differential sweep). Separate
    step.
- No changes under `crates/*/src/`, no `.crap-baseline.json` / `.iai-baseline.json` refresh, no
    `Cargo.toml` / `pom.xml` / `package.json` dependency edits (both suites already have every
    dependency they need: `node:test` is built in, gson and JUnit 5 are already declared).
- No edits under `.claude/context/specs/` (human-owned) and do not delete the issue from issues.md —
    the review agent handles issue resolution.
- Do not add or modify a CI job: `npm test` globs `__tests__/*.test.mjs` and surefire auto-discovers
    `**/*Test.java`, so both new files ride the existing `Node.js` and `Java` jobs.

## Implementation Notes

**Both local native artifacts are stale — rebuild before trusting any result.** The sentinel
conversion landed in commit `7acf0fa` (2026-07-26); `crates/iscc-napi/iscc-lib.linux-x64-gnu.node`
dates from Jun 17 and `target/debug/libiscc_jni.so` from Jul 25, both older.

- napi: `npx napi build --platform` run from `crates/iscc-napi` (debug is enough; the workspace core
    is already compiled in `target/debug`, so this is mostly a link step). Its outputs — `*.node`,
    `index.js`, `index.d.ts` — are all gitignored, so the rebuild leaves no tree diff.
- JNI: `cargo build -p iscc-jni` (writes `target/debug/libiscc_jni.so`, which surefire's `argLine`
    already points at).
- **Discriminating freshness probe:** `text_clean("a" + U+A7F1 + "b")` must return `"ab"`; a
    pre-sentinel build returns `"aSb"`. U+0378 rows do **not** discriminate (U+0378 is `Cn` in every
    Unicode version). For napi, from `crates/iscc-napi`:
    `node -e "const m=require('./index.js'); console.log(m.text_clean('a'+String.fromCodePoint(0xA7F1)+'b'))"`
    → must print `ab`.

**Fixture shape (verified at HEAD — do not re-derive).** Top-level keys are `_metadata`,
`text_clean`, `text_collapse`. `_metadata.unicode_data_version == "16.0.0"`. `text_clean` has **7**
cases, `text_collapse` has **5**. Each case is
`{"inputs": ["<string>"], "outputs": {"result": "<string>"}}` — one string input, one string output.
The file is pure ASCII with `\uXXXX` escapes (astral code points as surrogate pairs); both
`JSON.parse` and gson decode them correctly, and both languages use UTF-16 strings, so a plain
equality assertion is exact.

**napi test file** (`unicode_boundary.test.mjs`):

- Import `{ text_clean, text_collapse }` from `'../index.js'` (snake_case js_names) and
    `readFileSync` / `join` / `fileURLToPath` exactly as `conformance.test.mjs` does; path is
    `join(__dirname, '..', '..', 'iscc-lib', 'tests', 'unicode_boundary.json')`.
- One `it` for the metadata guard (version `16.0.0`, 7 and 5 case counts) so a truncated or renamed
    fixture cannot degrade the file to a zero-iteration loop, then a `describe` per section with one
    `it` per case driven by `Object.entries(...)`, asserting `strictEqual(fn(...inputs), result)`
    and naming the case in the message.
- **Zero skips** — napi executes the Rust core, so all 12 vectors must pass.

**JNI/Java test file** (`UnicodeBoundaryTest.java`):

- Package `io.iscc.iscc_lib`, class name must end in `Test` for surefire discovery.
- Load once in a `@BeforeAll` with
    `JsonParser.parseString(Files.readString(Path.of("../../iscc-lib/tests/unicode_boundary.json"))).getAsJsonObject()`
    — the maven basedir is `crates/iscc-jni/java`, the same relative shape `IsccLibTest` already
    uses for `data.json`.
- One `@Test` metadata guard (same three assertions) plus two `@TestFactory` methods returning
    `DynamicTest` streams (one per section), mirroring `IsccLibTest`'s conformance factory. Java
    method names are camelCase: `IsccLib.textClean(String)` / `IsccLib.textCollapse(String)`.
- **Zero skips** — the JNI binding wraps the same Rust core.

**Environment facts measured while scoping (do not re-probe):** `mvn` resolves fully offline here —
`mvn -o -B test-compile` in `crates/iscc-jni/java` is `BUILD SUCCESS` in 2.4 s against the cached
`~/.m2` (54 MB); use `-o` if online resolution stalls. `crates/iscc-jni/java/target/` is gitignored
(`.gitignore:79`). Node is v22.23.1 and `@napi-rs/cli` v3 is installed in
`crates/iscc-napi/node_modules`.

**Docs edit:** in `docs/unicode.md`, the sentence "…exercised by the Rust test suite, by the Python,
WASM, and Ruby binding suites (which all read the canonical fixture directly), and by the pure-Go
package via the vendored copy…" needs Node.js and Java added to the canonical-fixture list. Keep the
pure-Go clause intact. Run `mise run format` before committing so mdformat reflows the paragraph.

## Verification

- `cd crates/iscc-napi && npx napi build --platform` exits 0, and
    `node -e "const m=require('./index.js'); console.log(m.text_clean('a'+String.fromCodePoint(0xA7F1)+'b'))"`
    prints `ab` (proves the rebuilt addon carries the sentinel rule)
- `cd crates/iscc-napi && node --test __tests__/unicode_boundary.test.mjs` exits 0 with **0 fail, 0
    skipped** and **at least 13** passing tests (1 metadata guard + 7 + 5)
- `cd crates/iscc-napi && npm test` exits 0 (whole napi suite, new file included by the glob)
- `cargo build -p iscc-jni` exits 0, then `mvn -o -B test -f crates/iscc-jni/java/pom.xml` exits 0
    and its surefire summary reports **0 failures, 0 errors, 0 skipped**, with
    `crates/iscc-jni/java/target/surefire-reports/*UnicodeBoundaryTest.txt` showing **at least 13**
    tests run
- `git ls-files -- '*unicode_boundary.json'` returns exactly **2** paths (canonical +
    `packages/go/testdata/`), and `git diff --stat -- tests/test_vendored_fixtures.py` is empty — no
    vendored copy was added
- `uv run pytest tests/test_vendored_fixtures.py -q` — 8 passed (drift gate still green)
- `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json` is empty (no
    core source or baseline moved, so the CRAP and iai gates cannot react)
- `git status --porcelain` shows no untracked binary artifacts staged for commit (the `.node`,
    `index.js`, `index.d.ts` and `target/` outputs are all gitignored)
- `uv run scripts/check_docs_nav.py` exits 0 (23 pages, no new page added) and
    `grep -c 'Java' docs/unicode.md` is ≥ 1 in the "checked into the repository" paragraph — i.e.
    that paragraph names the Node.js and Java suites
- `mise run check` exits 0 (all prek hooks, including mdformat on `docs/unicode.md`)
- `cargo test -p iscc-lib` still passes (unchanged core, sanity only)

## Done When

The napi and JNI/Java suites both run all 12 canonical Unicode boundary vectors with zero skips and
zero failures against freshly rebuilt native artifacts, `docs/unicode.md` names both new suites, and
every verification command above passes with no vendored fixture copy added.
