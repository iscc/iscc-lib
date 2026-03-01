# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- CI job additions are small, single-file changes that provide high value. Pattern: copy existing
    job structure, swap language-specific setup action and build/test commands
- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- README files are "create" operations — less risky than code changes. Doc files excluded from
    3-file limit
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- State assessments can go stale — always verify claimed gaps by reading the actual files
- When state says "all automatable work complete," cross-check the spec's verification criteria
    against actual files — state assessment may miss spec requirements
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- For constant + validation additions: single-file change to lib.rs is ideal scope (constant
    definition + function modification + tests all live in one file)
- When previous next.md already contains correct scoping, verify line references are still accurate
    and refresh rather than rewrite from scratch — avoid unnecessary churn

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## gen_sum_code_v0 Binding Propagation (Issue #15)

Rust core complete (32/32 Tier 1 symbols, 310 tests). Python binding complete (32/32 Tier 1 symbols,
204 tests).

**Execution plan:**

1. ✅ Rust core — gen_sum_code_v0 + SumCodeResult (complete)
2. ✅ Python binding — accept str | os.PathLike (complete)
3. ✅ Node.js binding — NapiSumCodeResult + napi fn + 6 mocha tests (complete, 132 total tests)
4. → WASM binding — accept Uint8Array bytes (no filesystem in browser) (current step)
5. C FFI binding — iscc_gen_sum_code_v0 + opaque result struct
6. Java binding — JNI bridge + SumCodeResult record
7. Go binding — pure Go reimplementation (not a Rust wrapper)

**WASM binding specifics:**

- WASM has no filesystem — `gen_sum_code_v0` accepts `&[u8]` (Uint8Array) instead of a path
- Compose internally: DataHasher + InstanceHasher → finalize both → gen_iscc_code_v0
- `wasm_bindgen` supports `u64` natively (unlike napi-rs) — use `u64` for `filesize`
- Use `#[wasm_bindgen(getter_with_clone)]` struct (same as `IsccDecodeResult`)
- Tests via `wasm_bindgen_test` in `tests/unit.rs` — currently 70 tests (61 unit + 9 conformance)

## Binding Propagation Patterns

- **Python** (3 files): `src/lib.rs` (PyO3 wrapper), `__init__.py` (import + class + wrapper +
    __all__), `_lowlevel.pyi` (type stub). Most complex — needs result class + os.fspath. Separate.
- **Node.js** (2 files): `src/lib.rs` (napi struct + fn), `__tests__/functions.test.mjs` (tests).
    Moderate — structured result object is new pattern vs. plain string returns.
- **WASM** (1 file): different I/O model (Uint8Array, no filesystem). Separate step.
- **C FFI** (1-2 files): extern "C" + opaque result struct. Separate step.
- **Java** (2 files): JNI bridge + SumCodeResult record. Separate step.
- **Go** (1-2 files): pure Go reimplementation. Separate step.

## Documentation Status

16 docs pages deployed to lib.iscc.codes. Will need gen_sum_code_v0 updates after all bindings.

## CI/Release Patterns

- v0.0.3 released to all registries. Next release after binding propagation complete.
- Release workflow has `workflow_dispatch` with per-registry checkboxes

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them
