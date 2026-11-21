<!-- assessed-at: 8641acef38bf6f23b7d58e0674f8de520607f27a -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; CI green; 13 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 266 made the Go
port write canonical `meta` JSON the same way as the core (UTF-16 key order, `-0`, lone surrogates)
and added two shared vectors for it; that issue is closed. CI is green; 13 `normal` and 2 `low`
issues remain open.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; `gen_meta_code_v0` now holds 3 vectors
    (`test_0002_utf16_key_order`, `test_0003_negative_zero` added 266); the 5 tracked copies are
    byte-identical; C/C++ read the regenerated `data_edge_vectors.h`; all 12 harnesses run every
    section
- The ragged video vector carries a `diverges` marker; napi, WASM, JNI and .NET leave `name`,
    `description`, `meta`, `metahash` uncompared for meta
- No vector from 2^29 up, on purpose (decision of 2026-11-21; mismatch is iscc/iscc-core#141); no
    lone-surrogate vector, because serde_json cannot load such a fixture (Go unit tests cover it)

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 311 `#[test]` in `src/`
- `alg_cdc_params` saturates for every `u32` from 2 up; no upper bound (decision of 2026-11-21)
- `normal` (audit): one-shot gen functions re-implement streaming — `gen_instance_code_v0` hashes
    BLAKE3 twice, `gen_meta_code_v0` parses `meta` twice, WASM `gen_sum_code_v0` copies `SumHasher`
- `normal` (audit): specs and crate `CLAUDE.md` files are stale — `rust-core.md` says `Version` has
    only V0, napi/py/wasm `CLAUDE.md` name `iscc_lib::Error` (type is `IsccError`)
- **Postponed (human):** crate `>= 1.0.0`; MSRV verification; `no_std` support

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- Video frame values outside `i32` raise `OverflowError` (`tests/test_int_range.py`)
- `specs/python-bindings.md` still says text/video paths hold the GIL (part of the spec issue)
- The Windows behaviour is proven from the type only — no CI run on Windows (`low`, see CI/CD)

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

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- `normal`: `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`
- `algCdcChunks` takes a signed `int` — 2147483647 is the largest CDC average it can pass

## Go Bindings

**Status**: partially met (one `normal` issue)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections incl. the three meta vectors and the ragged video vector
- `code_meta.go` writes RFC 8785 JSON itself (`writeCanonical*`, `checkSurrogateEscapes`): UTF-16
    key order at every depth, `-0` as `0`, lone surrogate escapes rejected (fixed 266)
- `normal` (decided): add one `GOARCH=386` step to the `go` CI job — the 32-bit `algCdcParams`
    handling is untested in CI
- `TextTrim` clamps `nbytes <= 0`; hashers error after `Finalize` (API break, release-notes item)

## C# / .NET Bindings

**Status**: partially met (one `normal` issue)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet
- CI: `dotnet-test` 8.0/10.0 legs on a private `DOTNET_INSTALL_DIR` with `skip-lts-prepass: true`
- `normal`: the `PinnedRows` padding (and C++ `safe_data`) guard a NULL rejection the FFI no longer
    has — removal decided
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` issue, shared with .NET)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run incl.
    `alg_cdc_chunks`; ASAN `cpp` job; 0.7.0 vcpkg/Conan
- `normal`: remove the redundant `safe_data` padding
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: partially met (two `normal` issues)

- `encode_seq` / `decode_seq`; `test_data_edge.rb` runs all 9 sections strictly; 0.7.0 on RubyGems
- `normal` (human, decided): raise the floor from 3.1 to 3.3 in gemspec, CI, Gemfile, docs, and
    extend the `release.yml` cross-gem list to every supported series
- `normal`: document the implicit Float→Integer `to_int` conversion (kept as host idiom)
- `low`: published source gems up to 0.7.0 do not build

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0
- Kotlin resolves Gradle plugins from `mavenCentral()` first; Kotlin and macOS `swift` jobs green

## Documentation

**Status**: partially met (five `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; how-to Sum-Code comments show
    `KAA`; docs overstate language coverage and what ISO 24138 specifies (audit)
- `normal`: `docs/development.md` "Publish" says a tag push triggers the release
- `normal` (audit): spec/`CLAUDE.md` staleness, including `go-bindings.md` claiming an `io.Reader`
    API that `packages/go` lacks

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met — CI green; the v1.0.0 semver gate is human-gated

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates — binding marshalling is not gated
- No 32-bit CI leg for the Go port yet (`normal`, decided, see Go)
- `low`: no CI job runs a test suite on Windows — 21 Linux jobs, one macOS; in `release.yml` only
    `test-nuget` runs on `windows-latest`
- **Latest CI: PASSING** on `origin/develop` = `e195294`:
    https://github.com/iscc/iscc-lib/actions/runs/44924695693 — all jobs `success`; HEAD adds only
    the `cid(log)` commit, so green CI covers the code at HEAD

## Open Issues

15 open in issues.md: 0 critical, 13 `normal`, 2 `low`; none blocked on the human or upstream. The
Go canonical `meta` issue closed in 266. 7 sit in `postponed.md`.

## Next Milestone

Clear the 13 `normal` issues that block v0.8.0, correctness first: WASM coercion and the Go 32-bit
CI step. Then the Ruby floor, the FFI padding removal, the JNI loader, the one-shot duplication and
docs/spec alignment. Titusz then cuts v0.8.0 via `/release`.
