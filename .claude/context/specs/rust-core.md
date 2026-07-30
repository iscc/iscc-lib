# Spec: Rust Core Crate — Extended API Surface

The `iscc-lib` Rust core crate exposes a Tier 1 public API that all 11 binding surfaces wrap. Beyond
the 10 `gen_*_v0` functions, the crate provides text utilities, algorithm primitives, streaming
hashers, codec operations, and a conformance selftest — everything that `iscc/iscc-sdk` currently
imports from `iscc-core`.

Use deepwiki MCP to query `iscc/iscc-core` for exact signatures and behavior of the reference
implementations.

## Structured Return Types

All 10 `gen_*_v0` functions return structured result types (not plain strings). Each type carries
the same fields as the corresponding iscc-core Python dict. See `specs/python-bindings.md` for the
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

### Unicode handling follows the reference

`text_clean` and `text_collapse` apply the same processing steps as the reference implementation
(`iscc-core`), in the same order, with no additional measures for compatibility across Unicode
versions. Unicode data (normalization, general categories, case mapping) comes from the tables the
toolchain and dependencies ship — exactly as `iscc-core` inherits whatever `unicodedata` its CPython
interpreter ships. Output for characters whose classification changes between Unicode versions may
therefore vary with dependency/toolchain versions, matching the reference's own behavior across
CPython versions (its `requires-python` range spans Unicode 13.0 through 16.0).

**Decided by Titusz 2026-07-30:** the Unicode 16.0.0 pin built for v0.6.0 (unassigned-to-`U+FFFF`
freeze rule, frozen `Final_Sigma` tables, boundary vectors in every binding, differential sweep
gate) was removed and is postponed until upstream decides how to handle Unicode version drift
(<https://github.com/iscc/iscc-core/issues/137>). Do not reintroduce freeze tables, sentinels,
boundary fixtures, or sweep gates without a new human decision.

Severity of the drift is bounded: Data-Code and Instance-Code hash raw bytes (no Unicode
processing), and Meta-Code / Text-Code are similarity-preserving, so a differing code point moves
the code by a small Hamming distance; only exact-identifier equality is affected. See `decisions.md`
2026-07-26 ("Unicode determinism is a bounded-severity issue").

**Case mapping must be context-sensitive in every implementation.** The reference lowercases with
Python `str.lower()`, which applies the conditional `Final_Sigma` mapping (`Σ` → `ς` when preceded
by a cased character and not followed by one); Rust `str::to_lowercase()` does the same. A
context-insensitive mapping is a conformance defect in its own right, independent of Unicode
versions: it affects ordinary Greek text rather than a Unicode-version edge case. Every surface
implements the conditional mapping today — `packages/go`'s `TextCollapse` collapses `ΛΟΓΟΣ` to
`λογος` (not `λογοσ`), regression-tested in `packages/go/utils_test.go`.

**Verified when:**

- [ ] `iscc_lib::text_clean("hello\tworld")` returns `"helloworld"`
- [ ] `iscc_lib::text_remove_newlines("hello\nworld")` returns `"hello world"`
- [ ] `iscc_lib::text_trim("hello world", 5)` returns `"hello"`
- [ ] `iscc_lib::text_trim("é", 1)` returns `""` (1-byte truncation splits 2-byte char)
- [ ] `iscc_lib::text_collapse("Hello, World!")` returns `"helloworld"`
- [ ] `iscc_lib::text_collapse("café")` returns `"cafe"`
- [ ] All four functions are accessible from Python bindings as `iscc_lib.text_clean()` etc.
- [ ] All four functions are accessible from Node.js, WASM, and C FFI bindings
- [ ] `text_collapse` applies the conditional `Final_Sigma` mapping: `text_collapse("ΛΟΓΟΣ")`
    returns `"λογος"` (not `"λογοσ"`) in the Rust core and in `packages/go`

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
of **bare** base32 ISCC-UNIT strings **without** the `"ISCC:"` prefix, ordered by MainType (Meta,
Semantic, Content, Data, Instance). For non-composite ISCC-UNITs, returns a single-element list
containing the cleaned input.

**Verified when:**

- [ ] `iscc_decompose("ISCC:KEC...")` returns a `Vec<String>` with 4 elements (Meta, Content, Data,
    Instance)
- [ ] Each returned element is a bare base32 ISCC-UNIT string **without** the `"ISCC:"` prefix,
    matching iscc-core (which returns e.g. `["AAAYPXW445FTYNJ3", "EAARMJLTQCUWAND2", …]`)
- [ ] Output matches iscc-core `iscc_decompose` for identical input, including the input forms
    `iscc_clean` accepts — dash-separated, surrounding whitespace, and a lowercase `iscc:` scheme
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

## ISCC-IDv1 Operations (Experimental)

GitHub: https://github.com/iscc/iscc-lib/issues/43

**ISCC-IDv1** is the timestamp-based identifier issued by `iscc-hub`, specified in iscc-core's
`iscc_core/iscc_id.py`. It is **not part of ISO 24138** and is still evolving, so everything in this
section is marked experimental in doc comments (may change in a minor release) and is explicitly
**excluded from the v1.0.0 stability lock** described under "API Stability & Performance Invariants"
until the upstream format settles. Upstream has already moved the format once in a patch release
(the default realm changed in iscc-core 1.2.2).

**Structure** — 80-bit code = 16-bit header + 64-bit body; 16 base32 chars, optional `ISCC:` prefix:

- Header nibbles: MainType = 6 (`ID`), SubType = realm (0 = test, 1 = operational), Version = 1,
    Length = 0. Header bytes are fixed: `0x6010` (realm 0), `0x6110` (realm 1)
- Body (big-endian `u64`): `timestamp = body >> 12` (52-bit microseconds since the UNIX epoch),
    `hub_id = body & 0xFFF` (12-bit issuing-hub slot, 0–4095)

### Part 1 — the generic codec must accept Version 1 (conformance)

**This is the load-bearing half of the work, and it adds no new symbols.** In the reference,
`decode_header` performs *no* version validation at all — it decodes four varnibbles and returns
them — so `iscc_core.iscc_decode` is fully generic and handles ISCC-IDv1 today with no dedicated
function:

```text
iscc_core.iscc_decode("ISCC:MAIGHFECJMOPMIAB")
  -> (6, 0, 1, 0, b'c\x94\x82K\x1c\xf6 \x01')   # (ID, realm 0, V1, len 0, 64-bit body)
```

Our `codec::Version` enum has only `V0`, and `Version::try_from` rejects everything else, so **every
iscc-lib surface currently fails** on any ISCC-IDv1 — verified live against the built Python wheel:

```text
iscc_lib.iscc_decode("ISCC:MAIGHFECJMOPMIAB")     -> ValueError: invalid input: invalid Version: 1
iscc_lib.iscc_decompose("ISCC:MAIGHFECJMOPMIAB")  -> ValueError: invalid input: invalid Version: 1
```

Restoring parity on the already-shipped generic `iscc_decode` / `iscc_decompose` is a prerequisite
for Part 2 and is independently valuable: it is what makes iscc-lib a drop-in for `iscc-core` on
ID-bearing input. It can land as its own work package ahead of any new function.

### Part 2 — one new symbol: `gen_iscc_id_v1`

| Function         | Signature (Rust)                                                         | Behavior                                     |
| ---------------- | ------------------------------------------------------------------------ | -------------------------------------------- |
| `gen_iscc_id_v1` | `fn(timestamp: u64, hub_id: u16, realm: u8) -> IsccResult<IsccIdResult>` | Mint an ISCC-IDv1 from an explicit timestamp |

Minting is the only operation with no generic equivalent — `encode_component` takes a *digest*, not
`(timestamp, hub_id)`, so the 52/12 bit packing has to live somewhere. The result type mirrors the
reference's `dict(iscc=...)` exactly, like every other `gen_*` result in the crate:

```rust
/// Result of `gen_iscc_id_v1` — mirrors iscc-core's `{"iscc": ...}`.
pub struct IsccIdResult {
    pub iscc: String, // "ISCC:MAIGHFECJMOPMIAB"
}
```

**There is deliberately no `decode_iscc_id_v1`** (decided by Titusz, 2026-07-28). `iscc-core` has no
IDv1 decoder — `iscc_decode` is generic and covers it once Part 1 lands — so a dedicated decoder
would be an iscc-lib invention: permanent surface on 11 bindings, with no upstream to track as the
format evolves, in place of three lines of arithmetic. Callers decode like this:

```text
mt, st, vs, ln, body = iscc_decode("ISCC:MAIGHFECJMOPMIAB")   # (6, 0, 1, 0, 8 bytes)
n         = int.from_bytes(body, "big")
realm     = st            # 0 = test, 1 = operational
timestamp = n >> 12       # 1751831876325218
hub_id    = n & 0xFFF     # 1
```

Each binding's how-to page carries this recipe in its own language. `docs/howto/nodejs.md` and
`docs/howto/wasm.md` must show the `BigInt` form — the packed body exceeds `Number`'s safe range
even though the extracted timestamp does not (see "Integer widths across bindings"). The Go
package's `DecodeIsccID` and its `IsccIDv1Result` type are **deleted**, not renamed.

**Naming and parameter order follow the iscc-core reference**
(`gen_iscc_id_v1(timestamp, hub_id, realm_id)`), consistent with the `gen_<thing>_v<N>` grammar
every other Tier 1 constructor uses. Each binding applies its own casing transform: `GenIsccIDV1` in
Go, `genIsccIdV1` in Kotlin/Swift (UniFFI camel-cases), `gen_iscc_id_v1` in Python, Ruby **and both
JS surfaces** — napi and WASM deliberately export snake_case
(`#[napi(js_name = "gen_meta_code_v0")]` at `crates/iscc-napi/src/lib.rs:97`), so a camelCase name
here would be the only one on those surfaces — and `iscc_gen_iscc_id_v1` in the C FFI.

**The core is clock-free: `timestamp` is a required argument.** The reference defaults it to
`time.time_ns() // 1000`, but this crate has no time dependency and must not acquire one — the
release profile sets `panic = "abort"`, `wasm32-unknown-unknown` is a CI target with no guaranteed
wall clock, and there is binding precedent: iscc-core's clock-based `gen_flake_code_v0` /
`uid_flake_v0` were deliberately never ported. A "now" convenience, if ever wanted, belongs in a
binding wrapper, not in the core or behind a cargo feature (a feature that changes *which functions
exist* would make the "N Tier 1 symbols on every surface" criterion false by construction).

### Validation

Validation **order** is normative on every surface; the exact error **text** binds only the Rust
core: `timestamp >= 2^52` → `"Timestamp overflow"`; `hub_id >= 2^12` → `"HUB-ID overflow"`; `realm`
not in `(0, 1)` → `"Realm-ID must be 0 (test) or 1 (operational)"`. First failing check wins. Rust's
unsigned types make the reference's unchecked-negative path (which raises `OverflowError`, not
`ValueError`) unreachable.

Bindings keep their established error idiom and prefix rather than copying the string verbatim —
Go's existing `"iscc: timestamp overflow: %d (must be < 2^52)"` is correct and its
`strings.HasPrefix(err.Error(), "iscc:")` assertion (`packages/go/iscc_id_test.go:160`) stays green;
napi/WASM raise a JS `Error`, JNI throws, the C FFI fills its error-string out-param. Only the
*which check fires first* ordering is asserted cross-surface.

### Codec changes this requires

- `codec::Version` gains `V1 = 1` **and** `#[non_exhaustive]` in the same change. The enum is not
    currently `#[non_exhaustive]`, so adding a variant is a SemVer-major break; the 0.x window is
    the last free opportunity to add the attribute before the v1.0.0 lock makes every future version
    variant cost a major bump.
- `decode_header`, `encode_component` and `encode_header` accept Version 1 **only** when MainType is
    `Id`; every other MainType keeps rejecting Version > 0.
- This changes the observable behaviour of the already-shipped Tier 1 `iscc_decode` on all 11
    surfaces: inputs that previously failed with `invalid Version: 1` now succeed. The change is
    additive-permissive (no signature change), and it is *intended* — `iscc/iscc-monitor` maintains
    a tripwire test (ADR-0011) that flips red exactly when a binding starts accepting Version 1, as
    the signal to delete its interim in-repo codec port.

### ISCC-IDv0 is out of scope — permanently

Legacy **ISCC-IDv0** (wallet-address + SimHash derived, variable-length body with a uvarint
uniqueness counter) is **deprecated, was never part of ISO 24138, and iscc-lib owes it no backward
compatibility** (Titusz, 2026-07-28). Do not port `gen_iscc_id_v0`, `iscc_id_incr`, or
`alg_simhash_from_iscc_id`, and do not treat any IDv0 behaviour as a conformance obligation. No
IDv0-specific rejection path is needed either: generic `iscc_decode` already decodes an IDv0 header
(MainType `ID`, Version 0) and matches the reference numerically, and nothing consumes it further.

The driver is the opposite direction: **ISCC-IDv1 is live**. `iscc-hub` is running a production
network, so realm 1 (operational) is a real, in-use input, not a hypothetical — round-trip coverage
for realm 1 is mandatory, not a boundary-case nicety.

**Realm travels through `SubType` as a raw nibble.** Rust's unified `SubType` has no realm variants,
so `SubType::try_from(1)` yields `SubType::Image`. This is cosmetic, not a bug: only the nibble
value is written (`encode_header` writes `stype as u32`) and only the nibble value is read back
(`iscc_decode` returns `st as u8`), so realm 0 and realm 1 both round-trip correctly and match the
reference's numeric output exactly. Do **not** add realm variants to `SubType` — that would be a
SemVer-major change to a shared enum for a naming nicety.

### Integer widths across bindings

`gen_iscc_id_v1` takes the timestamp as `u64`. The 52-bit value maxes at `2^52 - 1` =
4,503,599,627,370,495, below `Number.MAX_SAFE_INTEGER` (`2^53 - 1`), so JavaScript `Number`
represents every valid timestamp losslessly — the napi and WASM bindings take a plain `number`, no
`BigInt` parameter.

Decoding is the asymmetric direction. `iscc_decode` returns the packed 64-bit body as **bytes**, and
`2^64 - 1` is not safe as a double, so the JS recipe must go through `BigInt` before narrowing:

```js
const [mt, st, vs, ln, body] = iscc_decode("ISCC:MAIGHFECJMOPMIAB");
const n = body.reduce((a, b) => (a << 8n) | BigInt(b), 0n);
const timestamp = Number(n >> 12n); // safe: < 2^53 - 1
const hubId = Number(n & 0xfffn);
const realm = st;
```

No binding exposes the packed `u64` as a number. Returning bytes and letting the caller widen is
what keeps the generic decode path portable across all 11 surfaces.

### Where the tests live

**The differential test is Python-only.** The `iscc-core` oracle is reachable only from Python, so
every byte-identity assertion runs under `pytest` against the built `iscc_lib` wheel, not from
`cargo test`. Model the file on `tests/test_iscc_decode_conformance.py` — module-scope inputs,
`_reference()` / `_subject()` helpers returning comparable tuples, and a parity loop that asserts
both sides raise on the same inputs. `cargo test` covers round-trip and validation ordering only.

**No new conformance fixture.** The single vector `ISCC:MAIGHFECJMOPMIAB` → (realm 0, hub_id 1,
timestamp 1751831876325218) is pinned inline in each surface's own test, alongside the round-trip
parameter space (realm 0/1 × hub_id 0/4095 × timestamp 0/`2^52 - 1`). The 2026-07-26 and 2026-07-27
fixture decisions do not apply — they govern a multi-vector table that only a fixture can carry; one
16-character constant does not warrant a canonical JSON file plus vendored copies plus three edits
to `tests/test_vendored_fixtures.py` (`FIXTURE_BASENAMES`, the `tracked_fixture_paths()` pathspec,
and `VENDORED_COPIES` — miss any one and the copies drift unpoliced).

### Regenerated bindings are part of their step, not follow-ups

- **Swift + Kotlin are one work package**, not four: edit `crates/iscc-uniffi/src/lib.rs`, then
    regenerate the three checked-in artifacts
    (`packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`,
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h`) in the same commit.
- **C FFI**: after adding the `extern "C"` symbols, regenerate `crates/iscc-ffi/include/iscc.h` with
    cbindgen and commit it. No local hook checks this — only the CI `c-ffi` job does (regenerate +
    `git diff --exit-code`), so a stale header passes `mise run check` and reds CI an iteration
    later. `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` is rewritten by `crates/iscc-ffi/build.rs`
    on every `cargo build -p iscc-ffi` and must be staged too.

**Verified when:**

- [ ] Tier 1 `iscc_decode` and `iscc_decompose` accept MainType `ID` with Version 1 on every one of
    the 11 surfaces, and still reject Version > 0 for every other MainType
- [ ] `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` returns `(6, 0, 1, 0, <8 bytes>)`, matching
    `iscc_core.iscc_decode` exactly
- [ ] `gen_iscc_id_v1(1751831876325218, 1, 0)` returns `ISCC:MAIGHFECJMOPMIAB`
- [ ] Decoding that code through generic `iscc_decode` and the documented bit-math recipe recovers
    realm 0, hub_id 1, timestamp 1751831876325218 — with and without the `ISCC:` prefix, on every
    surface, using the recipe as written in that surface's how-to page
- [ ] Round-trip `gen_iscc_id_v1` → `iscc_decode` holds across the parameter space: realm 0 and 1,
    hub_id 0 and 4095, timestamp 0 and `2^52 - 1`
- [ ] No `decode_iscc_id_v1` exists on any surface, and `packages/go` no longer exports
    `DecodeIsccID` or `IsccIDv1Result`
- [ ] Output is byte-identical to `iscc_core.gen_iscc_id_v1` for every case above, asserted by a
    pytest differential test against the installed `iscc-core` dev dependency
- [ ] The three validation checks fire in reference order on every surface (message text asserted
    for the Rust core only)
- [ ] `crates/iscc-lib/src/codec.rs` declares `#[non_exhaustive]` on `enum Version` in the same
    change that adds `V1 = 1`
- [ ] The advance handoff carries an `**API-BREAK:**` note recording that `#[non_exhaustive]` is
    itself a major break (`cargo-semver-checks` reports `enum_marked_non_exhaustive`), taken
    deliberately inside the 0.x window. The tool is not installed locally and the CI `semver` job is
    `continue-on-error: true`, so this is a written record, not a gate result.
- [ ] Both functions are accessible from all 11 bindings with idiomatic names and types

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
| ISCC-IDv1   | `gen_iscc_id_v1` (experimental)                                      |
| Constants   | `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`,         |
|             | `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`                                    |
| Streaming   | `DataHasher`, `InstanceHasher`                                       |
| Diagnostics | `conformance_selftest`                                               |

Total: 10 gen functions + 4 text utils + 4 algorithm primitives + 1 soft hash + 2 encoding + 3 codec
\+ 1 ISCC-IDv1 + 5 constants + 2 streaming types + 1 diagnostic = **33 public symbols** (+ the
`codec` module remains Tier 2 Rust-only for header encode/decode internals).

`gen_iscc_id_v1` is Tier 1 for binding-surface purposes (every language exposes it) but is **exempt
from the v1.0.0 stability lock** below while the upstream format is still evolving.

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
- **API compatibility** (new gate): the public surface of `iscc-lib` (the 33 Tier 1 symbols + the
    Tier 2 `codec` module) must not change in a backward-incompatible way without a major version
    bump. Enforced by `cargo-semver-checks` in CI against the previously published release. The two
    experimental ISCC-IDv1 symbols are carved out of this lock until the upstream format settles —
    the carve-out is documented here and in their doc comments, not implied.
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
