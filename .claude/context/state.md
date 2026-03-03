<!-- assessed-at: 3bd8d86798ee58915ce095836d2498b8b7ceb9a6 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings — partial scaffold (10/32 symbols)

The Ruby bindings scaffold was completed this iteration: `crates/iscc-rb/` exists with a functioning
Magnus bridge, Minitest smoke tests (10 runs, 24 assertions), and a compiled native extension.
However only 10 of 32 Tier 1 symbols are exposed. All 11 CI jobs remain green.

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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22635315189 ✅

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

- `crates/iscc-rb/` scaffold created with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
- **10 of 32 Tier 1 symbols** exposed: `gen_meta_code_v0`, `text_clean`, `text_remove_newlines`,
    `text_trim`, `text_collapse`, + 5 constants (META_TRIM_NAME, META_TRIM_DESCRIPTION,
    META_TRIM_META, IO_READ_SIZE, TEXT_NGRAM_SIZE) ✅
- Pure Ruby wrapper in `lib/iscc_lib.rb`: `MetaCodeResult < Result < Hash` pattern, keyword args ✅
- 10 Minitest smoke tests pass (24 assertions); `bundle exec rake compile` builds in release profile
    ✅
- `crates/iscc-rb/README.md` exists (31 lines) ✅
- **Missing**: 22 remaining gen/codec/streaming/algorithm symbols not yet implemented
- **Missing**: No conformance tests against `data.json`
- **Missing**: No dedicated `ruby` CI job (workspace Rust job uses `--exclude iscc-rb`)
- **Missing**: No RubyGems step in `release.yml`
- **Missing**: `scripts/version_sync.py` does not include gemspec

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- **Gap**: Ruby install instructions and quickstart not present (target requires Ruby/gem section)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 8 crates/packages (including `crates/iscc-rb/README.md` now) ✅
- **Gap**: `crates/iscc-rb/README.md` is a stub (31 lines); full installation/usage instructions not
    yet complete per target requirements

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 7 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; no Ruby tabs in multi-language
    examples

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22635315189: all 11 jobs SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22635315189
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- `iscc-rb` excluded from workspace Rust CI job (`--exclude iscc-rb`) — no dedicated Ruby CI job
- **Gap**: No `ruby` CI job; no `rubygems` publish step; Ruby not in `version_sync.py`

## Next Milestone

Implement the remaining 22 Tier 1 symbols in the Magnus bridge (`crates/iscc-rb/src/lib.rs`).
Suggested ordering:

1. **Codec functions** — `iscc_encode`, `iscc_decode`, `iscc_decompose`, `iscc_normalize` (4
    symbols); each maps to a straightforward Rust call, no result class needed
2. **Remaining gen functions** — `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
    `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
    `gen_iscc_code_v0`, `gen_sum_code_v0` (9 symbols); each needs a Ruby `*Result` class
3. **Algorithm + streaming** — `alg_simhash_from_iscc`, `DataHasher`, `InstanceHasher` (3 symbols)

After all 32 symbols are implemented: add conformance tests against `data.json`, add a dedicated
`ruby` CI job, add RubyGems step to `release.yml`, add Ruby to `version_sync.py`, add
`docs/howto/ruby.md`, and add Ruby install/quickstart to the root README.
