# Next Work Package

## Step: `data-edge.json` seed set with the Rust and Python harnesses

## Goal

Start the v0.8.0 criterion "Shared edge-case vectors run everywhere". Create
`crates/iscc-lib/tests/data-edge.json` with the spec's seed set and a schema test, run it from the
Rust core and Python (Python also checks each vector against iscc-core 1.5.0), and fold
`STRICT_BASE32_DIVERGENCES` in `tests/test_seq.py` into the file. This is the spec's suggested first
slice. It fixes the file format and the `iscc_decode` output encoding that the binding fan-out will
copy.

## Alternatives Considered

- **Chosen:** data-edge seed + Rust/Python harnesses. It is the only unstarted v0.8.0 release
    criterion, and the binding fan-out is blocked until the format exists
- **Rejected:** the Python `encode_seq` docstrings (`normal`, `[human]`). It is small, but it
    touches different files and does nothing for the release criterion. The handoff's Ruby Bignum
    pick is now `low`, so it is skipped

## Scope

- **Create**: `crates/iscc-lib/tests/data-edge.json`, `crates/iscc-lib/tests/test_data_edge.rs`,
    `tests/test_data_edge.py`
- **Modify**: `crates/iscc-lib/Cargo.toml` (add `exclude = ["tests/data-edge.json"]` to
    `[package]`), `tests/test_vendored_fixtures.py`, `tests/test_seq.py`
- **Reference**: `.claude/context/specs/edge-vectors.md` (format, rules, seed set),
    `tests/test_conformance.py` (loader style), `crates/iscc-lib/src/conformance.rs` (Rust JSON
    walking), `crates/iscc-lib/src/types.rs` (result fields)

## Not In Scope

- Vendored copies for Go/.NET/Kotlin/Swift and harnesses on the other 11 surfaces. Those belong to
    the next fan-out step (copy the file into the four `data.json` dirs then). Copying them now
    could hit surface build quirks, such as SwiftPM's unhandled-resource warning
- C/C++ generated table and recovering `scripts/gen_ffi_boundary_vectors.py`
- Changing any library behaviour. All seed outcomes below match the current Rust/Python output
- Seed vectors beyond the spec list plus the migrated `STRICT_BASE32_DIVERGENCES` rows
- The `encode_seq` docstring issue

## Implementation Notes

**Seed outcomes, probed with iscc_core 1.5.0 and the current iscc_lib** (re-check while writing):

- `iscc_decode` (input `[str]`). `ISCC:GAAAAAAAAA======`, `ISCC:GAAAAAAAAB` and
    `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` → error. The ref ACCEPTS all three, so each gets
    `"diverges": "decisions.md 2026-11-18: Base32 decoding stays strict"`. `ISCC:GAAAAAAAAA` →
    output. **Output encoding (new, fix it here):**
    `[maintype, subtype, version, length_index,   "bytes:<hex>"]` as JSON ints plus a `bytes:`
    string, e.g. `[3, 0, 0, 0, "bytes:00000000"]`. Python iscc_lib returns IntEnums, so `int()` them
- `encode_seq` (input `[[units]]`, output `"bytes:<hex>"` as in data.json):
    - `["gaawdlqng6ſsdpf2"]` and the sharp-s twin `["gaawdlqng6ßdpf2"]` → error. The ref accepts both
        (`300161ae0d37a521bcba`), so they get a `diverges` naming the decisions.md 2026-11-18 entry
        "Codec input is upper-cased ASCII-only, encode_seq included"
    - `["gaawdlqng6ssdpf2"]` → `bytes:300161ae0d37a521bcba`
    - `[]` → error (both)
    - `["ISCC:GAAAAAAAAA\u0000junk"]` → error (both)
    - Migrated from `STRICT_BASE32_DIVERGENCES`, all errors with the strict-base32 `diverges`:
        `ISCC:GAAAAAAAAB`, the 256-bit unit with last char `J`, and `ISCC:GAAAAAAAAA======`
- `gen_text_code_v0` `["Hello\u0000World", 64]` →
    `{"iscc": "ISCC:EAASKDNZNYGUUF5A",   "characters": 10}` (both agree; no `except` needed for
    rust/python)
- `gen_mixed_code_v0` `[[], 64]` and `gen_iscc_code_v0` `[[]]` → error. The ref raises
    `AssertionError` / `ValueError` respectively, so they agree. data.json passes no `wide` for
    iscc-code: call with `false`

**Rust harness** (`test_data_edge.rs`): read the file at runtime from
`concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data-edge.json")`, not `include_str!`, because the file
is excluded from the package. Structure it as a pure `fn check_doc(&Value) -> Vec<String>` (failure
list) plus one `#[test]` asserting the real file yields none. Dispatch on the section name.
**Unknown section → failure.** `except.rust`: `"error"` means assert Err, `"skip"` means skip.
Compare only the keys present in `outputs`. Add negative tests, ONE bad case each (learnings: one
bad case per call): an unknown section, an output mismatch, an expected error that succeeds, and an
expected output that errors. Each must yield a failure.

**Python** (`test_data_edge.py`):

- (a) A `validate_vector` schema check: requires `inputs` and `note`, and exactly one of `outputs` /
    `error`. `except` keys must be valid surface ids and values `error`/`skip`. `diverges` must
    start with `decisions.md `. Run it on every real vector, plus negative tests for each malformed
    shape
- (b) An iscc_lib harness over every vector. Unknown section → fail, honour `except.python`, and
    expect `ValueError` on errors
- (c) A reference check. For each vector, compute iscc_core's outcome (any exception = error; output
    compared like (b)). It must equal the stated outcome iff `diverges` is absent, and differ iff it
    is present
- Parametrize with vector ids so failures name the vector

`test_seq.py`: delete `STRICT_BASE32_DIVERGENCES` and the two tests that use it. Leave the other
differential tests alone. `test_vendored_fixtures.py`: add `data-edge.json` to `FIXTURE_BASENAMES`,
widen the `git ls-files` pathspec so it matches, and register the canonical path with an empty copy
list (the copies arrive in the fan-out).

Neither iai nor CRAP is affected (tests and a manifest only).

## Verification

- `cargo test -p iscc-lib --test test_data_edge` passes, including the 4 negative harness tests
- `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py` passes.
    The file holds every seed input listed in the spec's "Seed Set"
- `grep -n STRICT_BASE32_DIVERGENCES tests/test_seq.py` finds nothing
- `cargo package --list -p iscc-lib --allow-dirty` output contains no `data-edge` line, but still
    lists `tests/data.json`
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean
- `mise run check` exits 0

## Done When

All six verification criteria pass on the working tree.
