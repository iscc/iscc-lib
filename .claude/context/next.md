# Next Work Package

## Step: Static uncompared-key sets in the .NET, Java, napi and WASM data-edge harnesses

## Goal

Close the `normal` `[human]` issue "Four `data-edge.json` harnesses ignore expected output keys
their result lacks": each of the four harnesses compares every expected output key except a static
per-function set taken from its result type, in place of the runtime key intersection.

## Alternatives Considered

- **Chosen:** the four-harness fan-out. The behaviour is settled (`edge-vectors.md` → "Output
    subset", decisions.md 2026-11-20 "data-edge output subset"), the file side already enforces it
    (iteration 236), and it fully resolves a release-blocking issue
- **Rejected:** the docs step (Java how-to `genSumCodeV0` example + Ruby precompiled-gem wording).
    Cheap and also release-blocking, but it finishes no in-flight feature; it comes next

## Scope

**Fan-out:** "compare every expected key except a static per-function uncompared set; a missing key
fails", applied to 4 surfaces: .NET, Java, napi, WASM. One commit per surface. Test-only changes.

- **Modify** (tests only):
    - `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs` (`Compare`)
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java` (`compare`)
    - `crates/iscc-napi/__tests__/data_edge.test.mjs` (`compare`)
    - `crates/iscc-wasm/tests/data_edge.rs` (`compare`)
- **Reference**: `.claude/context/specs/edge-vectors.md` ("Rules" → "Output subset", Verified When),
    `crates/iscc-lib/tests/data.json`, each surface's gen signatures
    (`packages/dotnet/Iscc.Lib/Results.cs`,
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`,
    `crates/iscc-napi/index.d.ts`, `crates/iscc-wasm/src/lib.rs`), and the already-strict Ruby
    harness `crates/iscc-rb/test/test_data_edge.rb` for the pattern

## Not In Scope

- Binding source, `data-edge.json` (no new vector, no edited `except`), the Python validator, the
    C/C++ generator and the vendored copies
- Go's `edgeCompare` reading a missing key as `nil`: another surface, harmless while no `data.json`
    output is `null`; leave it
- Making the bindings return structured results (`characters`, `parts`): an API change, not a test
- The other `normal` issues (Java how-to, Ruby gems, derived-fact drift)

## Implementation Notes

- Measured: on all four surfaces `gen_text_code_v0`, `gen_mixed_code_v0` and `gen_iscc_code_v0`
    return a bare ISCC string (.NET wraps it in a one-field record holding `Iscc`). So the static
    uncompared sets are: `gen_text_code_v0` → `characters`, `gen_mixed_code_v0` → `parts`,
    `gen_iscc_code_v0` → none. Re-check each surface's signature before writing its set; do not copy
    the set across surfaces blindly.
- Write the set once per harness as a constant map keyed by section name (absent section = empty
    set) and pass the section name into `compare`. For an object: for every expected key not in that
    section's set, the actual must have the key and an equal value; a missing key fails with a
    message naming it. Drop the "no expected field" branch. Never read the returned keys to decide
    what to compare. Non-object outputs keep whole-value equality.
- Replace the "only the expected keys the result exposes are compared" doc comment at each site
    (these four are the only ones in the tree) with wording that states the static rule.
- Negative tests per surface, one fault per call (learnings: "One bad case per call"):
    1. **Key outside the set:** a doc run with one `gen_text_code_v0` vector, inputs
        `["Hello World", 64]`, outputs `{"iscc": "ISCC:EAASKDNZNYGUUF5A", "parts": []}` → exactly one
        failure naming `parts`. The `iscc` is correct (from `data.json`), so `parts` is the only
        fault, and `parts` is uncompared for mixed, so a single global set would not pass this. On
        .NET this vector carries no `except`, so it reaches the compare.
    2. **Result lost `iscc`:** call `compare` directly (the bindings cannot return a result without
        it) with section `gen_text_code_v0`, expected `{iscc, characters}` and actual
        `{"characters": 10}` → a failure naming `iscc`. Do not use `{}` as the actual: the old
        intersect code already rejected that, so it proves nothing.
- **.NET positive:** the only object-valued vector in the real file carries `except.dotnet`, so .NET
    never runs an object compare on real data. Add a synthetic positive: a doc run with the
    `data.json` `Hello World` vector (`{"iscc": "ISCC:EAASKDNZNYGUUF5A", "characters": 10}`) passes
    with zero failures. Adding the same positive on the other three surfaces is fine.
- Keep `.NET` failures returned as strings like the existing tests; xunit `[Fact]`, JUnit `@Test`,
    `node:test` `test(...)`, `#[wasm_bindgen_test]`, matching each file's existing style.
- Build prerequisites: .NET needs `cargo build -p iscc-ffi` first; napi needs a rebuild
    (`npm run build` in `crates/iscc-napi`) before `npm test` (stale `.node` artifacts); WASM tests
    run only under `wasm-pack test --node`, never `cargo test`.
- Gates: test-only binding work, so the CRAP, iai and semver gates are unaffected.

## Verification

- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` exits 0
    (after `cargo build -p iscc-ffi`), including the two negatives and the synthetic positive
- `mvn clean test -f crates/iscc-jni/java/pom.xml` exits 0, including the two negatives
- `npm test` in `crates/iscc-napi` (after a rebuild) exits 0, including the two negatives
- `wasm-pack test --node crates/iscc-wasm --features conformance` exits 0, including the two
    negatives
- `grep -rn "result exposes" packages/dotnet crates/iscc-jni/java/src crates/iscc-napi/__tests__ crates/iscc-wasm/tests`
    prints nothing, and none of the four `compare` functions reads the actual object's key set to
    choose which keys to compare (reviewer reads the four functions)
- `mise run check` exits 0

## Done When

All four harnesses compare every expected key except their static per-function set, each carries the
two negative tests, and every verification command above exits 0.
