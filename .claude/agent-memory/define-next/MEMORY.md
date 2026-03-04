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
- Repetitive doc additions across language guides: all 6 howto files follow identical structure
    (heading, 1-line description, fenced code block). Safe to batch all in one step

## Signature Change Propagation

- When a Rust core function signature changes, ALL Rust-based binding crates must be updated in the
    SAME step to keep CI green
- WASM binding has its OWN inline `gen_sum_code_v0` (no filesystem in WASM)
- Go binding is pure Go — completely independent of Rust core signatures

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Conformance Vector Loader Differences (critical for data.json updates)

- **Rust core** (`conformance.rs`): Uses `serde_json::Value`, accesses sections by name
    (`data["gen_meta_code_v0"]`). Ignores unknown top-level keys. Auto-discovers new vectors.
- **Python** (`test_conformance.py`): Accesses by name (`data[function_name]`). Safe.
- **Node.js** (`conformance.test.mjs`): Accesses `data.gen_meta_code_v0`. Safe.
- **WASM** (`conformance.rs`): Same as Rust core (uses `serde_json::Value`). Safe.
- **Java** (`IsccLibTest.java`): Uses `data.getAsJsonObject("gen_meta_code_v0")`. Safe.
- **Go** (`conformance.go`): Uses `map[string]map[string]vectorEntry` — parses ALL top-level keys.
    **BREAKS** on non-vector entries like `_metadata`. Must use `json.RawMessage` intermediate step.
- **C FFI**: No data.json loader (uses Rust core conformance_selftest).
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary) and
    `packages/go/testdata/data.json` (identical copy). Both must be updated together.

## Feature Flags Design (Issue #16) — RESOLVED

- Issue #16 fully resolved across iterations 13-15 (definitions, selftest, CI matrix)
- `default = ["meta-code"]`, `text-processing` (unicode deps), `meta-code` (implies text-processing)

## API Reference Page Patterns

- Existing API reference page sizes: rust-api.md (377), java-api.md (677), c-ffi-api.md (745)
- All share: YAML front matter → intro → constants → gen functions (with param tables) → utilities →
    codec → algo primitives → streaming → error handling
- `docs/api.md` is the Python API page (different naming convention from others)
- Nav entry in `zensical.toml` under `Reference` section
- Doc pages are a single CREATE + one nav MODIFY — well within 3-file limit

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md`
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.1.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` + Ruby headers to compile — cannot remove `--exclude iscc-rb`
    from Rust CI job without adding those deps. Cleaner to run clippy in the dedicated Ruby job
- `ruby/setup-ruby@v1` supports `working-directory` input for bundler cache (Gemfile location)
- Ruby CI pattern: checkout → rust toolchain (with clippy) → rust-cache → apt libclang-dev →
    setup-ruby → clippy → rake compile → rake test
- RubyGems release: use `oxidize-rb/actions/cross-gem@v1` for cross-platform precompiled gems. Runs
    on ubuntu via Docker (rake-compiler-dock). Secret: `GEM_HOST_API_KEY`. Version check via
    RubyGems API: `https://rubygems.org/api/v1/versions/iscc-lib.json`
- Reordered linting after release (handoff recommendation) — release infrastructure is higher value
    since it unblocks publishing

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- Go `json.Marshal` for float64: uses 'f' format for values >= 1e-6 and < 1e21, otherwise 'e'
    format. 1e20 < 1e21 → outputs "100000000000000000000". May or may not match JCS exactly for edge
    cases. Risk area for test_0017/test_0018.

## Propagation Gotchas

- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated. Iteration 1 missed the WASM binding assertion (fixed in iteration 2).
    Checklist:
    - `crates/iscc-lib/src/lib.rs` — Rust core (updated in iter 1)
    - `crates/iscc-wasm/tests/conformance.rs` — WASM (missed in iter 1, fixed iter 2)
    - Other bindings don't have hardcoded count assertions (they iterate dynamically)

## Ruby Bindings — COMPLETE (archived to MEMORY-archive.md)

- Ruby bindings fully met as of iter 13: all 32 symbols, conformance, CI, docs, linting, release
- Key ref: Magnus 0.7.1, `crates/iscc-rb/`, `docs/ruby-api.md` (781 lines), `docs/howto/ruby.md`
- `alg_simhash_from_iscc` is NOT in the 32 Tier 1 symbols

## Documentation Drift Detection

- **Go quickstart in README was completely stale** (used old WASM/wazero runtime pattern, caught
    iter 14). After major architecture changes (WASM→pure Go), always verify README quickstart
    snippets against actual function signatures
- **GenSumCodeV0 4-arg signature**: Go's `GenSumCodeV0(path, bits, wide, addUnits)` has 4 required
    params. Docs showed 3 args (missing `addUnits`). Always cross-check doc examples against actual
    Go source signatures in `packages/go/code_*.go`
- After fixing doc drift, remaining work is ALL low-priority (C#, C++, Swift, Kotlin bindings). CID
    loop approaches idle state

## Project Status

- All critical and normal priority work complete
- Ruby bindings fully met (iter 13)
- 5 open issues remaining: C# (low), C++ (low), Swift (low), Kotlin (low), CDC edge case (low),
    README logos (low)
- Current (iter 14): Fix stale Go code examples in README + howto guide
- After Go doc fix: CID loop enters idle state (all remaining gaps are low-priority)
