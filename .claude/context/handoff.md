## 2026-02-22 — Implement gen_instance_code_v0 with conformance tests

**Done:** Implemented `gen_instance_code_v0` — the first real `gen_*_v0` function — using BLAKE3
hashing and `encode_component`. Vendored the official `iscc-core/data.json` conformance test vectors
and established the conformance testing pattern for all subsequent function implementations.

**Files changed:**

- `Cargo.toml` (root): added `blake3`, `hex`, `serde`, `serde_json` to workspace.dependencies
- `crates/iscc-lib/Cargo.toml`: added `blake3` dependency, `hex`/`serde`/`serde_json`
    dev-dependencies
- `crates/iscc-lib/src/lib.rs`: implemented `gen_instance_code_v0` (BLAKE3 hash → encode_component →
    "ISCC:" prefix), replaced stub test with real output test and full conformance vector test
- `crates/iscc-lib/tests/data.json`: vendored conformance vectors from iscc-core (66KB, all 9
    function test suites)

**Verification:** 43 tests pass (33 codec + 2 instance code + 8 remaining stubs).
`cargo clippy -D warnings` clean. `cargo fmt --check` clean. No `unsafe` code. All 3
`gen_instance_code_v0` conformance vectors pass (empty/64-bit, zero-byte/128-bit, static/256-bit).

**Next:** Implement `gen_data_code_v0` — requires CDC (content-defined chunking) and MinHash
modules. Prior Rust implementation exists in `bio-codes/iscc-sum` and should be adapted. This is the
most complex remaining function but unblocks a critical path. Alternatively, `gen_meta_code_v0`
(SimHash over text features) could be next if a simpler path is preferred.

**Notes:** The conformance test pattern uses `include_str!` for portability and `serde_json::Value`
for flexible JSON parsing. The `"stream:"` prefix convention in test vectors denotes hex-encoded
byte data. The `hex` crate was added as a dev-dependency for decoding test vector data. The
conformance test infrastructure is reusable — future `gen_*_v0` functions just need to parse their
specific input format from the same `data.json` file.
