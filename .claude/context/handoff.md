## 2026-02-22 — Implement gen_image_code_v0 with DCT-based perceptual hash

**Done:** Implemented `gen_image_code_v0` with a new `dct.rs` module containing the Nayuki fast
recursive DCT algorithm, plus `soft_hash_image_v0` and supporting helpers (transpose, flatten,
median, bits-to-bytes) in `lib.rs`. All 3 conformance vectors pass.

**Files changed:**

- `crates/iscc-lib/src/dct.rs`: New module — fast recursive DCT (Nayuki algorithm) operating on f64
    values, matching the Cython-compiled reference exactly
- `crates/iscc-lib/src/lib.rs`: Added `pub(crate) mod dct`, implemented `soft_hash_image_v0` with 2D
    DCT pipeline and median-based bit extraction, replaced `gen_image_code_v0` stub with working
    implementation, added 4 conformance/unit tests and 7 DCT unit tests

**Verification:** `cargo test -p iscc-lib`: 128 tests pass (up from 117).
`cargo clippy -p iscc-lib -- -D warnings`: clean. `cargo fmt -p iscc-lib --check`: clean. No
`unsafe` code.

**Next:** Only 1 stub remains: `gen_video_code_v0`. After that, remaining work includes
`gen_meta_code_v0` meta object support (3 skipped vectors), Python/Node/WASM/FFI bindings, CI/CD,
benchmarks.

**Notes:**

- The next.md pseudocode had two inaccuracies vs the actual Python reference:
    1. DCT algorithm: next.md described naive O(n²) DCT-II with orthonormal normalization; the actual
        `iscc-core/dct.py` uses a fast recursive Nayuki algorithm (divide-and-conquer, no
        normalization, returns raw f64). This matters because the Nayuki algorithm gives exact 0.0
        for uniform inputs (via exact integer subtraction), while naive DCT accumulates
        floating-point noise that flips bits.
    2. Block offsets: next.md described positions `(col*8, row*8)` for 8×8 extraction, but the actual
        Python uses raw offsets `(col, row)` where slices are `((0,0), (1,0), (0,1), (1,1))` — blocks
        offset by 1, not 8, creating heavily overlapping regions.
- The `alg_dct` function accepts only even-length or single-element inputs (power-of-2 in practice,
    since image rows are 32 pixels). This matches the Python reference's validation.
- The DCT module uses f64 throughout, matching the Cython-compiled reference. The Python source
    shows `int(round())` but the compiled Cython version skips this and returns raw floats.
