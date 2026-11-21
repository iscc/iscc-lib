# Next Work Package

## Step: `alg_cdc_chunks` data-edge section on all 12 surfaces (reference-stable vectors)

## Goal

Add an `alg_cdc_chunks` section to `data-edge.json` and run it on the Rust core and all 11 bindings.
This lands the expensive part of the audit issue "No shared vector pins `alg_cdc_chunks` for a large
`avg_chunk_size`". **Reframed:** the issue's own values (2^29, 2147483647) cannot pass the Python
reference check on both CI legs (finding below), so this step ships only vectors on which every
iscc-core 1.5.0 build agrees. The large values are left for a decision.

## Alternatives Considered

- **Chosen:** the section fan-out with build-independent vectors. No decision is open for these
    vectors, all 12 harnesses have to learn the section anyway, and adding the large vectors later
    is then one JSON edit plus whatever the decision says.
- **Rejected:** the fan-out as the handoff scoped it (2^29 + 2147483647). Measured while scoping:
    iscc-core's compiled wheel (cp310–cp314, built only under `CIBUILDWHEEL=1`, see the reference
    `setup.py`) raises `OverflowError: value too large to convert to uint32_t` for
    `avg_chunk_size >= 2^29` (`cdc.pxd` types `ma` as `uint32_t`). 536870911 still works. Python
    3.15 has no wheel, so uv builds the sdist as pure Python, which returns one chunk
    (`uv run --no-project --python 3.15 --with iscc-core==1.5.0` → `cdc.py`, `1 1`). CI runs pytest
    on 3.10 (compiled) and 3.15 (pure). With or without `diverges`, one leg fails.
- **Rejected:** Python video `OverflowError` (1 file). It is independent and stays next in line.

## Scope

- **Fan-out:** the core's `alg_cdc_chunks` behaviour, applied to rust, python, nodejs, wasm, c, cpp,
    java, go, ruby, dotnet, swift, kotlin (one commit per surface)
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
- **Reference**: commit `066a149` and its per-surface siblings (`git log --oneline -12 066a149`):
    the `alg_simhash` section fan-out, the same shape. `specs/edge-vectors.md`.
    `reference/iscc-core/iscc_core/cdc.py`

## Not In Scope

- Vectors with `avg_chunk_size >= 2^29`, and any change to how the Python harness picks its
    reference build. That is the open decision. Report it in the handoff, do not settle it
- `utf32: true` vectors with a small average. The reference loops forever when a utf32 cut point is
    under 4 bytes (Rust has a `cut_point == 0` guard in `cdc.rs`), so the Python reference call
    would hang
- Vectors for `avg_chunk_size` 0 or 1, and any library-code change on any surface

## Implementation Notes

Vectors (input encoding as in `alg_simhash`: `["bytes:<hex>", <bool>, <int>]`; `outputs` is a list
of `"bytes:<hex>"`). Compute each expected list from `iscc_lib` and let the Python cross-check
confirm it against iscc-core:

- `test_0001_small_average_many_chunks`: bytes `00..1f` (32 bytes), `false`, `2`. Gives 17 chunks on
    the compiled reference, so it pins the masks, which the large values cannot
- `test_0002_largest_average_all_reference_builds`: same bytes, `false`, `536870911` (2^29 − 1, the
    largest value the compiled reference accepts). Gives one chunk equal to the input
- `test_0003_empty_input`: `"bytes:"`, `false`, `1024`. Gives `["bytes:"]`: the reference yields one
    empty chunk. The C/C++ generator needs its sentinel pattern for a zero-length byte array (no
    `{}` in C99). If a surface returns something else, drop this vector, keep the section, and
    report the surface in the handoff (no `except` without a decision, no binding fix here)

Per-harness facts: Java `algCdcChunks` takes an `int`, so 536870911 fits. napi/WASM pass an `f64`,
.NET `uint`, Swift/Kotlin `UInt32`. Python: `iscc_core.alg_cdc_chunks` returns a lazy generator, so
errors would surface in the converter, outside the `REJECTIONS` try. No vector here expects an
error, so a converter that builds the list is enough. Do not add `OverflowError` to `REJECTIONS`.
Rust `tests/*.rs` helpers score 0% coverage for CRAP: keep each helper at cyclomatic complexity ≤ 5
(CI-only `--fail-above` trap, learnings). Swift: toolchain recipe in `packages/swift/CLAUDE.md`
(`/tmp/swifttc` may be gone). Kotlin: check the JUnit XML, not the exit code.

**Finding for review:** rewrite the audit issue to its remainder, the 2^29/2147483647 vectors. They
are blocked on a decision about which iscc-core build is the reference when the builds disagree. The
decision of 2026-11-21 says "iscc-core accepts every value", which holds only for the pure-Python
build. This is also an upstream iscc-core defect: the compiled and pure builds disagree.

## Verification

- `cargo test -p iscc-lib --test test_data_edge` passes;
    `cargo llvm-cov -p iscc-lib --lcov   --output-path lcov.info && cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression --fail-above`
    exits 0
- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py   tests/test_vendored_fixtures.py`
    passes, with the iscc-core cross-check running the new section
- `uv run python scripts/gen_data_edge_vectors.py` leaves
    `git status --porcelain   crates/iscc-ffi/tests/data_edge_vectors.h` empty
- nodejs `npm test` (crates/iscc-napi), wasm
    `wasm-pack test --node crates/iscc-wasm --features   conformance`, java
    `mvn test -f crates/iscc-jni/java/pom.xml`, go `CGO_ENABLED=0 go test   ./...` (packages/go),
    ruby `bundle exec rake test` (crates/iscc-rb), dotnet `dotnet test   packages/dotnet` pass
- c `test_iscc` and cpp `build/tests/test_iscc` pass (CI build lines in `ci.yml`), kotlin
    `./gradlew test` JUnit XML shows `DataEdgeTest` with 0 failures, swift
    `swift test --filter   DataEdge` passes
- `python3 -c "import json;d=json.load(open('crates/iscc-lib/tests/data-edge.json'));assert   all(v['inputs'][2] < 2**29 for v in d['alg_cdc_chunks'].values())"`
    exits 0; `mise run check` exits 0

## Done When

The `alg_cdc_chunks` section runs green on all 12 surfaces and in the Python iscc-core cross-check,
and the handoff reports the reference-build finding.
