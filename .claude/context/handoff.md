# Handoff

## 2026-07-24 — Review of: Release the GIL for the Python text and video compute paths (issue #41)

**Verdict:** PASS

**Summary:** The advance agent added `py.detach(|| ...)` windows around the pure-Rust compute in the
5 remaining heavyweight iscc-py entry points (`gen_text_code_v0`, `gen_video_code_v0`,
`gen_video_code_v0_flat`, `soft_hash_video_v0`, `soft_hash_video_v0_flat`), mirroring the existing
`gen_image_code_v0` pattern exactly. Detach count moved 7 → 12; for every video path the detach
opens strictly after frame extraction, so the non-free-threading-safe borrowed `PyList_GetItem`
pointers never cross the release. Clean, minimal, in-scope work with 3 new concurrency-correctness
tests; conformance unchanged.

**Verification:**

- [x] `grep -c '\.detach(' crates/iscc-py/src/lib.rs` returns 12 — confirmed (7 pre-existing + 5
    new: text @138, video @190, video_flat @217, soft_hash_video @535, soft_hash_video_flat @241)
- [x] `cargo clippy -p iscc-py -- -D warnings` clean — Finished, no warnings
- [x] `cargo build -p iscc-py` succeeds — built via `maturin develop`
- [x] `maturin develop` + `pytest tests/` — 300 passed (297 pre-existing + 3 new); `test_gil.py -v`
    shows all 10 GIL tests pass
- [x] `mise run format` / `mise run check` — all 15 pre-commit hooks pass, no reformatting

**Issues found:**

- (none) — scope held to `lib.rs` + `test_gil.py` (+ handoff/advance-memory). No core, `.pyi`, or
    `__init__.py` changes; Python-facing signatures unchanged; `gil_used = true` untouched. No API
    break, no gate circumvention. Detach placement is sound: closures capture only owned-Rust-`Vec`
    borrows (`flat`/`frames`) or the immutable `&str`, never Python memory across the release.

**Codex review:** No issues. "The new detach windows wrap only pure-Rust computation after
Python-owned video inputs have been copied, while Python object construction remains attached. The
targeted Rust and concurrency tests pass, and no behavioral regressions were identified." Confirms
the review conclusion.

**Resolved this iteration:** Issue #41 deleted from issues.md; the three "Verified when" boxes in
`specs/python-bindings.md` → "GIL Release for Text/Video Compute Paths" marked `[x]` (human-authored
`[human]` spec issue). The GitHub issue (#41) closes at v0.6.0 release, per the "close on release"
note — a human/release action, not a CID one.

**Next:** The GIL-release theme is complete. Remaining v0.6.0 backlog (all `normal` `[human]`, one
per iteration): **#42 Enable WASM simd128** in the `@iscc/wasm` release build (self-contained
build-flag change spec'd in `specs/wasm-bindings.md` — the recommended next pick), #43 Go ISCC-IDv1
encode/decode, "Restore linux/aarch64 Python wheels", "Dependency review and refresh", plus two CI
release-infra items ("Migrate npm publishing to OIDC", "Fix broken single-registry re-trigger in
release.yml"). #42 is the most self-contained.

**Notes:**

- Meta/audio/mixed paths intentionally stay attached (negligible compute) — do not "complete" them.
- `_flat` variants have no dedicated Python-level concurrency tests: they are `_lowlevel`-only (not
    in the public `iscc_lib` API) and share the identical detach shape + core fn with the tested
    nested-list paths. This is acceptable, not a gap.
- No benchmarking done (correctly out of scope): detach sites are binding-layer only and do not
    touch the benchmarked core hot paths. The pytest-benchmark tables in the suite output are the
    pre-existing comparison benches, not a new gate.
- Verification requires the extension built first
    (`uv run maturin develop -m   crates/iscc-py/Cargo.toml`) before `pytest`/`ty check` see the
    updated `_lowlevel`.
