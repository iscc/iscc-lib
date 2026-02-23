## 2026-02-23 — Implement encode_base64 Tier 1 function

**Done:** Added `encode_base64` as a public Tier 1 API function in the codec module. It wraps
`data_encoding::BASE64URL_NOPAD.encode()` to produce RFC 4648 §5 base64url strings without padding,
matching iscc-core's `encode_base64` exactly. Added flat re-export in `lib.rs`.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: Added `pub fn encode_base64(data: &[u8]) -> String` after the
    base32 section, plus 4 tests (empty input, known value, roundtrip, no-padding verification)
- `crates/iscc-lib/src/lib.rs`: Added `pub use codec::encode_base64;` re-export

**Verification:** All 197 tests pass (147 unit + 28 algorithm primitives + 22 text utils), including
4 new `encode_base64` tests. Workspace-wide clippy clean. All pre-commit hooks pass. Function is
importable as `iscc_lib::encode_base64`. `encode_base64(&[0, 1, 2, 3])` returns `"AAECAw"` (correct
base64url). `encode_base64(&[])` returns `""`. No padding characters in any output.

**Next:** 19 of 22 Tier 1 symbols are now implemented. The remaining 3 are: `iscc_decompose` (codec
utility that splits an ISCC code into constituent unit codes), `DataHasher`/`InstanceHasher`
(streaming types), and `conformance_selftest`. `iscc_decompose` is the next simplest — it's a codec
function that decodes a composite ISCC-CODE and extracts individual unit codes.

**Notes:** The next.md suggested the known value for `[0,1,2,3]` would be `AAECAT` — this is
incorrect. The actual base64url encoding is `AAECAw`. The test uses the correct value verified
against the `data_encoding` crate behavior, which matches Python's
`base64.urlsafe_b64encode(bytes([0,1,2,3])).decode().rstrip("=")`.
