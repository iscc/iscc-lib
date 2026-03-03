<!-- assessed-at: c8016888a90a3b24f22ee53e56abbe0e71ec5fe2 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings — conformance tests complete; CI job and docs remaining

The Ruby Magnus bridge exposes all 32 Tier 1 symbols with conformance tests now complete: 111
Minitest tests, 295 assertions, 0 failures (50 conformance vectors covering 9 gen functions + 61
existing smoke/streaming tests). All 11 CI jobs are green on run 22646899223. The remaining Ruby
work is infrastructure and documentation: a dedicated Ruby CI job, RubyGems publish step,
`version_sync.py` gemspec update, Standard Ruby linting, `docs/howto/ruby.md`, and a full
`crates/iscc-rb/README.md`.

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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22646899223 ✅

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
- **All 32 of 32 Tier 1 symbols** exposed:
    - Gen functions: `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
        `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
        `gen_iscc_code_v0`, `gen_sum_code_v0` (10) ✅
    - Text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse` (4) ✅
    - Codec/diagnostic: `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`,
        `json_to_data_url`, `conformance_selftest` (6) ✅
    - Algorithm primitives: `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
        `soft_hash_video_v0` (5) ✅
    - Constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
        `TEXT_NGRAM_SIZE` (5) ✅
    - Streaming types: `DataHasher`, `InstanceHasher` (2) ✅
- Pure Ruby wrapper: 10 result classes (`*CodeResult < Result < Hash`), keyword args, method
    chaining for streaming types ✅
- **Conformance tests**: `test/test_conformance.rb` — 50 dynamically generated test methods covering
    all 9 gen\_\*\_v0 functions against official data.json vectors ✅
- 111 Minitest tests total (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- `crates/iscc-rb/README.md` exists (stub, ~31 lines) ✅
- **Missing**: No dedicated `ruby` CI job (workspace Rust job still uses `--exclude iscc-rb`)
- **Missing**: No RubyGems step in `release.yml`
- **Missing**: `scripts/version_sync.py` does not include gemspec
- **Missing**: Standard Ruby linting not configured (no `standard` gem in Gemfile, no
    `.standard.yml`, not wired into pre-commit hooks or CI)
- **Missing**: `docs/howto/ruby.md` guide does not exist; no `docs/ruby-api.md`

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
- **Gap**: `crates/iscc-rb/README.md` is a stub (~31 lines); full installation/usage instructions
    not yet complete
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

- **ALL PASSING** — latest CI run 22646899223: all 11 jobs SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22646899223
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- `iscc-rb` excluded from workspace Rust CI job (`--exclude iscc-rb`) — no dedicated Ruby CI job
- **Gap**: No `ruby` CI job; no `rubygems` publish step; Ruby not in `version_sync.py`
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)

## Next Milestone

Ruby conformance tests are complete (50/50 vectors, all 9 gen functions). The next work package
should add the Ruby CI job to unblock automated verification on every push:

1. **Ruby CI job** — add a `ruby` job to `.github/workflows/ci.yml` that runs
    `bundle exec rake compile && bundle exec rake test`; remove `--exclude iscc-rb` from the Rust
    CI job so clippy covers the Ruby bridge crate too
2. **Standard Ruby linting** — add `standard` gem to Gemfile, create `.standard.yml`, wire into
    `Rakefile` and CI
3. **RubyGems release** — add `rubygems` checkbox to `release.yml`; update `version_sync.py` to sync
    gemspec version from Cargo.toml
4. **Documentation** — write `docs/howto/ruby.md`; expand `crates/iscc-rb/README.md`; add Ruby
    section to root `README.md`
