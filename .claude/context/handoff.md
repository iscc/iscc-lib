## 2026-11-19 — Tier-1 33→35 doc/count sweep + `encode_seq`/`decode_seq` doc entries

**Done:** Every Tier-1 count now reads 35. All 11 how-to guides gained an `### ISCC-SEQ` subsection
in their codec section; `rust.md` had no codec section, so it gained a `## Codec operations`
section. The 4 hand-written API pages got seq entries, and the per-crate CLAUDE.md/README codec
lists plus the stale counts listed in next.md were re-derived from source. The changes are docs plus
one uniffi module-doc comment. No behaviour change.

**Files changed:**

- Counts 33→35: `crates/iscc-uniffi/src/lib.rs:3` (comment only), `docs/{java,ruby}-api.md`,
    `notes/00-overview.md`, `notes/04-api-compatibility-safety.md` (also "5 codec operations"),
    `.claude/agents/advance.md:142`, `crates/iscc-lib/CLAUDE.md` + `crates/iscc-wasm/CLAUDE.md`
    (count + codec list now includes `encode_seq`/`decode_seq`)
- API entries: `docs/rust-api.md` (`### ISCC-SEQ` after Codec Functions), `docs/java-api.md`
    (`encodeSeq`/`decodeSeq` under Codec), `docs/ruby-api.md` (`encode_seq`/`decode_seq` + Error
    Handling bullet), `docs/c-ffi-api.md` (`iscc_encode_seq`/`iscc_decode_seq` after
    `iscc_decompose`, + both free-function rows in the Memory Management table)
- How-tos: all 11 `docs/howto/*.md`. `c-cpp.md` also adds the seq functions to its ownership-table
    rows for `char**` and `IsccByteBuffer`
- Registry docs: `crates/iscc-ffi/CLAUDE.md`, `crates/iscc-jni/CLAUDE.md`, `packages/cpp/CLAUDE.md`,
    READMEs `crates/iscc-rb`, `packages/{cpp,dotnet,go,kotlin}`
- Re-derived counts:
    - `iscc-napi/CLAUDE.md`: 5 codec functions, plus it now names the missing `gen_iscc_id_v1` mint
        function
    - `iscc-py/CLAUDE.md`: 30 `#[pyfunction]`s (`grep -c` = 30, not 27)
    - `packages/go/CLAUDE.md`:
        - the selftest covers the 9 gen fns + ISCC-SEQ
        - 58 vectors = 50 gen + 4 + 4 (from data.json section sizes; the old figure was 46)
        - the codec.go row now lists ISCC-SEQ
    - `packages/dotnet/CLAUDE.md`: adds `SeqTests.cs` to the tree, plus the seq conformance methods
    - `packages/{swift,kotlin}/CLAUDE.md`: 19 methods (listed via grep: 9 gen + `testGenIsccIdV1` + 9
        seq)
    - `crates/iscc-rb/CLAUDE.md` Key Files: adds `test_iscc_id.rb` and `test_seq.rb`

**Verification:**

- Stale-count grep from next.md: no output (exit 1). A looser
    `\b3[1234]\b.*Tier|Tier.{0,12}\b3[1234]\b|3x/3x|3x (symbols|functions)` grep over the same paths
    \+ `crates/*/src/lib.rs` + `.claude/agents/*.md`: no output
- Seq-coverage `grep -LiE "encode_?seq" …` from next.md: no filenames (exit 1)
- Each signature was checked against source:
    - Rust: `codec.rs:818/857`
    - Python: `__init__.py:111/136`
    - napi: `index.d.ts:85/111`
    - WASM: `lib.rs:357/368`
    - Java: `IsccLib.java:288/300`
    - Go: `codec.go:879/919`
    - .NET: `IsccLib.cs:488/542`
    - C: `iscc.h:645/670`
    - C++: `iscc.hpp:366/384`
    - Ruby: `lib.rs:329/340`
    - Swift: `iscc_uniffi.swift:2125/2165`
    - Kotlin: `iscc_uniffi.kt:3015/3066`
- Examples were executed, not just eyeballed:
    - Python (after `maturin develop`): `34`, `2007a88248c5`, `['ISCC:EAD2…']`, round-trip assert OK
    - Rust: a throwaway integration test with the howto body passed (1 passed), then was deleted
    - Go: a temp `_test.go` printed `34 / 2007a88248c5 / [ISCC:…] / true`, then was deleted
    - Node: `node -e` against the napi `.node` printed `34 2007a88248c5 [ 'ISCC:…' ] true`. The
        `.node` was built at 21:13, after the last napi src commit (20:54). The same session checked
        the WASM element-wise compare logic on a `Uint8Array`
    - Java/Kotlin/Swift/.NET/C++/Ruby/C examples were not compiled. They use the content-equality
        idiom for each language (`Arrays.equals`, `contentEquals`, `AsSpan().SequenceEqual`, `==` on
        `Data`/`vector`/`String`)
- `uv run zensical build`: exit 0, "No issues found". `cargo build -p iscc-uniffi`: Finished
- `mise run format` is idempotent on its second run. `mise run check`: exit 0

**Next:** the Ruby Magnus `Vec<T>` segfault fix (`normal`, single file `crates/iscc-rb/src/lib.rs`;
take `RArray` + `into_iter()`). Ruby `encode_seq` itself takes `Vec<String>` (lib.rs:329), so it is
one of the affected call sites.

**Notes:**

- WASM docs say `units` "may be an array or any other iterable of strings". They never say it throws
    on non-arrays
- No input-acceptance parity claims. Every page states that base32 input is strict and casing is
    ASCII-only
- The C entries use `uintptr_t` exactly as in `iscc.h` (the next.md table). Older entries on the
    page use `size_t`; that inconsistency predates this step
- I did not mention .NET `EncodeSeq` NUL truncation in the docs, because it is an open `normal`
    issue whose fix is Titusz's call. The C++ CLAUDE.md now notes that `encode_seq` guards NUL
- `.claude/context/iterations.jsonl` was modified by the runner and is left unstaged
