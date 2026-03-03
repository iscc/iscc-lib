<!-- assessed-at: 78f4e04b8ba8d4d21e15c4f02addd2e6dc29f1e6 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings — 22/32 symbols; low-priority targets (C#, C++, Swift, Kotlin) deferred

The Ruby Magnus bridge advanced from 19 to 22 of 32 Tier 1 symbols, adding `gen_video_code_v0`,
`gen_mixed_code_v0`, and `gen_data_code_v0` with matching `VideoCodeResult`, `MixedCodeResult`, and
`DataCodeResult` wrapper classes. Binding specs for all languages were extracted into separate
`.claude/context/specs/` files. All 11 CI jobs remain green. The C#, C++, Swift, Kotlin issues are
now marked `low` priority (CID loop skips them); the Ruby issue remains `normal`.

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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22639088838 ✅

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
- **22 of 32 Tier 1 symbols** exposed:
    - Gen functions: `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
        `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0` (7)
    - Text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse` (4)
    - Codec/diagnostic: `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`,
        `json_to_data_url`, `conformance_selftest` (6)
    - Constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
        `TEXT_NGRAM_SIZE` (5)
- Pure Ruby wrapper: `VideoCodeResult`, `MixedCodeResult`, `DataCodeResult < Result < Hash` pattern,
    keyword args ✅
- 31 Minitest smoke tests; `bundle exec rake compile` builds in release profile ✅
- `crates/iscc-rb/README.md` exists (stub) ✅
- **Missing** (10 symbols): 3 gen functions (`gen_instance_code_v0`, `gen_iscc_code_v0`,
    `gen_sum_code_v0`), 5 algorithm primitives (`sliding_window`, `alg_simhash`, `alg_minhash_256`,
    `alg_cdc_chunks`, `soft_hash_video_v0`), 2 streaming types (`DataHasher`, `InstanceHasher`)
- **Missing**: No conformance tests against `data.json`
- **Missing**: No dedicated `ruby` CI job (workspace Rust job uses `--exclude iscc-rb`)
- **Missing**: No RubyGems step in `release.yml`
- **Missing**: `scripts/version_sync.py` does not include gemspec
- **Missing**: Standard Ruby linting not configured (no `standard` gem in Gemfile, no
    `.standard.yml`, not wired into pre-commit hooks or CI)

## C# / .NET Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/dotnet/` does not exist; no `csbindgen` integration; no CI job

## C++ Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/cpp/` does not exist; no `iscc.hpp`; no vcpkg/Conan manifests

## Swift Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (not
    started)

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- **Gap**: Ruby install instructions and quickstart not present
- **Gap**: C#, C++, Swift, Kotlin sections not present (target requires all 4; all `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages (including `crates/iscc-rb/README.md`) ✅
- **Gap**: `crates/iscc-rb/README.md` is a stub (31 lines); full installation/usage instructions not
    yet complete
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet (all `low` priority)

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 7 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; no Ruby tabs in multi-language
    examples
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22639088838: all 11 jobs SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22639088838
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- `iscc-rb` excluded from workspace Rust CI job (`--exclude iscc-rb`) — no dedicated Ruby CI job
- **Gap**: No `ruby` CI job; no `rubygems` publish step; Ruby not in `version_sync.py`
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)

## Next Milestone

Continue advancing the Ruby bindings toward 32/32 symbols. Suggested next work package:

1. **`gen_instance_code_v0`** — byte-slice based gen function; add `InstanceCodeResult` result class
2. **`gen_iscc_code_v0`** — multi-code aggregation; add `IsccCodeResult` result class
3. **`gen_sum_code_v0`** — path-based I/O; add `SumCodeResult` result class
4. **Algorithm primitives** — `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`
5. **Streaming types** — `DataHasher` and `InstanceHasher` with `push`/`finalize` interface

After all 32 symbols: add conformance tests against `data.json`, Standard Ruby linting, Ruby CI job,
RubyGems release step, version_sync, and documentation.
