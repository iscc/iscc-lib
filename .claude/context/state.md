<!-- assessed-at: c7eee56a6d52f0e8baf874a0fa0eee7e692552a6 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 4 `normal` issues block the release

ISCC-SEQ and `data-edge.json` (11 sections) run on the Rust core and all 11 bindings. The 256-bit
unit limit is now enforced in the Rust core (301) and the Go port (302), so all 11 surfaces reject
units above 256 bits; the shared vector rows and the binding API pages remain. 4 `normal` issues
still block v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met for the current sections; the bits `normal` issue adds rows

- `crates/iscc-lib/tests/data-edge.json` has 11 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section; C/C++ read the generated `data_edge_vectors.h`
- Pending: a `bits` above 256 encode row (marked `diverges`, iscc-core 1.5.0 accepts it) and a
    decode row for a unit header longer than 256 bits; both pinned on all 12 surfaces
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; one `normal` audit issue open)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist; 341 `#[test]` in `src/`
- 256-bit limit met: `codec::encode_length` accepts only 32 to 256 for Meta through Flake;
    `pub(crate)` helper `codec::unit_bit_length` checks every internal decode
- `normal` (audit): `conformance.rs` lists sections by hand in three places and runs runners up to
    three times; `lib.rs` has 5 iscc-only conformance loops; WASM `tests/conformance.rs` copies
    parsers with silent casts
- `low`: `iscc_decode` reads an 88-bit ISCC-ID that iscc-core rejects
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope; inherits the core's 256-bit limit)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness cross-checks iscc-core; CI 3.10/3.15
- `_lowlevel.pyi` still documents `bits` as "multiple of 32" only (bits issue, API pages part)
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
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md`
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 11 sections; 0.7.0 on Maven Central
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    11 sections; the `go` CI job also runs as `GOARCH=386`
- 256-bit limit met: `encodeLength` (`packages/go/codec.go`) accepts only 32 to 256; the only
    `decodeLength` caller outside its definition is the checked `unitBitLength`; `Above256` tests
    cover every gen function, both hashers, `GenSumCodeV0`, decode and the wide ISCC-CODE path

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 11 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` from the audit, one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job
- `normal` (audit): `packages/cpp/conanfile.py` calls `download()` with no `sha256`, unlike the
    vcpkg port's SHA512 pins; its target table is an untested third copy of the `build-ffi` matrix
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
- `release.yml` commits the checksum, then tags once (`fetch-depth: 0`, an existing tag is kept)

## Documentation

**Status**: partially met (one `normal` from the audit, bits API-page part pending, one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- `docs/rust-api.md`, `docs/howto/go.md` and the Go doc comments state the 32 to 256 range; the C
    FFI, Java, Ruby reference pages, the other how-to pages, `iscc.h`, `IsccLib.java` and
    `_lowlevel.pyi` still say only "multiple of 32"
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
- **Latest CI: PASSING.** Run 45026413052 on the origin tip `80897d52`, all jobs green:
    https://github.com/iscc/iscc-lib/actions/runs/45026413052
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

19 open in issues.md: 4 `normal`, 15 `low` (14 untriaged), no `critical`. The bits `normal` issue
(decided by Titusz 2026-11-22) now covers only the shared vector rows and the API pages; the 3 audit
`normal` issues are loop-workable. None is blocked on the human or upstream.

## Next Milestone

Clear the 4 open `normal` issues that block v0.8.0: the remaining 256-bit limit work
(`data-edge.json` rows on all 12 surfaces, binding API pages) and the three audit findings (Rust
conformance-table duplication, Conan checksum, stale specs and API docs). After that, Titusz cuts
v0.8.0 via `/release`.
