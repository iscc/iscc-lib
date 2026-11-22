<!-- assessed-at: fd9b6af71490e7cb02e938e40990915e9d187852 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 2 decided `normal` issues block the release

ISCC-SEQ and `data-edge.json` (10 sections) run on the Rust core and all 11 bindings. All 12
surfaces now reject an empty `alg_minhash_256` list (Go since iteration 299); only the shared
`data-edge.json` section for it remains. The other `normal` issue (a unit above 256 bits must be
rejected) is untouched.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met for the current sections; both `normal` issues each add a section

- `crates/iscc-lib/tests/data-edge.json` has 10 sections, `encode_component` included; the 5 tracked
    copies are byte-identical; all 12 harnesses run every section
- C/C++ read the generated `data_edge_vectors.h`
- Pending: an `alg_minhash_256` section (error row + non-empty row, no `except` expected) and a
    `bits` above 256 rejection vector
- Java's `DataEdgeTest` section table uses `Map.of`, which tops out at 10 pairs; the next section
    needs `Map.ofEntries`
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; one decided `normal` issue untouched)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist. `src/` has 325 `#[test]`
- `alg_minhash_256` returns `IsccResult<Vec<u8>>` and rejects an empty list with
    `features must not be empty`; internal callers use an unchecked helper
- `normal` (decided, upstream iscc/iscc-core#142): gen functions, hasher `finalize` and
    `encode_component` must reject a unit above 256 bits, and decode must reject such a header.
    Today meta/text/audio/data/instance return codes `iscc_decode` rejects
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope; the bits `normal` issue reaches it)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness cross-checks iscc-core; CI 3.10/3.15
- `alg_minhash_256([])` raises `ValueError`, pinned in `tests/test_algo.py`
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests; `@iscc/lib`
    0.7.0 on npm (OIDC); committed `package-lock.json`, `npm ci` in CI and release
- `alg_minhash_256([])` throws; no test pins it yet (the pending data-edge section will)
- `low`: `napi build` fails in the devcontainer checkout (9p rejects `link()`); CI is unaffected

## WASM Bindings

**Status**: met (v0.8.0 scope; two `low`)

- 35 Tier 1 symbols; SIMD release build met; hashers consume before checking `bits`
- `alg_minhash_256([])` throws; no test pins it yet
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met (one `low`)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md`
- `iscc_alg_minhash_256` reports the empty-list error, pinned in `src/tests.rs`
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 10 sections; `algMinhash256` throws
    `IllegalArgumentException` on an empty array (pinned); 0.7.0 on Maven Central
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: partially met (the bits `normal` issue needs Go changes)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    10 sections; the `go` CI job also runs as `GOARCH=386`
- `AlgMinhash256(features []uint32) ([]byte, error)` (`minhash.go:204`) rejects an empty or nil list
    with `features must not be empty`, pinned by `TestMinhash256Empty`; generators call the
    unchecked `algMinhash256`. Spec row and `docs/howto/go.md` updated
- `GenDataCodeV0` accepts 512 bits

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 10 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0; `NativeMethods.g.cs` regenerated for the minhash error
- Nit: the XML doc of `AlgMinhash256` in `IsccLib.cs` does not name the empty-list error
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job;
    empty-list `alg_minhash_256` test expects the error
- Nit: the `///` comment of `alg_minhash_256` in `iscc.hpp` does not name the error
- `low`: the port fails one vcpkg post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs all sections; `alg_minhash_256([])` raises
    `RuntimeError` (no test yet); floor 3.3
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 10 data-edge sections; regenerated `algMinhash256` is `throws`; `swift` CI job green
- `release.yml` commits the checksum, then tags once (`fetch-depth: 0`, an existing tag is kept)

## Documentation

**Status**: met (one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- All 11 how-to pages plus the API pages name the empty-list minhash error
- `low`: partial version-sync file lists: `docs/development.md` names 9 of 22, the release skill 13
- Pending with the bits work: API pages must state the 32 to 256 `bits` range

## Benchmarks

**Status**: met (one `low`)

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- `low`: `.iai-baseline.json` holds pre-dedup `bench_instance_code` rows (about 2× measured); a
    refresh is a `cid(human)` call

## CI/CD and Publishing

**Status**: met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles;
    `prepare-release` commits vcpkg hashes with the Swift checksum before tagging; 22 version-sync
    targets at 0.7.0; every ecosystem installs from a lockfile; CRAP baseline regenerated
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45017462731 on the origin tip `898b99a1`, 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45017462731
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

16 open in issues.md: 2 `normal`, 14 `low` (13 untriaged), no `critical`. Both `normal` are decided
by Titusz (2026-11-22) and loop-workable: the minhash issue is narrowed to the shared
`data-edge.json` section; the 256-bit unit limit has an upstream report but is not blocked on it. 7
sit in `postponed.md`.

## Next Milestone

Close the `alg_minhash_256` empty-list issue with a shared `data-edge.json` section running on all
12 surfaces, then implement the decided 256-bit unit limit across the core and all surfaces. After
that, Titusz cuts v0.8.0 via `/release`.
