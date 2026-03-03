<!-- assessed-at: 47f237d800d66cbbeb4aaba66b1a784888a406d3 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings — 19/32 symbols; 4 new binding targets added (C#, C++, Swift, Kotlin)

The Ruby Magnus bridge advanced from 16 to 19 of 32 Tier 1 symbols, adding `gen_text_code_v0`,
`gen_image_code_v0`, and `gen_audio_code_v0` with matching result classes in the pure Ruby wrapper.
Target scope expanded significantly: C# / .NET, C++, Swift, and Kotlin bindings were added as new
target sections in `target.md` along with corresponding issues. None of these new targets have any
code started. All 11 CI jobs remain green.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `data.json` updated to iscc-core v1.3.0 (50 total vectors; 4 new: test_0017–test_0020) ✅
- Rust conformance assertion updated: `assert_eq!(tested, 20, ...)` in `lib.rs` ✅
- 314 tests pass with default features ✅
- Feature matrix CI (5 steps) passed in the latest green run ✅

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅
- Python bindings use data.json by section name — 4 new vectors exercised without code changes ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅
- Node.js tests use data.json by section name — 4 new vectors exercised without code changes ✅

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]` ✅
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20` ✅
- `WASM (wasm-pack test)` = SUCCESS in CI run 22637279696 ✅

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass (per last green CI run) ✅
- C FFI tests use data.json by section name — new vectors exercised without code changes ✅
- `cbindgen` header freshness check in CI passed ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- 65 Maven tests pass (per last green CI run) ✅
- Java tests use data.json by section name — new vectors exercised without code changes ✅

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go ✅
- `parseConformanceData()` helper skips `_metadata` key in data.json ✅
- `packages/go/testdata/data.json` updated to iscc-core v1.3.0 (50 vectors) ✅
- 155 Go tests pass; `go vet` clean ✅; CGO_ENABLED=0 confirmed ✅

## Ruby Bindings

**Status**: partially met

- `crates/iscc-rb/` scaffold with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
- **19 of 32 Tier 1 symbols** exposed:
    - Gen functions: `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`
        (4)
    - Text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse` (4)
    - Codec/diagnostic: `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`,
        `json_to_data_url`, `conformance_selftest` (6)
    - Constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
        `TEXT_NGRAM_SIZE` (5)
- Pure Ruby wrapper: `TextCodeResult`, `ImageCodeResult`, `AudioCodeResult < Result < Hash` pattern,
    keyword args ✅
- 25 Minitest smoke tests; `bundle exec rake compile` builds in release profile ✅
- `crates/iscc-rb/README.md` exists (stub) ✅
- **Missing**: 13 remaining symbols: 6 gen functions (`gen_video_code_v0`, `gen_mixed_code_v0`,
    `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`), 4 algorithm
    primitives (`sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`), 1 utility
    (`alg_simhash_from_iscc`), 2 streaming types (`DataHasher`, `InstanceHasher`)
- **Missing**: No conformance tests against `data.json`
- **Missing**: No dedicated `ruby` CI job (workspace Rust job uses `--exclude iscc-rb`)
- **Missing**: No RubyGems step in `release.yml`
- **Missing**: `scripts/version_sync.py` does not include gemspec
- **Missing**: Standard Ruby linting not configured (no `standard` gem in Gemfile, no
    `.standard.yml`, not wired into pre-commit hooks or CI)

## C# / .NET Bindings

**Status**: not started

- Target defined in `target.md`: P/Invoke bindings via `csbindgen`, `packages/dotnet/` directory,
    NuGet package `Iscc.Lib` targeting .NET 8+
- Issue filed in `issues.md` with full implementation scope ✅
- **No code exists**: `packages/dotnet/` does not exist; no `csbindgen` integration; no CI job; no
    devcontainer .NET SDK

## C++ Bindings

**Status**: not started

- Target defined in `target.md`: header-only wrapper `iscc.hpp`, `packages/cpp/` directory,
    distribution via vcpkg/Conan/FFI release tarballs
- Issue filed in `issues.md` with full implementation scope ✅
- **No code exists**: `packages/cpp/` does not exist; no `iscc.hpp`; no vcpkg/Conan manifests

## Swift Bindings

**Status**: not started

- Target defined in `target.md`: UniFFI-generated Swift bindings, `packages/swift/` directory, SPM
    package, `crates/iscc-uniffi/` scaffolding crate
- Issue filed in `issues.md` with full implementation scope ✅
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist; no
    UniFFI integration

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`: UniFFI-generated KMP bindings, `packages/kotlin/` directory, shares
    `crates/iscc-uniffi/` with Swift
- Issue filed in `issues.md` with full implementation scope; depends on Swift ✅
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (also not
    started)

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- **Gap**: Ruby install instructions and quickstart not present
- **Gap**: C#, C++, Swift, Kotlin sections not present (target now requires all 4)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages (including `crates/iscc-rb/README.md`) ✅
- **Gap**: `crates/iscc-rb/README.md` is a stub (31 lines); full installation/usage instructions not
    yet complete
- **Gap**: Target now requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 7 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; no Ruby tabs in multi-language
    examples
- **Gap**: Target now requires C#, C++, Swift, Kotlin how-to guides and tabs (none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22637279696: all 11 jobs SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22637279696
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- `iscc-rb` excluded from workspace Rust CI job (`--exclude iscc-rb`) — no dedicated Ruby CI job
- **Gap**: No `ruby` CI job; no `rubygems` publish step; Ruby not in `version_sync.py`
- **Gap**: Target now requires CI jobs for C#, C++, Swift, Kotlin (none started)

## Next Milestone

Continue advancing the Ruby bindings toward 32/32 symbols — this is the most immediate bottleneck
before the new binding targets (C#, C++, Swift, Kotlin) can be tackled. Suggested next work package:

1. **`gen_video_code_v0`**, **`gen_mixed_code_v0`**, **`gen_data_code_v0`** — file/byte-based gen
    functions following the established pattern; add matching `VideoCodeResult`, `MixedCodeResult`,
    `DataCodeResult` result classes
2. **`gen_instance_code_v0`**, **`gen_iscc_code_v0`**, **`gen_sum_code_v0`** — complete the gen
    function set

After all 9 gen functions: add algorithm primitives + streaming types, then conformance tests,
Standard Ruby linting, Ruby CI job, RubyGems release step, and documentation.
