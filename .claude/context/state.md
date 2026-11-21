<!-- assessed-at: 4d0d94920f5255691ecba042506b3aca56f7ff8d -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features complete; two `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 255 closed the C
FFI `(NULL, 0)` issue: every pointer+length input goes through one `slice_or_empty` helper. Two
`normal` `[audit]` issues remain (integer range, published docs), none blocked on the human or
upstream. CI is green on `7538396`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 8 sections incl. `gen_meta_code_v0` and
    `gen_video_code_v0`; the ragged vector carries a `diverges` marker; the 5 tracked copies are
    byte-identical; C/C++ consume the generated `data_edge_vectors.h`
- napi, WASM, JNI and .NET leave `name`, `description`, `meta`, `metahash` uncompared for meta
- Not filed: the `parts` entry is untested on any surface; the WASM `compare` has no explicit
    non-object branch

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 308 lib tests
- Ragged frame signatures rejected via `check_frame_sig_lengths`; pinned by the shared vector
- `call_meta` in `tests/test_data_edge.rs` sits at CRAP 30.0, exactly the `--fail-above` limit
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification (both `low`)

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- Integer-range issue: raises `OverflowError` where `docs/howto/python.md` documents `ValueError`
- Python 3.10 floor is past end of life — `low` issue, human-only decision

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests green;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- **Bug (`normal`):** `u32`/`i32` parameters wrap silently on out-of-range input; only
    `gen_iscc_id_v1` runs `checked()`
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests green; SIMD release build met
- **Bug (`normal`):** same silent integer wrap as napi
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — needs a v0.8.0 release-notes line

## C FFI

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- `(NULL, 0)` is an empty input on every pointer+length export; NULL inner rows are a named error;
    `std::slice::from_raw_parts` appears only inside `slice_or_empty` (pinned by a Rust test); 121
    tests in `src/tests.rs`; rule documented in `docs/c-ffi-api.md` and `crates/iscc-ffi/CLAUDE.md`
- `low` (untriaged): C++ `safe_data` and .NET `PinnedRows` padding now guard a rejection the FFI no
    longer makes — redundant but correct

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 8 sections strictly; 0.7.0 on Maven Central
- Integer-range issue: JNI casts `bits as u32` unchecked; `low`: pending-exception guard untested

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green incl.
    the raw-JCS meta vector and the ragged video vector
- Published 0.7.0 still escapes `&`, `<`, `>`, U+2028/9 in canonical `meta` (fixed on develop)
- `low`: Go canonical `meta` differs from the core on UTF-16 key order, `-0` and lone surrogates

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 8 sections strictly; 0.7.0 on NuGet
- The 7 empty-input sentinel branches in `IsccLib.cs` are gone (FFI accepts `(NULL, 0)`)
- No .NET test passes an empty span to `GenImageCodeV0`, `EncodeBase64` or `AlgMinhash256`
- .NET 8 floor past end of life — `low`, human-only

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job green;
    0.7.0 vcpkg/Conan
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq`; `test_data_edge.rb` runs all 8 sections strictly; 0.7.0 on RubyGems
- Integer-range issue: raises `RangeError` where `docs/ruby-api.md` documents `RuntimeError`
- `low`: source gems up to 0.7.0 do not build; Ruby 3.1 floor EOL; Float accepted for integers

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 8 data-edge sections; the macOS `swift` job is green; shipped 0.7.0

## Documentation

**Status**: partially met

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- **Bug (`normal`):** published docs contradict what ships — README prints `result.iscc` from a
    string return, WASM snippet lacks `await init()`, Java/Kotlin/Swift docs deny shipped
    distribution, `architecture.md` and `development.md` carry stale claims (~11 files)
- `low`: Sum-Code comments show `KAA`; pkg-config example says `Version: 0.2.0`; `ci-cd.md`
    describes a tag trigger `release.yml` does not have

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 21 jobs (job table gated); `release.yml` is `workflow_dispatch`-only with 8 registry
    toggles; 21 version-sync targets at 0.7.0
- `iscc-ffi` sits outside the iai and CRAP gates — FFI marshalling regressions are not gated
- **Latest CI: PASSING** on `origin/develop` = `7538396` (FFI `(NULL, 0)` review):
    https://github.com/iscc/iscc-lib/actions/runs/44891435114
- HEAD is ahead of origin only by the `cid(log)` commit; green CI covers all code at HEAD

## Open Issues

17 open: 0 critical, 2 normal (both `[audit]`, both CID-workable), 15 low (human-directed; 10
`untriaged`). issues.md is at 273 of 300 lines.

## Next Milestone

Close the two `normal` issues that block v0.8.0: the out-of-range integer handling across the
bindings and the published-docs corrections. The release itself stays with Titusz.
