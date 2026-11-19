## 2026-11-19 — data-edge.json fan-out, group 1 finish — WASM, Java, .NET, Ruby harnesses

**Done:** Added the `data-edge.json` harness to WASM, Java, .NET and Ruby, one commit per surface
(`77f84c9`, `00f4778`, `e5910a1` + follow-up `9d294a3`, `60993af`). Each copies the reviewed
Rust/Node shape: a section → caller table, `checkDoc` returning failures, the real-file test, an
unknown-section test, plus expected-error-that-succeeds, output-mismatch and `except` tests. The
committed Go and Node harnesses were verified as they stand, not rewritten. No surface was held back
under the stop rule.

**Files changed:**

- `crates/iscc-wasm/tests/data_edge.rs`: new; `include_str!` of the canonical file, rejection =
    `Err`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`: new; canonical file,
    rejection = `IllegalArgumentException`, any other throwable reported as a failure
- `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`: new; vendored `testdata/data-edge.json`,
    rejection = `IsccException`, any other exception a failure
- `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`: `Content` entry for
    `testdata\data-edge.json`
- `crates/iscc-rb/test/test_data_edge.rb`: new; canonical file (explicit UTF-8), rejection =
    `RuntimeError` (`rescue RuntimeError` on the raw call only; other errors reported as failures)
- `crates/{iscc-wasm,iscc-jni,iscc-rb}/CLAUDE.md`, `packages/dotnet/CLAUDE.md`: test-file lists

**Verification:**

- Go: `CGO_ENABLED=0 go test -count=1 -run DataEdge -v ./...` → 5 named `TestDataEdge*` PASS; full
    `go test ./...` ok; `go vet ./...` clean;
    `grep -l data-edge packages/go/*.go | grep -v _test.go` printed nothing
- Node: `npm run build:debug && npm test` exit 0 → `ok 12 - data-edge.json` with 5 subtests (passes
    every vector / unknown section / expected error / output mismatch / honours except); 179 pass, 0
    fail
- WASM: `wasm-pack test --node crates/iscc-wasm --features conformance` → `tests/data_edge.rs` 5
    passed (conformance 11, unit 94, all ok). `cargo clippy -p iscc-wasm --tests -- -D warnings`
    clean, `cargo fmt` applied
- Java: `cargo build -p iscc-jni && mvn clean test -f crates/iscc-jni/java/pom.xml` exit 0 →
    surefire `DataEdgeTest` Tests run: 5, Failures: 0, Errors: 0 (5 named testcases); `IsccLibTest`
    103
- .NET: `cargo build -p iscc-ffi && dotnet test … -e LD_LIBRARY_PATH=$PWD/target/debug` → 5 named
    `DataEdgeTests.*` Passed, 130 total, 0 failed (re-run after the follow-up edit)
- Ruby: `bundle exec rake compile && bundle exec rake test` → `TestDataEdge` 5 runs, 0 failures;
    suite 159 runs, 0 failures. `bundle exec standardrb` exit 0
- `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py` → 84 passed
- `mise run check` exit 0 on the final tree (an earlier run flagged "files were modified" only
    because handoff/memory were being edited concurrently; a mdformat reflow of them followed)

**Next:** Fan-out group 2 (Swift, Kotlin — UniFFI) and then group 3 (C, C++ via a generated table +
staleness check, generator recoverable from commit `ea69169`). Each needs a vendored copy registered
in `VENDORED_COPIES` in the same step as its harness. Fallback: the `normal` Python `encode_seq`
docstring issue.

**Notes:**

- .NET `GenMixedCodeV0([])` and `GenIsccCodeV0([])` raise `IsccException`, so they pass; the
    gen_text NUL vector passes through `except.dotnet = error`. JNI and Ruby keep the NUL and match
    the stated `outputs` (`ISCC:EAASKDNZNYGUUF5A`, Ruby also `characters: 10`)
- Gen `outputs` comparison: WASM, JNI and .NET results expose only `iscc` (bare string or one-field
    record), so only `iscc` is compared there; Ruby's Hash results also expose `characters`/`parts`,
    which are compared. An object result exposing none of the expected keys is a failure, never a
    pass
- .NET follow-up commit: `gen_text_code_v0` is a rejection-only caller on .NET (its only vector is
    `except.dotnet = error`), so the expected-error-that-succeeds test now also feeds it a valid
    input and asserts 3 failures — proving the caller reaches the binding
- Ruby: `RuntimeError` includes `FrozenError`; acceptable since it is the surface's error type
- No `except` entry, binding change or `data-edge.json` edit was made. No vector counts hardcoded
- Nothing touches `crates/iscc-lib/src` or `crates/iscc-lib/tests/*.rs` (CRAP/iai unaffected)
