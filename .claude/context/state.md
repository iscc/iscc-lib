<!-- assessed-at: 7c9700d16d5aced92636396c39fc50c5ef991095 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 2 `normal` audit issues block the release

ISCC-SEQ and `data-edge.json` (11 sections) run on the Rust core and all 11 bindings. Iteration 305
closed the Conan checksum issue: the recipe now verifies a pinned SHA256 per target. Two `normal`
audit issues still block v0.8.0: the Rust conformance runner duplication and the spec/API-doc drift.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 11 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section; C/C++ read the generated `data_edge_vectors.h`
- 256-bit rows pin the unit limit on encode and decode (`iscc_decode` `test_0006_unit_above_256`
    carries the reference encoder's 288-bit Meta-Code)
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; one `normal` audit issue open)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist; 341 `#[test]` in `src/`; 256-bit limit enforced
- `normal` (audit): `conformance.rs` lists sections by hand in three places and runs runners up to
    three times; `lib.rs` has 5 iscc-only conformance loops; WASM `tests/conformance.rs` copies
    parsers with silent casts
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
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met (one `low`; doc gaps under Documentation)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 11 sections; `genSumCodeV0` Javadoc states 64
    to 256; 0.7.0 on Maven Central
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
- `packages/cpp/conanfile.py` passes `sha256=` to `download()` from `packages/cpp/conandata.yml` (5
    target triples, v0.7.0 hashes) and raises on a missing hash; `scripts/update_vcpkg_hashes.py`
    writes it, `prepare-release` stages it; tests tie its keys to the `build-ffi` matrix
- No CI job executes the recipe (tests read it with `ast`); the review left this unfiled
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
- Unit range 32 to 256 on API/how-to pages, `iscc.h`, `_lowlevel.pyi`; `gen_sum_code_v0` 64 to 256
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
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45031720460 on the origin tip `516beea7`, all jobs green:
    https://github.com/iscc/iscc-lib/actions/runs/45031720460
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

18 open in issues.md: 2 `normal`, 16 `low` (15 untriaged), no `critical`. The Conan checksum issue
was closed in iteration 305. Both `normal` issues come from the audit and are loop-workable; none is
blocked on the human or upstream.

## Next Milestone

Clear the 2 open `normal` audit issues that block v0.8.0: the duplicated Rust conformance section
lists and runners, and the stale specs and API-doc gaps. After that, Titusz cuts v0.8.0 via
`/release`.
