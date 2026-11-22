## 2026-11-22 — Shared vectors and API pages for the 256-bit unit limit

**Done:** Added the four `data-edge.json` rows from next.md (meta `bits=288` error,
`encode_component` 288 error and 256 output, `iscc_decode` 288-bit unit error) to the canonical
fixture and its 4 vendored copies, and regenerated the C header. Every API page, how-to and API
comment that said only "multiple of 32" now gives the range of 32 to 256. No harness code changed.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json`: `gen_meta_code_v0/test_0005_bits_above_256`,
    `encode_component/test_0003_bit_length_above_256`, `encode_component/test_0004_bit_length_256`,
    `iscc_decode/test_0006_unit_above_256`. The two encode-288 rows carry
    `"diverges": "decisions.md 2026-11-22: A unit is at most 256 bits"` and cite iscc/iscc-core#142
    in `note`. The decode row has no `diverges`
- `packages/{dotnet/Iscc.Lib.Tests/testdata,go/testdata,kotlin/src/test/resources,swift/Tests/IsccLibTests}/data-edge.json`:
    byte-for-byte copies (`cp`)
- `crates/iscc-ffi/tests/data_edge_vectors.h`: regenerated (4 new row names present)
- `crates/iscc-ffi/src/lib.rs` (`///` only) → `crates/iscc-ffi/include/iscc.h` (cbindgen line from
    next.md) + `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (`cargo build -p iscc-ffi`)
- `crates/iscc-jni/java/.../IsccLib.java`, `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: comment
    text only
- `docs/{c-ffi-api,java-api,ruby-api}.md`: `bits` rows → `(default: 64, multiple of 32, 32 to 256)`,
    `bit_length` rows → `(multiple of 32, 32 to 256)` (mdformat realigned the tables)
- `docs/howto/{dotnet,java,kotlin,nodejs,python,ruby,swift,wasm}.md`: error example comment →
    `bits must be a multiple of 32 from 32 to 256` (same wording as `docs/howto/go.md`)

**Verification:**

- The decode string was re-derived: base32 of `00 08` + bytes `00..23` =
    `ISCC:AAEAAAICAMCAKBQHBAEQUCYMBUHA6EARCIJRIFIWC4MBSGQ3DQOR4HZAEERCG`, as in next.md
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`:
    211 passed, 0 skipped (includes the iscc-core cross-check of the `diverges` markers)
- `cargo test -p iscc-lib --test test_data_edge`: 7 passed. `CGO_ENABLED=0 go test -count=1 ./...`
    passes on amd64 and on `GOARCH=386`
- Surface suites, run in sequence in the background, all OK: Python (`maturin develop` + all of
    `tests/`, 950 passed), `cargo test -p iscc-ffi`, .NET (155 passed), C++ via `g++` (127 passed),
    Java `mvn clean test` (exit 0), Ruby `rake compile test` (175 runs, 0 failures), Kotlin gradle
    `:test` (ran fresh, `DataEdgeTest` result written), Swift (`DataEdgeTests` 11 tests, 30 total, 0
    failures), WASM `wasm-pack test --node` (8 + 135 passed). napi built in a `git ls-files | tar`
    copy under `/tmp` with `CARGO_TARGET_DIR=/tmp/napi-target`: 243/243 passed
- Re-running the header generator and cbindgen, and the builds above, add no new `git status`
    entries (same 24 paths before and after)
- `uv run zensical build`: exit 0 ("No issues found"). `mise run check`: exit 0 on the second run
    (the first run's mdformat realigned the edited tables)
- `git grep -n "multiple of 32" -- docs crates packages | grep -v 256` prints the two test comments
    (`codec.rs:1220`, `codec_test.go:275`) plus three `crates/iscc-lib/src/streaming.rs` lines
    (43/128/197). Those are false hits: each `///` sentence continues on the next line with "from 32
    to 256." (already in place since 301), and a line-based grep cannot see that. I left them
    unchanged

**Next:** The bits issue should now be closed. Next are the three `[audit]` `normal` issues, e.g.
the conformance-table refactor in `conformance.rs`/`lib.rs`.

**Notes:**

- The meta row's input is the object `{"a": 1}`, as next.md specified (the C generator's `meta_text`
    needs an object or a string). `error` texts are short reasons that no harness matches
- No CRAP, iai or semver impact: no Rust behaviour changed (only test fixtures, comments and docs)
- Issue ledger: the issue "No vector pins the 256-bit unit limit, and most API pages lack the range"
    is resolved by this step. issues.md is not mine to edit
