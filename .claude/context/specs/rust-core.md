# Spec: Rust Core Crate — Extended API Surface

The `iscc-lib` Rust core crate exposes a Tier 1 public API that all binding crates (Python, Node.js,
WASM, C FFI) wrap. Beyond the 9 `gen_*_v0` functions, the crate provides text utilities, algorithm
primitives, streaming hashers, codec operations, and a conformance selftest — everything that
`iscc/iscc-sdk` currently imports from `iscc-core`.

Use deepwiki MCP to query `iscc/iscc-core` for exact signatures and behavior of the reference
implementations.

## Structured Return Types

All 9 `gen_*_v0` functions return structured result types (not plain strings). Each type carries the
same fields as the corresponding iscc-core Python dict. See `specs/python-bindings.md` for the
complete field listing per function.

**Verified when:**

- [ ] Each `gen_*_v0` function returns a struct with named fields, not `String`
- [ ] All fields from `specs/python-bindings.md` are present in the corresponding struct
- [ ] Conformance tests verify additional fields (metahash, name, characters, datahash, filesize,
    parts) match iscc-core output for every test vector

## Text Utility Functions

Four text processing functions are public Tier 1 API, callable from all bindings:

| Function               | Signature (Rust)                          | Behavior                                                                                                         |
| ---------------------- | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `text_clean`           | `fn(text: &str) -> String`                | NFKC normalize, remove control chars (except newlines), collapse consecutive empty lines, strip outer whitespace |
| `text_remove_newlines` | `fn(text: &str) -> String`                | Replace all whitespace sequences (including newlines) with single space                                          |
| `text_trim`            | `fn(text: &str, nbytes: usize) -> String` | Truncate to `nbytes` UTF-8 bytes at a valid character boundary, trim whitespace                                  |
| `text_collapse`        | `fn(text: &str) -> String`                | NFD normalize, lowercase, remove whitespace/control/mark/punctuation chars, NFKC recombine                       |

These functions exist internally today as `pub(crate)`. They become `pub` in `lib.rs` (not just the
utils module).

### Unicode data version is part of the conformance contract

`text_clean` and `text_collapse` classify code points by Unicode general category, so their output
depends on the Unicode data version of the underlying tables. That version is therefore part of the
output contract, not an implementation detail: characters assigned after the declared version are
`Unassigned` (category `Cn`), fall inside category `C`, and get stripped — so a table upgrade
silently changes ISCCs for text containing newly assigned characters.

**Severity is bounded — the requirements below are the complete intended response.** Data-Code and
Instance-Code are unaffected (they hash raw bytes, with no Unicode processing), and the units that
are affected — Meta-Code and Text-Code — are similarity-preserving, so a single differing code point
moves the code by a small Hamming distance and similarity matching keeps working; only
exact-identifier equality changes. Newly assigned code points are also not present in text data at
scale. Residual exposure is exact-match lookups on short inputs plus the exact `name` /
`description` fields. Treat work beyond these requirements as scope creep — see `decisions.md`
2026-07-26, "Unicode determinism is a bounded-severity issue".

**Declared Unicode data version: 16.0.0** (decided by Titusz 2026-07-25; matches CPython 3.14's
`unicodedata`). Requirements:

1. **Freeze rule — map unassigned code points to the noncharacter sentinel `U+FFFF` before
    normalization.** A code point unassigned in Unicode 16.0.0 is **replaced by `U+FFFF`**, not
    deleted and not reclassified, using a vendored table of unassigned ranges (731 ranges covering
    819,533 code points, generated from `unicodedata2==16.0.0` by a checked-in generator script).
    The category filter of each function then stays **exactly as the reference defines it** — no
    predicate change and no table lookup at the filter step — because `U+FFFF` is category `Cn` and
    the reference's own category-`C` filter removes it.

    Three properties make the sentinel correct, and each is load-bearing:

    - **Conformance.** The sentinel occupies the replaced character's position through normalization
        and case folding, so composition-blocking and `Final_Sigma` context match `iscc-core`
        exactly. Deleting the character instead (the iteration-133 design) changes adjacency and
        unblocks canonical composition, Hangul jamo composition and `Final_Sigma` — IEP-0003 defines
        conformance as producing the same code as the reference implementation for the same input, so
        that is non-conformance, not a wording problem.
    - **Table-version invariance.** `U+FFFF` is a noncharacter: category `Cn`, `ccc = 0`, and no
        decomposition — **permanently**, under Unicode's Noncharacter stability policy, so it can
        never gain an assignment or a decomposition in any future version. Mapping *into* it
        therefore severs all dependence on what tables a dependency ships. Leaving the original
        character in place and merely reclassifying it does **not**: `U+A7F1` is unassigned in 16.0
        but has a compatibility decomposition to `S` under 17.0, so it dissolves during NFKC before
        any filter sees it and injects a spurious letter (`e U+A7F1 U+0301` → `eŚ`).
    - **No ambiguity.** `U+FFFF` present in the input is itself unassigned in 16.0, so it maps to the
        sentinel and is stripped — which is what the reference does with it.

    Because `Cn ⊂ C`, removal of unassigned code points is already mandated by IEP-0003 Processing
    step 4; the freeze rule fixes *which* code points count as unassigned and preserves *where*
    removal happens. It does not add a step. Characters assigned in ≤ 16.0.0 are stable across
    table versions (normalization by Unicode stability policy; general categories verified
    empirically — zero reclassifications across 15.1 → 16 → 17). Implementations must assert that
    their normalization library passes `U+FFFF` through unchanged; this is required of any
    conformant normalizer, but it is the one assumption the design rests on.

2. **Table dependencies must supply Unicode 16.0.0 or newer data.** Exact pins are not required —
    the freeze rule guarantees the output — so the current `unicode-general-category` 1.1.0
    (16.0.0) and `unicode-normalization` 0.1.25 (17.0.0) both qualify and may be upgraded freely.

3. **Conformance vectors cover the 16.0.0 boundary** in every binding, so the declared version is
    enforced by CI instead of being rediscovered by inspection: a character assigned in 16.0 that
    must be retained (e.g. U+1FAE9), a 16.0 character with a canonical decomposition (e.g.
    U+113C5), and a post-16.0 character the freeze rule must strip (e.g. U+20C1, assigned in
    Unicode 17). The vendored `iscc-core/data.json` vectors all predate Unicode 16 and cannot catch
    this class of drift.

4. **A differential sweep proves exact equivalence to uniform Unicode 16.0.0 tables, over single
    code points *and* sequences.** The sweep compares `text_clean` / `text_collapse` output against
    a reference built on uniform 16.0.0 tables, and the divergence set must be **empty** — the
    sentinel design achieves 0, so any nonzero result is a defect, not a residual to accept. The
    single-code-point denominator is the **1,112,064 Unicode scalar values**, not 1,114,112 code
    points: `text_clean` / `text_collapse` take `&str`, so the 2,048 surrogates `U+D800`–`U+DFFF`
    are unrepresentable in the input and cannot be swept end-to-end (`char::from_u32` rejects
    them). They need no separate check — they cannot reach the freeze rule. The sweep must cover
    sequence classes as well — at minimum base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased and
    Cn-between-marks — because **a per-code-point sweep is not sufficient**: the iteration-133
    pre-filter scored 0 divergences on single code points while failing 42 of 140 sequence cases,
    and a category override scored 1 and 10 respectively. Any change to a Unicode-table dependency
    must re-run the full sweep and report the result; a green vector suite is not sufficient
    evidence of output neutrality.

The reference implementation does not pin a Unicode version yet: `iscc-core` inherits CPython's
`unicodedata`, and since it declares `requires-python = ">=3.9,<4.0"` that spans **four** Unicode
versions — 13.0.0 on 3.9/3.10, 14.0.0 on 3.11, 15.0.0 on 3.12, 15.1.0 on 3.13, 16.0.0 on 3.14, with
deltas of +838 / +4,489 / +627 / +5,185 newly retained code points. `iscc-core` output has therefore
never been deterministic across its own supported range; the 3.13 → 3.14 break is the fifth instance
of an ongoing problem, not a new regression. The same architecture proposed here — a conditional
`unicodedata2==16.0.0` dependency for Python < 3.14 plus the sentinel freeze rule — is proposed
upstream in <https://github.com/iscc/iscc-core/issues/137> and converges the whole supported range
on one behaviour. Per ISO 24138 Annex D the reference implementation is normative, so the
`iscc-core` release adopting it settles the standard's answer.

Exactly two divergence classes are accepted, and no others:

- (a) **Runtimes whose tables predate 16.0.** The freeze rule cannot add knowledge the runtime
    lacks. Applies to `iscc-core` before it adopts `unicodedata2`, and to `packages/go` until go1.27
    (`decisions.md` 2026-07-26). Go's `unicode` package ships Unicode 15.0.0, so `packages/go`
    strips characters *assigned* in 16.0 that must be retained — verified on `U+1FAE9` and
    `U+113C5`, both of which `TextClean` drops. This class covers only the table-dependent boundary
    vectors; it is **not** a licence to skip the `Final_Sigma` sequence vector, whose Go failure has
    a different cause (see below).
- (b) **Characters assigned between 15.1 and 16.0** hash differently than `iscc-core` on CPython ≤
    3.13 produced historically — accepted by decision, not by accident (`decisions.md` 2026-07-26,
    "Declared Unicode version stays 16.0.0"). Lowering the declared version to 15.1.0 to preserve
    that output was considered and rejected: it would strip the 6 living scripts added in 16.0
    (Gurung Khema, Kirat Rai, Todhri, Ol Onal, Garay, Sunuwar), collapsing any document written
    wholly in one of them to `""` so that all such documents collide on one degenerate Text-Code.

Divergence caused by the runtime's tables being *newer* than 16.0 is **not** accepted and must be
zero — that is what requirement 1's sentinel and requirement 4's sweep exist to guarantee. Likewise
adjacency-sensitive divergence on sequences containing unassigned code points is **not** accepted.

**Case mapping must be context-sensitive in every implementation.** The reference lowercases with
Python `str.lower()`, which applies the conditional `Final_Sigma` mapping (`Σ` → `ς` when preceded
by a cased character and not followed by one); Rust `str::to_lowercase()` does the same. A
context-insensitive mapping is a conformance defect in its own right, independent of Unicode
versions, and `packages/go` currently has one: `TextCollapse` uses `strings.ToLower`, so `ΛΟΓΟΣ`
collapses to `λογοσ` instead of `λογος`. This is the freeze rule's only cross-implementation
prerequisite and it is tracked separately in `issues.md`; the `Final_Sigma` boundary vector cannot
pass in Go until it lands, and — unlike divergence class (a) — it affects ordinary Greek text rather
than a Unicode-version edge case.

**Verified when:**

- [ ] `iscc_lib::text_clean("hello\tworld")` returns `"helloworld"`
- [ ] `iscc_lib::text_remove_newlines("hello\nworld")` returns `"hello world"`
- [ ] `iscc_lib::text_trim("hello world", 5)` returns `"hello"`
- [ ] `iscc_lib::text_trim("é", 1)` returns `""` (1-byte truncation splits 2-byte char)
- [ ] `iscc_lib::text_collapse("Hello, World!")` returns `"helloworld"`
- [ ] `iscc_lib::text_collapse("café")` returns `"cafe"`
- [ ] All four functions are accessible from Python bindings as `iscc_lib.text_clean()` etc.
- [ ] All four functions are accessible from Node.js, WASM, and C FFI bindings
- [ ] Code points unassigned in Unicode 16.0.0 are **mapped to `U+FFFF`** before normalization in
    `text_clean` / `text_collapse` — not deleted, not reclassified — via a vendored range table with
    a checked-in generator script, and the category filter is left exactly as the reference defines
    it
- [ ] The normalization library passes `U+FFFF` through unchanged (asserted by a test, since the
    freeze rule depends on it)
- [ ] Sequences containing an unassigned code point match `iscc-core` rather than composing:
    `text_clean("e\u{0378}\u{0301}")` returns `U+0065 U+0301` (not `U+00E9`),
    `text_clean("\u{1100}\u{0378}\u{1161}")` returns `U+1100 U+1161` (not `U+AC00`), and
    `text_collapse("\u{0391}\u{03A3}\u{0378}\u{0392}")` returns `U+03B1 U+03C2 U+03B2` (final sigma,
    not `U+03C3`)
- [ ] A code point that is unassigned in 16.0 but *decomposes* under newer tables does not leak:
    `text_clean("e\u{A7F1}\u{0301}")` returns `U+0065 U+0301`, **not** `U+0065 U+015A` — this is the
    case a category override gets wrong
- [ ] A conformance vector set covering the Unicode 16.0 boundary (retained 16.0 character,
    decomposing 16.0 character, stripped post-16.0 character) is exercised by the Rust test suite
    and by every binding's conformance test, except `packages/go` which skips the two
    table-dependent cases with a tracking note until go1.27 (`decisions.md` 2026-07-26); the
    post-16.0-stripped case needs no skip, since `U+20C1` is unassigned in Go's 15.0 tables too
- [ ] `packages/go` lowercases with `cases.Lower(language.Und)` (already available via its existing
    `golang.org/x/text` dependency) rather than `strings.ToLower`, so `TextCollapse("ΛΟΓΟΣ")`
    returns `"λογος"`; only then is the `Final_Sigma` sequence vector enabled for Go
- [ ] A full-code-space **and sequence-class** differential sweep shows **zero** divergence from a
    uniform-Unicode-16.0.0-tables reference (0 of the 1,112,064 Unicode scalar values — surrogates
    are excluded because `&str` cannot carry them — and 0 of the sequence cases); a
    per-code-point-only sweep does not satisfy this criterion

## Algorithm Primitives

Four algorithm functions are public Tier 1 API, callable from all bindings. These are the building
blocks used by `iscc-sdk` for granular content processing (text chunking, text fingerprinting).

| Function          | Signature (Rust)                                         | Behavior                                                      |
| ----------------- | -------------------------------------------------------- | ------------------------------------------------------------- |
| `sliding_window`  | `fn(text: &str, width: usize) -> Vec<String>`            | Generate overlapping character n-grams of given width         |
| `alg_minhash_256` | `fn(features: &[u32]) -> Vec<u8>`                        | 64-dimensional MinHash compressed to 256-bit (32-byte) digest |
| `alg_cdc_chunks`  | `fn(data: &[u8], utf32: bool, avg_chunk_size: u32) -> …` | Content-defined chunking with gear rolling hash               |
| `alg_simhash`     | `fn(hash_digests: &[Vec<u8>]) -> Vec<u8>`                | Similarity-preserving hash from equal-length digests          |

Note: The bound `alg_cdc_chunks` returns owned data (`Vec<Vec<u8>>`) suitable for FFI. The internal
implementation may use borrowed slices for performance.

Note: The bound `alg_simhash` takes `&[Vec<u8>]` (concrete type) rather than the internal
`&[impl AsRef<[u8]>]` generic, since generics cannot cross FFI boundaries.

**Verified when:**

- [ ] `iscc_lib::sliding_window("abcde", 3)` returns `["abc", "bcd", "cde"]`
- [ ] `iscc_lib::sliding_window("ab", 3)` returns `["ab"]` (input shorter than width)
- [ ] `iscc_lib::alg_minhash_256(&[0, 1, 2, 3])` returns a 32-byte `Vec<u8>`
- [ ] `iscc_lib::alg_cdc_chunks(data, false, 1024)` returns non-empty chunk list for non-empty input
- [ ] `iscc_lib::alg_simhash(&[vec![0u8; 32]])` returns a 32-byte `Vec<u8>`
- [ ] All four functions are accessible from Python bindings
- [ ] All four functions are accessible from Node.js, WASM, and C FFI bindings
- [ ] `alg_minhash_256` output matches iscc-core `alg_minhash_256` for identical input features
- [ ] `alg_cdc_chunks` output matches iscc-core `alg_cdc_chunks` for identical input data

## Soft Hash Functions

One soft hash function is public Tier 1 API. It is used by `iscc-sdk` directly for granular video
processing (generating per-segment simprints), independent of `gen_video_code_v0`.

| Function             | Signature (Rust)                                    | Behavior                                                          |
| -------------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| `soft_hash_video_v0` | `fn(frame_sigs: &[Vec<i32>], bits: u32) -> Vec<u8>` | 256-bit similarity hash from MPEG-7 frame signatures via WTA-Hash |

**Verified when:**

- [ ] `iscc_lib::soft_hash_video_v0(frame_sigs, 256)` returns a 32-byte `Vec<u8>`
- [ ] Output matches iscc-core `soft_hash_video_v0` for identical frame signatures
- [ ] Function is accessible from all bindings

## Encoding Utilities

Two encoding functions are public Tier 1 API.

| Function           | Signature (Rust)            | Behavior                                                                                                                                                    |
| ------------------ | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `encode_base64`    | `fn(data: &[u8]) -> String` | Standard base64 encoding (RFC 4648, no padding)                                                                                                             |
| `json_to_data_url` | `fn(json: &str) -> String`  | Accepts a JSON string, base64-encodes it, returns a `data:application/json;base64,...` data URL. Uses `application/ld+json` when `@context` key is present. |

`json_to_data_url` enables all bindings to convert native dict/object types to data URLs by
serializing to JSON once (language-specific) then delegating encoding to Rust.

**Verified when:**

- [ ] `iscc_lib::encode_base64(&[0, 1, 2, 3])` returns the correct base64 string
- [ ] `iscc_lib::json_to_data_url("{\"key\":\"value\"}")` returns
    `"data:application/json;base64,..."`
- [ ] `iscc_lib::json_to_data_url("{\"@context\":...}")` returns
    `"data:application/ld+json;base64,..."`
- [ ] Output matches iscc-core behavior for identical input
- [ ] Both functions are accessible from all bindings

## Codec Operations

Three codec functions are public Tier 1 API.

### iscc_decompose

Decomposes a composite ISCC-CODE into its constituent ISCC-UNIT strings.

| Function         | Signature (Rust)                                 | Behavior                                                         |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------------------- |
| `iscc_decompose` | `fn(iscc_code: &str) -> IsccResult<Vec<String>>` | Split composite ISCC-CODE into individual ISCC-UNIT code strings |

The input is a composite ISCC-CODE string (with or without `"ISCC:"` prefix). The output is a list
of ISCC-UNIT strings, each with `"ISCC:"` prefix, ordered by MainType (Meta, Semantic, Content,
Data, Instance). For non-composite ISCC-UNITs, returns a single-element list containing the input.

**Verified when:**

- [ ] `iscc_decompose("ISCC:KEC...")` returns a `Vec<String>` with 4 elements (Meta, Content, Data,
    Instance)
- [ ] Each returned element starts with `"ISCC:"`
- [ ] Output matches iscc-core `iscc_decompose` for identical input
- [ ] Single ISCC-UNITs (non-composite) return a single-element list
- [ ] Function is accessible from all bindings

### encode_component

Encodes a raw digest into an ISCC unit string with the correct header. Promoted from Tier 2 to Tier
1 so all bindings get it without custom codec reimplementation. Existing Rust implementation lives
in `crates/iscc-lib/src/codec.rs`.

| Function           | Signature (Rust)                                                                              | Behavior                                              |
| ------------------ | --------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `encode_component` | `fn(mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8]) -> IsccResult<String>` | Construct ISCC unit from header fields and raw digest |

Uses `u8` for enum fields (not Rust enums) so bindings can pass integers directly. Each binding
provides language-idiomatic enum wrappers (Python `IntEnum`, TS `const enum`, etc.) that map to the
same values as `codec::MainType`/`SubType`/`Version`.

Rejects `MainType::Iscc` (composite codes are assembled in `gen_iscc_code_v0`). Returns error if
`digest.len() < bit_length / 8`.

**Verified when:**

- [ ] `encode_component(3, 0, 0, 64, &digest)` returns a valid Data-Code ISCC unit string
- [ ] Output matches iscc-core `encode_component` for identical inputs
- [ ] Returns error for `MainType::Iscc` (mtype=5)
- [ ] Returns error when digest is shorter than `bit_length / 8`
- [ ] Function is accessible from all bindings

### iscc_decode

Decodes an ISCC unit string into its header components and raw digest. Inverse of
`encode_component`. Existing helpers in `codec.rs` (`decode_base32`, `decode_header`,
`decode_length`) provide the building blocks.

| Function      | Signature (Rust)                                               | Behavior                                                                 |
| ------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `iscc_decode` | `fn(iscc_unit: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>` | Decode ISCC unit into (maintype, subtype, version, length_index, digest) |

Strips optional `ISCC:` prefix, base32-decodes, parses header, computes bit_length via
`decode_length()`, returns exactly `bit_length / 8` bytes from the tail as digest. Returns `u8` for
enum fields (same pattern as `encode_component`).

**Verified when:**

- [ ] Round-trip: `iscc_decode(encode_component(mt, st, vs, bits, digest))` recovers original fields
- [ ] Output matches iscc-core `iscc_decode` for identical inputs
- [ ] Handles both `"ISCC:..."` prefixed and bare base32 input
- [ ] Function is accessible from all bindings

## Single-Call ISCC-SUM Generation

One combined code generation function is public Tier 1 API. It generates both Data-Code and
Instance-Code in a single pass with Rust-native file I/O, eliminating Python→Rust boundary crossing
overhead.

GitHub: https://github.com/iscc/iscc-lib/issues/15

| Function          | Signature (Rust)                                                      | Behavior                                                        |
| ----------------- | --------------------------------------------------------------------- | --------------------------------------------------------------- |
| `gen_sum_code_v0` | `fn(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` | Open file in Rust, single-pass Data+Instance, compose ISCC-CODE |

Internally: opens the file, reads with optimal buffer size, feeds both CDC/MinHash (Data-Code) and
BLAKE3 (Instance-Code) from the same read buffer, then composes the final ISCC-CODE using
`gen_iscc_code_v0` logic. Returns in a single call (1 boundary crossing instead of ~46,000 for a 1.5
GB file through Python).

**Result type:**

```rust
pub struct SumCodeResult {
    pub iscc: String,
    pub datahash: String,
    pub filesize: u64,
}
```

**Verified when:**

- [ ] `gen_sum_code_v0(path, 64, false)` produces the same ISCC as separate
    `gen_data_code_v0(data, 64)` + `gen_instance_code_v0(data, 64)` → `gen_iscc_code_v0`
- [ ] `datahash` matches `gen_instance_code_v0` output for the same file
- [ ] `filesize` matches actual file size
- [ ] Function is accessible from all bindings
- [ ] Python binding accepts `str | os.PathLike` for path
- [ ] Handles missing/unreadable files with appropriate error

## Streaming Hashers

Two streaming processor types are public Tier 1 API. They enable processing large files without
loading them entirely into memory. Used by `iscc-sdk` in `code_sum` to generate Data-Code and
Instance-Code from file streams.

| Type             | API Pattern                                                      | Produces                            |
| ---------------- | ---------------------------------------------------------------- | ----------------------------------- |
| `DataHasher`     | `new() -> update(&[u8]) -> finalize(bits) -> DataCodeResult`     | Data-Code + datahash                |
| `InstanceHasher` | `new() -> update(&[u8]) -> finalize(bits) -> InstanceCodeResult` | Instance-Code + datahash + filesize |

Both types accumulate data via repeated `update()` calls, then produce the final ISCC code via
`finalize()`. The result types carry the same fields as the corresponding `gen_*_v0` structured
returns.

`DataHasher` internally accumulates CDC chunk features for MinHash. `InstanceHasher` internally
accumulates a BLAKE3 streaming hash and byte count.

**Verified when:**

- [ ] `DataHasher::new().update(data).finalize(64)` produces the same ISCC as
    `gen_data_code_v0(data, 64)`
- [ ] `InstanceHasher::new().update(data).finalize(64)` produces the same ISCC as
    `gen_instance_code_v0(data, 64)`
- [ ] Splitting data across multiple `update()` calls produces identical output to a single call
- [ ] `InstanceHasher` result includes correct `datahash` and `filesize`
- [ ] Both types are accessible from Python as classes with `update()` and `finalize()` methods
- [ ] Both types accept file-like objects in Python (anything with `.read()`)
- [ ] Both types are accessible from Node.js, WASM, and C FFI bindings

## Algorithm Constants

Five algorithm configuration constants are public Tier 1 API, defined as `pub const` in the Rust
core. These are standardized parameters from ISO 24138 — read-only by design.

| Constant                | Type    | Value     | Description                                        |
| ----------------------- | ------- | --------- | -------------------------------------------------- |
| `META_TRIM_NAME`        | `usize` | 128       | Max UTF-8 byte length for name metadata            |
| `META_TRIM_DESCRIPTION` | `usize` | 4096      | Max UTF-8 byte length for description metadata     |
| `META_TRIM_META`        | `usize` | 128_000   | Max decoded payload size in bytes for meta element |
| `IO_READ_SIZE`          | `usize` | 4_194_304 | Buffer size for streaming file reads (4 MB)        |
| `TEXT_NGRAM_SIZE`       | `usize` | 13        | Character n-gram width for text features           |

GitHub: https://github.com/iscc/iscc-lib/issues/18 (META_TRIM_META) Spec: iscc/iscc-ieps#24
Upstream: iscc/iscc-core#132

`META_TRIM_META` guards `gen_meta_code_v0` against unbounded payload processing. Validation:

1. Pre-decode: reject Data-URL strings exceeding `META_TRIM_META * 4/3 + 256` bytes
2. Post-decode: reject payloads exceeding `META_TRIM_META` bytes

Each binding exposes these as module-level constants and optionally as a `core_opts` namespace
object for iscc-core API parity.

**Verified when:**

- [ ] Constants are `pub const` in the Rust core crate root
- [ ] `iscc_lib::META_TRIM_NAME == 128`
- [ ] `iscc_lib::META_TRIM_DESCRIPTION == 4096`
- [ ] `iscc_lib::META_TRIM_META == 128_000`
- [ ] `iscc_lib::IO_READ_SIZE == 4_194_304`
- [ ] `iscc_lib::TEXT_NGRAM_SIZE == 13`
- [ ] Constants are accessible from all bindings
- [ ] Python binding exposes `core_opts.meta_trim_meta` (SimpleNamespace) for iscc-core parity
- [ ] `gen_meta_code_v0` rejects payloads exceeding `META_TRIM_META` with `IsccError::InvalidInput`
- [ ] Pre-decode fast check rejects obviously oversized Data-URL strings
- [ ] Tests cover boundary cases: at limit (passes), over limit (error)

## Conformance Selftest

One diagnostic function is public Tier 1 API. Runs all vendored conformance vectors and reports
pass/fail.

| Function               | Signature (Rust) | Behavior                                               |
| ---------------------- | ---------------- | ------------------------------------------------------ |
| `conformance_selftest` | `fn() -> bool`   | Run all conformance vectors, return `true` if all pass |

**Verified when:**

- [ ] `iscc_lib::conformance_selftest()` returns `true`
- [ ] Function is accessible from all bindings
- [ ] `iscc_lib.conformance_selftest()` returns `True` in Python

## Complete Tier 1 API Summary

The full public API surface bound in all languages:

| Category    | Functions / Types                                                    |
| ----------- | -------------------------------------------------------------------- |
| Code gen    | `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`,         |
|             | `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`,       |
|             | `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`,      |
|             | `gen_sum_code_v0`                                                    |
| Text utils  | `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`   |
| Algorithms  | `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash` |
| Soft hashes | `soft_hash_video_v0`                                                 |
| Encoding    | `encode_base64`, `json_to_data_url`                                  |
| Codec       | `iscc_decompose`, `encode_component`, `iscc_decode`                  |
| Constants   | `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`,         |
|             | `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`                                    |
| Streaming   | `DataHasher`, `InstanceHasher`                                       |
| Diagnostics | `conformance_selftest`                                               |

Total: 10 gen functions + 4 text utils + 4 algorithm primitives + 1 soft hash + 2 encoding + 3 codec
\+ 5 constants + 2 streaming types + 1 diagnostic = **32 public symbols** (+ the `codec` module
remains Tier 2 Rust-only for header encode/decode internals).

## Quality Gates

- [ ] `cargo test -p iscc-lib` passes (all existing + new tests)
- [ ] `cargo clippy -p iscc-lib -- -D warnings` clean
- [ ] `cargo fmt -p iscc-lib --check` clean
- [ ] No `unsafe` without documented justification
- [ ] Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)
- [ ] All new public functions have doc comments with examples
- [ ] All new public functions have unit tests

## API Stability & Performance Invariants

From v1.0.0 onward the Rust core is a **stability-committed** crate: downstream projects depend on
it in production, so every iteration must preserve backward compatibility and performance unless a
change is explicitly sanctioned by a human and reflected in the version number.

### Backward compatibility

- **Output compatibility** (already enforced): every `gen_*_v0` function must keep producing output
    identical to the conformance vectors and to `iscc-core`. This is the primary contract for
    downstream consumers and is covered by the existing conformance suite.
- **API compatibility** (new gate): the public surface of `iscc-lib` (the 32 Tier 1 symbols + the
    Tier 2 `codec` module) must not change in a backward-incompatible way without a major version
    bump. Enforced by `cargo-semver-checks` in CI against the previously published release.
- **SemVer contract:** the crate follows strict SemVer from v1.0.0 — breaking changes require a
    major bump (2.0.0), additive changes are minor, fixes are patch. The 0.4.0 → 1.0.0 transition is
    the one release allowed to break freely; after it, 1.x is locked.

### Performance parity or improvement

- No change may regress a benchmarked `gen_*_v0` (or hot hashing/CDC/MinHash) path beyond a small
    tolerance (default **10%**) versus the committed baseline. Improvements are always welcome.
- Measured deterministically with `iai-callgrind` (instruction counts under valgrind) so the gate is
    stable on shared CI runners. A committed baseline is the comparison point, refreshed
    deliberately when a regression is accepted or an improvement lands.
- The existing `criterion` benches remain for local wall-clock profiling and human-facing speedup
    numbers; `iai-callgrind` is the CI gate.

**Verified when:**

- [ ] Crate version is >= 1.0.0 and follows strict SemVer
- [ ] `cargo-semver-checks` runs in CI against the last published release and fails on an
    unsanctioned breaking change to the public API
- [x] `iai-callgrind` instruction-count benches exist for the hot `gen_*_v0` paths with a committed
    baseline
- [x] CI fails when any benchmarked path regresses > 10% vs the baseline
- [ ] Conformance vectors still pass (output backward compatibility)
