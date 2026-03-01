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

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## gen_sum_code_v0 Implementation Plan (Issue #15)

Target requires 32 Tier 1 symbols. 31 done. gen_sum_code_v0 + SumCodeResult are the final pair.

**Key design facts discovered during scoping:**

- No Python reference exists — `gen_sum_code_v0` is a Rust-only convenience function
- No conformance vectors — correctness verified by equivalence to the two-pass approach
- First function in the crate that introduces file I/O (std::fs::File + std::io::Read)
- IO_READ_SIZE = 4_194_304 (4 MB) constant already exists for buffer sizing
- Composes DataHasher + InstanceHasher in single read loop, then gen_iscc_code_v0
- SumCodeResult has 3 fields: iscc, datahash, filesize (spec also mentions optional `units`)
- WASM binding needs special design (no path-based I/O in browser) — defer to binding step

**Execution plan:**

1. ✅ Issue #18 (META_TRIM_META) — all 6 bindings complete
2. → Issue #15 Rust core — gen_sum_code_v0 + SumCodeResult (iter 5)
3. Issue #15 Python binding — accept str | os.PathLike
4. Issue #15 remaining bindings — Node.js, C FFI, Java, Go (WASM needs design decision)

## Binding Propagation Patterns

Constants propagation to 6 bindings exceeds the 3-file limit if done all at once. Split by binding
complexity:

- **Python** (3 files): `src/lib.rs` (PyO3 m.add), `__init__.py` (import + core_opts + __all__),
    `_lowlevel.pyi` (type stub). Most complex — needs `core_opts` namespace update for iscc-core
    parity. Do separately.
- **Node.js + WASM + C FFI** (3 Rust files): each is a 1-3 line mechanical addition following
    existing patterns. Natural batch.
- **Java + Go** (2 files): Java is Java source (literal value), Go is Go source (literal value).
    Both hardcode values. Natural batch.

## Documentation Status

All 4 spec-required Reference pages complete: Rust API, Python API, C FFI, Java API. 16 docs pages
deployed to lib.iscc.codes. Will need updates for new symbols after implementation.

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- `scripts/version_sync.py` uses only stdlib — can run as `python scripts/version_sync.py --check`
- v0.0.3 released to all registries. Next release after new symbols are complete.

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them
- META_TRIM_META pre-decode formula: `META_TRIM_META * 4/3 + 256` accounts for base64 inflation plus
    media type header
