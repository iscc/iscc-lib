---
name: iscc-seq
description: ISCC-SEQ (IEP-0020, #45) Rust core implementation facts — raw header helpers, read_unit, conformance runners, known divergences, oracle recipe for binding slices
metadata:
  type: project
---

# ISCC-SEQ (#45)

ISCC-SEQ #45 Rust core DONE (iter 195):
`codec::{read_unit, encode_seq, decode_seq,     SEQ_MAINTYPES}` (seq fns re-exported at crate root;
Tier-1 count sweep 33→35 deferred until all surfaces bind). Built on private RAW header helpers
`decode_header_raw` (returns `tail_start`, `#[inline]`) / `encode_header_raw` (u32s, no typed
gates); typed `decode_header`/`encode_header` are thin wrappers (iai unchanged). `read_unit` uses
`data.get(offset..).unwrap_or(&[])` → out-of-range offset = "Malformed" like ref. Seq conformance
runners in conformance.rs (`decode_bytes` for `bytes:<hex>`). Known divergence: `decode_base32`
rejects non-zero trailing bits (`GAAAAAAAAB`), iscc-core accepts — affects every decode, not
seq-specific. Bindings: test oracle values via
`uv run --no-project --with iscc-core==1.5.0 python -c ...`

Binding slices (next): each surface wraps `iscc_lib::encode_seq(&[&str])` / `decode_seq(&[u8])`;
conformance harnesses must decode `bytes:<hex>` in inputs AND outputs (outputs are a string / list,
not objects). Then the Tier-1 33→35 doc/count sweep (like IDv1 iter 190).

## Surface status

- **Python DONE (iter 197)**: `lib.rs` `encode_seq(Vec<String>) -> PyBytes` (Vec<u8> would be
    list[int]), `decode_seq(&[u8])`; type contract (bare str/bytes → TypeError, iterables → list,
    bytearray/memoryview → bytes) lives in `__init__.py` wrapper. `tests/test_seq.py` has the
    strict-base32 differential vs `iscc_core` (in dev group — import directly). Rebuild `.so` with
    maturin first. Annotate as `Iterable[str]` (ty flags generator vs `Sequence`)
- Spare-bit trap: 64-bit units have NO spare bits; use 32-bit `GAAAAAAAAB` or 256-bit `…GBI`→`…GBJ`.
    `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` itself has dirty spare bits (fails base32 in iscc_lib, not
    MainType) — canonical ISCC-CODE for the MainType arm: `ISCC:KUAALNA7GRZESMRFUURK6SRMTS7WU`
- **Go DONE (iter 202)**: `EncodeSeq`/`DecodeSeq` in `codec.go` (pure Go, own port); unexported
    `readUnit`/`seqMainTypes` on raw helpers `encodeHeaderRaw`/`decodeHeaderRaw` (u32, returns
    `tailStart`); typed `encodeHeader`/`decodeHeader` wrap them. Selftest: separate
    `parseSeqConformanceData` + `runEncodeSeqTests`/`runDecodeSeqTests` (`seqVector` raw outputs).
    Tests `seq_test.go`. GOTCHA: `testing.AllocsPerRun` on an inlinable fn with `_ =` result can
    elide the copy → false 0; assign to a package-level sink var
- **napi + WASM DONE (iter 204)**: napi `encode_seq(Vec<String>) -> Buffer`
    (`Buffer::from(Vec<u8>)`), `decode_seq(Buffer)`; wasm `encode_seq(JsValue)` via
    `serde_wasm_bindgen::from_value` (JS string → `TypeError: Reflect.get…`; napi →
    `code: 'ArrayExpected'`; neither reaches core), `decode_seq(&[u8])`. Tests: napi
    `__tests__/seq.test.mjs` + conformance seq suites; wasm `tests/unit.rs` seq block +
    `conformance.rs` (`decode_bytes`)
- **C FFI DONE (iter 205)**: `iscc_encode_seq(const char*const*, n) -> IsccByteBuffer`,
    `iscc_decode_seq(const u8*, len) -> char**`; reuse existing free fns. Rust tests walk data.json
    via `serde_json` dev-dep + `hex_bytes`; negatives pin message substrings
    (`assert_last_error_contains`). ISCC-CODE
    `ISCC:KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY` reaches the MainType arm. C#
    binding: `iscc_encode_seq(byte** units, nuint)`
- **.NET DONE (iter 206)**: `IsccLib.EncodeSeq(string[])` / `DecodeSeq(ReadOnlySpan<byte>)` in
    `IsccLib.cs` codec block; empty `units` needs a non-null `byte*` stack sentinel (fixed on an
    empty array = NULL → wrong FFI error); `DecodeSeq` needs none (len-0 checked first). Tests:
    `ConformanceTests.cs` (`DecodeBytes`, 3 seq theories incl. byte round trip) + `SeqTests.cs`
- Remaining surfaces: JNI, Ruby, UniFFI, C++. Then 33→35 sweep (incl. `packages/go/CLAUDE.md` stale
    "9 gen"/"46 vectors", `packages/dotnet/CLAUDE.md` "9 gen … 50 vectors")
