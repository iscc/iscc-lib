<!-- assessed-at: 00fc7cccf9fa4b9ec7e4e51e691e4dd3fb9cf735 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; CI green again; 17 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 260 fixed the red
.NET 10 CI leg (private `DOTNET_INSTALL_DIR`, `fail-fast: false`); `develop` is green on `28a1f46`.
Review filed 1 and the iteration-260 audit filed 5 `normal` issues, so 17 `normal` issues are open,
none blocked on the human or upstream.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 8 sections; the ragged video vector carries a
    `diverges` marker; the 5 tracked copies are byte-identical; C/C++ read the generated
    `data_edge_vectors.h`
- napi, WASM, JNI and .NET leave `name`, `description`, `meta`, `metahash` uncompared for meta
- `normal` (audit): no vector pins `alg_cdc_chunks` at an overflowing `avg_chunk_size`, where Rust
    and Go cut differently (see Rust core)

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 308 lib tests; code untouched since iteration 252
- `normal` (audit): `alg_cdc_chunks` rejects only `avg_chunk_size < 2`; `alg_cdc_params` overflows
    `u32` from 2^29 (debug panic, release wrap) and differs from Go
- `normal` (audit): one-shot gen functions re-implement streaming — `gen_instance_code_v0` hashes
    BLAKE3 twice, `gen_meta_code_v0` parses `meta` twice, WASM `gen_sum_code_v0` copies `SumHasher`
- `normal` (audit): specs and crate `CLAUDE.md` files are stale again — `rust-core.md` says
    `Version` has only V0, napi/py/wasm `CLAUDE.md` name `iscc_lib::Error` (type is `IsccError`)
- **Missing (human-gated, postponed):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: partially met (one `normal` issue)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `normal` (audit): out-of-range video frame values raise `ValueError` on Linux/macOS but
    `OverflowError` on Windows (`PyLong_AsLong`); `tests/test_int_range.py` has no video case
- `specs/python-bindings.md` still says text/video paths hold the GIL (part of the spec issue)

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); numeric parameters are range-checked
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal` issue, plus the shared duplication issue)

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests; SIMD release build met
- `normal`: the wasm-bindgen glue converts `null`, strings and booleans to numbers instead of
    rejecting them
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — needs a v0.8.0 release-notes line

## C FFI

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- `(NULL, 0)` is an empty input on every pointer+length export via `slice_or_empty`
- `normal`: the pkg-config example in `docs/howto/c-cpp.md` says `Version: 0.2.0`

## Java Bindings

**Status**: partially met (one `normal` issue)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 8 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- `normal`: `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`
- The pending-exception guard test is postponed

## Go Bindings

**Status**: partially met (two `normal` issues)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green incl.
    the raw-JCS meta vector and the ragged video vector
- `normal`: Go canonical `meta` differs from the core on UTF-16 key order, `-0` and lone surrogates
- `normal` (audit): `TextTrim` panics on a negative `nbytes`; `DataHasher`/`InstanceHasher` accept
    `Push`/`Finalize` after `Finalize` and return a wrong code silently
- Published 0.7.0 still escapes `&`, `<`, `>`, U+2028/9 in canonical `meta` (fixed on develop)

## C# / .NET Bindings

**Status**: partially met (two `normal` issues)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 8 sections strictly; 0.7.0 on NuGet
- CI: `dotnet-test` 8.0/10.0 legs and the `dotnet` aggregator green; `fail-fast: false`; private
    `DOTNET_INSTALL_DIR` on the `setup-dotnet` step
- `normal` (filed 260): setup-dotnet's LTS pre-pass still installs the latest LTS into the private
    dir, so the 10.0 leg (`LatestMajor`) turns red when .NET 12 ships; the matrix comment is wrong
- `normal`: the `PinnedRows` padding (and C++ `safe_data`) guard a NULL rejection the FFI no longer
    has — removal decided
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` issue, shared with .NET)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `normal`: remove the redundant `safe_data` padding
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: partially met (two `normal` issues)

- `encode_seq` / `decode_seq`; `test_data_edge.rb` runs all 8 sections strictly; 0.7.0 on RubyGems
- `normal` (human, decided): raise the floor from 3.1 to 3.3 in gemspec, CI, Gemfile, docs, and
    extend the `release.yml` cross-gem list to every supported series
- `normal`: document the implicit Float→Integer `to_int` conversion (kept as host idiom)
- `low`: published source gems up to 0.7.0 do not build

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 8 data-edge sections; the macOS `swift` job is green; shipped 0.7.0
- `specs/swift-bindings.md` Release Flow matches `release.yml` (an existing tag is not moved)

## Documentation

**Status**: partially met (five `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; how-to Sum-Code comments show
    `KAA`; docs overstate language coverage and what ISO 24138 specifies [audit]
- `normal`: `docs/development.md` "Publish" says a tag push triggers the release
- `normal` (audit): spec/`CLAUDE.md` staleness (also `go-bindings.md` IDv1 "only on develop",
    `packages/go/CLAUDE.md` test list, FFI-wrapping claim, missing Go tag)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met — CI green; the v1.0.0 semver gate is human-gated

- `ci.yml` has 22 job keys (`dotnet-test` matrix + `if: always()` `dotnet` aggregator);
    `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21 version-sync targets at
    0.7.0
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates — binding marshalling is not gated
- The .NET 10 leg's future dependence on the latest LTS is an open `normal` issue (see .NET)
- **Latest CI: PASSING** on `origin/develop` = `28a1f46`:
    https://github.com/iscc/iscc-lib/actions/runs/44904982105 (all jobs `success`)
- HEAD is 4 commits ahead of origin (audit + `cid(log)`, `.claude` only); green CI covers all code

## Open Issues

18 open in issues.md: 0 critical, 17 `normal`, 1 `low`; none untriaged and none blocked on the human
or upstream. Iteration 260 closed none; review filed 1 (.NET LTS pre-pass) and the audit filed 5 (Go
`TextTrim`/hashers, `alg_cdc_chunks` overflow, one-shot duplication, Python video exception class,
stale specs/`CLAUDE.md`). 6 more sit in `postponed.md`.

## Next Milestone

Clear the 17 `normal` issues that block v0.8.0 (correctness defects in Rust core, Go and Python
first, then CI robustness, Ruby floor and docs/spec alignment); after that Titusz cuts v0.8.0 via
`/release`.
