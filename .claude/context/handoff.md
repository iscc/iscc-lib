## 2026-02-22 — Bootstrap Rust workspace with core crate skeleton

**Done:** Created the virtual Cargo workspace with centralized dependency management and the
`iscc-lib` core crate containing all 9 `gen_*_v0` stub functions. Each stub returns
`Err(IsccError::NotImplemented)` and has a corresponding smoke test. All verification checks pass
cleanly.

**Files changed:**

- `Cargo.toml`: Virtual workspace root with `[workspace.package]`, `[workspace.dependencies]`
    (thiserror), and `[profile.release]` settings
- `crates/iscc-lib/Cargo.toml`: Core library crate inheriting workspace metadata, with thiserror
    dependency
- `crates/iscc-lib/src/lib.rs`: `IsccError` enum, `IsccResult<T>` type alias, 9 public `gen_*_v0`
    stub functions with docstrings, and 9 smoke tests

**Verification:** All 5 criteria pass:

- `cargo check --workspace` — exits 0
- `cargo test -p iscc-lib` — 9 tests pass, 0 failures
- `cargo clippy -p iscc-lib -- -D warnings` — no warnings
- `cargo fmt --check` — properly formatted
- No `unsafe` blocks in any file

**Next:** Implement the ISCC codec module (base32/base58 encoding, ISCC header construction). This
is a prerequisite for all `gen_*_v0` functions since they return ISCC strings. The conformance
vectors from `iscc-core/data.json` should be vendored to enable test-driven implementation.

**Notes:** Function signatures were adjusted from the next.md approximations based on deepwiki
queries of `iscc/iscc-core`:

- `gen_meta_code_v0`: removed `extra` parameter (not in Python reference); kept
    `name, description, meta, bits`
- `gen_audio_code_v0`: changed parameter from `&[f32]` to `&[u32]` — Chromaprint vectors are integer
    sequences
- `gen_instance_code_v0`: added `bits: u32` parameter (Python reference accepts it)
- `gen_iscc_code_v0`: changed to `(codes, wide: bool)` matching Python's `wide=False` parameter
