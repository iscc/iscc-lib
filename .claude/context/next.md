# Next Work Package

## Step: Complete the C/C++ how-to, register it FULL, and drop the 0.4 crate pin

## Goal

Close the docs part of "Derived facts in guidance and docs are written by hand and drift":
`docs/howto/c-cpp.md`, the last how-to outside the Tier 1 doc gate's registry, documents every Tier
1 symbol and is registered `FULL`. In the same step, close "The minimal-build snippet pins
`iscc-lib` to 0.4" by making both snippets version-free.

## Alternatives Considered

- **Chosen:** c-cpp.md completion + registration, with the two-line 0.4 fix folded in. Both are
    docs-only `normal` v0.8.0 blockers with no open decision, and the 0.4 fix alone is too small to
    fill an iteration. `FULL` over `EXEMPT`: all 17 missing symbols exist in `iscc.hpp` and
    `iscc.h`, and no C++ API page is in the registry, so `EXEMPT` would leave the C++ wrappers for
    those symbols documented nowhere
- **Rejected:** part 3 of the drift issue (counts, file lists and line numbers in specs and
    per-crate `CLAUDE.md`). It is a wide sweep and needs its own step. Next after this one

## Scope

- **Create**: none
- **Modify**: `scripts/check_tier1_docs.py` (add `docs/howto/c-cpp.md` to `FULL`, keep alphabetical
    order); `docs/howto/c-cpp.md`; `docs/howto/rust.md` ("Feature flags" snippet);
    `crates/iscc-lib/README.md` (same snippet)
- **Reference**: `packages/cpp/include/iscc/iscc.hpp` (C++ names and signatures),
    `crates/iscc-ffi/include/iscc.h`, `docs/howto/dotnet.md` (section layout to mirror: Text
    utilities, Encoding utilities, Algorithm primitives, Constants), `docs/c-ffi-api.md` (the C
    reference to link), `.claude/context/specs/c-ffi-dx.md` §1 (sections the guide must keep)

## Not In Scope

- Editing `iscc.hpp`, `iscc.h` or any binding source. This step is docs plus one registry line
- The `ISCC:KAA…` Sum-Code comment fix (a `low` issue) and `howto/rust.md`'s "All 10 `gen_*_v0`
    functions" line
- Part 3 of the drift issue (specs and per-crate `CLAUDE.md` counts)
- Making the 0.4 lines `version_sync.py` targets: that adds a derived fact the issue wants gone
- A C++ signature-pinning test in the style of `tests/test_rust_api_signatures.py`

## Implementation Notes

- The gate currently reports these 17 symbols missing from c-cpp.md: `META_TRIM_NAME`,
    `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `TEXT_NGRAM_SIZE`, `encode_base64`,
    `encode_component`, `json_to_data_url`, `gen_audio_code_v0`, `gen_image_code_v0`,
    `gen_instance_code_v0`, `gen_mixed_code_v0`, `gen_video_code_v0`, `soft_hash_video_v0`,
    `text_clean`, `text_collapse`, `text_remove_newlines`, `text_trim`. Re-derive the list with the
    gate after registering, since the doc must pass `check_full`
- Add them inside "C++ wrapper (`iscc.hpp`)": extend "Gen functions" with the missing gen calls and
    add subsections for text utilities, encoding utilities, algorithm primitives (with
    `soft_hash_video_v0`) and constants, in the `dotnet.md` style. In C++ the constants are
    functions (`iscc::meta_trim_name()`, `iscc::text_ngram_size()`, …). One sentence plus a link to
    `../c-ffi-api.md` covers the matching C functions
- Do not write new counts ("the four text utilities", "all 35") or hand-typed signature lists. Show
    usage through examples. Unchecked derived facts are the defect this issue tracks
- Every new or extended C++ block must compile and run. Wrap it in `main()`, then
    `cargo build -p iscc-ffi`, then
    `g++ -std=c++17 -I packages/cpp/include -I crates/iscc-ffi/include … -liscc_ffi`, linked against
    the cargo target dir with `LD_LIBRARY_PATH` set. Every output comment (`// "helloworld"`) must
    be the measured output, not a guess. Use the reference-style sample inputs from `dotnet.md`
    where they fit
- Any C type named in prose must carry its `iscc_` prefix (`tests/test_c_ffi_docs.py` enforces this,
    e.g. `iscc_IsccByteBuffer`). C++ class names such as `iscc::DataHasher` are fine
- 0.4 fix: in both files, replace the ```` ```toml ```` block with a ```` ```bash ```` block holding
    `cargo add iscc-lib --no-default-features` (flag confirmed in `cargo add --help`). Adjust the
    sentence before it if it no longer reads right
- No Rust source changes, so the CRAP, iai and semver gates are unaffected

## Verification

- `uv run scripts/check_tier1_docs.py` exits 0
- Registry one-liner exits 0:
    `uv run python -c "import sys,glob; sys.path.insert(0,'scripts'); import check_tier1_docs as c; assert 'docs/howto/c-cpp.md' in c.FULL; assert all(p in c.FULL or p in c.EXEMPT for p in sorted(glob.glob('docs/howto/*.md')))"`
- `uv run pytest tests/test_check_tier1_docs.py tests/test_c_ffi_docs.py` passes
- `git grep -nE 'version = "0\.' -- docs crates/iscc-lib/README.md` prints nothing, and both files
    contain `cargo add iscc-lib --no-default-features`
- Every new or changed C++ block in `docs/howto/c-cpp.md` compiles with `g++ -std=c++17` and runs
    against a fresh `libiscc_ffi`, printing what its comments claim (the handoff lists each block
    and its result)
- `uv run zensical build` exits 0 and `mise run check` exits 0

## Done When

All six verification criteria pass, and `docs/howto/c-cpp.md` is registered `FULL` and names every
Tier 1 symbol.
