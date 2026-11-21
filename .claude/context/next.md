# Next Work Package

## Step: Register the Rust docs and the Python API page with the Tier 1 doc gate

## Goal

Part 1 of the `normal` issue "Derived facts in guidance and docs are written by hand and drift", for
the docs whose truth lives in `crates/iscc-lib/src/lib.rs`: complete `docs/rust-api.md`,
`docs/howto/rust.md` and `docs/architecture.md` → "Tier 1" and register them `FULL`. Also register
`docs/api.md` `EXEMPT`. Two of these pages carry false claims: `rust-api.md` and `architecture.md`
say gen functions return `IsccResult<String>`, and `api.md` says Python returns JSON strings.

## Alternatives Considered

- **Chosen:** the three Rust-sourced docs plus `docs/api.md`. They share one source of truth
    (`lib.rs`, plus the `iscc_lib` module for `api.md`), and the same false `IsccResult<String>`
    claim sits in two of them, so the pattern rule says fix both in one step. The `api.md` fix is a
    few lines, and its example crashes for every reader today.
- **Rejected:** `docs/howto/c-cpp.md` now. It is missing 17 symbols that come from different sources
    (`iscc.h`, `iscc.hpp`), and whether it is `FULL` or `EXEMPT` depends on `c-ffi-dx.md` /
    `cpp-bindings.md`. It is the next step. Part 3 of the issue (counts in specs and `CLAUDE.md`
    files) comes after that.

## Scope

- **Modify**: `scripts/check_tier1_docs.py` (registry only: 3 paths to `FULL`, `docs/api.md` to
    `EXEMPT` with a reason)
- **Modify (docs)**: `docs/rust-api.md`, `docs/howto/rust.md`, `docs/architecture.md`, `docs/api.md`
- **Reference**: `crates/iscc-lib/src/lib.rs`, `crates/iscc-lib/src/types.rs`,
    `crates/iscc-lib/src/codec.rs` (`encode_base64`, `iscc_decompose`), `streaming.rs`, `utils.rs`,
    `cdc.rs`, `minhash.rs`, `simhash.rs`, `conformance.rs`; `docs/howto/go.md` (an already-`FULL`
    how-to, for layout only, never for types)

## Not In Scope

- `docs/howto/c-cpp.md`: that is the next step
- Part 3 of the issue (counts, file lists, line numbers in specs and per-crate `CLAUDE.md`)
- Any change to the gate's matching or parsing logic, or to the per-crate READMEs (already correct:
    they print `result.iscc`)
- Rewriting the rest of `architecture.md` (the Tier 2 text and the module table) beyond the "Tier 1"
    subsection

## Missing symbols (measured with `named_symbols` against the derived set)

- `rust-api.md`: the 5 constants, `alg_cdc_chunks`, `alg_minhash_256`, `alg_simhash`,
    `sliding_window`, `conformance_selftest`, `encode_base64`, `iscc_decompose`, `json_to_data_url`,
    `soft_hash_video_v0`, the 4 `text_*` utilities
- `howto/rust.md`: the 5 constants, `encode_base64`, `encode_component`, `iscc_decompose`,
    `soft_hash_video_v0`
- `architecture.md`: 22 symbols. It currently names only the gen functions and the hashers

## Implementation Notes

- **Every signature and return type is read from the source, never typed from memory or copied from
    a sibling doc** (iteration 190 shipped a made-up return type this way). Each gen function
    returns its own `*CodeResult` struct (`lib.rs` lines 339–1078). `json_to_data_url` and
    `encode_component` really do return `IsccResult<String>`, and `encode_base64` returns a plain
    `String`. Video takes a generic `S: AsRef<[i32]> + Ord`: copy that bound as written.
- **`rust-api.md`:** fix the intro sentence (line 9), every gen signature (including the multi-line
    `gen_meta_code_v0`), the "Returns" prose, the `IsccError` paragraph (line 357) and "Error
    Handling" (line 436). No result type implements `Display`, so `println!("{result}")` (line 19)
    and `iscc.starts_with(...)` in the per-function examples do not compile: use `result.iscc`. Add
    a full-signature section for each missing symbol, grouped like the target's categories (text
    utilities, algorithm primitives, encoding, codec, constants, diagnostics). For anything less
    obvious than "returns `.iscc`", check the example by running it in a throwaway test you then
    delete. Do not commit scratch files.
- **`howto/rust.md`:** add usage-level mentions (a short example or a sentence) inside the existing
    sections ("Codec operations", "Algorithm primitives") plus a short constants section. It is a
    how-to, so it does not need full signatures.
- **`architecture.md` → "Tier 1":** drop the pseudo-signature block and the sentence saying the gen
    functions are *the* Tier 1 entrypoints. Replace them with a names-only, grouped list of every
    crate-root Tier 1 symbol and keep the SemVer sentence. Do not hand-type any more signatures
    there.
- **`api.md`:** the bindings return `dict` subclasses (`MetaCodeResult` → `IsccResult` → `dict`,
    probed), so `json.loads(...)` raises `TypeError`. Rewrite the intro sentence and the example to
    use `result["iscc"]` (attribute access works as well, as in the Python how-to). In the `EXEMPT`
    reason, say the page renders the API from the `iscc_lib` module via `::: iscc_lib`
    (mkdocstrings), so its source lists nothing by hand.
- **No new hand-written counts** ("35 symbols", "10 gen functions") in any added text. Counts are
    part 3 of the issue and are banned by the hygiene rule. Use "all Tier 1 symbols" or "every gen
    function".
- `docs/` pages and the registry tuple only: no Rust source changes, so CRAP, iai and semver are
    unaffected.

## Verification

- `uv run scripts/check_tier1_docs.py` exits 0
- `uv run python -c "import sys; sys.path.insert(0, 'scripts'); import check_tier1_docs as c; assert {'docs/rust-api.md', 'docs/howto/rust.md', 'docs/architecture.md'} <= set(c.FULL); assert 'docs/api.md' in c.EXEMPT"`
    exits 0
- `uv run pytest tests/test_check_tier1_docs.py` passes
- `grep -n "json.loads" docs/api.md` prints nothing. Every `IsccResult<String>` hit in
    `docs/rust-api.md` and `docs/architecture.md` belongs to a `json_to_data_url` or
    `encode_component` signature (reviewer reads the `grep -n -B3` output)
- Every return type in a `rust-api.md` signature equals the one in the source (reviewer diffs the
    `-> …` of each documented `pub fn` against `lib.rs` / `codec.rs`)
- `uv run zensical build` exits 0 and `mise run check` exits 0

## Done When

All four docs are registered, the gate and its tests pass, and no page claims a gen function returns
a string or that Python returns JSON.
