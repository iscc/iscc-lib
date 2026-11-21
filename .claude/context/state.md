<!-- assessed-at: d4d4d593f6028106f80e54e94cf5ae12b030ce37 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; a `critical` JNI test regression and 5 `normal` issues block it

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 278 rewrote the
JNI `NativeLoader` fallback (review: loader correct) but its three new child-JVM tests fail in the
`release.yml` → `test-jni` layout, so the next Maven publish would be blocked; the commit is local
and unpushed (NEEDS_WORK). CI on the pushed tip `c6bc616` is fully green. 15 issues are open: 1
`critical`, 5 `normal`, 9 `low`.

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
    `conformance_selftest` runs the seq vectors through one `run_section`; 315 `#[test]` in `src/`
- `normal` (audit): `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; five
    per-surface tests pin it and no decision covers it
- `normal` (audit): one-shot gen functions re-implement streaming (double BLAKE3, double `meta`
    parse, WASM `SumHasher` copy)
- **Postponed (human):** crate `>= 1.0.0`; MSRV verification; `no_std` support

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
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

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`

## Java Bindings

**Status**: partially met (one `critical` release regression at local HEAD, one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- Local HEAD (`8a99a3c`, unpushed): `NativeLoader` no longer throws on an unknown OS/arch; it falls
    back to `System.loadLibrary` and attaches the bundled-library error as suppressed; docs, README
    and crate CLAUDE.md updated
- `critical`: the three `loaderFallsBack*` child-JVM tests pass the parent's `java.library.path`
    (`target/debug`), which is absent in the `test-jni` release layout, so `publish-maven` would be
    blocked; the CI `java` job stays green because it builds the library first
- `low` (untriaged): each bundled load leaves an empty `iscc-jni-*` temp directory (pre-existing)

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; `go` CI job also runs as `GOARCH=386`

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs; `NativeMethods.g.cs` diffed against fresh csbindgen in `c-ffi`
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor is 3.3 (gemspec, CI, `release.yml`
    cross-gems 3.3 and 3.4); dev Ruby 3.4.13 pinned in `mise.toml`; Float truncation documented
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 precompiled binary
    before publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope; one `low` spec mismatch)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0; 30 `#[test]` in iscc-uniffi
- The `kotlin` CI job regenerates the UniFFI Kotlin + Swift outputs and diffs them
- `low`: target.md still says the release workflow force-updates the tag; the Swift spec describes
    commit-checksum-then-tag-once (target.md edit needs Titusz)

## Documentation

**Status**: partially met (three `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; `docs/ecosystem.md`,
    `architecture.md` and `howto/nodejs.md` still count 10 standard functions; WASM Node.js setup
    (see WASM)
- `docs/howto/java.md` describes the loader fallback at local HEAD (unpushed)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (one `low`; the JNI `critical` would break `test-jni`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- No CI job runs the Java suite in the release layout (library bundled as a resource only), which is
    why the JNI regression is invisible to CI
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING** on the pushed tip `c6bc616`:
    https://github.com/iscc/iscc-lib/actions/runs/44952542030 (github-actions suite `success`).
    **Not covered:** local HEAD carries unpushed code commit `8a99a3c` (JNI loader, tests, docs)

## Open Issues

15 open in issues.md: 1 `critical`, 5 `normal`, 9 `low` (8 `low` untriaged). Only the
`alg_minhash_256` entry needs a decision; none is blocked on upstream. 7 sit in `postponed.md`.

## Next Milestone

Repair the `critical` JNI release-layout test regression so the local loader work can be pushed with
Maven publishing intact, then clear the 5 `normal` issues that block v0.8.0 (`alg_minhash_256([])`
decision, one-shot gen duplication, three user-docs accuracy entries). Then Titusz cuts v0.8.0 via
`/release`.
