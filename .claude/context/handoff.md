## 2026-02-22 — Implement ISCC codec module with type enums and encoding primitives

**Done:** Created `crates/iscc-lib/src/codec.rs` with the complete codec foundation: type enums
(MainType, SubType, Version), varnibble variable-length encoding/decoding, ISCC header
encoding/decoding, length encoding/decoding, base32 encoding/decoding (via `data-encoding` crate),
and `encode_component` — the key function that all `gen_*_v0` unit code generators depend on.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: New module with all codec primitives and 33 unit tests
- `crates/iscc-lib/src/lib.rs`: Added `pub mod codec;` declaration
- `crates/iscc-lib/Cargo.toml`: Added `data-encoding.workspace = true` dependency
- `Cargo.toml`: Added `data-encoding = "2"` to workspace.dependencies

**Verification:** All 42 tests pass (33 new codec tests + 9 existing stubs).
`cargo clippy -D warnings` clean. `cargo fmt --check` clean. Key verification criteria met:

- Varnibble roundtrip: tested for boundary values 0, 7, 8, 71, 72, 583, 584, 4679
- `encode_header(0, 0, 0, 1)` produces expected `[0x00, 0x01]` (2 bytes)
- Base32 roundtrip: tested for various byte data, case-insensitive decode verified
- `encode_component` produces output matching iscc-core known vector
    (`gen_meta_code_v0("Hello World")` → body `"AAAWKLHFPV6OPKDG"`)
- `decode_header` roundtrips with `encode_header` for all 8 MainType values
- `encode_length` / `decode_length` roundtrip for standard types

**Next:** Implement the first `gen_*_v0` function — `gen_meta_code_v0` is a good candidate since it
depends only on SimHash (text feature hashing) and `encode_component` (now available).
Alternatively, `gen_instance_code_v0` (BLAKE3 hash) or `gen_data_code_v0` (CDC + MinHash, already in
iscc-sum) could be ported from the prior Rust work in `bio-codes/iscc-sum`. Vendoring `data.json`
conformance vectors should happen alongside the first gen function to enable test-driven
development.

**Notes:** Module visibility was set to `pub` (Tier 2) rather than `pub(crate)` as originally
specified in next.md. This follows `notes/04-api-compatibility-safety.md` which prescribes
`pub mod codec` as Tier 2 (Rust-only utilities), and avoids dead_code warnings that would otherwise
be unavoidable during incremental development when pub(crate) items aren't yet consumed by gen\_\*
functions. The `encode_component` function correctly rejects `MainType::Iscc` (matching iscc-core
behavior — ISCC-CODEs are assembled directly in `gen_iscc_code_v0`, not via `encode_component`).
Varnibble encoding uses a `Vec<bool>` bit buffer approach for clarity; if performance matters later,
this can be replaced with direct bit manipulation on a `u64` accumulator.
