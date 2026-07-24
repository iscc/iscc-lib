# Handoff

## 2026-07-24 — Release the GIL for the Python text and video compute paths (issue #41)

**Done:** Added `py.detach(|| ...)` windows around the pure-Rust compute in the 5 remaining
heavyweight iscc-py entry points: `gen_text_code_v0`, `gen_video_code_v0`, `gen_video_code_v0_flat`,
`soft_hash_video_v0`, and `soft_hash_video_v0_flat` — mirroring the existing `gen_image_code_v0`
pattern exactly. For all video paths the detach opens strictly AFTER `extract_frame_sigs` /
`flat_bytes_to_frames` completes, so the non-free-threading-safe borrowed `PyList_GetItem` pointers
never cross the release; the closures capture only borrows of owned Rust `Vec`s (`flat`/`frames`) or
the immutable `&str` text borrow. Extended `tests/test_gil.py` with 3 threaded-correctness tests
(text, video code, soft-hash video).

**Files changed:**

- `crates/iscc-py/src/lib.rs`: 5 new `py.detach` sites (detach count 7 → 12); safety comments at the
    video sites explaining why the detach must follow frame extraction. `PyDict`/`PyBytes`
    construction stays attached. No signature changes — the injected `py: Python<'_>` params already
    existed on all 5 functions.
- `tests/test_gil.py`: added `test_gen_text_code_v0_concurrent_matches_single_thread` (~210 KB text
    payload), `test_gen_video_code_v0_concurrent_matches_single_thread` and
    `test_soft_hash_video_v0_concurrent_matches_single_thread` (64 synthetic frames × 380 i32), each
    asserting 16-thread results are byte-identical to the single-threaded result; module docstring
    updated to mention issue #41.

**Verification:**

- `grep -c '\.detach(' crates/iscc-py/src/lib.rs` returns **12** (was 7; +5 as specified).
- `cargo clippy -p iscc-py -- -D warnings` — clean (Finished, no warnings).
- `cargo build -p iscc-py` — succeeds.
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` — built and installed cp310-abi3 wheel.
- `uv run pytest tests/` — **300 passed** (297 pre-existing incl. unchanged conformance in
    `test_conformance.py`, +3 new); `tests/test_gil.py -v` shows all 10 GIL tests pass.
- `mise run format` — no reformatting; tree contains only `lib.rs`, `test_gil.py`, and the
    runner-owned `iterations.jsonl` (left unstaged).
- `mise run check` — all 15 pre-commit hooks pass.

**Next:** Remaining v0.6.0 backlog items, one per iteration: #42 (WASM simd128 in the `@iscc/wasm`
release build — pure build-flag change, spec'd in `specs/wasm-bindings.md`), #43 (Go ISCC-IDv1
encode/decode), #49 (linux/aarch64 Python wheels), dependency refresh. #42 looks like the next most
self-contained pick.

**Notes:**

- Scope held: `gen_meta_code_v0`, `gen_audio_code_v0`, `gen_mixed_code_v0` intentionally left
    attached (negligible compute, per next.md / spec). `gil_used = true` untouched. No Rust core,
    `_lowlevel.pyi`, or `__init__.py` changes — the Python-facing API is unchanged.
- The spec's "Verified when" list (`specs/python-bindings.md` → "GIL Release for Text/Video Compute
    Paths") is now fully satisfied: text detaches, video/soft-hash (incl. `_flat`) detach after
    extraction, conformance unchanged.
- The `_flat` variants have no dedicated Python-level concurrency tests (they are not exposed in the
    public `iscc_lib` API — `_lowlevel` only); they share the identical detach shape and core
    function with the tested nested-list paths. next.md's test spec asked for text + nested-list
    video cases only.
- No benchmarking done, per next.md ("Not In Scope") — correctness under contention is what the
    tests verify; the detach sites are binding-layer only and do not touch the benchmarked core hot
    paths.
