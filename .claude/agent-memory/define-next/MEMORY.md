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
- **Handoff "IDLE" can be stale** — state assessment may create new normal-priority issues AFTER the
    handoff. Always check issues.md directly rather than trusting handoff "IDLE" signals
- **Generated files (tool output) don't count toward the 3-file modification limit** — they're
    created by running a tool, not hand-written

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

## UniFFI Scaffolding (Swift/Kotlin foundation)

- UniFFI v0.31.0 is the latest stable version (checked 2026-03-21)
- Proc macro approach: `#[uniffi::export]`, `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`
    — no UDL files or `build.rs` needed
- `uniffi::setup_scaffolding!()` required at crate root
- Key type constraints: no `usize` (use `u64`), no borrowed types (use owned `String`, `Vec<u8>`),
    no generics on exported functions (use concrete types like `Vec<Vec<i32>>`)
- Constants must be wrapped as getter functions — UniFFI can't export `const` directly
- Streaming types use `Mutex<Option<Inner>>` pattern for interior mutability with `&self` methods
- `crate-type = ["cdylib", "staticlib", "lib"]` needed for both dynamic and static linking
- Swift bindings step sequence: UniFFI crate -> binding generation -> Swift package -> **CI** ->
    docs
- Kotlin depends on Swift (shares UniFFI crate), so Swift must be done first
- **Binding generation**: add `bindgen` feature (`uniffi/cli`) + `[[bin]]` with `required-features`.
    Build cdylib first, then `uniffi-bindgen generate --library libiscc_uniffi.so --language swift`
- **Generated outputs**: `iscc_uniffi.swift`, `iscc_uniffiFFI.h`, `iscc_uniffiFFI.modulemap`

## Swift CI Job Details

- **macOS runner required** — `macos-14` (Apple Silicon, Xcode 15+, Swift 5.9+ pre-installed)
- `cargo build -p iscc-uniffi` produces `libiscc_uniffi.dylib` on macOS (`.so` on Linux)
- SPM needs `-Xlinker -L<path>` to find the dylib at link time
- Runtime discovery: use `-Xlinker -rpath -Xlinker <path>` (cleaner than `DYLD_LIBRARY_PATH` which
    SIP can strip on macOS)
- `Swatinem/rust-cache@v2` works on macOS runners
- After Swift CI: remaining items are docs, README sections, version sync, CLAUDE.md, release

## Dev Environment Constraints

- **No Swift toolchain** in Linux devcontainer — `swift test` can only run on macOS (CI)
- `uniffi-bindgen` not pre-installed — use in-crate binary via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`
- `cargo build -p iscc-uniffi` produces `target/debug/libiscc_uniffi.so` on Linux (verified)

## Conformance Vector Loader Differences (critical for data.json updates)

- **Rust core** (`conformance.rs`): Uses `serde_json::Value`, auto-discovers new vectors.
- **Go** (`conformance.go`): Uses `map[string]map[string]vectorEntry` — parses ALL top-level keys.
    **BREAKS** on non-vector entries like `_metadata`. Must use `json.RawMessage` intermediate step.
- **C# .NET**: `ConformanceTests.cs` uses `System.Text.Json`, loads from vendored
    `testdata/data.json`. Must skip `_metadata` key.
- **C FFI**: No data.json loader (uses Rust core `conformance_selftest`).
- **C++ wrapper**: Same as C FFI — uses `conformance_selftest()` call, no data.json parsing.
- **Swift**: XCTest with `JSONSerialization`, skip `_metadata` key, vendor data.json in Tests
    directory with SPM `.copy()` resource.
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, `packages/dotnet/Iscc.Lib.Tests/testdata/data.json`, and
    `packages/swift/Tests/IsccLibTests/data.json` (all identical). Must be updated together.

## Project Status

- **All 8 existing bindings complete** (Rust, Python, Node.js, WASM, C FFI, Java, Go, Ruby, C#/.NET,
    C++)
- v0.3.1 released, 14 CI jobs green
- **UniFFI scaffolding crate complete** (iteration 1, PASS)
- **Swift SPM package created** (iteration 2, PASS) — CI job is next
- **2 normal-priority issues**: Swift bindings (in progress — CI next), Kotlin bindings (depends on
    Swift)

## Version Sync Script Patterns

- `scripts/version_sync.py` uses `(file_path, get_fn, sync_fn)` triples in TARGETS list
- JSON targets (package.json, vcpkg.json): can reuse
    `_get_package_json_version`/`_sync_package_json`
- Python targets (pyproject.toml): use `^version` anchored regex — won't work for indented lines
- `conanfile.py` has `    version = "0.2.0"` INDENTED — needs non-anchored regex
- `version_sync.py --check` runs in CI (`version-check` job)

## CI/Release Patterns

- v0.3.1 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- 6 smoke test jobs gate 6 publish jobs in release.yml
- FFI tarball staging: Unix uses `cp` + `tar czf`, Windows uses `Copy-Item` + `Compress-Archive`

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- `cbindgen` export prefix `iscc_` on types but not on functions

## Propagation Gotchas

- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)
