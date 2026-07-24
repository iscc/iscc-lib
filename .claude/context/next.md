# Next Work Package

## Step: Release the GIL for the Python text and video compute paths (issue #41)

## Goal

Wrap the pure-Rust compute of `gen_text_code_v0`, `gen_video_code_v0`, and `soft_hash_video_v0`
(including their `*_flat` variants) in `py.detach(|| ...)` so CPU-bound text/video hashing overlaps
across Python threads instead of serializing on the GIL — completing the GIL-release pass started in
0.5.0 (#39), which intentionally excluded these two heavyweight paths. Output bytes are unchanged,
so conformance is unaffected. This is the first, most self-contained v0.6.0 backlog item.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` — add a `py.detach(|| ...)` window around the pure-Rust
    compute in these 5 functions:
    1. `gen_text_code_v0` (currently line ~136) — detach around
        `iscc_lib::gen_text_code_v0(text, bits)`.
    2. `gen_video_code_v0` (line ~178) — detach around
        `iscc_lib::gen_video_code_v0(&frame_slices, bits)`, **after** `extract_frame_sigs` returns.
    3. `gen_video_code_v0_flat` (line ~200) — detach around the
        `iscc_lib::gen_video_code_v0(&frame_refs, bits)` compute, after `flat_bytes_to_frames`.
    4. `soft_hash_video_v0` (line ~513) — detach around
        `iscc_lib::soft_hash_video_v0(&frame_slices, bits)`, after `extract_frame_sigs`.
    5. `soft_hash_video_v0_flat` (line ~221) — detach around the compute, after
        `flat_bytes_to_frames`.
- **Reference**:
    - `.claude/context/specs/python-bindings.md` → "GIL Release for Text/Video Compute Paths"
        (verified-when list).
    - `.claude/context/issues.md` → "Release GIL for text/video binding compute paths" (#41).
    - Existing detach pattern already applied to `gen_image_code_v0` (line ~151), `gen_data_code_v0`
        (~294), `gen_instance_code_v0` (~308), `gen_sum_code_v0` (~345), and the three `update()`
        methods (~554/603/654) — mirror it exactly.
    - `tests/test_gil.py` — existing concurrency-correctness pattern to extend.

## Not In Scope

- Do **not** add `py.detach` to `gen_meta_code_v0`, `gen_audio_code_v0`, or `gen_mixed_code_v0` —
    their compute is negligible (short strings / small vectors) and they stay attached by design.
- Do **not** move any Python-object extraction inside a detach window. `extract_frame_sigs` uses raw
    borrowed `PyList_GetItem` pointers that are **not** free-threading-safe — the detach must open
    strictly after all frame extraction (and after `flat_bytes_to_frames`) completes, wrapping only
    the pure-Rust `Vec<&[i32]>` / `&[u8]` compute.
- Do **not** flip the module's `gil_used = true` flag to `false` — that requires a separate FFI
    free-threading audit.
- Do **not** touch the Rust core crate, `_lowlevel.pyi`, `__init__.py`, or any other binding — the
    Python-facing signature is unchanged (the injected `py: Python<'_>` param is not exposed) and
    the core API is untouched.
- Do **not** attempt to benchmark or CI-gate throughput — the speedup is memory-bandwidth dependent
    and intentionally not gated (see #39). Correctness under concurrency is what we verify.

## Implementation Notes

- Follow the `gen_image_code_v0` shape exactly: build the pure-Rust inputs first, then
    `let r = py.detach(|| iscc_lib::gen_...(&inputs, bits)).map_err(|e| PyValueError::new_err(e.to_string()))?;`
    and construct the `PyDict` / `PyBytes` afterward (dict/bytes construction must stay attached).
- For the video functions the borrowed slices (`frame_slices: Vec<&[i32]>` /
    `frame_refs: Vec<&[i32]>`) point into owned Rust `Vec`s (`flat`, `frames`), not Python memory,
    so they satisfy PyO3's `Ungil`/`Send` bounds across the release — the same reasoning that makes
    the existing `&[u8]` data detach sound. For `gen_text_code_v0`, `text: &str` borrows an
    immutable Python `str`, which is sound to hold across a detach exactly like the immutable
    `bytes` borrow in `gen_data_code_v0`.
- The soft-hash variants return `Vec<u8>`; keep the `PyBytes::new(py, &result)` call outside the
    detach closure (it needs the GIL).
- Extend `tests/test_gil.py` with threaded-correctness tests mirroring the existing ones: a
    `gen_text_code_v0` case over a large text payload, and a `gen_video_code_v0` /
    `soft_hash_video_v0` case over a synthetic nested frame-signature list (e.g. a few dozen frames
    of a fixed-length `i32` row). Assert every threaded result equals the single-threaded result.
    Synthetic inputs are fine — these tests check output-consistency under contention, not
    conformance.

## Verification

- `grep -c '\.detach(' crates/iscc-py/src/lib.rs` returns `12` (was 7; +5 new sites: text, video,
    video_flat, soft_hash_video, soft_hash_video_flat).
- `cargo clippy -p iscc-py -- -D warnings` is clean.
- `cargo build -p iscc-py` succeeds.
- `maturin develop -m crates/iscc-py/Cargo.toml` builds, then `pytest tests/` passes (all existing
    tests + the new text/video concurrency tests; conformance in `tests/test_conformance.py` green
    and unchanged).
- `mise run format` leaves the tree clean (no reformatting diff on commit).

## Done When

`gen_text_code_v0`, `gen_video_code_v0`/`_flat`, and `soft_hash_video_v0`/`_flat` each release the
GIL around their pure-Rust compute (detach opening only after frame extraction), the detach count is
12, and the full Python test suite — including new text/video concurrency-correctness tests and
unchanged conformance vectors — passes clean.
