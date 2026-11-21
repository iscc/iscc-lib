<!-- assessed-at: e78cfe529a70b29606c3a7e738ac3973005a7cac -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 7 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings, and all code is pushed and
covered by green CI. Iteration 282 made the generated C edge header split long strings into
4,095-byte literals, closing the MSVC `normal` issue; review filed a `low` follow-up because one
Microsoft page documents a 2,048-byte limit. 17 issues are open: 7 `normal`, 10 `low`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical
    (about 144 KB each); all 12 harnesses run every section
- C/C++ read the generated `data_edge_vectors.h` (150 KB); strings over 4,095 bytes become static
    buffers filled by `iscc_edge_init()`, which both test programs call
- `low`: 4,095-byte pieces exceed the 2,048 bytes one MSVC reference page allows; unmeasured (no
    MSVC in CI)
- The ragged video vector carries a `diverges` marker; napi, WASM, JNI and .NET leave `name`,
    `description`, `meta`, `metahash` uncompared for meta. No `alg_minhash_256` section

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; four `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 315 `#[test]` in `src/`
- `normal`: `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; no decision
- `normal`: one-shot gen functions re-implement streaming (double BLAKE3, double `meta` parse)
- `normal`: nine `test_gen_*_conformance` tests in `lib.rs` re-parse `data.json` beside
    `conformance.rs`; the Rust and Go runners disagree on malformed vectors
- `normal`: public `streaming::SumHasher` and `IsccError` sit in no tier list; docs and `CLAUDE.md`
    cite a nonexistent `iscc_lib::api` module. **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (two `normal`, two `low`)

- 35 Tier 1 symbols; SIMD release build met; scalar parameters type-checked in Rust
- `normal`: `docs/howto/wasm.md` and the crate README show a Node.js setup the `--target web`
    package cannot run; `wasm.md:82` still says all 10 gen functions return a string
- `normal`: hasher `finalize()` consumes the hasher on a core-rejected `bits` but not on a
    JS-rejected one; no cross-binding rule recorded, no test
- `low`: byte/string parameters glue-converted; a throwing serde-read list element kills the
    instance

## C FFI

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- The test program builds against the split-literal edge header (MSVC doubt: see data-edge)

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries; 65 `@Test` in `IsccLibTest.java`
- CI `java` job runs the suite in both the CI and the `release.yml` → `test-jni` layouts
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; `go` CI job also runs as `GOARCH=386`
- `GenMetaCodeV0` rejects meta over `MetaTrimMeta`, with boundary tests for both meta forms
- Go per-function tests re-read `data.json` beside `conformance.go` (tracked under Rust core)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs; `NativeMethods.g.cs` diffed against fresh csbindgen in `c-ffi`
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run (test calls
    `iscc_edge_init()`); ASAN `cpp` job
- `normal`: `packages/cpp/portfile.cmake` still pins the five v0.2.0 tarball SHA512s while the URL
    follows `vcpkg.json` 0.7.0, so the vcpkg port cannot install; no gate checks it

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor 3.3 (gemspec, CI, `release.yml`
    cross-gems 3.3 and 3.4); dev Ruby 3.4.13 pinned in `mise.toml`
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope; one `low` spec mismatch)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0; 30 `#[test]` in iscc-uniffi
- The `kotlin` CI job regenerates the UniFFI Kotlin + Swift outputs and diffs them
- `low`: target.md says the release workflow force-updates the tag; the Swift spec describes
    commit-checksum-then-tag-once (target.md edit needs Titusz)

## Documentation

**Status**: partially met (two `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: WASM Node.js setup (see WASM); Rust API tier docs (see Rust core)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0; the vcpkg hashes are not a release step (see C++)
- `low`: no CI job runs a test suite on Windows (so MSVC header limits go unseen); `iscc-ffi`, napi
    and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 44963348649 on the origin tip `2ccf9f1` is green on all 24 jobs:
    https://github.com/iscc/iscc-lib/actions/runs/44963348649. Local HEAD adds one unpushed
    `.claude`-only log commit; no code is uncovered

## Open Issues

17 open in issues.md: 7 `normal`, 10 `low` (9 untriaged), no `critical`. Two need a behaviour
decision first (`alg_minhash_256([])`, WASM `finalize()`); none is blocked on upstream. 7 sit in
`postponed.md`.

## Next Milestone

Clear the 7 `normal` issues that block v0.8.0 — the vcpkg hashes and the WASM Node.js docs are
user-facing gaps; the Rust API tier docs, duplicate `data.json` parsers, one-shot/streaming
duplication and the two behaviour decisions follow. Then Titusz cuts v0.8.0 via `/release`.
