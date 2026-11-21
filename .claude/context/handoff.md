## 2026-11-21 — `alg_cdc_chunks` data-edge section on all 12 surfaces (reference-stable vectors)

**Done:** Added an `alg_cdc_chunks` section (3 vectors) to `data-edge.json` and its 4 vendored
copies, taught all 12 harnesses and the C/C++ header generator the section, and regenerated
`data_edge_vectors.h`. The section is green on every surface and in the iscc-core cross-check. No
library code changed.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json` + 4 vendored copies: section with
    `test_0001_small_average_many_chunks` (32 bytes, avg 2 → 17 chunks),
    `test_0002_largest_average_all_reference_builds` (avg 536870911 → 1 chunk),
    `test_0003_empty_input` (`bytes:`, avg 1024 → `["bytes:"]`). Spliced in as text, because the
    fixture is hand-formatted and `json.dumps` would reflow all of it
- `crates/iscc-lib/tests/test_data_edge.rs`: `bytes_arg`, `bool_arg`, `chunks_json`, `call_cdc`;
    `.crap-baseline.json` refreshed (lines moved)
- `tests/test_data_edge.py`: `prepare_alg_cdc_chunks` + `convert_chunks` (the converter builds the
    list from iscc-core's lazy generator; `REJECTIONS` unchanged)
- napi `data_edge.test.mjs`, WASM `tests/data_edge.rs`, Go `data_edge_test.go` (shared
    `edgeDecodeBytes` pulled out of `edgeBytesArg`), Java `DataEdgeTest.java` (`decodeBytes` pulled
    out of `byteArrays`), .NET `DataEdgeTests.cs` (`ToBytes` likewise), Ruby `test_data_edge.rb`,
    Kotlin `DataEdgeTest.kt` (`bool`; `optBool` now delegates to it), Swift `DataEdgeTests.swift`
- `scripts/gen_data_edge_vectors.py`: struct `iscc_edge_alg_cdc_chunks_vector` (data str, `int`
    utf32, `uint32_t` avg, chunk `iscc_edge_list`), `row_alg_cdc_chunks`, `require_u32`,
    `require_bool`, `NULL_LIST` for error rows. Empty data renders as `{"", 0}`, so the pointer is
    never NULL. Header regenerated
- `crates/iscc-ffi/tests/test_iscc.c` (`edge_chunks_eq`, `check_edge_alg_cdc_chunks`),
    `packages/cpp/tests/test_iscc.cpp` (`check_edge_alg_cdc_chunks`)
- `tests/test_gen_data_edge_vectors.py`: rows test, error-row test, 4 parametrized fail-closed cases
    (non-bytes data, non-bool utf32, avg 2^32, bool avg)

**Verification:**

- Vector values: iscc_lib, iscc-core 1.5.0 compiled (Python 3.10, `cdc.cpython-310-…so`) and
    iscc-core pure (Python 3.15) give identical output for all 3 vectors
- rust `cargo test -p iscc-lib --test test_data_edge`: 7 passed. CI CRAP lines exit 0 (`call_cdc` cc
    4 = 20.0). After the baseline refresh: "0 regressed, 0 new, 0 moved, 162 unchanged"
- python
    `pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`:
    156 passed, including the 3 `test_reference_outcome[alg_cdc_chunks/…]` rows
- Regenerating the header with `gen_data_edge_vectors.py` is a no-op against the working tree (diff
    stat unchanged)
- nodejs `npm test`: 238 pass. wasm
    `wasm-pack test --node … --features conformance --test data_edge`: 8 passed. go
    `CGO_ENABLED=0 go test ./...` ok, `go vet` clean. java `mvn clean test`: rc 0, the DataEdgeTest
    XML shows 8 tests, 0 failures. ruby `rake test`: 168 runs, 0 failures. dotnet: 154 passed
- c (CI gcc line): 145 passed, 0 failed, all 3 cdc rows PASS, and 0 warnings under
    `-std=c99 -Wall -Wextra -pedantic`. cpp (cmake + ASAN): 115 passed, 0 failed. kotlin
    `./gradlew test`: DataEdgeTest XML shows tests=6, failures=0. swift
    `swift test --filter DataEdge` (6.1.2 Linux toolchain): 11 tests, 0 failures
- Mutation: `bytes:1d1e`→`1d1f` in the canonical file, the Go copy and the regenerated header turned
    rust, python, go, node, c and ruby red on `test_0001`. All restored afterwards
- `2**29` assert exits 0. `mise run check` rc 0. Pre-push hooks (ty, ruff S/C901, pytest) on the
    Python files rc 0 after a ty fix in `require_u32`

**Next:** Settle the reference-build decision below. After it, the 2^29/2147483647 vectors are a
JSON edit plus whatever the decision requires. Otherwise go to the Python video frame
`OverflowError` (1 file).

**Notes:**

- **Finding for review (reference build):** the compiled iscc-core wheel (cp310–cp314) raises
    `OverflowError` for `avg_chunk_size >= 2^29` (`cdc.pxd` types `ma` as `uint32_t`). The pure
    build (Python 3.15 has no wheel, so the sdist installs) returns one chunk. CI runs pytest on
    both legs, so no vector ≥ 2^29 can pass both, with or without `diverges`. The decision of
    2026-11-21 ("iscc-core accepts every value") holds only for the pure build. Please rewrite the
    audit issue to that remainder: blocked on which build is the reference when they disagree. This
    is also an upstream defect (the compiled and pure builds disagree). Not settled here
- All 3 vectors use `utf32: false`. A utf32 vector with a small average would hang the reference
    (next.md Not In Scope)
- Commit order follows the `alg_simhash` precedent: per-surface harness commits first, then fixture,
    vendored copies, generator and header together. The intermediate commits are not
    self-consistent: the C/C++ harnesses need the regenerated header, and the Rust "covers every
    section" test needs the fixture
- Gradle left only ignored `packages/kotlin/.gradle/` and `build/`. Swift built into
    `/tmp/swiftbuild`
