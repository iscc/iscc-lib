<!-- assessed-at: 9a140825037d7ad6fb7e39968325e5b85b297188 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 3 decided `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 296 added an
`encode_component` section (10 sections now) on all 12 surfaces, closing the shared-vector issue.
Titusz then decided three behaviours (2026-11-22): `alg_minhash_256([])` is an error, the first
`finalize()` always consumes a hasher, and a unit above 256 bits is rejected. All 3 `normal` issues
are now decided, loop-workable work.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met for the current sections; two `normal` issues each add a section

- `crates/iscc-lib/tests/data-edge.json` has 10 sections, `encode_component` included (short-digest
    error row marked `diverges`, exact-length row); the 5 tracked copies are byte-identical; all 12
    harnesses run every section
- C/C++ read the generated `data_edge_vectors.h`, which carries the `encode_component` struct
- Pending: an `alg_minhash_256` section and a `bits` above 256 rejection vector (see Rust core)
- Java's `DataEdgeTest` section table uses `Map.of`, which tops out at 10 pairs; the next section
    needs `Map.ofEntries`
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two decided `normal` issues)

- Tier 1 has 35 symbols at the crate root. Tier 2 as target.md now lists it exists: `codec`
    (`decode_units`, the enum-typed `encode_component`, `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher`. `src/` has 324 `#[test]`
- `normal` (decided): `alg_minhash_256([])` must report an error on every surface; today it returns
    32 bytes of `0xFF`, and its Rust and Go signatures cannot return an error. Five tests pin `0xFF`
- `normal` (decided, upstream iscc/iscc-core#142): gen functions, hasher `finalize` and
    `encode_component` must reject a unit above 256 bits, and decode must reject such a header.
    Today meta/text/audio/data/instance return codes `iscc_decode` rejects
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope; both core `normal` issues reach it)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness cross-checks iscc-core; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests; `@iscc/lib`
    0.7.0 on npm (OIDC); committed `package-lock.json`, `npm ci` in CI and release
- `low`: `napi build` fails in the devcontainer checkout (9p rejects `link()`); CI is unaffected
- Whether its hashers consume on a failed first `finalize` is unmeasured (WASM issue asks for it)

## WASM Bindings

**Status**: partially met (one decided `normal`, two `low`)

- 35 Tier 1 symbols; SIMD release build met; `gen_sum_code_v0` feeds one core `SumHasher`
- `normal` (decided): `finalize()` on all three hashers runs the JS argument checks before
    `inner.take()`, so a JS-rejected `bits` leaves the hasher usable; needs a fix, a test of both
    kinds, a `wasm.md` sentence, and a check of every other surface with a hasher
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md`

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 10 sections; 0.7.0 on Maven Central with five
    bundled native libraries; the CI `java` job tests both the CI and `test-jni` layouts
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope; core `normal` issues need Go changes too)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    10 sections; the `go` CI job also runs as `GOARCH=386`; 311 `func Test`
- `AlgMinhash256([]uint32) []byte` cannot report an error yet; `GenDataCodeV0` accepts 512 bits

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 10 sections; 0.7.0 on NuGet; `dotnet-test`
    on .NET 8.0 and 10.0; `NativeMethods.g.cs` diffed against a fresh csbindgen
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job;
    `portfile.cmake` SHA512s written in `prepare-release`
- `low`: the port fails one vcpkg post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs all sections; floor 3.3 (gemspec, CI, cross-gems for
    3.3 and 3.4)
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 10 data-edge sections; 0.7.0 shipped; the `kotlin` CI job diffs regenerated UniFFI outputs
- target.md and the Swift spec now say the release tags once and never moves a tag; `release.yml`
    commits the checksum, then tags (`fetch-depth: 0`, so an existing tag is kept; the trailing
    `git push --force` on the tags is redundant)

## Documentation

**Status**: met (one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- `low`: partial version-sync file lists: `docs/development.md` names 9 of 22, the release skill 13
- Pending with the `normal` work: API pages must state the 32 to 256 `bits` range and the WASM
    `finalize` rule

## Benchmarks

**Status**: met (one `low`)

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- `low`: `.iai-baseline.json` holds pre-dedup `bench_instance_code` rows (about 2× measured); a
    refresh is a `cid(human)` call

## CI/CD and Publishing

**Status**: met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles;
    `prepare-release` commits the vcpkg hashes with the Swift checksum before tagging; 22
    version-sync targets at 0.7.0; every ecosystem installs from a lockfile
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45006494417 on the origin tip `e2495d85`, 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45006494417
- Local HEAD adds 2 unpushed commits (`cid(log)`, `cid(human)`) that touch only `.claude`

## Open Issues

16 open in issues.md: 3 `normal`, 13 `low` (12 untriaged), no `critical`. All 3 `normal` are decided
by Titusz (2026-11-22) and loop-workable: `alg_minhash_256([])` error, WASM `finalize()`
consumption, and the 256-bit unit limit. The bits issue has an upstream report but is not blocked on
it. 7 sit in `postponed.md`.

## Next Milestone

Implement the three decided `normal` behaviours across the core and all 12 surfaces, each pinned by
shared `data-edge.json` vectors where the input is JSON-expressible. After that, Titusz cuts v0.8.0
via `/release`.
