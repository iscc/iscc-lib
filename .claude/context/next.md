# Next Work Package

## Step: `@iscc/lib` npm README states the `gen_sum_code_v0` result type and names every Tier 1 symbol

## Goal

Fix the `normal` issue "`@iscc/lib` npm README says every code generator returns a string": the npm
page must say that `gen_sum_code_v0` returns a `NapiSumCodeResult` object, list the whole Tier 1
surface the addon exports, and be pinned by a docs test and the Tier 1 doc gate.

## Alternatives Considered

- **Chosen:** napi README result type — a false claim on a published registry page that a reader
    acts on (`[object Object]`); docs plus one test, ranked first by the last two handoffs.
- **Rejected:** "`publish-ffi` skips a half-uploaded FFI archive on a rerun" (`normal`) — it bites
    only on a failed upload during a release recovery, and fixing it needs a design choice
    (post-upload asset check or delete `starter` assets first). Next in line.
- Other normals: Rust tier docs (4-5 files), duplicate `data.json` parsers (audit-sized), one-shot
    dedup (core, iai gate); `alg_minhash_256([])` and WASM `finalize()` need a behaviour decision
    from the human first.

## Scope

- **Create**: `tests/test_napi_docs.py`
- **Modify**: `crates/iscc-napi/README.md`, `scripts/check_tier1_docs.py` (add the README to `FULL`)
- **Reference**: `crates/iscc-napi/src/lib.rs` (`NapiSumCodeResult` at ~309, exports with
    `js_name`), `crates/iscc-wasm/README.md` lines 75-95 (the sibling wording to mirror),
    `tests/test_wasm_docs.py` (test style), `docs/howto/nodejs.md:37-40` (already correct),
    `tests/test_check_tier1_docs.py`

## Not In Scope

- Any change to `crates/iscc-napi/src/` — returning a string from `gen_sum_code_v0` is not the fix.
- The WASM README's "Generate an ISCC-SUM from raw bytes" row or any other WASM page.
- `crates/iscc-ffi/README.md:78` ("All code generators return a string pointer … except") — it
    already names its exception.
- Adding a `SumHasher` mention: napi exports no `SumHasher` (it is not Tier 1).
- The `publish-ffi` rerun issue and the version-sync file lists.

## Implementation Notes

- Replace README line 52 with wording like the WASM README: nine code generators return the ISCC
    string directly; `gen_sum_code_v0` returns a `NapiSumCodeResult` object with `iscc`, `datahash`
    and `filesize`, plus `units` (Data-Code and Instance-Code strings) when `add_units` is `true`.
    Check the field names against the struct in `src/lib.rs`.
- Fill the Utilities list so the page names all 35 Tier 1 symbols (any casing counts): add
    `json_to_data_url` to Encoding; `encode_component`, `iscc_decode`, `encode_seq`, `decode_seq`,
    `gen_iscc_id_v1` (experimental) to Codec; a **Constants** line with `META_TRIM_NAME`,
    `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`. napi exports these
    as real `u32` constants (`#[napi(js_name = …)] pub const`), not getter functions — do not copy
    the WASM "(getter functions)" note.
- Filling the list crosses `DISCOVERY_THRESHOLD` (30) in `scripts/check_tier1_docs.py`, so register
    `crates/iscc-napi/README.md` in `FULL` (alphabetical position). `tests/test_check_tier1_docs.py`
    derives its expectations from `FULL`, so no count edit should be needed there; if a test pins a
    count, follow the gate, not a hard-coded number.
- `tests/test_napi_docs.py` (module docstring, plain functions, no mocks), at least:
    - the README names `NapiSumCodeResult` and the sentence "return ISCC strings directly" is absent
        from both `crates/iscc-napi/README.md` and `docs/howto/nodejs.md` (normalise whitespace first,
        mdformat reflows at 100 columns);
    - every field named on the README's result sentence (`iscc`, `datahash`, `filesize`, `units`) is a
        field of `pub struct NapiSumCodeResult` parsed out of `src/lib.rs`, so a renamed field breaks
        the test.
- Keep inline code spans short so mdformat does not wrap inside them. Run `mise run format` before
    committing. No Rust change: CRAP, iai and semver gates are untouched; napi need not be built.

## Verification

- `uv run pytest tests/test_napi_docs.py tests/test_check_tier1_docs.py` passes
- `uv run python scripts/check_tier1_docs.py` exits 0 and `crates/iscc-napi/README.md` is in `FULL`
- `git grep -n 'return ISCC strings directly' -- ':!.claude'` prints nothing
- `grep -n NapiSumCodeResult crates/iscc-napi/README.md` prints at least one line
- `uv run zensical build` exits 0 and `mise run check` exits 0

## Done When

The npm README states the `gen_sum_code_v0` result type, names every Tier 1 symbol under the Tier 1
doc gate, and all verification commands pass.
