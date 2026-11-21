## 2026-11-21 — Register the Rust docs and the Python API page with the Tier 1 doc gate

**Done:** Completed `docs/rust-api.md`, `docs/howto/rust.md` and `docs/architecture.md` → "Tier 1"
and registered them `FULL`; registered `docs/api.md` `EXEMPT`. Fixed the false `IsccResult<String>`
gen-function claims (rust-api, architecture) and the false "Python returns JSON strings" claim
(api.md).

**Files changed:**

- `scripts/check_tier1_docs.py`: registry only — 3 paths added to `FULL` (alphabetical),
    `docs/api.md` added to `EXEMPT` with the mkdocstrings reason
- `docs/rust-api.md`: intro, every gen signature (each returns its own `*CodeResult`; video copies
    the `<S: AsRef<[i32]> + Ord>` bound), "Returns" prose, examples use `result.iscc` with real
    asserted values; new sections Text Utilities, Algorithm Primitives (incl. `soft_hash_video_v0`),
    Encoding, Codec (`iscc_decode`, `iscc_decompose`, `encode_component`, ISCC-SEQ moved here from
    "Types"), Streaming, Constants, Diagnostics; `IsccError` and Error Handling text fixed;
    `## Functions` → `## Code Generation`
- `docs/howto/rust.md`: "Decoding and decomposing" + "Encoding" subsections under Codec operations,
    `soft_hash_video_v0` bullet under Algorithm primitives, a Constants section
- `docs/architecture.md`: Tier 1 subsection is now a names-only grouped list + SemVer sentence +
    link to the Rust API reference; pseudo-signature block removed
- `docs/api.md`: intro + example use `result["iscc"]` / `result.iscc`; no `json`

**Verification:**

- `uv run scripts/check_tier1_docs.py` → `OK: 19 full and 2 exact Tier 1 docs match …`, exit 0
- Registry assertion one-liner from next.md → exit 0
- `uv run pytest tests/test_check_tier1_docs.py` → 36 passed
- `grep -n json.loads docs/api.md` → nothing. `grep -n -B3 "IsccResult<String>"` on rust-api +
    architecture → only the `json_to_data_url` (rust-api:497) and `encode_component` (rust-api:559)
    signatures
- Return types: a throwaway regex script diffed every `pub fn` signature (params + `->`) in
    rust-api.md against the source files — all 30 OK; the two video signatures (nested generics)
    diffed verbatim against lib.rs — identical except the trailing `{`
- Examples: every non-signature ```` ```rust ```` block of rust-api.md (27) and howto/rust.md (24)
    was extracted into a throwaway `crates/iscc-lib/tests/zz_scratch.rs`, compiled and run (deleted
    afterwards, with the `example.bin` it wrote). All compile; all pass except the ones that open
    placeholder files that don't exist (`document.pdf`, `large_file.bin` ×2 — pre-existing text)
- Python result type probed: `MetaCodeResult → IsccResult → dict`, `r["iscc"] == r.iscc`
- `uv run zensical build` exit 0; `mise run check` exit 0 (first run: mdformat realigned tables,
    second run clean)

**Next:** `docs/howto/c-cpp.md` (missing 17 symbols, sources `iscc.h` / `iscc.hpp`; FULL vs EXEMPT
depends on `c-ffi-dx.md` / `cpp-bindings.md`), then part 3 of the issue (counts in specs and
per-crate `CLAUDE.md`).

**Notes:**

- Added a Streaming section to rust-api.md (`DataHasher`/`InstanceHasher` signatures) although
    next.md did not list them as missing — they were only named in prose; the section completes the
    page. `DataHasher::finalize` is `mut self` in source; the doc shows `self` (rustdoc rendering)
- Pre-existing counts left untouched (part 3): howto/rust.md "All 10 `gen_*_v0` functions". No new
    counts were added
- rust-api.md "Types" intro still says the `codec` module provides the types although `IsccError`
    lives in `lib.rs` — pre-existing, outside the listed fixes
- No Rust source changed: CRAP, iai and semver gates unaffected
