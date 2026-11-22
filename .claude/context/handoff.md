## 2026-11-22 — Review of: `alg_minhash_256` data-edge section on all 12 surfaces

**Verdict:** PASS

**Summary:** The step adds an `alg_minhash_256` section with two vectors (the empty-list error and a
four-feature digest) to `data-edge.json` and its 4 vendored copies, and all 12 harnesses and the C
generator run it. No library code changed behaviour; the only non-test edits are doc lines in
`IsccLib.cs` and `iscc.hpp` and the generator script. The fan-out is honest: one decided behaviour
(decisions.md 2026-11-22), one commit per surface, no surface made a design choice of its own.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge` passes (7 passed, also under the three other CI
    feature sets), and the CI CRAP lines exit 0 (0 regressed, 7 new, none above the threshold)
- [x] `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`
    passes (199 passed); `test_reference_outcome` ran both `alg_minhash_256` vectors
- [x] After `uv run python scripts/gen_data_edge_vectors.py`,
    `git status --porcelain crates/iscc-ffi` prints nothing (also after `cargo build -p iscc-ffi`:
    `iscc.h` and `NativeMethods.g.cs` did not move)
- [x] nodejs `npm test` in a `/tmp` copy (243 pass, 0 fail), wasm
    `wasm-pack test --node crates/iscc-wasm --features conformance`, java `mvn clean test`
    (`DataEdgeTest` 8 tests, 0 failures), go `CGO_ENABLED=0 go test ./...` (also `GOARCH=386`), ruby
    `rake test` (175 runs, 0 failures), dotnet `dotnet test packages/dotnet/Iscc.Lib.Tests/` (155
    passed) all pass
- [x] c `test_iscc` (152 passed) and cpp `test_iscc` (ASAN build, 123 passed) pass with both rows
    PASS; kotlin JUnit XML shows `DataEdgeTest` tests=6, failures=0, errors=0; swift
    `swift test --filter DataEdge` executed 11 tests with 0 failures
- [x] The JSON one-liner exits 0 (2 vectors, no `except`, no `diverges`); both greps show an
    empty-list error line above each of the four functions; `mise run check` exits 0

Beyond next.md's list (3 probes): iscc-core 1.5.0 returns the stated digest and raises `ValueError`
on the empty list under Python 3.10 (compiled) and 3.15 (pure Python). A two-row mutation of all 5
JSON copies (wrong digest on row 2, row 1 flipped to expect the old all-`0xFF` output) made the
Python, Go, Ruby, C, C++, WASM, Java, .NET, Swift and Kotlin harnesses fail, each naming both rows.
`cargo clippy --workspace --all-targets -- -D warnings` is clean.

**Issues found:**

- (none filed). Two nits, not worth an entry: the Swift helper `feature(_:_:)` is always called with
    index 0, and `test_alg_minhash_256_rows_render_features_and_digest` assumes the empty row is row
    0 of its section

**Codex review:** no actionable findings (run over the whole range with
`--base <define-next commit>`).

**Next:** the last `normal` issue, "Gen functions accept `bits` above 256 and return a unit that no
decoder reads". It is a core behaviour change and its "Resolved when" spans the core, the Go port,
shared vectors and the API pages, so it needs more than one step. A workable order:

1. The Rust core alone: an upper bound of 256 in `codec::encode_length` and `codec::decode_length`,
    so that every gen function, `SumHasher`/hasher `finalize` and `encode_component` reject it. The
    10 surfaces built on the core inherit it. This touches a benchmarked path, so the step must run
    `mise run bench:iai:check` and the CI CRAP lines, and it must check that no `data.json` vector
    uses a unit above 256 bits
2. The Go port (`packages/go`), then the `data-edge.json` rows (marked `diverges`, since iscc-core
    1.5.0 returns the undecodable code; new rows in existing sections need no new harness section),
    then the API pages ("32 to 256")

Not measured so far, per the issue: the hashers' `finalize(bits)` on any surface.

**Notes:**

- Resolved and deleted: "No shared vector pins the `alg_minhash_256` empty-list error". One `normal`
    issue is left; the other 14 entries are `low`
- state.md still says "10 sections" in five places; the file now has 11 (update-state's file)
- The hand-written empty-list tests on the Rust core, C FFI, UniFFI, Python, Java, C++ and Go stay
    in place next to the shared vector. They also pin the error text, which the vectors do not
- learnings.md is at its 200-line budget; this review added no entry
