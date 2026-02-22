## 2026-02-22 — Implement gen_text_code_v0 with MinHash

**Done:** Implemented `gen_text_code_v0` — the Text Content-Code generator — using MinHash (not
SimHash). Created the `minhash` module with `alg_minhash_256` ported from `bio-codes/iscc-sum`,
added `xxhash-rust` dependency, and implemented `soft_hash_text_v0` which uses the pipeline:
`text_collapse → sliding_window(13) → xxh32 → alg_minhash_256`.

**Files changed:**

- `Cargo.toml`: added `xxhash-rust = { version = "0.8", features = ["xxh32"] }` to workspace deps
- `crates/iscc-lib/Cargo.toml`: added `xxhash-rust.workspace = true` to dependencies
- `crates/iscc-lib/src/minhash.rs`: new module with MPA/MPB constants, `minhash()`,
    `minhash_compress()`, `alg_minhash_256()`, plus 7 unit tests
- `crates/iscc-lib/src/lib.rs`: added `pub(crate) mod minhash`, implemented `soft_hash_text_v0` and
    `gen_text_code_v0`, replaced stub test with 2 unit tests + conformance test

**Verification:** 85 tests pass (was 76; +7 minhash unit tests, +2 gen_text_code_v0 unit tests, -1
removed stub test; the conformance test covers all 5 vectors). `cargo clippy -D warnings` clean.
`cargo fmt --check` clean. No `unsafe` code. All 5 conformance vectors pass (ISCC codes and
character counts verified).

**Next:** `gen_data_code_v0` — also uses MinHash (via the now-available `alg_minhash_256`) plus
content-defined chunking (CDC). The CDC module needs to be ported from `bio-codes/iscc-sum`. This is
the most complex remaining gen function. Alternatively, `gen_audio_code_v0` (SimHash on Chromaprint
features) or `gen_image_code_v0` (DCT-based) are simpler options.

**Notes:**

- The learnings file incorrectly states `gen_text_code_v0` uses SimHash with `sliding_window(3)` and
    BLAKE3. The correct pipeline is MinHash with `sliding_window(13)` and xxh32. The review agent
    should correct this in learnings.md.
- MPA/MPB constants (64 × u64 each) are inlined in `minhash.rs` to stay within the 3-file limit.
    When `gen_data_code_v0` is implemented, these could be factored to a shared `constants.rs` if
    desired, but it's fine as-is.
- The `minhash` function does NOT use rayon (unlike `bio-codes/iscc-sum`) — keeps the core crate
    dependency-light per next.md instructions. Sequential iteration is sufficient.
- `_characters` is computed in `gen_text_code_v0` but not yet returned (no result struct). This
    matches the approach used for `_metahash` in `gen_meta_code_v0`. The conformance test verifies
    character counts independently via `utils::text_collapse`.
