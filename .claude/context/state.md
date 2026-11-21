<!-- assessed-at: 033b08510fca7d37454dd4aecaec5d7835d78415 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — release-ready; waiting on Titusz to cut it and triage `low` issues

Every v0.8.0 criterion is met: ISCC-SEQ and `data-edge.json` run on the Rust core and all 11
bindings, CI is green on the tip, and no `critical` or `normal` issue is open. Iteration 257 fixed
the published docs and READMEs and closed the last `normal` issue. 18 `low` issues remain (13
untriaged), all human-directed, and the v1.0.0 criteria stay human-gated, so the status is not DONE.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 8 sections incl. `gen_meta_code_v0` and
    `gen_video_code_v0`; the ragged vector carries a `diverges` marker; the 5 tracked copies are
    byte-identical; C/C++ consume the generated `data_edge_vectors.h`
- napi, WASM, JNI and .NET leave `name`, `description`, `meta`, `metahash` uncompared for meta
- Not filed: the `parts` entry is untested on any surface; the WASM `compare` has no explicit
    non-object branch

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 308 lib tests; untouched since iteration 252
- `call_meta` in `tests/test_data_edge.rs` sits at CRAP 30.0, exactly the `--fail-above` limit
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification (both `low`)

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- `docs/howto/python.md` documents `OverflowError` for out-of-range integers
- Python 3.10 floor is past end of life — `low` issue, human-only decision

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests green;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- Numeric parameters are `f64` narrowed by range-checked helpers; bad values throw
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests green; SIMD release build met
- README, crate README and how-to now show `await init()` before use
- `low` (untriaged): the wasm-bindgen glue coerces `null`, strings and booleans to numbers
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — needs a v0.8.0 release-notes line

## C FFI

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- `(NULL, 0)` is an empty input on every pointer+length export via `slice_or_empty`
- `low` (untriaged): C++ `safe_data` and .NET `PinnedRows` padding are now redundant

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 8 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries; docs now describe that distribution
- `low` (untriaged): `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`
- `low`: pending-exception guard untested

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green incl.
    the raw-JCS meta vector and the ragged video vector
- Published 0.7.0 still escapes `&`, `<`, `>`, U+2028/9 in canonical `meta` (fixed on develop)
- `low`: Go canonical `meta` differs from the core on UTF-16 key order, `-0` and lone surrogates

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 8 sections strictly; 0.7.0 on NuGet
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
- `docs/ruby-api.md` documents `RangeError` for out-of-range integers
- `low`: source gems up to 0.7.0 do not build; Ruby 3.1 floor EOL; Float accepted for integers

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 8 data-edge sections; the macOS `swift` job is green; shipped 0.7.0
- Swift and Kotlin READMEs/how-tos now describe the shipped XCFramework and Maven Central JAR

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- Iteration 257 fixed the published-docs drift across 9 files (README JS return type, WASM init,
    Java/Kotlin/Swift distribution, `architecture.md`, `development.md`)
- `low` (untriaged): `architecture.md` lower sections cover half the surfaces; Sum-Code comments
    show `KAA`; pkg-config example says `Version: 0.2.0`; `ci-cd.md` describes a tag trigger

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 21 jobs (job table gated); `release.yml` is `workflow_dispatch`-only with 8 registry
    toggles; 21 version-sync targets at 0.7.0
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates — binding marshalling is not gated
- **Latest CI: PASSING** on `origin/develop` = `844f8c4` (all jobs green, check-suite `success`):
    https://github.com/iscc/iscc-lib/actions/runs/44895258278 — the previous tip `870bbe6` also
    finished fully green
- HEAD is ahead of origin only by the `cid(log)` commit; no unpushed code

## Open Issues

18 open: 0 critical, 0 normal, 18 low (all human-directed; 13 `untriaged`). issues.md is at 298 of
300 lines.

## Next Milestone

No CID-workable gap remains for v0.8.0. The next steps are Titusz's: cut v0.8.0 via `/release`
(release notes should mention the `alg_cdc_chunks` WASM return type), triage the 13 untriaged `low`
issues, and decide when to start the human-gated v1.0.0 work.
