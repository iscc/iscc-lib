# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- Doc files are excluded from the 3-file modification limit — can batch all 6 howto guides in one
    step since they follow identical patterns
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- State assessments can go stale — always verify claimed gaps by reading the actual files
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- When previous next.md already contains correct scoping, verify line references are still accurate
    and refresh rather than rewrite from scratch — avoid unnecessary churn
- Repetitive doc additions across language guides: all 6 howto files follow identical structure
    (heading, 1-line description, fenced code block). Safe to batch all in one step

## Signature Change Propagation

- When a Rust core function signature changes, ALL Rust-based binding crates must be updated in the
    SAME step to keep CI green. The 3-file guideline can be exceeded when the additional files are
    mechanical 1-line call-site fixes (e.g., adding `, false` to 4 binding calls)
- WASM binding (`iscc-wasm`) has its OWN inline implementation of `gen_sum_code_v0` using
    `DataHasher`/`InstanceHasher` directly (no filesystem in WASM) — it does NOT call
    `iscc_lib::gen_sum_code_v0`, so signature changes don't break it
- Go binding is pure Go — completely independent of Rust core signatures
- Binding call sites for `gen_sum_code_v0`:
    - `iscc-py/src/lib.rs:335`
    - `iscc-napi/src/lib.rs:232`
    - `iscc-ffi/src/lib.rs:847`
    - `iscc-jni/src/lib.rs:414`
    - `benchmarks.rs:199,214` (test-adjacent, excluded from file count)

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## gen_sum_code_v0 Units Design

- `data_result.iscc` and `instance_result.iscc` are ALREADY computed internally (needed for
    `gen_iscc_code_v0` call). When `add_units=false`, these strings are computed but dropped
- `SumCodeResult` has `#[non_exhaustive]` — adding fields is backward compatible for consumers but
    NOT for constructors (only the crate itself constructs it, so this is fine)
- Issue #21 specifies `Option<Vec<String>>` gated by `add_units: bool` — respect this design even
    though always-populating would be zero-cost (FFI/WASM string marshalling overhead matters)
- Propagation order for units: Rust core → Python → Node.js/WASM/JNI/FFI → Go (each a step)

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md` — both
    are published to npm
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.0.4 released to all registries. Next release after units support is complete.
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax

## Binding Units Exposure Patterns

- Python: `SumCodeResult(IsccResult)` class with type annotations in `__init__.py`. Wrapper calls
    `_gen_sum_code_v0(os.fspath(path), bits, wide, add_units)`. Dict keys omit optional `None`
    fields (pattern: only `set_item` when `Some`)
- Python tests in `tests/test_smoke.py` use `tmp_path` fixture, follow existing
    `test_gen_sum_code_v0_*` pattern
- C FFI: `IsccSumCodeResult` uses `*mut *mut c_char` (NULL-terminated string array) for `units` —
    same representation as `iscc_decompose`/`iscc_sliding_window` return. Reuse
    `vec_to_c_string_array` helper and `iscc_free_string_array` for cleanup
- C FFI: `test_iscc.c` compiles with gcc in CI, must update 3 existing call sites (tests 24-26) when
    signature changes. cbindgen regenerates header; CI checks freshness
- C example `iscc_sum.c` uses streaming hashers, NOT `iscc_gen_sum_code_v0` — no update needed

## Project Status

- Iteration 9: WASM done (iter 8). Now exposing `add_units` in C FFI binding
- Issue #21 progress: Rust core ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI (this step) → JNI → Go
- 2 open issues: #21 (units support, partially done), #16 (feature flags, normal/low)
- v0.0.4 released to all registries
