# Next Work Package

## Step: `encode_component` data-edge section on all 12 surfaces

## Goal

Pin the decided short-digest rule of the Tier 1 `encode_component` (decisions.md 2026-11-22) with a
shared vector run by every harness. Picks up the `normal` [audit] issue "No shared vector pins the
short-digest rule of `encode_component`"; it is the only `normal` issue the loop can move without
Titusz.

## Alternatives Considered

- **Chosen:** the `encode_component` section fan-out: decided behaviour, already implemented on the
    core and Go, and its absence blocks v0.8.0
- **Rejected:** `alg_minhash_256([])` and the WASM hasher `finalize()` rule. Both `normal` issues
    need a behaviour decision from Titusz first. Everything else open is `low`

## Scope

- **Fan-out:** the decided `encode_component` short-digest rule, applied to rust, python, nodejs,
    wasm, c, cpp, java, go, ruby, dotnet, swift, kotlin (one commit per surface). No library file
    changes behaviour
- **Modify**: `crates/iscc-lib/tests/data-edge.json` and its 4 vendored copies
    (`packages/dotnet/Iscc.Lib.Tests/testdata/`, `packages/go/testdata/`,
    `packages/kotlin/src/test/resources/`, `packages/swift/Tests/IsccLibTests/`);
    `scripts/gen_data_edge_vectors.py` + regenerated `crates/iscc-ffi/tests/data_edge_vectors.h`;
    harnesses `crates/iscc-lib/tests/test_data_edge.rs`, `tests/test_data_edge.py`,
    `crates/iscc-napi/__tests__/data_edge.test.mjs`, `crates/iscc-wasm/tests/data_edge.rs`,
    `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`,
    `packages/go/data_edge_test.go`, `crates/iscc-rb/test/test_data_edge.rb`,
    `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/DataEdgeTest.kt`,
    `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`; `tests/test_gen_data_edge_vectors.py`
- **Reference**: commits `80eee7d9`..`8b24a5b2` (the `alg_cdc_chunks` section fan-out, same shape:
    one commit per harness, then JSON + generator). `specs/edge-vectors.md`.
    `reference/iscc-core/iscc_core/codec.py` (`encode_component`, slices the digest)

## Not In Scope

- A row with a digest longer than `bit_length/8`: only Go and the Python binding were compared
- Rows with two faults (bad Version plus short digest): Go and Rust name different faults
- Rows for `bits` above 256 (the open `low` upstream issue), and any library-code change
- Comparing error messages on any surface

## Implementation Notes

Two vectors, inputs `[mtype, stype, version, bit_length, "bytes:<hex>"]`, `outputs` a plain string
(the unit without `ISCC:`, as the core returns it). Probed by define-next on iscc-core 1.5.0 under
Python 3.10 and 3.15 (same result) and on the local Python binding:

- `test_0001_short_digest`: `[0, 0, 0, 64, "bytes:01020304"]`, `error` (digest shorter than
    bit_length/8), `diverges` naming decisions.md 2026-11-22 "encode_component rejects a digest
    shorter than bit_length/8". iscc-core returns `AAAQCAQDAQ`; iscc_lib raises
- `test_0002_exact_length_digest`: `[0, 0, 0, 64, "bytes:0001020304050607"]`, outputs
    `"AAAQAAICAMCAKBQH"` on iscc-core and iscc_lib

Each note is one sentence naming the idiosyncrasy (iscc-core slices a short digest and emits a unit
whose header claims more bits than its body). Python: `iscc_core.encode_component` takes the same
five arguments, so the cross-check needs only a preparer that decodes the digest.

Per-surface argument types: uniffi (Swift `UInt8`/`UInt32`, Kotlin `UByte`/`UInt`) takes
`(mtype, stype, version: u8, bit_length: u32, digest)`. Check the Java, .NET, napi, WASM, Ruby and C
signatures in each surface's existing `encode_component` smoke test. C generator: add a struct and
`row_encode_component`. The generator and every harness fail closed on an unhandled section, so all
must land before the final JSON commit is green.

Traps: Rust `tests/*.rs` helpers score 0% coverage under CRAP, keep each at cyclomatic complexity ≤
5 (CI-only `--fail-above`). napi: `napi build` fails in the 9p checkout, so build and test in a
`git archive` copy under `/tmp` (learnings). Swift toolchain recipe: `packages/swift/CLAUDE.md`.
Kotlin: read the JUnit XML, not the exit code. No hot-path or Rust `src/` change, so iai and semver
do not move.

## Verification

- `cargo test -p iscc-lib --test test_data_edge` passes, and the CI CRAP lines from `ci.yml`
    (llvm-cov then cargo crap with `--fail-regression --fail-above`) exit 0
- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py` and
    `uv run pytest tests/test_vendored_fixtures.py` pass, with the iscc-core cross-check running
    both `encode_component` vectors
- After `uv run python scripts/gen_data_edge_vectors.py`, `git status --porcelain crates/iscc-ffi`
    prints nothing
- nodejs `npm test` (in a `/tmp` copy), wasm `wasm-pack test --node crates/iscc-wasm` with the
    conformance feature, java `mvn test -f crates/iscc-jni/java/pom.xml`, go
    `CGO_ENABLED=0 go test ./...`, ruby `bundle exec rake test`, dotnet
    `dotnet test packages/dotnet` all pass
- c `test_iscc` and cpp `test_iscc` (build lines from `ci.yml`) pass; kotlin JUnit XML shows
    `DataEdgeTest` with 0 failures; swift `swift test --filter DataEdge` passes
- `python3 -c "import json;assert len(json.load(open('crates/iscc-lib/tests/data-edge.json'))['encode_component'])==2"`
    exits 0, and `mise run check` exits 0

## Done When

The `encode_component` section runs green on all 12 surfaces and in the Python iscc-core
cross-check.
