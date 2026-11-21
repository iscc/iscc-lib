## 2026-11-21 — Review of: Python video frame values out of `i32` range raise `OverflowError` on every platform

**Verdict:** PASS

**Summary:** `extract_frame_sigs` in `crates/iscc-py/src/lib.rs` reads each frame value with
`PyLong_AsLongLong` and maps a failed `i32::try_from` to `OverflowError`, so `gen_video_code_v0` and
`soft_hash_video_v0` raise the same class whatever the width of the C `long`. The diff is 1 code
file, 1 docs sentence and 1 test file, exactly the scope of next.md. The issue "Python video frame
values raise a platform-dependent exception class" is resolved and deleted.

**Verification:**

- [x] `uv run pytest tests/test_int_range.py` passes — 16 passed after
    `uv run maturin develop -m crates/iscc-py/Cargo.toml`; the video cases cover both functions
- [x] `uv run pytest` passes — 794 passed
- [x] `grep -n "PyLong_AsLong(" crates/iscc-py/src/lib.rs` prints nothing
- [x] `grep -n "PyOverflowError" crates/iscc-py/src/lib.rs` shows the mapping — line 93, inside
    `extract_frame_sigs`, plus the import
- [x] `cargo clippy -p iscc-py --all-targets -- -D warnings` clean
- [x] `mise run check` exits 0, no file modified

One extra probe on the rebuilt extension, both entry points: `2**31`, `2**63 - 1` and a bad value in
the last position of a second frame raise `OverflowError` with the binding's message; `-(2**63) - 1`
raises CPython's own `OverflowError`; a tuple of tuples takes the same path; a `float` or `None`
stays `TypeError`; empty and ragged input stay `ValueError`.

**Issues found:**

- Filed `low`, untriaged: "No CI job runs a test suite on Windows". CI is 21 Linux jobs and one
    macOS job, and the release workflow runs the Python wheel on Linux only. This step's Windows
    behaviour is therefore proven from the type (`c_longlong` is 64 bits on every platform), not
    from a run
- Not filed (wording nit): the how-to sentence "Python raises it while converting the argument" is
    loose for frame values, where the binding raises it. The statement a caller relies on, that the
    error comes before the library runs, holds

**Codex review:** no actionable findings. Codex rebuilt the binding and ran its own integer-boundary
probes. It reported unrelated Git-fixture test failures that passed once it disabled inherited
commit signing; those are from its environment, the suite is green here.

**Next:** the Go `GOARCH=386` CI step (issue "No CI job runs the Go port as a 32-bit build", decided
`normal`, one `ci.yml` step). Other small candidates: the Ruby floor move to 3.3, or the stale spec
and `CLAUDE.md` statements (audit-scope step, up to 8 files).

**Notes:**

- `decisions.md` was 422 lines. The runner's `archive_overflow` rotated the two oldest entries (both
    of 2026-11-18, on `encode_seq` and ASCII-only upper-casing) into `decisions-archive.md`; the
    file is at 398 lines. Both stay binding
- `state.md` still lists the video exception class as open; update-state will drop it
- No decisions.md entry from this review: the 2026-11-21 decision on out-of-range integers already
    fixes `OverflowError` as the Python class
- Binding-only change: no CRAP, iai or semver impact; no Tier 1 signature changed
