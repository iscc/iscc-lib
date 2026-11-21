## 2026-11-21 — Complete the C/C++ how-to, register it FULL, and drop the 0.4 crate pin

**Done:** `docs/howto/c-cpp.md` now names every Tier 1 symbol and is registered `FULL`. The 17
missing symbols are covered by runnable C++ examples inside "C++ wrapper (`iscc.hpp`)": a second
block under "Gen functions", an `encode_component` block under "Codec and utilities", and the
subsections Text utilities, Encoding utilities, Algorithm primitives and Constants, plus one
sentence linking `../c-ffi-api.md` for the C functions. Both minimal-build snippets now read
`cargo add iscc-lib --no-default-features`.

**Files changed:**

- `scripts/check_tier1_docs.py`: `docs/howto/c-cpp.md` added to `FULL`, before
    `docs/howto/dotnet.md`
- `docs/howto/c-cpp.md`: six new C++ blocks with output comments, plus short prose. No existing
    block or section was edited or removed (every `c-ffi-dx.md` §1 section is still there)
- `docs/howto/rust.md`, `crates/iscc-lib/README.md`: the ```` ```toml ```` `version = "0.4"` block
    is now a ```` ```bash ```` `cargo add` block. The sentence before it still reads right, so it is
    unchanged

**Verification:**

- `uv run scripts/check_tier1_docs.py` → `OK: 20 full and 2 exact Tier 1 docs match` (before the doc
    edit, with the registry line added, it listed exactly the 17 symbols from next.md)
- Registry one-liner → exit 0
- `uv run pytest tests/test_check_tier1_docs.py tests/test_c_ffi_docs.py` → 42 passed
- `git grep -nE 'version = "0\.' -- docs crates/iscc-lib/README.md` → no output (exit 1). Each file
    has one `cargo add iscc-lib --no-default-features` line
- C++ blocks: a `/tmp` script pulled every ```` ```cpp ```` block that contains `int main()` out of
    the doc. Each was compiled alone with
    `g++ -std=c++17 -Wall -I packages/cpp/include -I crates/iscc-ffi/include … -L target/debug -liscc_ffi`
    against the `cargo build -p iscc-ffi` library, then run with `LD_LIBRARY_PATH`. No warnings,
    every run exited 0, and every output matched its comment:
    - Gen functions (new block): `EEAYAAAAAAAAAAAA`, `EIAQABPGNYAADYSA`, `EMAQAAAAAAAAAAAA`,
        `EQASDJINXFXA2SQX`, `IAA24FYDB5G4T7JW`, `KUABGAWXH2VYUQWIVYLQGD2NZH6TM` (each with the `ISCC:`
        prefix)
    - encode_component: `AAAQCAQDAQCQMBYI`, `1`, `1`
    - Text utilities: `1`, `Hello World Foo`, `Hello`, `helloworld`
    - Encoding utilities: `SGVsbG8`, `data:application/json;base64,eyJrZXkiOiJ2YWx1ZSJ9`
    - Algorithm primitives: `Hell orld`, `255 0`, `32`, `1`, `8`
    - Constants: `128`, `4096`, `128000`, `4194304`, `13`
    - The existing Quick start, Streaming and Conformance blocks also compile and run (exit 0)
    - mdformat rewrapped prose in the check run. A re-extract showed every code block byte-identical
- `uv run zensical build` → exit 0, no warnings. `mise run check` → exit 0 (the first run failed
    only because mdformat rewrapped prose; the second run was clean)

**Next:** part 3 of "Derived facts in guidance and docs are written by hand and drift": the counts,
file lists and line numbers in the specs and the per-crate `CLAUDE.md` files.

**Notes:**

- I left the existing "Gen functions" block alone and added a second runnable block under the same
    heading. The old block still calls `gen_sum_code_v0("document.pdf")`, which throws at run time
    (placeholder file), and it is not wrapped in `main()`. The same holds for the existing "Codec
    and utilities" block (`"ISCC:KEC..."` placeholder) and the ISCC-SEQ / ISCC-IDv1 / Error handling
    blocks: none of them have `main()`. I did not touch any of them
- Measured: the ISCC-CODE of Data-Code + Instance-Code is `ISCC:KUA…`, not `KAA…`. The new block
    shows the measured value. The `low` Sum-Code comment issue elsewhere is still out of scope
- The C++ constants comments say "max bytes" rather than copying dotnet's wording, because the
    values are UTF-8 byte limits
- No Rust source changed, so the CRAP, iai and semver gates are unaffected
