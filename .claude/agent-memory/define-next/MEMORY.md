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

## gen_sum_code_v0 — Status: all 7 bindings DONE, docs sweep pending

Issue #15 fully resolved. All 7 bindings complete. Current step: documentation sweep to add
gen_sum_code_v0 references to all READMEs, docs pages, and code comments.

**Files needing "9 → 10" comment updates** (found by grep):

- `crates/iscc-ffi/src/lib.rs` line 3 (module docstring)
- `crates/iscc-lib/src/conformance.rs` line 17
- `crates/iscc-lib/benches/benchmarks.rs` line 1
- `crates/iscc-wasm/tests/unit.rs` line 4
- `crates/iscc-wasm/tests/conformance.rs` line 1

**READMEs needing gen_sum_code_v0 table row** (9 files):

- `README.md` (Implementors Guide list, line ~203-213)
- 7 per-crate READMEs (each has Code Generators table with 9 rows)
- `crates/iscc-wasm/pkg/README.md` (exact copy of wasm README, must stay synced)

**Docs pages referencing gen functions** (12 files found by grep):

- `docs/c-ffi-api.md` — needs new section for `iscc_gen_sum_code_v0` (returns struct, not string)
- `docs/benchmarks.md` — "the 9" text (don't add benchmark data, none exists yet)
- 10 other docs files — check each for function lists vs. example-only mentions

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md` — both
    are published to npm
- `docs/c-ffi-api.md` documents C FFI with full function signatures. `gen_sum_code_v0` returns a
    struct (`IsccSumCodeResult`) unlike all other gen functions which return strings — document the
    struct and its free function
- When updating "9 gen functions" to "10", search broadly — comments in test files, benchmark files,
    and docstrings all reference the count

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
