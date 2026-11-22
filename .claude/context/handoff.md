## 2026-11-22 — `alg_minhash_256` data-edge section on all 12 surfaces

**Done:** I added an `alg_minhash_256` section with two vectors to `data-edge.json` and its 4
vendored copies: `test_0001_empty_list` (`[[]]`, error `features must not be empty`) and
`test_0002_four_features` (`[[7, 42, 2147483648, 4294967295]]`, output `bytes:de7197a4…68b4b8`).
Neither has an `except` or `diverges` entry. All 12 harnesses and the C generator now handle the
section. The .NET and C++ docs of `AlgMinhash256`/`AlgSimhash` now name the empty-list error. There
is one commit per surface, then a final commit with the JSON, the generator and the header.

**Files changed:**

- `tests/test_data_edge.py`: `("alg_minhash_256", [[]])` added to `SEED_SET`, converter
    `convert_encode_seq` (bytes → `bytes:<hex>`)
- `crates/iscc-lib/tests/test_data_edge.rs`: `call_minhash` and the helpers `u32_item`,
    `u32_list_arg`
- `packages/go/data_edge_test.go`: `edgeCallMinhash` (`[]uint32` via `edgeArg`)
- `crates/iscc-napi/__tests__/data_edge.test.mjs`, `crates/iscc-wasm/tests/data_edge.rs`
    (`u32_list_arg` → JS array): handler entries
- `DataEdgeTest.java`: `SECTIONS` switched to `Map.ofEntries`, plus `callMinhash` and `uint32Bits`
    (reads a long, checks the range 0..2^32-1, casts to the `int` bit pattern)
- `crates/iscc-rb/test/test_data_edge.rb`, `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`
    (`ToUIntArray`), `packages/swift/.../DataEdgeTests.swift` (`feature` via `DataEdge.scalar`),
    `packages/kotlin/.../DataEdgeTest.kt` (`uintList`): handler entries
- `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`:
    `check_edge_alg_minhash_256`, added to the section runs
- `packages/dotnet/Iscc.Lib/IsccLib.cs`, `packages/cpp/include/iscc/iscc.hpp`: doc lines only
- `scripts/gen_data_edge_vectors.py`: struct `iscc_edge_u32_list {values, count}`, vector struct,
    `require_u32_list`, `u32_list_array` (`Nu` literals, a `0u` sentinel when empty),
    `row_alg_minhash_256`, preamble line. `data_edge_vectors.h` is regenerated (31 rows)
- `tests/test_gen_data_edge_vectors.py`: tests for the renderer, the rows and failing closed on bad
    features (5 cases)
- `data-edge.json` and the 4 vendored copies: the section was spliced in as text, so the
    hand-formatting is unchanged and all 5 copies hold the same bytes

**Verification:**

- Rust: `cargo test -p iscc-lib --test test_data_edge` gives 7 passed. With a mutated digest in the
    JSON, the row fails with expected/got. The CI CRAP lines (`cargo llvm-cov` then
    `cargo crap … --fail-regression --fail-above`) both exit 0: 0 regressed, new fns `u32_item` 2.0,
    `u32_list_arg` 6.0, `call_minhash` 6.0
- Python:
    `pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py   tests/test_vendored_fixtures.py`
    gives 199 passed. The `-k minhash -v` run shows `test_reference_outcome` (iscc-core 1.5.0)
    PASSED for both vectors, and the seed row is present
- Regenerating the header is a no-op against the working tree, and `git status` showed no change to
    `iscc.h` or `NativeMethods.g.cs`. The header is committed in the final commit, so
    `git status --porcelain crates/iscc-ffi` is empty only after that commit
- nodejs, built in a `/tmp` tree copy: `npm test` gives 243 pass, 0 fail. With a mutated digest the
    data-edge test fails (`# fail 1`)
- wasm: `wasm-pack test --node crates/iscc-wasm --features conformance` exits 0, data_edge 8/8 ok
- java: `cargo build -p iscc-jni && mvn clean test` exits 0. The JUnit XML for `DataEdgeTest` shows
    tests=8, failures=0, errors=0
- go: `CGO_ENABLED=0 go test ./...` ok, `GOARCH=386 go test ./...` ok, both `go vet` runs clean,
    gofmt clean
- ruby: `rake test` gives 175 runs, 0 failures. standardrb is clean
- dotnet: `dotnet test … -e LD_LIBRARY_PATH=…` gives Passed 155, Failed 0
- c: gcc with `-Wall` builds clean, 152 passed, 0 failed, with both rows PASS. cpp: cmake ASAN
    build, 123 passed, 0 failed, with both rows PASS
- kotlin: the JUnit XML for `DataEdgeTest` shows tests=6, failures=0, errors=0. swift: Swift 6.1.2
    on Linux, `swift test --filter DataEdge` executed 11 tests with 0 failures
- The next.md JSON one-liner exits 0. Both grep criteria show an empty-list line above each of the 4
    functions. `mise run check` exits 0

**Next:** the other `normal` issue, "Gen functions accept `bits` above 256" (a core behaviour
change, so it should land on its own).

**Notes:**

- The hand-written empty-list tests (C++ `alg_minhash_256({})` and others) are untouched. I did not
    add or remove any. next.md excluded adding them and did not ask for removal
- Mutation proofs ran on Rust and Node only (one digest mutation each). The other harnesses fail
    closed on an unknown section, so they are proven to dispatch, but they had no mutation run in
    this step
- The CRAP log lists `call_meta`/`call_text` (cfg twins) as "removed" and then as new, after a line
    shift. The gate still exits 0, and the baseline does not need a refresh for that
