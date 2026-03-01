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

Rust core complete (32/32 Tier 1 symbols, 310 tests). Binding propagation in progress.

**Key design facts:**

- No Python reference exists — `gen_sum_code_v0` is a Rust-only convenience function
- No conformance vectors — correctness verified by equivalence to the two-pass approach
- SumCodeResult has 3 fields: iscc, datahash, filesize (optional `units` deferred)
- WASM binding needs special design (no path-based I/O in browser) — defer to binding step

**Execution plan:**

1. ✅ Rust core — gen_sum_code_v0 + SumCodeResult (complete)
2. → Python binding — accept str | os.PathLike (current step)
3. Node.js, WASM, C FFI, Java bindings (batch or individual steps)
4. Go binding (pure Go reimplementation — not a Rust wrapper)

**Python binding specifics:**

- PyO3 wrapper accepts `&str` for path (Python `str`), not `&Path` directly
- `os.fspath()` converts `PathLike` in the Python wrapper before calling low-level Rust
- Same dict keys as `gen_instance_code_v0`: `iscc`, `datahash`, `filesize`
- `SumCodeResult(IsccResult)` follows established pattern of typed dict subclass
- Python files at `crates/iscc-py/python/iscc_lib/` (not `python/iscc_lib/` from repo root)

## Binding Propagation Patterns

- **Python** (3 files): `src/lib.rs` (PyO3 wrapper), `__init__.py` (import + class + wrapper +
    __all__), `_lowlevel.pyi` (type stub). Most complex — needs result class + os.fspath. Separate.
- **Node.js + WASM + C FFI** (3 Rust files): mechanical additions. Natural batch.
- **Java + Go** (2 files): idiomatic path types needed. Natural batch.

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
