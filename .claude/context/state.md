<!-- assessed-at: 909e7b4a4410772ee778ca5f3a0fc35d3d528d7d -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 2 decided `normal` issues block the release

ISCC-SEQ and `data-edge.json` (10 sections) run on the Rust core and all 11 bindings. Iteration 297
made the first hasher `finalize()` consume the hasher on WASM, napi, JNI, Python and Ruby, whatever
rejects the argument, and closed that `normal` issue. Two decided `normal` issues remain:
`alg_minhash_256([])` must be an error, and a unit above 256 bits must be rejected.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met for the current sections; both `normal` issues each add a section

- `crates/iscc-lib/tests/data-edge.json` has 10 sections, `encode_component` included; the 5 tracked
    copies are byte-identical; all 12 harnesses run every section
- C/C++ read the generated `data_edge_vectors.h`
- Pending: an `alg_minhash_256` section and a `bits` above 256 rejection vector (see Rust core)
- Java's `DataEdgeTest` section table uses `Map.of`, which tops out at 10 pairs; the next section
    needs `Map.ofEntries`
- `low`: the 4,095-byte string pieces exceed the 2,048 bytes one MSVC reference page allows

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two decided `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `codec` (incl. `read_unit`, `SEQ_MAINTYPES`) and
    `streaming::SumHasher` exist. `src/` has 324 `#[test]`
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
- Hasher `finalize()` takes the inner hasher before converting arguments; `tests/test_streaming.py`
    pins rejected-first-`finalize` consumption
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests; `@iscc/lib`
    0.7.0 on npm (OIDC); committed `package-lock.json`, `npm ci` in CI and release
- Hasher `finalize(bits?)` takes `Option<Unknown>` and consumes before checking; crate has no
    `unsafe` block
- `low`: `napi build` fails in the devcontainer checkout (9p rejects `link()`); CI is unaffected

## WASM Bindings

**Status**: met (v0.8.0 scope; two `low`)

- 35 Tier 1 symbols; SIMD release build met; `gen_sum_code_v0` feeds one core `SumHasher`
- All three hashers `take()` the inner hasher before `opt_u32` checks; 10 `*_consumes` tests in
    `tests/unit.rs`; `docs/howto/wasm.md` states the rule
- `low`: byte/string conversion in the glue; a throwing list element during serde kills the instance

## C FFI

**Status**: met (one `low`)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md`
- `low`: no test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift, Kotlin (they
    comply by code order)

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 10 sections; hasher `finalize` consumes before
    the `bits` check, pinned in `IsccLibTest.java`; 0.7.0 on Maven Central with 5 native libraries
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope; core `normal` issues need Go changes too)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all
    10 sections; the `go` CI job also runs as `GOARCH=386`
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

- `encode_seq` / `decode_seq`; data-edge runs all sections; `_finalize` converts `bits` after taking
    the inner hasher, pinned in `test_iscc_lib.rb`; floor 3.3
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 10 data-edge sections; 0.7.0 shipped; the `kotlin` CI job diffs regenerated UniFFI outputs
- `release.yml` commits the checksum, then tags once (`fetch-depth: 0`, an existing tag is kept)

## Documentation

**Status**: met (one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate
- How-to pages for 8 surfaces plus `java-api.md` / `ruby-api.md` state "first `finalize()` consumes,
    even when it fails"; `docs/howto/java.md`, `docs/c-ffi-api.md` and the Javadoc are less explicit
- `low`: partial version-sync file lists: `docs/development.md` names 9 of 22, the release skill 13
- Pending with the `normal` work: API pages must state the 32 to 256 `bits` range

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
- **Latest CI: PASSING.** Run 45011030542 on the origin tip `869abbbf`, 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45011030542
- Local HEAD adds 1 unpushed `cid(log)` commit that touches only `.claude`

## Open Issues

16 open in issues.md: 2 `normal`, 14 `low` (13 untriaged), no `critical`. Both `normal` are decided
by Titusz (2026-11-22) and loop-workable: the `alg_minhash_256([])` error and the 256-bit unit
limit. The bits issue has an upstream report but is not blocked on it. 7 sit in `postponed.md`.

## Next Milestone

Implement the two remaining decided `normal` behaviours (`alg_minhash_256([])` error, `bits` above
256 rejected) across the core and all 12 surfaces, each pinned by shared `data-edge.json` vectors.
After that, Titusz cuts v0.8.0 via `/release`.
