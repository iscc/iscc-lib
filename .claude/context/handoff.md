## 2026-11-22 — `@iscc/lib` npm README states the `gen_sum_code_v0` result type and names every Tier 1 symbol

**Done:** The napi README now says nine generators return the ISCC string and `gen_sum_code_v0`
returns a `NapiSumCodeResult` object (`iscc`, `datahash`, `filesize`, plus `units` with
`add_units`). Its Utilities list names all 35 Tier 1 symbols, including a Constants line without the
WASM "(getter functions)" note. The README is registered in `FULL`, and a docs test pins the claim.

**Files changed:**

- `crates/iscc-napi/README.md`: result-type sentence (mirrors the WASM README); Encoding gains
    `json_to_data_url`, Codec gains `encode_component`, `iscc_decode`, `encode_seq`, `decode_seq`,
    `gen_iscc_id_v1` (experimental); new Constants line
- `scripts/check_tier1_docs.py`: `crates/iscc-napi/README.md` added to `FULL` (after
    `crates/iscc-ffi/CLAUDE.md`)
- `tests/test_napi_docs.py` (new, 4 tests): the README names `NapiSumCodeResult`; the "all return
    strings" phrase is absent from the README and `docs/howto/nodejs.md` (whitespace normalised);
    every field on the README result sentence is a field of `pub struct NapiSumCodeResult` parsed
    from `src/lib.rs`; the struct's field set equals the README's four fields, so an added field
    fails too

**Verification:**

- `uv run pytest tests/test_napi_docs.py tests/test_check_tier1_docs.py`: 40 passed
- `uv run python scripts/check_tier1_docs.py`: "OK: 22 full and 2 exact Tier 1 docs match …" (21
    before this step)
- `git grep -n 'return ISCC strings directly' -- ':!.claude'`: no output (exit 1)
- `grep -n NapiSumCodeResult crates/iscc-napi/README.md`: line 53
- Negative proof: with `git show HEAD:crates/iscc-napi/README.md` swapped in, `test_napi_docs.py`
    gave 3 failed, 1 passed (the nodejs.md test passes because that page was already correct). With
    the edited README restored, 4 passed
- `uv run zensical build`: exit 0. `mise run check`: exit 0. The first run failed only because ruff
    format rewrapped the `RESULT_SENTENCE_RE` line in the test. The second run was clean

**Next:** The `normal` issue "`publish-ffi` skips a half-uploaded FFI archive on a rerun". It needs
a design choice: check the uploaded assets after the upload, or delete `starter` assets first.

**Notes:**

- The test's banned phrase is written as `"return ISCC " + "strings directly"`, with a comment. As a
    single literal, the test file itself would match next.md's repo-wide `git grep` criterion
- No Rust change, so the CRAP, iai and semver gates are untouched. napi was not built (docs only)
