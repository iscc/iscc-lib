# Handoff

## 2026-07-28 — Restore the zero-copy JNI byte-array helper and test the seven untested natives

**Done:** Deleted the hand-rolled `build_byte_array` (extra `Vec<i8>` alloc + copy per returned
`byte[]`) and switched all five return sites to jni 0.22.4's un-deprecated
`Env::byte_array_from_slice` (`isccDecode`, `algSimhash`, `algMinhash256`, `algCdcChunks` per-chunk
frame, `softHashVideoV0`). Added 11 JUnit tests giving the seven never-tested natives their first
Java coverage, including the two required runtime array-class assertions.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: removed `build_byte_array` (15 lines); five call sites now
    `env.byte_array_from_slice(...)` with each site's existing `throw_and_default` error shape
    unchanged.
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: 11 new `@Test` methods —
    `conformanceSelftestReturnsTrue` (asserts `true` from the crate's only `jboolean` return),
    `encodeBase64KnownValue` ("Hello" → "SGVsbG8", from SmokeTests.cs),
    `textRemoveNewlinesCollapsesToSingleLine` ("Hello\\nWorld" → "Hello World", from SmokeTests.cs),
    `isccDecomposeReturnsStringArray` (2 units + `"[Ljava.lang.String;"` class assert),
    `algSimhashKnownValue` (complementary F0/0F digests → all 0xFF, from
    test_algorithm_primitives.rs), `algSimhashEmptyInput` (32 zero bytes), `algSimhashSingleDigest`
    (identity), `algMinhash256EmptyFeatures` (32 × 0xFF, from test_algo.py/Rust core),
    `algMinhash256Deterministic`, `algCdcChunksSplitsAndReassembles` (`"[[B"` class assert +
    reassembly of an 8 KiB cycling pattern), `softHashVideoV0MatchesGenVideoCodeBody` (8-byte digest
    == `isccDecode(genVideoCodeV0(...))` body, from the Rust consistency test).
- `crates/iscc-jni/CLAUDE.md`: type-mapping `byte[]` row and the byte-return pitfall bullet now
    teach `env.byte_array_from_slice()` (mdformat realigned the table columns).
- `.claude/agent-memory/advance/deps-refresh.md`: corrected the iter-168 note that claimed
    `byte_array_from_slice` was removed.

**Verification:** (all six next.md checks pass, this session)

- `grep -rn "build_byte_array\|Vec<i8>" crates/iscc-jni/` → exit 1 (nothing, source and CLAUDE.md).
- Seven-name loop over `IsccLib\.$m(` in the Java test sources printed nothing — all seven called.
- `cargo build -p iscc-jni` + `mvn clean test -f crates/iscc-jni/java/pom.xml` → **93 tests (80
    IsccLibTest + 13 UnicodeBoundaryTest), 0 failures, 0 errors** — ≥ 89 floor met (82 + 11).
- `IsccLibTest.java` asserts both `"[Ljava.lang.String;"` and `"[[B"` via `getClass().getName()`.
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` → exit 0.
- `mise run check` → exit 0 (second run all-Passed, no hook modifications); tracked tree carries
    only the intended diff plus the runner-touched context files that were dirty before this step
    (`decisions*.md`, `iterations.jsonl` — not staged).

**Next:** Per the previous review's ranking, the remaining candidates are the doc-drift entry
(`crates/iscc-rb/CLAUDE.md:108` magnus + human-gated `specs/java-bindings.md` half) and the
`release.yml` action-freshness pass. All 33 JNI natives now have at least one JUnit test.

**Notes:**

- Deviation from next.md: it said to put the `"[[B"` assertion by "extending the existing chunk
    test", but the only existing `algCdcChunks` test was the negative
    `algCdcChunksNegativeAvgChunkSize` (assertThrows only, nothing to extend). I added one focused
    positive test `algCdcChunksSplitsAndReassembles` carrying the class assertion instead of mixing
    a positive path into the negative test. No duplicate CDC coverage was created.
- `Env::byte_array_from_slice` confirmed at jni-0.22.4 `src/env.rs:3349` with no `#[deprecated]`
    attribute (read this session); it does `JByteArray::new` + `set_region` over a transmuted
    `&[i8]` — the exact body the deleted helper duplicated with an extra allocation.
- Hot-path note: this change only removes an alloc+copy in the JNI (binding) layer; the benchmarked
    core paths in `iscc-lib` are untouched, so no bench run was warranted.
- Test-count arithmetic: next.md predicted "at least 7 new"; the seven functions needed 11 focused
    tests to cover the derived expected values (three algSimhash cases, two algMinhash256 cases, one
    CDC positive) — hence 93, not 89.
