## 2026-11-20 — `alg_simhash` section in `data-edge.json`, run on all 12 surfaces

**Done:** Added the `alg_simhash` section to `data-edge.json` (and its 4 vendored copies):
`test_0001_empty_list` (`[[]]` → error) and `test_0002_even_count_tie`
(`[["bytes:f00f","bytes:3c3c"]]` → `"bytes:fc3f"`). Every harness got a handler. Each one decodes
the `bytes:<hex>` inputs outside the raw call (Python `PREPARERS`, Node/Ruby an optional third tuple
slot, the others in their existing prepare step). The C/C++ generator got a binary list path and a
new struct. Commits: one per surface (12), then a final commit with the fixture, the generator and
the header.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json` + 4 vendored copies: the new section (still byte-identical)
- `crates/iscc-lib/tests/test_data_edge.rs`: `bytes_item`, `bytes_list_arg`, `call_simhash`
    (cyclomatic complexity ≤ 2); `.crap-baseline.json` regenerated
- `tests/test_data_edge.py`: `decode_bytes`, `prepare_alg_simhash`, `PREPARERS`/`prepare_inputs`
    (used for iscc_lib, iscc_core and the raw-raises path), a converter, a `SEED_SET` row for `[]`
- `crates/iscc-napi/__tests__/data_edge.test.mjs`, `crates/iscc-wasm/tests/data_edge.rs`,
    `DataEdgeTest.java`, `packages/go/data_edge_test.go`, `DataEdgeTests.cs`,
    `crates/iscc-rb/test/test_data_edge.rb`, `DataEdgeTests.swift`, `DataEdgeTest.kt`: the handler
- `scripts/gen_data_edge_vectors.py`: `require_bytes_list`, `byte_list_arrays` (`list_arrays` now
    delegates to it), `row_alg_simhash`, the `iscc_edge_alg_simhash_vector` struct, preamble
    comment; `crates/iscc-ffi/tests/data_edge_vectors.h` regenerated (TOTAL 16 → 18)
- `tests/test_gen_data_edge_vectors.py`: 3 tests (binary list render, the alg_simhash rows plus the
    empty-list sentinel, a non-`bytes:` digest fails closed)
- `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`: `check_edge_alg_simhash`
    plus a negative test (expected `300c`, the AND of the digests, must fail)

**Verification:**

- `cargo test -p iscc-lib --test test_data_edge`: 7 passed. CI coverage + CRAP lines: exit 0, then
    the baseline was regenerated (same lcov). The re-run reports 0 regressed, 0 new, 0 moved, 148
    unchanged, exit 0
- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`:
    110 passed
- napi after `npm run build:debug`: 179/179.
    `wasm-pack test --node crates/iscc-wasm --test data_edge`: 5 passed. `mvn clean test` after
    `cargo build -p iscc-jni`: 108 run, 0 failures (DataEdgeTest 5/0). `go test ./...` ok; `go vet`
    and `gofmt -l` are clean. .NET (ci.yml form): 150 passed. Ruby after `rake compile:dev`:
    `rake test` 160 runs, 0 failures
- Swift 6.1.2 Linux toolchain reinstalled in `/tmp/swifttc`: `swift test` ran 25 tests, 0 failures
    (DataEdgeTests 6/0). Kotlin `./gradlew test --rerun-tasks`: the JUnit XML has
    `DataEdgeTest tests="6" failures="0" errors="0"`, including `testDataEdgeVectorsPass()`
- C (gcc as in ci.yml): 99 passed, 0 failed, including
    `PASS: data_edge/alg_simhash/test_0001_empty_list`, `.../test_0002_even_count_tie` and the
    negative. C++ (CMake + ASAN, `build-uv/`): 98 passed, 0 failed, with the same rows
- Mutation check: I changed `fc3f` to `300c` in the copy each harness reads. Rust, Python (lib and
    ref), Node, Go, Ruby and Swift each reported
    `alg_simhash/test_0002_even_count_tie: expected "bytes:300c", got "bytes:fc3f"`. The copies were
    then restored, and `test_vendored_fixtures` is green
- A fresh regen leaves the header unchanged (`render == disk`), and the `len(...['alg_simhash'])==2`
    assert passes. `mise run check` exit 0 (the first run let ruff reformat one file, the second run
    was clean). Pre-push hooks on all 21 changed files: clippy, cargo test, ty, ruff S/C901 and
    pytest all pass

**Next:** Part 2 of the issue: the five C++ `iscc.hpp` wrappers that pass `ptrs.data()`. Each one
needs its own `{}` test, because the `alg_simhash` vector cannot pin that pointer (the FFI checks
`num_digests == 0` first). Then the gate-leniency issue in `scripts/check_spec_paths.py`.

**Notes:**

- Not added, as next.md said: a mismatched-length vector. iscc-core 1.5.0 silently truncates
    `[b"\x00", b"\x00\x00"]`, while Rust and Go reject it. That divergence has no decisions.md entry
    yet, so the human has to decide it first.
- No surface disagreed with either vector, so there are no `except` or `diverges` markers. iscc-core
    gives `fc3f` and raises `IndexError` on `[]`, which I checked with
    `uv run --with iscc-core==1.5.0`.
- Fan-out: about 20 files. Rust also touched `.crap-baseline.json`, because new functions moved the
    line numbers of existing ones (the learnings flag line moves as CI-only reds).
- The Java, .NET and Kotlin mutation runs were skipped to save time. Their handlers use the same
    compare path as `encode_seq` (`bytes:` strings), and the shared "section has no vectors" check
    proves they read the new section.
- The advance MEMORY.md is 152 lines, over its own budget of 140 (it was 147 before). It needs
    pruning.
