# Next Work Package

## Step: Tier-1 33→35 doc/count sweep + `encode_seq`/`decode_seq` doc entries

## Goal

Bring the documentation in line with the 35-symbol Tier 1 surface now that ISCC-SEQ is bound on all
11 surfaces (iter 210): every Tier-1 count reads 35, and the hand-written API pages and all 11
how-to guides document `encode_seq`/`decode_seq`. Docs-only, no behaviour change. Follows the iter
190 IDv1 sweep shape.

## Alternatives Considered

- **Chosen:** the doc sweep. It is the last ISCC-SEQ gap in the target ("available everywhere"), and
    today no doc page mentions seq at all (every howto/API page: 0 hits for `encode_?seq`).
- **Rejected:** the Ruby Magnus `Vec<T>` segfault fix (`normal`, single file). It is a real crash
    but not seq-specific. It goes next, once the seq feature set is complete and documented.

## Scope

**Fan-out:** one identical change (Tier-1 count → 35, plus a seq entry/example in each surface's own
casing), applied across the doc surfaces below. All files are docs, notes, or comments, so they sit
outside the 3-file code budget.

- **Modify (count 33 → 35):** `crates/iscc-lib/CLAUDE.md:31` and `crates/iscc-wasm/CLAUDE.md:142`
    (both head symbol lists, so add `encode_seq`/`decode_seq` to the list too),
    `docs/java-api.md:8`, `docs/ruby-api.md:9`, `notes/00-overview.md:153`,
    `notes/04-api-compatibility-safety.md:21` (also extend its per-category breakdown: 5 codec
    operations), `.claude/agents/advance.md:142`, and the module doc at
    `crates/iscc-uniffi/src/lib.rs:3`. That last edit is comment-only: the generated Swift/Kotlin
    bindings do not carry it, so no regen.
- **Modify (seq API entries):** `docs/rust-api.md` (Codec Functions, ~L396), `docs/java-api.md`
    (Codec, ~L410), `docs/ruby-api.md` (Encoding & Codec, ~L440), `docs/c-ffi-api.md` (Codec
    Operations, after `iscc_decompose`).
- **Modify (seq howto example, in each page's codec section):** all 11 pages in `docs/howto/`.
- **Modify (agent/registry docs, one line each where codec ops are already listed):**
    `crates/iscc-ffi/CLAUDE.md:65`, `crates/iscc-jni/CLAUDE.md`, `packages/cpp/CLAUDE.md`, and the
    READMEs that already list codec ops (`crates/iscc-rb`, `packages/{cpp,dotnet,go,kotlin}`).
    **Stale counts (re-derive each from source):** `crates/iscc-napi/CLAUDE.md:11` (codec function
    count), `crates/iscc-py/CLAUDE.md:28` (27 pyfunctions), `packages/go/CLAUDE.md:30,116`,
    `packages/dotnet/CLAUDE.md:37,94`, `packages/{swift,kotlin}/CLAUDE.md:29,57` (test methods are
    now 19), and the `crates/iscc-rb/CLAUDE.md` Key Files table (add `test_iscc_id.rb`,
    `test_seq.rb`).
- **Reference:** `docs/howto/go.md:393-417` (IDv1 subsection, template for a codec subsection);
    `crates/iscc-lib/tests/data.json` sections `encode_seq`/`decode_seq`.

## Not In Scope

- `docs/c-ffi-api.md` type-name prefixing (open `normal` issue: rename vs alias is a design choice).
    New `iscc_encode_seq`/`iscc_decode_seq` entries follow the page's current `IsccByteBuffer`
    style.
- The Python `encode_seq` docstring issue (source files) and `docs/api.md` (generated via
    `::: iscc_lib`, which picks seq up from `__all__`). Do not hand-edit either.
- Any source/signature/test change beyond the uniffi module-doc comment. No new docs page: that
    would require four-place nav parity.
- `target.md`, `specs/`, `learnings.md`, `state.md`, archives. Leave them alone.
- READMEs that list only gen functions (e.g. `crates/iscc-jni`, `packages/swift`) need no seq line.

## Implementation Notes

Real signatures (from source, not memory). Copy them exactly:

| Surface | encode                                                                         | decode                                                                      |
| ------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| Rust    | `encode_seq(units: &[&str]) -> IsccResult<Vec<u8>>`                            | `decode_seq(data: &[u8]) -> IsccResult<Vec<String>>`                        |
| Python  | `encode_seq(units: Iterable[str]) -> bytes`                                    | `decode_seq(data: bytes-like) -> list[str]`                                 |
| napi    | `encode_seq(units: Array<string>): Buffer`                                     | `decode_seq(data: Buffer): Array<string>`                                   |
| WASM    | `encode_seq(units)` → `Uint8Array`                                             | `decode_seq(data: Uint8Array): string[]`                                    |
| Java    | `byte[] IsccLib.encodeSeq(String[])`                                           | `String[] IsccLib.decodeSeq(byte[])`                                        |
| Go      | `EncodeSeq(units []string) ([]byte, error)`                                    | `DecodeSeq(data []byte) ([]string, error)`                                  |
| .NET    | `byte[] IsccLib.EncodeSeq(string[])`                                           | `string[] IsccLib.DecodeSeq(ReadOnlySpan<byte>)`                            |
| C       | `iscc_encode_seq(const char *const *units, uintptr_t num_units)` → byte buffer | `char **iscc_decode_seq(const uint8_t *data, uintptr_t data_len)`           |
| C++     | `iscc::encode_seq(const std::vector<std::string>&) -> std::vector<uint8_t>`    | `iscc::decode_seq(const std::vector<uint8_t>&) -> std::vector<std::string>` |
| Ruby    | `IsccLib.encode_seq(Array)` → binary `String`                                  | `IsccLib.decode_seq(String)` → `Array`                                      |
| Swift   | `encodeSeq(units: [String]) throws -> Data`                                    | `decodeSeq(data: Data) throws -> [String]`                                  |
| Kotlin  | `encodeSeq(units: List<String>): ByteArray`                                    | `decodeSeq(data: ByteArray): List<String>`                                  |

- C frees (verified in `iscc.h`): byte buffer → `iscc_free_byte_buffer`, string array →
    `iscc_free_string_array`.
- Example vector (`test_0002_single_unit`): unit
    `ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI` ↔ 34 bytes, hex starts
    `2007a88248c5`. Show the decoded units carrying the `ISCC:` prefix.
- State the round trip as BYTES: `encode_seq(decode_seq(data)) == data`.
- Traps:
    - napi and WASM export snake_case names.
    - WASM `units` accepts any JS iterable, so never write "throws on non-array" for WASM.
    - Do not claim input-acceptance parity with iscc-core. Base32 input is strict (no padding, zero
        spare bits) and upper-casing is ASCII-only.
    - Never blanket-replace "9" with "10": the 9 conformance gen fns differ from the 10 gen fns.
        Re-derive every non-33 count from source (`grep -c` the real file), or drop the number.
- Run `mise run format` before committing (mdformat `--wrap 100`).

## Verification

- The stale-count grep prints nothing:
    ```
    grep -rnE "\b33 (Tier 1|symbols|public symbols)|all 33 Tier|Tier 1 \(33" docs notes crates/*/CLAUDE.md crates/*/README.md packages/*/CLAUDE.md packages/*/README.md README.md crates/iscc-uniffi/src/lib.rs .claude/agents/advance.md
    ```
- Every page documents seq. This command prints no filenames:
    ```
    grep -LiE "encode_?seq" docs/howto/*.md docs/rust-api.md docs/java-api.md docs/ruby-api.md docs/c-ffi-api.md crates/iscc-{lib,wasm,ffi,jni}/CLAUDE.md packages/cpp/CLAUDE.md crates/iscc-rb/README.md packages/{cpp,dotnet,go,kotlin}/README.md
    ```
- Every seq signature in the docs matches the Implementation Notes table. This is a manual review,
    with no fabricated return types.
- `uv run zensical build` exits 0 and reports "No issues found".
- `mise run check` passes, and `cargo build -p iscc-uniffi` still succeeds after the comment edit.

## Done When

Every Tier-1 count reads 35, all 11 how-to pages and the 4 hand-written API pages document
`encode_seq`/`decode_seq` with signatures matching the table, and the verification commands pass.
