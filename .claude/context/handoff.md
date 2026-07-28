# Handoff

## 2026-07-28 — Review of: Restore the zero-copy JNI byte-array helper and test the seven untested natives

**Verdict:** PASS

**Summary:** The hand-rolled `build_byte_array` (one `Vec<i8>` alloc + copy per returned `byte[]`)
is gone; all five return sites now call jni 0.22.4's `Env::byte_array_from_slice`, whose body I read
at `jni-0.22.4/src/env.rs:3350` — `JByteArray::new` + `set_region` over a transmuted `&[i8]`, no
`#[deprecated]`, no allocation. Eleven new JUnit tests give the seven previously untested natives
their first Java coverage; all 33 natives declared in `IsccLib.java` now have at least one caller.
Every expected value in the new tests traces to a real sibling source, not to invention.

**Verification:**

- [x] `grep -rn "build_byte_array\|Vec<i8>" crates/iscc-jni/` → exit 1, no matches (source **and**
    `CLAUDE.md`); repo-wide the name survives only in `state.md`/`issues.md` prose
- [x] Seven-name loop over `IsccLib\.$m(` printed nothing. Stronger census: 33 natives parsed out of
    `IsccLib.java`, **0 uncovered**
- [x] `cargo build -p iscc-jni && mvn clean test -f crates/iscc-jni/java/pom.xml` → **93 tests, 0
    failures, 0 errors** (80 IsccLibTest + 13 UnicodeBoundaryTest), ≥ 89 floor met. Not a "≥ N"
    illusion: `@Test` count went 29 → 40 (+11) and IsccLibTest ran 69 → 80 (+11) — exact match, so
    no parameterized rows silently collapsed
- [x] `IsccLibTest.java` asserts `"[Ljava.lang.String;"` (line 596) and `"[[B"` (line 685) via
    `getClass().getName()`
- [x] `cargo clippy -p iscc-jni --all-targets -- -D warnings` → exit 0
- [x] `mise run check` → exit 0, 18 hooks Passed, tracked tree unchanged (only the runner's own
    `decisions*.md` / `iterations.jsonl` rotation was dirty, as it was before the step)

**Extra probes (2 of 3 budget):**

- `mvn test -DargLine="-Xcheck:jni -Djava.library.path=$PWD/target/debug"` → 93/93, **zero**
    `WARNING: JNI` lines. Surefire's `argLine` is *overridden*, not merged, so the library path must
    be re-supplied. This is the check next.md did not ask for and the one that matters most for a
    JNI marshalling change.
- Expected-value provenance: `algSimhash` 0xF0/0x0F → all-0xFF, empty → 32 zero bytes, single →
    identity all match `test_algorithm_primitives.rs:95-118`; `algMinhash256` empty → 32 × 0xFF
    matches `:141`; `softHashVideoV0` 380-zero frame == `genVideoCodeV0` body matches `:270`;
    `"Hello"` → `"SGVsbG8"` and `"Hello\nWorld"` → `"Hello World"` match `SmokeTests.cs:220/62`.
    Nothing was invented.

**Issues found:**

- (nothing blocking) The advance agent also edited `.claude/agent-memory/advance/deps-refresh.md` —
    outside next.md's Scope list, but it is that role's own memory correcting the false "removed in
    0.22" note it wrote at 168. Correct behaviour, not scope creep.
- Documented deviation accepted: next.md said to hang the `"[[B"` assertion off "the existing chunk
    test", but the only existing one was `algCdcChunksNegativeAvgChunkSize` (assertThrows-only).
    Adding a focused positive test instead is the better call and creates no duplicate coverage.
- Scope: 1 non-test, non-doc file (`crates/iscc-jni/src/lib.rs`) — well inside the 3-file cap. No
    `Not In Scope` item was touched (`specs/java-bindings.md`, `crates/iscc-rb/CLAUDE.md`,
    `issues.md`, the helper reshaping and the pre-existing deref/`&Env` asymmetries all left alone).
- Gate integrity: `git diff @{upstream}..HEAD` over all 4 unpushed commits shows no suppression,
    skip, threshold or hook weakening. No Tier 1/2 API change, no benchmarked path touched.

**Issues resolved (deleted from issues.md):** "JNI `build_byte_array` re-implements a non-deprecated
upstream helper" and "Seven of the 33 JNI natives have no Java test". Also trimmed the doc-drift
entry's stale line count (`src/lib.rs` is 1152 lines now, not 1168).

**Codex review:** Clean — "The JNI helper replacement preserves behavior while removing the
intermediate allocation, and the added tests compile and pass. No actionable regressions were
found."

**Next:** Two CID-doable `normal` items remain. Recommended: **the `release.yml` action-freshness
pass**, the last schedulable slice of the v0.6.0 dependency issue (uniffi 0.32 and criterion 0.8 are
human-gated). It is static-evidence-only — lean on `uv run scripts/check_release_workflow.py` and
`--check-action-inputs` (zero `warning: skipped` lines is part of a pass) plus
`gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing any `@vN`; no CI run ever
exercises that file. Cheaper alternative: the Ruby half of the doc-drift issue
(`crates/iscc-rb/CLAUDE.md:108` teaches `RString::from_slice`; the code has used
`ruby.str_from_slice` since magnus 0.8) — a one-line doc fix needing no authorization that could
ride along as a doc file without consuming scope budget. The Java-spec half stays human-gated.

**Notes:**

- `crates/iscc-jni/src/lib.rs` is 1152 lines — still under the "split at ~1500" threshold its
    `CLAUDE.md` sets, but the margin is shrinking; worth watching, not acting on.
- The JNI `.so` is gitignored and goes stale silently. Any future local JNI verification must
    `cargo build -p iscc-jni` **before** `mvn clean test`; plain `mvn test` also reuses stale test
    classes.
- `learnings.md` was at exactly 200 lines; two entries were compressed to make room, now 199.
    `decisions.md` sits at 397/400 after the runner's rotation — no decision entry was warranted
    here (a routine pass restoring an upstream API needs no rationale record).
