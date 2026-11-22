<!-- assessed-at: caa32e8bc3e5feff9f17bcdb21f0603de3257190 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 4 `normal` issues block the release

ISCC-SEQ and `data-edge.json` (11 sections, `alg_minhash_256` added in iteration 300) run on the
Rust core and all 11 bindings, and the minhash empty-list issue is closed. The iteration-300 audit
filed 3 new `normal` issues (Rust conformance duplication, Conan checksum, stale specs/API docs), so
4 `normal` issues now block v0.8.0, alongside the untouched 256-bit unit limit.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met for the current sections; the bits `normal` issue adds rows

- `crates/iscc-lib/tests/data-edge.json` has 11 sections (`alg_minhash_256` with an empty-list error
    row and a digest row); the 5 tracked copies are byte-identical; all 12 harnesses run every
    section
- C/C++ read the generated `data_edge_vectors.h`; Java's section table now uses `Map.ofEntries`
- Pending: `bits` above 256 rejection rows (expected `diverges`, iscc-core 1.5.0 accepts them)
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two `normal` issues open)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist; `alg_minhash_256` rejects an empty list
- `normal` (decided, upstream iscc/iscc-core#142): gen functions, hasher `finalize` and
    `encode_component` must reject a unit above 256 bits, and decode must reject such a header.
    Untouched: meta/text/audio/data/instance still return codes `iscc_decode` rejects
- `normal` (audit): `conformance.rs` lists sections by hand in three places and runs runners up to
    three times; `lib.rs` has 5 iscc-only conformance loops with hard-coded counts; WASM
    `tests/conformance.rs` copies parsers with silent casts
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope; the bits `normal` issue reaches it)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness cross-checks iscc-core; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests (minhash
    section included); `@iscc/lib` 0.7.0 on npm (OIDC); `npm ci` from a committed lockfile
- `low`: `napi build` fails in the devcontainer checkout (9p rejects `link()`); CI is unaffected

## WASM Bindings

**Status**: met (v0.8.0 scope; two `low`; the audit's WASM test-reader duplication is under Rust
core)

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

**Status**: partially met (the bits `normal` issue needs Go changes)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    11 sections; the `go` CI job also runs as `GOARCH=386`
- `AlgMinhash256` returns `([]byte, error)` and rejects an empty list
- `GenDataCodeV0` accepts 512 bits

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 11 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0; `AlgMinhash256` XML doc names the empty-list error
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` from the audit, one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job
- `normal` (audit): `packages/cpp/conanfile.py:84` calls `download()` with no `sha256`, unlike the
    vcpkg port's SHA512 pins; its target table is an untested third copy of the `build-ffi` matrix
- `low`: the vcpkg port fails one post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs all 11 sections (minhash included); floor 3.3
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 11 data-edge sections; `swift` CI job green
- `release.yml` commits the checksum, then tags once (`fetch-depth: 0`, an existing tag is kept)

## Documentation

**Status**: partially met (one `normal` from the audit, one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- `normal` (audit): stale specs (`swift-bindings.md` checkbox, `documentation.md` counts,
    `c-ffi-dx.md` feature flags); `docs/c-ffi-api.md` omits the failed-finalize rule; the
    `encode_component` short-digest error is missing on 6 surfaces; `README.md` binding list
    (:49-51) omits Swift
- `low`: partial version-sync file lists: `docs/development.md` names 9 of 22, the release skill 13
- Pending with the bits work: API pages must state the 32 to 256 `bits` range

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
- **Latest CI: PASSING.** Run 45019851735 on the origin tip `a8d04343`, 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45019851735
- Local HEAD adds 4 unpushed commits (`cid(log)`, `cid(audit)`) that touch only `.claude`

## Open Issues

18 open in issues.md: 4 `normal`, 14 `low` (13 untriaged), no `critical`. The bits `normal` issue is
decided by Titusz (2026-11-22); the 3 audit `normal` issues are loop-workable. None is blocked on
the human or upstream. 7 sit in `postponed.md`.

## Next Milestone

Clear the 4 open `normal` issues that block v0.8.0: the decided 256-bit unit limit across the core
and all surfaces, and the three audit findings (Rust conformance-table duplication, Conan checksum,
stale specs and API docs). After that, Titusz cuts v0.8.0 via `/release`.
