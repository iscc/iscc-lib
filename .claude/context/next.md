# Next Work Package

## Step: data-edge.json fan-out, group 1 — Node, WASM, Java, Go, .NET, Ruby harnesses

## Goal

Run every `data-edge.json` vector from the conformance suites of the six bindings that parse JSON
and build locally (Node, WASM, Java, Go, .NET, Ruby). This is the first of three surface groups for
the v0.8.0 "Shared edge-case vectors run everywhere" criterion.

## Alternatives Considered

- **Chosen:** group 1 of the fan-out. Its behaviour is fixed by `specs/edge-vectors.md` → Rules, and
    every surface's toolchain is on `$PATH`, so all six suites can be checked in this session
- **Rejected:** all 10 bindings at once. Swift needs a toolchain install, Kotlin is its UniFFI twin,
    and C/C++ need the recovered generator, so together they exceed one ~90-minute session

## Scope

**Fan-out:** `edge-vectors.md` → Rules (every vector runs, `except` is the only deviation, an
unknown section fails, the rejection is asserted on the raw call only), applied to Node, WASM, Java,
Go, .NET and Ruby. Commit once per surface. The shared edits go in the first commit.

- **Create**: `crates/iscc-napi/__tests__/data_edge.test.mjs`,
    `crates/iscc-wasm/tests/data_edge.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`,
    `packages/go/data_edge_test.go`, `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`,
    `crates/iscc-rb/test/test_data_edge.rb`, and the vendored copies
    `packages/go/testdata/data-edge.json` and
    `packages/dotnet/Iscc.Lib.Tests/testdata/data-edge.json`
- **Modify**: `crates/iscc-lib/tests/data-edge.json` (one note, see below),
    `tests/test_vendored_fixtures.py` (register the two copies),
    `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` (add a `Content` entry for the copy),
    `packages/go/seq_test.go` (remove the mirrored list, see below), and the "Test Patterns" / "File
    Layout" lines of `crates/iscc-wasm/CLAUDE.md` and `packages/dotnet/CLAUDE.md`
- **Reference**: `tests/test_data_edge.py` and `crates/iscc-lib/tests/test_data_edge.rs` (the
    templates), `.claude/context/specs/edge-vectors.md`, and each surface's existing conformance
    test for how it loads JSON and decodes `bytes:<hex>`

## Not In Scope

- Swift and Kotlin harnesses (group 2), C and C++ (group 3, generated table plus staleness check).
    Do not vendor Swift or Kotlin copies yet: `test_no_unregistered_tracked_copy` would demand them
- Python harness changes; the `normal` Python docstring and .NET pin-loop issues
- Published-artifact absence checks (sdist, gem, npm, Go zip): a later step after all groups
- Deleting single per-binding tests that happen to share an input with a vector (napi
    `seq.test.mjs`, JNI `IsccLibTest`, Ruby `test_seq.rb`, WASM `unit.rs`, Go `codec_test.go`
    decoder tables). They also pin each surface's error type
- Any Go-port or core behaviour change (see the stop rule below)

## Implementation Notes

- **Note fix first:** in `iscc_decode/test_0005_empty_string`, replace "the Rust core a
    ValueError-class error" with wording such as "the Rust core rejects it". Then copy the file
    byte-for-byte to the two vendored paths.

- **Where each surface loads the file.** Node, Java and Ruby read the canonical
    `crates/iscc-lib/tests/data-edge.json` with the same relative-path scheme as their `data.json`
    test. WASM uses `include_str!("../../iscc-lib/tests/data-edge.json")` in a
    `#[wasm_bindgen_test]` file. Go and .NET read their vendored copy. Go reads it only from
    `_test.go`, never through `//go:embed` in `conformance.go` or any other non-test file.

- **Harness shape, per surface.** Copy the Rust template's shape in the surface's idiom:

    - a table that maps each section name (`iscc_decode`, `encode_seq`, `gen_text_code_v0`,
        `gen_mixed_code_v0`, `gen_iscc_code_v0`) to a small caller;
    - a `checkDoc(doc)` that returns a list of failures;
    - a test that the real file yields no failures;
    - a test that a synthetic doc with an unknown section yields an "unknown section" failure.

    The effective expectation is `except[<surface id>]` if present, else `error`/`outputs`. Surface
    ids: `nodejs`, `wasm`, `java`, `go`, `dotnet`, `ruby`. `skip` means no call. On an expected
    rejection, assert that the raw call alone raises the surface's error type: Node `Error`, WASM
    `Err`, Java `IllegalArgumentException`, Go non-nil `error`, .NET `IsccException`, Ruby
    `RuntimeError`. Probe the type before relying on it, and never match error text. Convert results
    only after a successful call:

    - `iscc_decode` → `[mt, st, vs, length, "bytes:<hex>"]`;
    - `encode_seq` → `"bytes:<hex>"`;
    - for gen `outputs` objects, compare each listed key (`iscc`, `characters`) to the result field in
        that surface's naming.

    Adding a vector to a handled section must need no harness edit. .NET rejects the gen_text NUL
    vector via its `except` entry; Go, JNI and Ruby keep the NUL and match the stated `outputs`.

- **Go:** `seqStrictDivergences` in `seq_test.go` declares itself a mirror of the file. Delete
    `TestEncodeSeqStrictBase32DivergencesRejected`, because the harness covers it. Keep the twin
    acceptance test, but give it a twin-only table (name, twin, hex).

- **Stop rule:** scoping read the Go source and found that `isccClean` rejects `""` and
    `softHashCodesV0` / `GenIsccCodeV0` reject fewer than 2 codes, so Go is expected to pass. If any
    vector still fails on a surface for a real behavioural reason, do not add an `except`, and do
    not fix the binding or the Go port in this step. Leave that surface's harness out, finish the
    others, and report the vector in your handoff.

- Nothing here touches `crates/iscc-lib/src` or `tests/*.rs`: CRAP and iai gates are unaffected.

## Verification

- `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py` passes, both copies are
    registered under `data-edge.json`, and
    `grep -q "ValueError-class" crates/iscc-lib/tests/data-edge.json` exits 1
- Node: in `crates/iscc-napi`, `npm run build:debug && npm test` passes, including both data-edge
    tests
- WASM: `wasm-pack test --node crates/iscc-wasm --features conformance` passes
- Java: `cargo build -p iscc-jni && mvn clean test -f crates/iscc-jni/java/pom.xml` passes
- Go: in `packages/go`, `CGO_ENABLED=0 go test -count=1 ./...` and `go vet ./...` pass, and
    `grep -l data-edge packages/go/*.go | grep -v _test.go` prints nothing
- .NET:
    `cargo build -p iscc-ffi && dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`
    passes
- Ruby: in `crates/iscc-rb`, `bundle exec rake compile && bundle exec rake test` passes, and
    `bundle exec standardrb` is clean
- `mise run check` exits 0

## Done When

All six harnesses run every vector with an unknown-section negative test, the two vendored copies
are registered, and every verification command above passes.
