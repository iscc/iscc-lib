## 2026-11-20 — Review of: `alg_simhash` section in `data-edge.json`, run on all 12 surfaces

**Verdict:** PASS

**Summary:** The `alg_simhash` section (empty list → error, even-count tie → `bytes:fc3f`) is in
`data-edge.json` and its four vendored copies, and every one of the 12 harnesses runs it. Each
harness decodes the `bytes:<hex>` inputs outside the raw call, so a decode error can never count as
a rejection. The fan-out is honest: 13 commits, one per surface plus the fixture/generator commit,
no core or binding behaviour change, no `except` or `diverges` marker.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge` — 7 passed
- [x] `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`
    — 110 passed (includes the iscc-core 1.5.0 reference check on both vectors)
- [x] CI's `cargo llvm-cov -p iscc-lib --lcov` + `cargo crap … --fail-regression --fail-above` —
    exit 0 (0 regressed, 0 new, 0 moved, 148 unchanged)
- [x] Node after a fresh `build:debug` — 179/179, `data-edge.json` subtest ok
- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` — data_edge 5 passed
- [x] `mvn clean test` — `DataEdgeTest` 5 tests, 0 failures (108 total)
- [x] `go test ./...` in `packages/go` — ok; `go vet` and `gofmt -l` clean
- [x] .NET — 150 passed, all five `DataEdgeTests` listed by name
- [x] Ruby `rake test` after `rake compile:dev` — 160 runs, 0 failures, six `TestDataEdge` tests
- [x] Swift (Linux recipe) — 25 tests, 0 failures, `DataEdgeTests` 6/0
- [x] Kotlin `./gradlew test --rerun-tasks` — JUnit XML `DataEdgeTest tests="6" failures="0"`,
    including `testDataEdgeVectorsPass()`
- [x] C test (gcc as in `ci.yml`) — 99 passed, 0 failed, both `data_edge/alg_simhash/…` rows
- [x] C++ test (CMake + ASAN) — 98 passed, 0 failed, both `data_edge/alg_simhash/…` rows
- [x] Generator re-run leaves `data_edge_vectors.h` unchanged; the `len(...['alg_simhash'])==2`
    assert passes
- [x] `mise run check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` clean

Extra probe (1 of 3): the advance agent skipped the mutation run on Java, .NET and Kotlin. I changed
the expected `fc3f` to `300c` in the copies they read. Java, .NET, Kotlin and WASM each failed with
`alg_simhash/test_0002_even_count_tie: expected "bytes:300c", got "bytes:fc3f"`. The files were
restored afterwards.

**Issues found:**

- Nit, fixed here: a comment in `tests/test_data_edge.py` still described the raw call as
    `(*inputs)`; it now names `prepare_inputs`
- Nit, not filed: Python `decode_bytes` uses `bytes.fromhex`, which also accepts upper-case digits
    and whitespace. The other harnesses are stricter. No current vector is affected
- Not this loop's file: the advance `MEMORY.md` is 152 lines, over its 140-line budget

**Codex review:** Reviewed the whole advance range (`--base` the define-next commit). No actionable
findings.

**Next:** The remaining part of the issue "C++ list wrappers pass NULL for an empty vector"
(rewritten in issues.md to that smaller problem): the five `iscc.hpp` wrappers that pass
`ptrs.data()`, three of them also `lens.data()`. Each needs its own test on `{}` that checks the
message is not the FFI's "must not be NULL" one. After that, the gate-leniency issue in
`scripts/check_spec_paths.py`.

**Notes:**

- When the C++ step closes that issue, its `**Spec:**` field applies: the Tier 1 table in
    `rust-core.md` still shows `alg_simhash` and `sliding_window` returning a bare `Vec`, while the
    code returns `IsccResult<…>`. The issue is `[human]`-sourced, so the reviewer that closes it
    corrects those two rows
- Still open for Titusz: a mismatched-length digest list. iscc-core 1.5.0 silently truncates
    `[b"\x00", b"\x00\x00"]`, Rust and Go reject it. No decisions.md entry covers it, so no vector
    was added
- `cmake` is not on `$PATH` in the devcontainer; `uvx cmake` works for the C++ suite
- No performance-sensitive path and no public API was touched
