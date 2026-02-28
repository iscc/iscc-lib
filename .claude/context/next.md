# Next Work Package

## Step: Add Java API reference page

## Goal

Create `docs/java-api.md` documenting the full Java API surface (`IsccLib` class, `IsccDecodeResult`
class, constants, streaming hashers), and wire it into the site navigation and `llms.txt`. This is
the last spec-required Reference page — the documentation spec lists "Java API" alongside Rust API,
Python API, and C FFI in the Reference section.

## Scope

- **Create**: `docs/java-api.md`
- **Modify**: `zensical.toml` (add `{ "Java API" = "java-api.md" }` to Reference nav section)
- **Modify**: `docs/llms.txt` (add Java API reference line)
- **Reference**:
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — authoritative source for
        all 30 Tier 1 symbols (382 lines, 30 methods + 4 constants)
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccDecodeResult.java` — decode result type
        (42 lines, 5 public final fields)
    - `docs/c-ffi-api.md` — pattern to follow for structure and depth (694 lines)
    - `docs/rust-api.md` — pattern to follow for function documentation style
    - `.claude/context/specs/documentation.md` — spec requiring "Java API" in Reference

## Not In Scope

- Javadoc generation or mkdocstrings integration for Java (the page is hand-written markdown like
    the Rust API and C FFI pages)
- Maven Central publishing configuration or setup documentation
- `NativeLoader.java` internals — not part of the public API surface
- Updating the Java how-to guide (`docs/howto/java.md`) — already complete
- Updating other Reference pages (Rust API, Python API, C FFI) — they are already final

## Implementation Notes

**Page structure** — follow the C FFI reference page pattern, adapted for Java:

1. **Front matter**: `icon: lucide/book-open`, description for Java API
2. **Intro paragraph**: Java library for ISCC via JNI, all 30 Tier 1 symbols as static methods on
    `IsccLib`
3. **Installation**: Maven and Gradle dependency snippets (use `io.iscc:iscc-lib:0.0.2`)
4. **Quick example**: `IsccLib.genMetaCodeV0("title", null, null, 64)` → ISCC string
5. **Constants section**: 4 `public static final int` fields (`META_TRIM_NAME`,
    `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) in a table
6. **Classes section**: `IsccDecodeResult` with its 5 public final fields in a table
7. **Functions sections** (organized by category, matching IsccLib.java order):
    - Conformance: `conformanceSelftest()`
    - Code generation: all 9 `gen*V0` methods with signature, parameter table, return type, and
        throws clause
    - Text utilities: `textClean`, `textRemoveNewlines`, `textTrim`, `textCollapse`
    - Encoding: `encodeBase64`, `jsonToDataUrl`
    - Codec: `encodeComponent`, `isccDecode`, `isccDecompose`
    - Sliding window: `slidingWindow`
    - Algorithm primitives: `algSimhash`, `algMinhash256`, `algCdcChunks`, `softHashVideoV0`
    - Streaming hashers: `DataHasher` lifecycle (`dataHasherNew` → `dataHasherUpdate` →
        `dataHasherFinalize` → `dataHasherFree`) and same for `InstanceHasher`
8. **Error handling**: all methods throw `IllegalArgumentException` on invalid input; streaming
    hashers throw after finalize
9. **Memory management note**: streaming hashers use opaque `long` handles — callers MUST call
    `*Free` to release native memory

**Content source**: transcribe directly from the Javadoc in `IsccLib.java` and
`IsccDecodeResult.java`. Do NOT invent undocumented behavior.

**Nav entry**: insert `{ "Java API" = "java-api.md" }` after `{ "C FFI" = "c-ffi-api.md" }` in the
Reference section of `zensical.toml`.

**llms.txt entry**: add `- [Java API](https://lib.iscc.codes/java-api.md): Java API reference` after
the C FFI line, following the existing pattern.

## Verification

- `uv run zensical build` succeeds (site builds with new page)
- `grep -q 'java-api.md' zensical.toml` exits 0 (nav entry present)
- `grep -q 'java-api' docs/llms.txt` exits 0 (llms.txt reference added)
- `grep -c 'genMetaCodeV0\|genTextCodeV0\|genImageCodeV0\|genAudioCodeV0\|genVideoCodeV0\|genMixedCodeV0\|genDataCodeV0\|genInstanceCodeV0\|genIsccCodeV0' docs/java-api.md`
    returns ≥ 9 (all gen functions documented)
- `grep -c 'dataHasherNew\|dataHasherUpdate\|dataHasherFinalize\|dataHasherFree' docs/java-api.md`
    returns ≥ 4 (DataHasher lifecycle documented)
- `grep -q 'IsccDecodeResult' docs/java-api.md` exits 0 (decode result type documented)
- `grep -q 'META_TRIM_NAME' docs/java-api.md` exits 0 (constants documented)
- `mise run check` passes (all hooks clean)

## Done When

All 8 verification commands pass — the Java API reference page is complete, wired into navigation
and llms.txt, and the site builds cleanly.
