# Next Work Package

## Step: `alg_minhash_256` data-edge section on all 12 surfaces

## Goal

Pin the decided empty-list rejection of `alg_minhash_256` (decisions.md 2026-11-22) with a shared
vector that every harness runs. Picks up the `normal` issue "No shared vector pins the
`alg_minhash_256` empty-list error"; it closes that issue and unblocks one of the two v0.8.0
blockers.

## Alternatives Considered

- **Chosen:** the `alg_minhash_256` section fan-out: behaviour decided and implemented on all 12
    surfaces since iteration 299, so the step is harness and generator work only
- **Rejected:** the 256-bit unit limit in the Rust core (the other `normal` issue). It is a core
    behaviour change and must land alone; finishing the minhash issue first closes one coherent
    feature before opening the next

## Scope

- **Fan-out:** the decided `alg_minhash_256` empty-list rejection, applied to rust, python, nodejs,
    wasm, c, cpp, java, go, ruby, dotnet, swift, kotlin (one commit per surface, then the JSON +
    generator commit). No library code changes behaviour
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
- **Modify (doc comments only, the review nit of iteration 298):**
    `packages/dotnet/Iscc.Lib/IsccLib.cs` and `packages/cpp/include/iscc/iscc.hpp`
- **Reference**: commits `6064506d`..`7041f47f` (the `encode_component` section fan-out, same
    shape); the `alg_simhash` section in `data-edge.json` and its handler in each harness (closest
    model: a list input, a `bytes:<hex>` output, an empty-list error row); `specs/edge-vectors.md`

## Not In Scope

- Any change to library behaviour or error messages on any surface; comparing error text
- More rows (duplicates, a single feature, large lists): two rows as below, nothing else
- Hand-written empty-list tests on napi, WASM, Ruby, .NET, Swift or Kotlin: the section replaces
    them
- The `bits` above 256 work, and the other `low` issues in issues.md

## Implementation Notes

Two vectors in a new `alg_minhash_256` section, inputs `[[<uint32>, ...]]`, outputs `bytes:<hex>`
(as in `alg_simhash`). Probed by define-next on iscc-core 1.5.0 under Python 3.10 (compiled) and
3.15 (pure Python), and on the local Python binding:

- `test_0001_empty_list`: `[[]]`, `error` `"features must not be empty"`. iscc-core raises
    `ValueError` on both builds (`min()` of an empty sequence), so no `diverges`; no `except`
- `test_0002_four_features`: `[[7, 42, 2147483648, 4294967295]]`, outputs
    `"bytes:de7197a4343a0d7422964c15b69982ffa3706205863421259c99cdf30668b4b8"` on iscc-core (both
    builds), iscc_lib and Go. The values span both ends of `uint32`, so each harness's integer
    conversion is exercised (Java `int[]` needs the unsigned values as their `int` bit pattern;
    check each surface's existing `alg_minhash_256` test for its parameter type)

Each `note` is one sentence naming the idiosyncrasy (an empty list was hashed into 32 `0xFF` bytes
before the decision). Python: `iscc_core.alg_minhash_256` is exported at top level; add the empty
row to the rejection list next to `("alg_simhash", [[]])` and a converter for the bytes output. The
existing `REJECTIONS` tuple already holds `ValueError`.

Java: the `SECTIONS` table in `DataEdgeTest.java` is `Map.of` with 10 pairs, its maximum; switch it
to `Map.ofEntries(Map.entry(...), ...)`. C generator: add a struct and a row renderer for a
`uint32_t` list (an empty row must render; mirror the `alg_simhash` empty-row handling). The
generator and every harness fail closed on an unhandled section, so all must land before the final
JSON commit is green.

Doc nits: name the empty-list error on `AlgMinhash256` AND `AlgSimhash` in `IsccLib.cs`
(`<exception cref="IsccException">Thrown when ... is empty.</exception>`, as on line 383) and on
`alg_minhash_256` AND `alg_simhash` in `iscc.hpp` (`/// Throws IsccError if ... is empty.`, as on
line 353). Both functions reject an empty list, so both get the line. A `///` edit in `iscc.hpp` or
`IsccLib.cs` touches no exported FFI item, so `iscc.h` and `NativeMethods.g.cs` must not move.

Traps: Rust `tests/*.rs` helpers score 0% coverage under CRAP, keep each at cyclomatic complexity ≤
5 (CI-only `--fail-above`). napi: `napi build` fails in the 9p checkout, so build and test in a
`git archive` copy under `/tmp` (learnings). Swift toolchain recipe: `packages/swift/CLAUDE.md`.
Kotlin: read the JUnit XML, not the exit code. No hot-path or Rust `src/` change, so iai, CRAP on
`src/` and semver do not move.

## Verification

- `cargo test -p iscc-lib --test test_data_edge` passes, and the CI CRAP lines from `ci.yml`
    (llvm-cov then cargo crap with `--fail-regression --fail-above`) exit 0
- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`
    passes, with the iscc-core cross-check running both `alg_minhash_256` vectors
- After `uv run python scripts/gen_data_edge_vectors.py`, `git status --porcelain crates/iscc-ffi`
    prints nothing
- nodejs `npm test` (in a `/tmp` copy), wasm `wasm-pack test --node crates/iscc-wasm` with the
    conformance feature, java `mvn test -f crates/iscc-jni/java/pom.xml`, go
    `CGO_ENABLED=0 go test ./...` (also `GOARCH=386`), ruby `bundle exec rake test`, dotnet
    `dotnet test packages/dotnet` all pass
- c `test_iscc` and cpp `test_iscc` (build lines from `ci.yml`) pass; kotlin JUnit XML shows
    `DataEdgeTest` with 0 failures; swift `swift test --filter DataEdge` passes
- `python3 -c "import json;d=json.load(open('crates/iscc-lib/tests/data-edge.json'))['alg_minhash_256'];assert len(d)==2 and not any('except' in v or 'diverges' in v for v in d.values())"`
    exits 0; `grep -B4 -E 'Alg(Minhash256|Simhash)\(' packages/dotnet/Iscc.Lib/IsccLib.cs` and
    `grep -B4 -E 'alg_(minhash_256|simhash)\(const' packages/cpp/include/iscc/iscc.hpp` show an
    empty-list error line above each of the four functions; `mise run check` exits 0

## Done When

The `alg_minhash_256` section runs green on all 12 surfaces and in the Python iscc-core cross-check,
with no `except` or `diverges` entry.
