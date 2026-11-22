<!-- assessed-at: 0e0f5d1468da1aae575df46ecefb46c5f7c6a151 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — release criteria met; only `low` issues remain

ISCC-SEQ and `data-edge.json` (11 sections) run on the Rust core and all 11 bindings. Iteration 307
closed the last `normal` audit issue (spec and API-doc drift). No `critical` or `normal` issue is
open and CI is green on the origin tip, so v0.8.0 is ready for Titusz to cut via `/release`. DONE is
blocked by 18 open `low` issues and the human-gated v1.0.0 criteria.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 11 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section; C/C++ read the generated `data_edge_vectors.h`
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: met (v0.8.0 scope; two `low`)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist; 337 `#[test]` in `src/`; one `SECTIONS` table in `conformance.rs`
    drives conformance
- `low`: `iscc_decode` reads an 88-bit ISCC-ID that iscc-core rejects; `gen_mixed_code_v0` with
    `bits` below 8 panics in debug builds
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`; semver gate stays informational until 1.0.0

## Python Bindings

**Status**: met (v0.8.0 scope; one `low`)

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

- 35 Tier 1 symbols; SIMD release build; data-edge runs all 11 sections; strict shared vector
    readers in `tests/common/mod.rs`
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met (one `low`)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs
- `docs/c-ffi-api.md` now states that a failed first `_finalize()` consumes the hasher
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 11 sections; 0.7.0 on Maven Central
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope; one `low`)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    11 sections; the `go` CI job also runs as `GOARCH=386`
- `low`: `GenMixedCodeV0` with `bits` 0 to 7 panics (`softHashCodesV0` index out of range)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 11 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; Conan
    recipe verifies a pinned SHA256 per target
- `low`: the vcpkg port fails one post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs all 11 sections; floor 3.3
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 11 data-edge sections; `swift` and `kotlin` CI jobs green
- `release.yml` commits the checksum (plus vcpkg and Conan hashes), then tags once

## Documentation

**Status**: met (three `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README (names Swift in every binding list) plus 12 per-crate/package READMEs
- `encode_component` short-digest rule documented on all surfaces; `c-ffi-dx.md`, `documentation.md`
    and `swift-bindings.md` now state shipped facts
- `low`: `encode_component` docs on 5 surfaces omit the ISCC-ID `bit_length` case; spec says all
    how-to guides use tabs (they have none); version-sync lists in `docs/development.md` and the
    release skill are partial

## Benchmarks

**Status**: met (one `low`)

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- `low`: `.iai-baseline.json` holds pre-dedup `bench_instance_code` rows (about 2× measured)

## CI/CD and Publishing

**Status**: met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 22
    version-sync targets at 0.7.0; every ecosystem installs from a lockfile
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45036383921 on the origin tip `32cd3f91`, all 24 check runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45036383921
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

18 open in issues.md, all `low` (16 untriaged), no `critical` or `normal`. Iteration 307 closed the
spec/API-doc drift audit issue and filed two `low` doc issues.

## Next Milestone

v0.8.0 meets its release criteria: features on all 11 surfaces, `version:check` in CI, CI green, no
`critical` or `normal` issue open. The next step is the human's: cut v0.8.0 via `/release`. The open
`low` issues are not release blockers; whether to work any of them is a triage call.
