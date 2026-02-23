## 2026-02-23 — Add structured result types for gen functions

**Done:** Created `types.rs` with 9 dedicated result structs (`MetaCodeResult`, `TextCodeResult`,
etc.) matching the iscc-core Python dict returns. Updated all 9 `gen_*_v0` functions in `lib.rs` to
return these structs instead of plain `String`. Updated all 4 binding crates (py, wasm, napi, ffi)
to extract `.iscc` from the structs, maintaining backward compatibility. All conformance tests now
verify additional fields (metahash, name, description, characters, parts, datahash, filesize) from
the structs.

**Files changed:**

- `crates/iscc-lib/src/types.rs`: New file with 9 `#[non_exhaustive]` result structs
- `crates/iscc-lib/src/lib.rs`: All 9 gen functions return structs; `build_meta_data_url` helper
    added for meta JSON→Data-URL conversion; all tests updated to use `.iscc` field access and
    verify additional fields
- `crates/iscc-py/src/lib.rs`: Added `.map(|r| r.iscc)` to all 9 functions (still returns String to
    Python)
- `crates/iscc-wasm/src/lib.rs`: Added `.map(|r| r.iscc)` to all 9 functions
- `crates/iscc-napi/src/lib.rs`: Added `.map(|r| r.iscc)` to all 9 functions
- `crates/iscc-ffi/src/lib.rs`: Added `.map(|r| r.iscc)` before `result_to_c_string` in all 9
    functions

**Verification:**

- `cargo test --workspace`: 163 tests pass (143 core + 20 FFI)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `mise run check`: all 14 pre-commit hooks pass
- `uv run pytest tests/`: 49 Python tests pass (conformance + smoke)
- Conformance tests verify: iscc, metahash, name, description, characters, parts, datahash, filesize
    from struct fields

**Next:** The Python bindings still return plain strings. The next step should implement the
Python-bindings spec from `notes/specs/python-bindings.md` — converting the PyO3 layer to return
`dict` with all fields (iscc, metahash, name, description, characters, parts, datahash, filesize)
matching iscc-core's dict returns. The structured Rust types are now ready to drive this.

**Notes:**

- Added `build_meta_data_url` helper to `lib.rs` for converting JSON meta input to Data-URL format
    in the `MetaCodeResult.meta` field. This matches iscc-core behavior where `gen_meta_code_v0`
    returns a Data-URL in the `meta` field regardless of whether input was JSON or Data-URL.
- All structs are `#[non_exhaustive]` for forward compatibility — new fields can be added without
    breaking downstream.
- Binding crates (py, wasm, napi, ffi) were updated beyond the 3-file scope because the API change
    would break workspace compilation. The changes are mechanical (adding `.iscc` extraction).
- `gen_meta_code_v0` now normalizes description in all code paths (previously only in the text
    path), so `MetaCodeResult.description` is always the normalized version.
