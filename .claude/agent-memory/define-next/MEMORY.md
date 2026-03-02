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
4. ✅ WASM binding — WasmSumCodeResult + &[u8] input + 6 tests (complete, 75 total tests)
5. ✅ C FFI binding — IsccSumCodeResult struct + iscc_gen_sum_code_v0 extern "C" (complete, 82 Rust
    tests + 57 C assertions)
6. → Java binding — JNI bridge + SumCodeResult class (current step)
7. Go binding — pure Go reimplementation (not a Rust wrapper)

**Java JNI binding specifics:**

- Existing JNI gen functions return `jstring` (just the `.iscc` field). `gen_sum_code_v0` needs
    `jobject` return (like `isccDecode` at lib.rs line 588) because it returns `iscc`, `datahash`,
    `filesize`
- `SumCodeResult.java`: immutable class with `public final String iscc, datahash; long filesize` —
    follows `IsccDecodeResult.java` pattern
- JNI constructor signature: `"(Ljava/lang/String;Ljava/lang/String;J)V"` for (String, String, long)
- `jboolean` is `u8` in jni crate — compare `wide != 0` for Rust bool
- No `data.json` vectors for gen_sum_code_v0 — tests use temp files and equivalence checks against
    `genDataCodeV0 + genInstanceCodeV0 → genIsccCodeV0`
- 992 lines in lib.rs currently; ~385 lines in IsccLib.java; ~473 lines in tests

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
