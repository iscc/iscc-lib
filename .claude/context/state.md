<!-- assessed-at: e5ebca4 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 11 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 269 repaired the
two `@iscc/wasm` type-check defects; the `critical` issue is closed and CI is green on the pushed
tip, which covers all code at HEAD. 15 issues are open: 0 `critical`, 11 `normal`, 4 `low`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    C/C++ read the generated `data_edge_vectors.h`; all 12 harnesses run every section
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
- `normal` (audit): specs and crate `CLAUDE.md` files are stale (`rust-core.md` `Version` enum,
    `iscc_lib::Error` vs `IsccError`, the `specs/wasm-bindings.md` bridge sketch)
- **Postponed (human):** crate `>= 1.0.0`; MSRV verification; `no_std` support

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- Video frame values outside `i32` raise `OverflowError` (`tests/test_int_range.py`)
- `specs/python-bindings.md` still says text/video paths hold the GIL (part of the spec issue)

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); numeric parameters are range-checked
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: met (v0.8.0 scope; two `low` issues, plus the shared duplication issue)

- 35 Tier 1 symbols; SIMD release build met; number and boolean parameters arrive as `JsValue` and
    are type-checked in Rust; `number_vec` reads elements through `catch` imports (no
    `Array::from`); `gen_meta_code_v0` declares `description?` / `meta?` again
- Tests: 11 conformance + 8 data-edge + 126 unit; the `wasm` CI job passed on `799dc2d`
- `low`: byte and string parameters are still glue-converted (`"abc"` hashes as three zero bytes; a
    number for a string traps)
- `low`: list inputs read through serde (`gen_video_code_v0`, `gen_mixed_code_v0`,
    `gen_iscc_code_v0`, `encode_seq`, …) let a throwing element kill the instance (predates 268)
- Release-notes items: `alg_cdc_chunks` returns `Uint8Array[]`; numeric strings, `null` for a
    required number, non-boolean flags, and a `Set`/generator for `cv`/`features` now raise errors

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

## Go Bindings

**Status**: met (v0.8.0 scope; shares the audit spec-staleness issue)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; RFC 8785 canonical meta JSON matches the core
- The `go` CI job reruns `go test` + `go vet` as a `GOARCH=386` build
- `go-bindings.md` claims an `io.Reader` API that `packages/go` lacks (audit spec issue)

## C# / .NET Bindings

**Status**: partially met (one `normal` issue)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs
- `normal`: the `PinnedRows` padding (and C++ `safe_data`) guard a NULL rejection the FFI no longer
    has — removal decided
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` issue, shared with .NET)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `normal`: remove the redundant `safe_data` padding

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

## Documentation

**Status**: partially met (five `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; how-to Sum-Code comments show
    `KAA`; docs overstate language coverage and what ISO 24138 specifies (audit)
- `normal`: `docs/development.md` "Publish" says a tag push triggers the release
- `normal` (audit): spec/`CLAUDE.md` staleness

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (one `low` issue; release criteria otherwise met)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates; no check diffs the built WASM
    `.d.ts` against a committed snapshot (review gate idea)
- `low`: no CI job runs a test suite on Windows
- **Latest CI: PASSING** on `origin/develop` = `799dc2d`:
    https://github.com/iscc/iscc-lib/actions/runs/44930626782 — `WASM (wasm-pack test)` and `Semver`
    both green; the only unpushed commit (`e5ebca4`) touches `.claude/` alone

## Open Issues

15 open in issues.md: 0 `critical`, 11 `normal`, 4 `low`; none blocked on the human or upstream. The
`critical` WASM entry was closed; a `low` serde-list exception issue was filed. 7 sit in
`postponed.md`.

## Next Milestone

Clear the 11 `normal` issues that block v0.8.0 — Ruby floor, FFI padding cleanup, JNI loader
fallback, Ruby Float docs, gen/streaming duplication, and the docs/spec accuracy entries. Then
Titusz cuts v0.8.0 via `/release`.
