<!-- assessed-at: 8e3b3a6645206c94b21684d9bb1962c067b7421e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 1 `normal` audit issue blocks the release

ISCC-SEQ and `data-edge.json` (11 sections) run on the Rust core and all 11 bindings. Iteration 306
closed the conformance-duplication audit issue: one `SECTIONS` table in `conformance.rs` now drives
Rust conformance, and the WASM harness reads vectors strictly. One `normal` audit issue (spec and
API-doc drift, 9 files) still blocks v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 11 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section; C/C++ read the generated `data_edge_vectors.h`
- 256-bit rows pin the unit limit on encode and decode
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: met (v0.8.0 scope; two `low`)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist; 337 `#[test]` in `src/`; 256-bit limit enforced
- `conformance.rs` has one `SECTIONS` table driving `conformance_selftest`, the test helpers, a
    table-vs-`data.json` test and a non-empty-section test; the 5 duplicate `lib.rs` loops are gone
    (meta, text, mixed, instance loops with hard-coded counts remain)
- `low`: `iscc_decode` reads an 88-bit ISCC-ID that iscc-core rejects; `gen_mixed_code_v0` with
    `bits` below 8 panics in debug builds (`soft_hash_codes_v0` underflow)
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness cross-checks iscc-core; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests; `@iscc/lib`
    0.7.0 on npm (OIDC); `npm ci` from a committed lockfile
- `low`: `napi build` fails in the devcontainer checkout (9p rejects `link()`); CI is unaffected

## WASM Bindings

**Status**: met (v0.8.0 scope; two `low`)

- 35 Tier 1 symbols; SIMD release build met; data-edge runs all 11 sections
- `tests/conformance.rs` reads every vector through strict shared readers in `tests/common/mod.rs`
    (no silent `as` casts) and takes case counts from the file
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met (one `low`; doc gaps under Documentation)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 11 sections; 0.7.0 on Maven Central
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope; one `low`)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    11 sections; the `go` CI job also runs as `GOARCH=386`; 256-bit limit enforced
- `low`: `GenMixedCodeV0` with `bits` 0 to 7 panics (`softHashCodesV0` index out of range)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 11 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job
- Conan recipe verifies a pinned SHA256 per target from `packages/cpp/conandata.yml`; no CI job
    executes the recipe (tests read it with `ast`)
- `low`: the vcpkg port fails one post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs all 11 sections; floor 3.3
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 11 data-edge sections; `swift` CI job green
- `release.yml` commits the checksum (plus vcpkg and Conan hashes), then tags once

## Documentation

**Status**: partially met (one `normal` audit item open, one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- `normal` (audit): stale specs (`swift-bindings.md` checkbox, `documentation.md` counts,
    `c-ffi-dx.md` feature flags); `docs/c-ffi-api.md` omits the failed-finalize rule; the
    `encode_component` short-digest error is missing on 6 surfaces; `README.md` omits Swift
- `low`: partial version-sync file lists: `docs/development.md` names 9 of 22, the release skill 13

## Benchmarks

**Status**: met (one `low`)

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- `low`: `.iai-baseline.json` holds pre-dedup `bench_instance_code` rows (about 2× measured)

## CI/CD and Publishing

**Status**: met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 22
    version-sync targets at 0.7.0; every ecosystem installs from a lockfile
- `.crap-baseline.json` refreshed in iteration 306 (only `conformance.rs` rows changed)
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45034383698 on the origin tip `097e7e9d`, all 24 check runs green
    (including CRAP and Perf, which prove the refreshed baseline):
    https://github.com/iscc/iscc-lib/actions/runs/45034383698
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

17 open in issues.md: 1 `normal`, 16 `low` (15 untriaged), no `critical`. Iteration 306 closed the
conformance-duplication audit issue. The remaining `normal` issue is loop-workable; it is not
blocked on the human or upstream.

## Next Milestone

Clear the last `normal` audit issue that blocks v0.8.0: specs and API docs that describe shipped
work as planned or leave out shipped rules. After that, Titusz cuts v0.8.0 via `/release`.
