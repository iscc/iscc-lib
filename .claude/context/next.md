# Next Work Package

## Step: Shared vectors and API pages for the 256-bit unit limit

## Goal

Close the `normal` issue "No vector pins the 256-bit unit limit, and most API pages lack the range":
add `data-edge.json` rows that pin the rejection on all 12 surfaces, and make every API page and API
comment that says only "multiple of 32" state the range of 32 to 256.

## Alternatives Considered

- **Chosen:** vectors + API-page sweep together. The behaviour is decided (decisions.md 2026-11-22
    "A unit is at most 256 bits") and implemented in the core (301) and Go (302). The rows go into
    existing sections, so no harness changes. The docs are comment and docs edits only. One step
    closes the whole issue.
- **Rejected:** the `[audit]` conformance-table refactor in `conformance.rs`/`lib.rs`. It is also
    `normal`, but it is a larger refactor. Finishing the bits issue first completes a feature set
    that is already 2/3 done.

## Scope

- **Create**: none
- **Modify**:
    - `crates/iscc-lib/tests/data-edge.json` + its 4 vendored copies
        (`packages/{dotnet/Iscc.Lib.Tests/testdata,go/testdata,kotlin/src/test/resources,swift/Tests/IsccLibTests}/data-edge.json`),
        regenerated `crates/iscc-ffi/tests/data_edge_vectors.h`
    - Comment-only: `crates/iscc-ffi/src/lib.rs` (`///` at :822) → regenerated
        `crates/iscc-ffi/include/iscc.h` + `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`;
        `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (:249);
        `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (:33)
    - Docs: `docs/c-ffi-api.md`, `docs/java-api.md`, `docs/ruby-api.md`,
        `docs/howto/{dotnet,java,kotlin,nodejs,python,ruby,swift,wasm}.md`
    - File budget: 0 behaviour files. Fixtures are test files, the headers are regenerated, and the
        rest are comments and docs (decision 2026-11-20).
- **Reference**: `.claude/context/specs/edge-vectors.md`, `docs/rust-api.md` and `docs/howto/go.md`
    (wording already done there), `scripts/gen_data_edge_vectors.py`

## Not In Scope

- Any harness code change. All rows go into sections every harness already runs. If a harness pins a
    row count, update the count, nothing else.
- A row for `gen_iscc_code_v0` with an over-long unit (the outcome depends on `wide` and on unit
    order, see handoff), for image/video (they fail earlier with other errors), or for `encode_seq`.
- The 88-bit ISCC-ID `low` issue, and the `[audit]` issues (the `encode_component` short-digest docs
    gap is part of the specs/docs audit issue; do not fold it in).
- Rewording API pages that state no `bits` constraint at all (e.g. `docs/api.md`, napi/WASM docs).

## Implementation Notes

Add these rows. Each was measured on iscc-core 1.5.0 under CPython 3.10.22 **and** 3.15.2, with
identical results:

| Section            | Row                              | Inputs                                                                   | Expect                                                               | `diverges` |
| ------------------ | -------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------- | ---------- |
| `gen_meta_code_v0` | `test_0005_bits_above_256`       | `["Hello", "", {"a": 1}, 288]`                                           | `error`                                                              | yes        |
| `encode_component` | `test_0003_bit_length_above_256` | `[0, 0, 0, 288, <36 bytes 00..23>]`                                      | `error`                                                              | yes        |
| `encode_component` | `test_0004_bit_length_256`       | `[0, 0, 0, 256, <32 bytes 00..1f>]`                                      | `outputs: "AADQAAICAMCAKBQHBAEQUCYMBUHA6EARCIJRIFIWC4MBSGQ3DQOR4HY"` | no         |
| `iscc_decode`      | `test_0006_unit_above_256`       | `["ISCC:AAEAAAICAMCAKBQHBAEQUCYMBUHA6EARCIJRIFIWC4MBSGQ3DQOR4HZAEERCG"]` | `error`                                                              | no         |

- iscc-core returns a code for both 288 encode rows (meta:
    `ISCC:AAEAAZJM4W5VJ565N475OAMKUW56I5MHIYTFZ7JQ47P5RJKC6RI6FR2G`), so they carry `diverges`.
    Write it in the existing form: `"decisions.md 2026-11-22: A unit is at most 256 bits"`. The
    notes can cite iscc/iscc-core#142.
- The decode string is header `0x00 0x08` (Meta, length index 8 = 288 bits) over bytes `00..23`.
    iscc-core raises "288 is not a valid LN", so the row has no `diverges`.
- Write digests as `"bytes:<hex>"`. Write the meta input as an object, not `null`: the C generator's
    `meta_text` takes only an object or a string.
- The `error` text is a short English reason (harnesses never match it), e.g. "unit longer than 256
    bits".
- Copy the canonical file to the 4 vendored copies byte for byte, then regenerate the header:
    `uv run --script scripts/gen_data_edge_vectors.py`.
- Docs wording: follow `docs/rust-api.md` (`default: 64, multiple of 32, 32 to 256`) in tables. For
    `bit_length`, say `multiple of 32, 32 to 256`. In the how-to error examples, follow
    `docs/howto/go.md`. After the `///` edit, run `cargo build -p iscc-ffi` (rewrites
    `NativeMethods.g.cs`) and the CI cbindgen line:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`.
    Commit both outputs.
- Run the surface suites that read the file locally: Rust, Python, Go (also `GOARCH=386`), .NET,
    Java, Ruby, C/C++, Kotlin, Swift. Build napi in a `git archive` copy under `/tmp` (9p,
    learnings). WASM: `wasm-pack test --node crates/iscc-wasm`.
- Tests-only step plus docs: no CRAP, iai or semver impact.

## Verification

- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    passes. This includes the iscc-core cross-check that matches the `diverges` markers.
- `cargo test -p iscc-lib --test test_data_edge` and `CGO_ENABLED=0 go test -count=1 ./...` in
    `packages/go` pass, and the 4 new row names appear in `crates/iscc-lib/tests/data-edge.json`.
- Regenerating the C header, `cargo build -p iscc-ffi` and the cbindgen line above all leave
    `git status --porcelain` empty after the commit.
- `git grep -n "multiple of 32" -- docs crates packages | grep -v 256` prints only the two test
    comments (`crates/iscc-lib/src/codec.rs`, `packages/go/codec_test.go`).
- `uv run zensical build` exits 0 and `mise run check` exits 0.
- The .NET, Java, Ruby, C, C++, Kotlin, Swift, napi and WASM data-edge suites pass locally (commands
    are in the notes).

## Done When

All verification criteria pass: the four rows run green on every surface and no API page says only
"multiple of 32".
