# Next Work Package

## Step: data-edge.json fan-out, group 1 finish — WASM, Java, .NET, Ruby harnesses

## Goal

Close fan-out group 1 of the v0.8.0 "Shared edge-case vectors run everywhere" criterion. Add the
`data-edge.json` harness to WASM, Java, .NET and Ruby. The step also covers the Go and Node
harnesses: they are committed (`a84b665`, `be0c4d3`), but a devcontainer crash cut the previous
advance session short before review. Verify those two as they stand; do not rewrite them.

## Alternatives Considered

- **Chosen:** the four remaining group-1 surfaces. The behaviour is fixed by `specs/edge-vectors.md`
    → Rules, every toolchain is on `$PATH`, and the crashed session spent about 10 min on Go and 5
    min on Node, so four surfaces fit one session
- **Rejected:** the `normal` Python `encode_seq` docstring issue (one file). It is cheap, but it
    would leave group 1 half done, and two harnesses would wait unreviewed for another iteration

## Scope

**Fan-out:** `edge-vectors.md` → Rules (every vector runs, `except` is the only deviation, an
unknown section fails, the rejection is asserted on the raw call only), applied to WASM, Java, .NET
and Ruby. Commit once per surface.

- **Create**: `crates/iscc-wasm/tests/data_edge.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`,
    `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`, `crates/iscc-rb/test/test_data_edge.rb`
- **Modify**: `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` (a `Content` entry for
    `testdata\data-edge.json`, same shape as the `data.json` one), and the test-file lists in
    `crates/iscc-wasm/CLAUDE.md`, `packages/dotnet/CLAUDE.md`, `crates/iscc-jni/CLAUDE.md` and
    `crates/iscc-rb/CLAUDE.md`
- **Reference**: `crates/iscc-napi/__tests__/data_edge.test.mjs` and `packages/go/data_edge_test.go`
    (the committed group-1 harnesses), `crates/iscc-lib/tests/test_data_edge.rs`,
    `tests/test_data_edge.py`, `.claude/context/specs/edge-vectors.md`, and each surface's existing
    conformance test for how it loads JSON and decodes `bytes:<hex>`

## Not In Scope

- Already done in `a84b665`; do not redo it: the note reword, both vendored copies (Go, .NET), their
    `VENDORED_COPIES` registration, and the `seq_test.go` cleanup
- Swift and Kotlin (group 2) and C/C++ (group 3). Do not vendor more copies:
    `test_no_unregistered_tracked_copy` would demand harnesses for them
- Deleting per-binding tests that share an input with a vector (WASM `unit.rs`, JNI `IsccLibTest`,
    Ruby `test_seq.rb`, .NET `SeqTests.cs`/`NulInputTests.cs`). They pin each surface's error type
- The `normal` .NET pin-loop issue, even though .NET `GenMixedCodeV0([])` is in this harness's path
    (see the stop rule)
- Any binding, Go-port or core behaviour change; any edit to `data-edge.json` itself

## Implementation Notes

- **Where each surface loads the file:** Java and Ruby read the canonical
    `crates/iscc-lib/tests/data-edge.json`, with the same relative path scheme as their `data.json`
    test (`Path.of("../../iscc-lib/tests/…")`, `File.expand_path(…, __dir__)` with explicit UTF-8).
    WASM uses `include_str!("../../iscc-lib/tests/data-edge.json")` in a `#[wasm_bindgen_test]`
    file. .NET reads its vendored `testdata/data-edge.json` from the output directory, like
    `ConformanceTests.cs`.

- **Harness shape:** mirror the Node/Go harnesses in the surface's idiom:

    - a section-name → small-caller table for the five sections;
    - a `checkDoc(doc)` that returns failures;
    - a test that the real file yields none;
    - a test that a synthetic doc with an unknown section yields an "unknown section" failure.

    Hardcode no vector count (WASM CLAUDE.md notes that its `data.json` counts are hardcoded; this
    harness must not copy that). Adding a vector to a handled section needs no harness edit.

- **Expectation:** `except[<id>]` if present, else `error`/`outputs`. The ids are `wasm`, `java`,
    `dotnet` and `ruby`; `skip` means no call. On an expected rejection, assert that the raw call
    alone raises the surface's error type, never its message text:

    - WASM: `Err`
    - Java: `IllegalArgumentException`
    - .NET: `IsccException`
    - Ruby: `RuntimeError` (`to_magnus_err` in `crates/iscc-rb/src/lib.rs`)

    Convert results only after a successful call:

    - `iscc_decode` → `[mt, st, vs, length, "bytes:<hex>"]`
    - `encode_seq` → `"bytes:<hex>"`
    - gen `outputs`: compare each listed key that the surface's result exposes. Compare a bare string
        result (WASM, JNI) to `iscc`. Never widen a return type.

    .NET meets the gen_text NUL vector through its `except` entry. JNI and Ruby keep the NUL and must
    match the stated `outputs`.

- **Stop rule:** if a vector fails on a surface for a real behavioural reason, do not add an
    `except` and do not fix the binding. Leave that surface's harness out, finish the others, and
    name the vector in the handoff. Example: .NET `GenMixedCodeV0([])` may surface the FFI's NULL
    message, but that still raises `IsccException`, so it passes. Only a wrong type or a wrong
    success counts.

- Nothing touches `crates/iscc-lib/src` or `crates/iscc-lib/tests/*.rs`, so the CRAP and iai gates
    are unaffected.

## Verification

Each surface run must name its data-edge tests as passed; exit 0 alone does not count.

- `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py` passes, and
    `mise run check` exits 0
- Go (carried over): in `packages/go`, `CGO_ENABLED=0 go test -count=1 ./...` and `go vet ./...`
    pass, and `grep -l data-edge packages/go/*.go | grep -v _test.go` prints nothing
- Node (carried over): in `crates/iscc-napi`, `npm run build:debug && npm test` passes, including
    both data-edge tests
- WASM: `wasm-pack test --node crates/iscc-wasm --features conformance` passes
- Java: `cargo build -p iscc-jni && mvn clean test -f crates/iscc-jni/java/pom.xml` passes
- .NET:
    `cargo build -p iscc-ffi && dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`
    passes
- Ruby: in `crates/iscc-rb`, `bundle exec rake compile && bundle exec rake test` passes, and
    `bundle exec standardrb` is clean

## Done When

All six group-1 harnesses (Go, Node, WASM, Java, .NET, Ruby) run every vector, each with an
unknown-section negative test, and every command above passes. The one sanctioned exception is a
surface held back under the stop rule and reported in the handoff.
