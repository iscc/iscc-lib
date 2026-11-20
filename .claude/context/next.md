# Next Work Package

## Step: `alg_simhash` section in `data-edge.json`, run on all 12 surfaces

## Goal

Covers part 1 of the issue "`alg_simhash([])` has no shared vector; C++ list wrappers pass NULL for
an empty vector": add an `alg_simhash` section (empty list → error, plus one even-count tie vector)
and a handler in every harness. Ruby, Swift, Kotlin, C and C++ get their first test on `[]`.

## Alternatives Considered

- **Chosen:** the shared `alg_simhash` section. The behaviour is settled (decisions.md 2026-11-20,
    core and Go since iteration 225), and five surfaces have no test on it at all.
- **Rejected:** the five C++ `ptrs.data()` → `detail::safe_data` wrappers (part 2). It is a smaller
    change, but it is a behaviour change on one surface, and this vector cannot pin its
    `alg_simhash` wrapper anyway (the FFI handles `num_digests == 0` before its NULL checks,
    `lib.rs:1207`). It comes next, as its own step.

## Scope

**Fan-out:** the `alg_simhash` data-edge section (the behaviour is fixed by the core and
decisions.md 2026-11-20), applied to Rust, Python, Node.js, WASM, Java, Go, .NET, Ruby, Swift,
Kotlin, C and C++. **This cannot be split by surface group.** Once the section is in the JSON, every
harness without a handler fails, the generator fails closed on the unknown section, and the four
vendored copies have to stay identical. Only the final tree is judged, so commit per surface in any
order, but keep the fixture, its copies and the generator in the last commit.

- **Modify (fixture):** `crates/iscc-lib/tests/data-edge.json` + the 4 vendored copies
    (`packages/{go/testdata,dotnet/Iscc.Lib.Tests/testdata,kotlin/src/test/resources}/data-edge.json`,
    `packages/swift/Tests/IsccLibTests/data-edge.json`)
- **Modify (harnesses):** `crates/iscc-lib/tests/test_data_edge.rs`, `tests/test_data_edge.py`,
    `crates/iscc-napi/__tests__/data_edge.test.mjs`, `crates/iscc-wasm/tests/data_edge.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`,
    `packages/go/data_edge_test.go`, `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`,
    `crates/iscc-rb/test/test_data_edge.rb`,
    `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/DataEdgeTest.kt`,
    `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`
- **Modify (generator):** `scripts/gen_data_edge_vectors.py`, the regenerated
    `crates/iscc-ffi/tests/data_edge_vectors.h`, and `tests/test_gen_data_edge_vectors.py` if a
    pinned section list needs the new name
- **Reference:** `.claude/context/specs/edge-vectors.md`,
    `reference/iscc-core/iscc_core/simhash.py`, `packages/swift/CLAUDE.md` (Linux Swift recipe),
    `.github/workflows/ci.yml` (CRAP lines)

## Not In Scope

- The five C++ `iscc.hpp` wrappers that pass `ptrs.data()` (part 2 of the issue, the next step)
- The Swift `String ==` / `as? Int` looseness and the Python `0`/`False` conflation (their own
    issue)
- A mismatched-length vector. The reference silently truncates `[b"\x00", b"\x00\x00"]` while Rust
    and Go reject it. That divergence has no decisions.md entry, so it needs one from the human
    first. Mention it in the handoff and do not add it
- Any core, Go or binding behaviour fix, and any `except` entry. If a surface disagrees with a
    vector, stop and report it; do not patch around it inside the fan-out
- The stale `alg_simhash` / `sliding_window` return type in the `rust-core.md` Tier 1 table

## Implementation Notes

- **Vectors** (exactly two, inputs = one positional list of `bytes:<hex>` digests):
    - `test_0001_empty_list`: inputs `[[]]`, `error` "hash_digests must not be empty". Reference:
        `IndexError` (already in `REJECTIONS`), so there is no `diverges` and no `except`
    - `test_0002_even_count_tie`: inputs `[["bytes:f00f", "bytes:3c3c"]]`, outputs `"bytes:fc3f"`.
        With two digests, a bit set in exactly one of them is a tie, and the reference sets it
        (`>= n/2`). OR is `fc3f`, AND is `300c`, so a `>` threshold shows up. Probed against iscc-core
        1.5.0: `fc3f`. The note names the tie rule
- **Output form:** a bare `bytes:<hex>` string, like `encode_seq`. Reuse each harness's existing
    bytes-to-fixture helper. The new piece on every surface is decoding `bytes:<hex>` **inputs**
    into that language's byte-list type
- **Python:** `outcome()` calls `getattr(module, section)(*inputs)` with raw JSON. Add a per-section
    input preparer that decodes `bytes:<hex>` for `alg_simhash`, applied identically to `iscc_lib`
    and `iscc_core`. Never decode `bytes:` strings in other sections. Keep the conversion outside
    the `try`. Add a `SEED_SET` row for the empty list
- **Generator:** `list_arrays` takes strings. Give it a bytes path: a digest list renders through
    `fixture_bytes`, with an explicit length per item, and keeps the non-NULL one-element sentinel
    for `[]`. Add a struct
    `iscc_edge_alg_simhash_vector { name, c, cpp, iscc_edge_list digests,   iscc_edge_str simhash }`
    and a renderer, and update the preamble comment. In C, call
    `iscc_alg_simhash((const uint8_t *const *)v->digests.items, v->digests.lens, v->digests.count)`;
    C++ builds a `std::vector<std::vector<uint8_t>>`. Regenerate with
    `uv run --script scripts/gen_data_edge_vectors.py`
- **Rust `test_data_edge.rs` CRAP:** CI scores a test helper at 0% coverage, so keep every new
    function at cyclomatic complexity ≤ 5, using the existing `const` section table plus small
    functions. Run CI's two CRAP lines. If a score moves, regenerate `.crap-baseline.json` with
    `mise run coverage && mise run crap:baseline` in this step
- **Toolchains:** rebuild the gitignored native artifacts before running napi, Ruby and JNI. Swift
    is gone from `/tmp/swifttc`: reinstall it per `packages/swift/CLAUDE.md` (Debian 12 tarball,
    about 5 min). Kotlin: run via `./gradlew` and force the tests to run, then assert the new test
    names in the JUnit XML under `build/test-results`, never the exit code alone
- `.NET`: `cargo build -p iscc-ffi` first, then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`

## Verification

- `cargo test -p iscc-lib --test test_data_edge` and
    `uv run pytest tests/test_data_edge.py   tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`
    pass; CI's `cargo llvm-cov   -p iscc-lib --lcov --output-path lcov.info` + its
    `cargo crap … --fail-regression --fail-above` line (copied from `ci.yml`) exit 0
- Node (`npm test` in `crates/iscc-napi` after a fresh build),
    `wasm-pack test --node   crates/iscc-wasm`, `mvn -f crates/iscc-jni/java/pom.xml clean test`,
    `go test ./...` in `packages/go`, the .NET command above, and `bundle exec rake test` in
    `crates/iscc-rb` all pass
- Swift `swift test` (Linux recipe) and Kotlin `./gradlew test` pass, and Kotlin's JUnit XML shows
    the data-edge test ran with 0 failures
- The C test (`cargo build -p iscc-ffi` + compile/run `crates/iscc-ffi/tests/test_iscc.c` as in
    `ci.yml`) and the C++ test (CMake/ctest as in `ci.yml`) pass, each reporting rows from the
    `alg_simhash` section
- `uv run --script scripts/gen_data_edge_vectors.py` followed by
    `git status --porcelain crates/iscc-ffi/tests/data_edge_vectors.h` prints nothing, and
    `python3 -c "import json;assert len(json.load(open('crates/iscc-lib/tests/data-edge.json'))['alg_simhash'])==2"`
    exits 0
- `mise run check` exits 0

## Done When

The `alg_simhash` section holds the two vectors and all 12 surfaces run them green, with the
generated header unchanged by a fresh regeneration.
