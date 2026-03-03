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

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md`
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.1.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean

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

## Ruby Bindings (Magnus) — Dev Environment

- Ruby 3.1.2 installed in devcontainer (system Ruby from Debian Bookworm)
- Ruby headers at `/usr/include/ruby-3.1.0` (ruby-dev package)
- `libclang-dev` NOT installed — needed by rb-sys/bindgen. Must add to Dockerfile
- `bundler` available via user-install at `~/.local/share/gem/ruby/3.1.0/bin/bundle`
- `gem install X --user-install` works (system gem dir is read-only)
- Magnus latest stable: check crates.io. 0.7.x and 0.8.x lines exist
- PyO3 bridge (iscc-py) is 648 lines for all 32 symbols — Magnus bridge should be similar size
- New binding crate scaffold: many files to CREATE but only root Cargo.toml to MODIFY (well within
    3-file limit since creates don't count)

## Ruby Bindings — Symbol Implementation Plan

Remaining 7 of 32 Tier 1 symbols (gen batches 1-3 done in iters 5-8). Now by complexity:

1. ~~**Gen functions batch 3** (3 symbols): done in iter 8 → 25/32~~
2. **Algorithm primitives** (5 symbols, array types): `sliding_window`, `alg_simhash`,
    `alg_minhash_256`, `alg_cdc_chunks`, `soft_hash_video_v0` — iter 9 (current)
3. **Streaming types** (2 types): `DataHasher`, `InstanceHasher`

Note: `alg_simhash_from_iscc` is NOT in the 32 Tier 1 symbols — do not include it. Handoff review
agent incorrectly listed it — always verify against target.md's 32-symbol list.

## Ruby Bridge Patterns

- Gen functions use `_` prefix (e.g., `_gen_meta_code_v0`) with Ruby wrapper providing keyword args
- Utility/codec functions exposed directly (no prefix, no Ruby wrapper needed)
- Binary data: Ruby `String` holds bytes; in Magnus use `RString` + `unsafe { .as_slice() }` for
    input (same safety comment pattern as gen_image/data_code_v0)
- `iscc_decode` returns tuple → use Ruby Array in Magnus
- `gen_sum_code_v0` takes a file path String → convert to `std::path::Path` in bridge. Smoke tests
    need `require "tempfile"` to create temp files
- `gen_instance_code_v0` accepts `bits` for API consistency but always produces 256-bit output
- `gen_iscc_code_v0` return type has only `iscc` field (no `units` — that's `gen_sum_code_v0`)
- `gen_sum_code_v0` return: `units` key only present when `add_units: true` (Option\<Vec<String>>)
- Algorithm primitives: exposed directly (no `_` prefix, no Ruby wrapper, no result class). Return
    binary `RString` for byte outputs. `alg_minhash_256` is infallible (no Result). `alg_cdc_chunks`
    returns `RArray` of binary `RString` (must copy slices to owned). `soft_hash_video_v0` reuses
    the same `RArray → Vec<Vec<i32>>` pattern as `gen_video_code_v0`

## Project Status

- Issue #16 fully resolved (iterations 13-15)
- v0.1.0 released to all registries
- WASM CI regression resolved (iter 2)
- 6 open issues: Ruby bindings (normal), C# (low), C++ (low), Swift (low), Kotlin (low), README
    logos (low)
- Ruby scaffold complete (iter 3): 10/32 symbols
- Ruby codec/encoding/diagnostic done (iter 4): 16/32 symbols
- Ruby gen batch 1 done (iter 5): text/image/audio → 19/32
- Ruby gen batch 2 done (iter 6): video/mixed/data → 22/32
- Ruby gen batch 3 done (iter 8): instance/iscc/sum → 25/32
- Current: Ruby algorithm primitives (iter 9): sliding_window/simhash/minhash/cdc/video → 30/32
