<!-- assessed-at: c8ccc72f459b9bffb5784baf60ac2395b5b0dfba -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 10 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 276 rewrote
`conformance_selftest` in Rust and Go around one generic section runner and closed that audit issue.
CI on `f6a43b8` is fully green; the run on the pushed tip `7bbb1f0` (which carries the selftest
rewrite) is 22/24 green with Kotlin and Bench still running. 17 issues are open: 0 `critical`, 10
`normal`, 7 `low`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    C/C++ read the generated `data_edge_vectors.h`; all 12 harnesses run every section
- The ragged video vector carries a `diverges` marker; napi, WASM, JNI and .NET leave `name`,
    `description`, `meta`, `metahash` uncompared for meta
- No vector from 2^29 up, on purpose (decision of 2026-11-21); no lone-surrogate vector (serde_json
    cannot load it; Go unit tests cover it); no `alg_minhash_256` section (see Rust core)

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; two `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 315 `#[test]` in `src/`
- `conformance.rs` runs all sections through one `run_section` + `report_case` pair (audit issue
    closed 276); `.crap-baseline.json` regenerated
- `normal` (audit): `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; five
    per-surface tests pin it and no decision covers it
- `normal` (audit): one-shot gen functions re-implement streaming (double BLAKE3, double `meta`
    parse, WASM `SumHasher` copy)
- **Postponed (human):** crate `>= 1.0.0`; MSRV verification; `no_std` support

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- The pre-push `pytest` hook and `mise run test` run `mise run py:develop` first (debug build)
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); numeric parameters are range-checked
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal` docs issue, two `low`)

- 35 Tier 1 symbols; SIMD release build met; scalar parameters type-checked in Rust; tests 11
    conformance + 8 data-edge + 126 unit
- `normal` (audit): `docs/howto/wasm.md` and the crate README show a Node.js setup the published
    `--target web` package cannot run, and say every gen function returns a string
- `low`: byte/string parameters are still glue-converted; serde-read list inputs let a throwing
    element kill the instance

## C FFI

**Status**: met (one `normal` docs issue)

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- `normal`: the pkg-config example in `docs/howto/c-cpp.md` says `Version: 0.2.0`

## Java Bindings

**Status**: partially met (one `normal` issue)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- `normal`: `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; `go` CI job also runs as `GOARCH=386`
- `conformance.go` uses one generic `runSection` + `reportCase` (276); a too-short `inputs` array is
    now a parse failure, not a panic

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs
- `NativeMethods.g.cs` is diffed against a fresh csbindgen run in the `c-ffi` CI job
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `iscc.hpp` passes `.data()`, so empty vectors reach the FFI as `(NULL, 0)`

## Ruby Bindings

**Status**: partially met (one `normal`, three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor is 3.3 (gemspec `>= 3.3.0`, CI `ruby`
    job 3.3, `release.yml` cross-gems 3.3 and 3.4); dev Ruby 3.4.13 pinned in `mise.toml`
- `normal`: integer list elements accept a Float and truncate it
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 precompiled binary
    before publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0; 30 `#[test]` in iscc-uniffi
- The `kotlin` CI job regenerates the UniFFI Kotlin + Swift outputs and diffs them against the
    committed copies (modulo trailing whitespace)

## Documentation

**Status**: partially met (five `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; Sum-Code examples show `KAA`;
    docs overstate language coverage and ISO 24138 scope; WASM Node.js setup (see WASM)
- `normal`: `docs/development.md` "Publish" says a tag push triggers the release
- The pkg-config version entry is counted under C FFI

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (one `low`; release criteria otherwise met)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- Generated-artifact gates: `c-ffi` diffs `NativeMethods.g.cs`, `kotlin` diffs the UniFFI outputs;
    anchored by `tests/test_generated_artifact_gates.py`
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: IN PROGRESS** on the pushed tip `7bbb1f0`:
    https://github.com/iscc/iscc-lib/actions/runs/44950595748 — 22 of 24 jobs green, Kotlin and
    Bench (compile check) still running. Newest fully green run is on `f6a43b8` (run 44948765142);
    it predates the selftest rewrite, so that code is not yet covered by a completed green run.
    HEAD's only unpushed commit is `.claude/` only

## Open Issues

17 open in issues.md: 0 `critical`, 10 `normal`, 7 `low`; none blocked on the human or upstream (the
`alg_minhash_256` entry needs a decision). The `conformance_selftest` duplication issue was closed
this iteration. 7 sit in `postponed.md`.

## Next Milestone

Confirm the tip CI run finishes green, then clear the 10 `normal` issues that block v0.8.0 — the
code defects (JNI loader fallback, Ruby Float truncation, `alg_minhash_256` decision, one-shot gen
duplication) and the user-docs accuracy entries. Then Titusz cuts v0.8.0 via `/release`.
