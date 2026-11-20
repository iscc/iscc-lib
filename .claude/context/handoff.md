## 2026-11-20 — Static uncompared-key sets in the .NET, Java, napi and WASM data-edge harnesses

**Done:** Each of the four harnesses now holds a constant section-keyed map of uncompared keys
(`gen_text_code_v0` → `characters`, `gen_mixed_code_v0` → `parts`, any other section → none), and
`compare` takes the section name. For an object it walks the EXPECTED keys, skips the static set,
and fails on a missing key (`missing field <k> in …`) or a value mismatch. The "no expected field"
branch and the runtime key intersection are gone. Each surface got three tests: a synthetic positive
(the `data.json` Hello World vector), a key outside the set (`parts` on text), and a direct
`compare` call whose result has no `iscc`.

**Files changed** (one commit per surface):

- `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs` (a4722dd): `Uncompared` dict,
    `Compare(section, …)`, section threaded through `CheckVector`/`JudgeOk`, 3 `[Fact]`s
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java` (b59ee1f): `UNCOMPARED`
    `Map<String, Set<String>>`, `compare(section, …)`, 3 `@Test`s
- `crates/iscc-napi/__tests__/data_edge.test.mjs` (eb1acc5): `UNCOMPARED` object,
    `compare(section, …)`, 3 `it(...)` cases
- `crates/iscc-wasm/tests/data_edge.rs` (749a45a): `UNCOMPARED` const table + `uncompared()` lookup,
    `compare(section, …)`, section threaded through `check_vector`/`judge`/`judge_output`, 3
    `#[wasm_bindgen_test]`s

**Verification:**

- Signatures re-checked per surface before writing the sets: .NET `TextCodeResult`/
    `MixedCodeResult`/`IsccCodeResult` are each `record(string Iscc)`. Java `genTextCodeV0`/
    `genMixedCodeV0`/`genIsccCodeV0` return `String`. napi `index.d.ts` and WASM `lib.rs` return
    `string`/`String` for all three
- `cargo build -p iscc-ffi` then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=…/target/debug`: 153 passed, 0
    failed. The `~DataEdge` filter gives 8 (5 existing + 3 new)
- `cargo build -p iscc-jni`, then `mvn -q clean test -f crates/iscc-jni/java/pom.xml`: surefire
    shows DataEdgeTest 8/0 failures and IsccLibTest 108/0
- `npm run build` then `npm test` in `crates/iscc-napi`: 182 pass, 0 fail. data_edge has 8 ok
- `wasm-pack test --node crates/iscc-wasm --features conformance`: 11 + 8 + 94 passed, 0 failed.
    `cargo clippy -p iscc-wasm --tests --target wasm32-unknown-unknown --features conformance -- -D warnings`
    is clean
- Mutation check (napi, in a throwaway copy that is now deleted): putting back the old intersect
    `compare` turns exactly tests 7 (key outside set) and 8 (no `iscc`) to `not ok`. Tests 1–6 stay
    ok. On the other three surfaces the negatives are the same fixtures and fail under the old logic
    by construction, but I did not run a mutant there
- `grep -rn "result exposes" packages/dotnet crates/iscc-jni/java/src crates/iscc-napi/__tests__ crates/iscc-wasm/tests`
    prints nothing (exit 1)
- `mise run check` exits 0

**Next:** The open `normal` issue about lenient harnesses should now be resolved. The spec's
Verified When line ("every harness fixes its uncompared keys statically") should hold across all
surfaces. Next is the docs step that next.md rejected for this iteration: the Java how-to
`genSumCodeV0` example and the Ruby precompiled-gem wording.

**Notes:**

- Small addition beyond next.md: every `compare` (except WASM) now returns
    `expected an object, got …` when the expected value is an object but the actual value is not.
    Before, .NET and Java threw on the cast. WASM's `actual.get(key)` on a non-object gives `None`,
    so it reports `missing field iscc`
- Not touched, as scoped: Go's `edgeCompare` (missing key reads as `nil`), binding source, fixtures
- Test-only change, so the CRAP, iai and semver gates are unaffected
