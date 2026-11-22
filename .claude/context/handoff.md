## 2026-11-22 — `encode_component` data-edge section on all 12 surfaces

**Done:** `data-edge.json` and its 4 vendored copies gain an `encode_component` section with two
vectors: `test_0001_short_digest` (`[0,0,0,64,"bytes:01020304"]`, error, `diverges` names
decisions.md 2026-11-22) and `test_0002_exact_length_digest` (output `"AAAQAAICAMCAKBQH"`). All 12
harnesses, the C/C++ header generator and its tests handle the section. No library file changed.

**Files changed:** (one commit per surface, then JSON + generator)

- `tests/test_data_edge.py`: `prepare_encode_component` (decodes the digest) and the `str` converter
- `crates/iscc-lib/tests/test_data_edge.rs`: `u8_arg`, `header_args`, `call_encode_component`
- `packages/go/data_edge_test.go`: `edgeCallEncodeComponent`
- `crates/iscc-napi/__tests__/data_edge.test.mjs`: `prepareComponent`, `identity` as the converter
    (its docstring now says it also passes a bare string result)
- `crates/iscc-wasm/tests/data_edge.rs`: `number_arg`, `call_encode_component`
- `crates/iscc-jni/java/.../DataEdgeTest.java`: `callEncodeComponent` (10th `Map.of` pair, the max)
- `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`: `CallEncodeComponent`
- `crates/iscc-rb/test/test_data_edge.rb`: `prepare_component`, `convert_string`
- `packages/swift/.../DataEdgeTests.swift`: `byteArg`, `prepareComponent`, `convertString`
- `packages/kotlin/.../DataEdgeTest.kt`: `byte`, `prepareComponent`, `convertString`
- `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`: per-row checks, run after
    `alg_cdc_chunks`
- `scripts/gen_data_edge_vectors.py`: struct `iscc_edge_encode_component_vector` (`uint8_t` header,
    `uint32_t bit_length`, digest, unit), `require_u8`, `row_encode_component`; header regenerated
    (`ISCC_EDGE_TOTAL_COUNT` 27 → 29)
- `tests/test_gen_data_edge_vectors.py`: rendered-row test, 4 bad-input fail-closed cases, a
    non-string-unit case
- the 5 `data-edge.json` copies: section spliced as text before the final `}` (same bytes in all 5)

**Verification:**

- Vectors probed on iscc-core 1.5.0 under `--python 3.10` and `3.15`: `AAAQCAQDAQ` (short) and
    `AAAQAAICAMCAKBQH` (exact). The Python binding raises `digest length 4 < bit_length/8 (8)` on
    the short one and returns `AAAQAAICAMCAKBQH` on the exact one
- `cargo test -p iscc-lib --test test_data_edge`: 7 passed. CI CRAP lines
    (`cargo llvm-cov -p iscc-lib --lcov`, then `cargo crap … --fail-regression --fail-above`): exit
    0, `↑ 0 regressed`; `header_args` and `call_encode_component` score 20.0 (cc 4), `u8_arg` 2.0
- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`:
    185 passed; `-k encode_component -v` shows schema, iscc_lib and the iscc-core cross-check
    passing for both vectors
- After the final commit, `uv run python scripts/gen_data_edge_vectors.py` then
    `git status --porcelain crates/iscc-ffi` prints nothing
- go `CGO_ENABLED=0 go test ./...` ok (also `GOARCH=386`, `go vet` clean); nodejs `npm test` in a
    `/tmp` copy (built with `napi build --platform`): 238 pass, 0 fail; wasm
    `wasm-pack test --node crates/iscc-wasm --features conformance`: 11 + 8 + 127 passed; java
    `mvn clean test`: exit 0, DataEdgeTest XML `tests="8" failures="0" errors="0"`; ruby
    `bundle exec rake test`: 168 runs, 0 failures; dotnet: 155 passed
- c (`gcc` line from ci.yml): 150 passed, 0 failed, both `data_edge/encode_component/*` PASS; cpp
    (cmake ASAN build): 121 passed, 0 failed, both rows PASS
- kotlin JUnit XML: `DataEdgeTest tests="6" failures="0" errors="0"`; swift
    `swift test --filter DataEdge`: 11 tests, 0 failures
- JSON count one-liner: exits 0. `mise run check`: exits 0
- Test that the vector catches a regression: Go from before `3cd346f1` (codec + call sites + tests
    restored in a `mktemp -d` copy) fails `TestDataEdgeVectorsPass` with exactly
    `encode_component/test_0001_short_digest: expected an error, got AAAQCAQDAQ`. Copy removed

**Next:** the `normal` issues left both need a decision from Titusz: `alg_minhash_256([])` and the
WASM hasher `finalize()` rule. Everything else open is `low`.

**Notes:**

- The commits for C and C++ do not compile on their own: the struct exists only once the final
    commit regenerates the header. The `alg_cdc_chunks` fan-out had the same order (fail-closed
    sections make it one unit)
- CRAP output lists `call_meta`/`call_text` (cfg twins) as "removed since baseline" and 7 as "new".
    Nothing regressed and the gate exits 0, so the baseline was not refreshed. Whether the same
    listing showed before this step was not checked
- WASM passes each header field as `JsValue::from(u32)` read with `u32_arg`. The binding's own
    `to_u8` does the narrowing, so the harness does not check the range first. Every other surface
    reads the fields into its native `u8`/`UByte`/`byte`/`int` type
