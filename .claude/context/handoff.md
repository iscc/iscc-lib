## 2026-02-23 — Review of: Add alg_simhash, alg_minhash_256, and alg_cdc_chunks to Python bindings

**Verdict:** PASS

**Summary:** Clean implementation of 3 algorithm primitive PyO3 bindings (`alg_simhash`,
`alg_minhash_256`, `alg_cdc_chunks`) bringing Python from 17/23 to 20/23 Tier 1 symbols. All 116
Python tests pass (18 new), 230 Rust tests pass, clippy clean workspace-wide, all 14 pre-commit
hooks pass. Cross-validation against iscc-core confirms byte-for-byte correctness for all three
functions.

**Issues found:**

- (none)

**Next:** Remaining 3 Tier 1 symbols: `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`. The
streaming types (`DataHasher`, `InstanceHasher`) require PyO3 `#[pyclass]` wrappers with
`update()`/`finalize()` methods — a step up in complexity from the thin `#[pyfunction]` wrappers
done so far. `soft_hash_video_v0` is a pure function and simpler to bind. Alternatively, expand
horizontally to other binding crates (Node.js, WASM, C FFI) for the algo primitives just added.

**Notes:** State.md needs updating to reflect 116 Python tests (was 98) and 20/23 Tier 1 symbols
(was 17/23). The algo primitive binding pattern is now fully established: functions that don't
return `Result` and don't need `IsccResult` wrapping are direct passthrough re-exports in
`__init__.py` (unlike `gen_*_v0` functions that get `IsccResult` wrappers).
