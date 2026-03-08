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

- **Rust core** (`conformance.rs`): Uses `serde_json::Value`, auto-discovers new vectors.
- **Go** (`conformance.go`): Uses `map[string]map[string]vectorEntry` — parses ALL top-level keys.
    **BREAKS** on non-vector entries like `_metadata`. Must use `json.RawMessage` intermediate step.
- **C# .NET**: `ConformanceTests.cs` uses `System.Text.Json`, loads from vendored
    `testdata/data.json`. Must skip `_metadata` key.
- **C FFI**: No data.json loader (uses Rust core `conformance_selftest`).
- **C++ wrapper**: Same as C FFI — uses `conformance_selftest()` call, no data.json parsing.
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, and `packages/dotnet/Iscc.Lib.Tests/testdata/data.json` (all
    identical). Must be updated together.

## Project Status

- **All 8 bindings complete** (Rust, Python, Node.js, WASM, C FFI, Java, Go, Ruby, C#/.NET, C++)
- v0.2.0 released, 14 CI jobs green
- **1 normal-priority issue remains**: Language logos in README/docs — cosmetic
- After logos, only `low`-priority items remain (Swift/Kotlin) — CID loop will go idle

## Version Sync Script Patterns

- `scripts/version_sync.py` uses `(file_path, get_fn, sync_fn)` triples in TARGETS list
- JSON targets (package.json, vcpkg.json): can reuse
    `_get_package_json_version`/`_sync_package_json`
- Python targets (pyproject.toml): use `^version` anchored regex — won't work for indented lines
- `conanfile.py` has `    version = "0.2.0"` INDENTED — needs non-anchored regex
- `version_sync.py --check` runs in CI (`version-check` job)

## CI/Release Patterns

- v0.2.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- 6 smoke test jobs gate 6 publish jobs in release.yml
- FFI tarball staging: Unix uses `cp` + `tar czf`, Windows uses `Copy-Item` + `Compress-Archive`

## FFI Tarball Layout (for vcpkg portfile AND Conan recipe)

- Tarballs named `iscc-ffi-v{VERSION}-{TARGET}.tar.gz` (5 platforms, `.zip` for Windows)
- Flat layout inside `iscc-ffi-v{ver}-{target}/`: `iscc.hpp`, `iscc.h`,
    `libiscc_ffi.so`/`.dylib`/`.dll`, static lib, `LICENSE`
- 5 targets: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, aarch64-apple-darwin,
    x86_64-apple-darwin, x86_64-pc-windows-msvc
- `conanfile.py` is excluded from `ty check` in `pyproject.toml` (conan not a project dep)

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- `cbindgen` export prefix `iscc_` on types but not on functions

## Propagation Gotchas

- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)

## Conan 2.x Pre-Built Binary Pattern

- `source()` has no `self.settings` access — platform-specific downloads must go in `build()`
- Use `conan.tools.files.download` + `conan.tools.files.unzip` (not urllib/zipfile)
- Drop `exports_sources`, `generators`, `compiler`/`build_type` settings for pre-built recipes
- Keep `os` + `arch` settings for platform mapping
- Windows DLLs go in `bin/`, import libs in `lib/`; Unix shared libs go in `lib/`
- No Conan CLI in dev environment — verification via syntax check + grep patterns
