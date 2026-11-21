<!-- assessed-at: 908bd7bae481f38d1e806312c448fce1f2a52997 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 6 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 277 was a
docs-accuracy batch that closed five `normal` issues (pkg-config version, Sum-Code prefix, ISO and
language-coverage claims, the Publish section, Ruby Float docs) and filed two leftovers. CI on
`7bbb1f0` is fully green; the run on the pushed tip `c6bc616` is 22/24 green with Kotlin and Perf
still running. 14 issues are open: 0 `critical`, 6 `normal`, 8 `low`.

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
- The pkg-config example no longer pins a stale version (closed 277)

## Java Bindings

**Status**: partially met (one `normal` issue)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- `normal`: `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; `go` CI job also runs as `GOARCH=386`
- `conformance.go` uses one generic `runSection` + `reportCase`

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

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor is 3.3 (gemspec, CI, `release.yml`
    cross-gems 3.3 and 3.4); dev Ruby 3.4.13 pinned in `mise.toml`
- Float-to-integer truncation is now documented behaviour: decisions.md "Ruby integer parameters
    keep Ruby's implicit `to_int` conversion"; `docs/ruby-api.md` states it (closed 277)
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 precompiled binary
    before publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope; one `low` spec mismatch)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0; 30 `#[test]` in iscc-uniffi
- The `kotlin` CI job regenerates the UniFFI Kotlin + Swift outputs and diffs them against the
    committed copies
- `specs/swift-bindings.md` now describes commit-checksum-then-tag-once; target.md still says the
    release workflow force-updates the tag (`low`, untriaged; target.md edit needs Titusz)

## Documentation

**Status**: partially met (three `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- Closed 277: Sum-Code examples now show `KUA`; `docs/llms.txt` names all languages; ISO scope
    claims and the `development.md` Publish section match `release.yml`
- `normal`: `architecture.md` lower sections cover half the surfaces; `docs/ecosystem.md`,
    `architecture.md` and `howto/nodejs.md` still count 10 standard functions; WASM Node.js setup
    (see WASM)

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
- **Latest CI: IN PROGRESS** on the pushed tip `c6bc616`:
    https://github.com/iscc/iscc-lib/actions/runs/44952542030 — 22 of 24 jobs green, Kotlin and Perf
    (iai-callgrind) still running. Run 44950595748 on `7bbb1f0` (selftest rewrite) finished fully
    green. The tip delta since then is docs, doc comments and one Ruby comment block only. HEAD's
    only unpushed commit is `.claude/` only

## Open Issues

14 open in issues.md: 0 `critical`, 6 `normal`, 8 `low` (7 `low` untriaged). Only the
`alg_minhash_256` entry needs a decision; none is blocked on upstream. 7 sit in `postponed.md`.

## Next Milestone

Confirm the tip CI run finishes green, then clear the 6 `normal` issues that block v0.8.0: the JNI
loader fallback, the `alg_minhash_256([])` decision, the one-shot gen duplication, and the three
user-docs accuracy entries (architecture sections, the 10-function count, `@iscc/wasm` Node setup).
Then Titusz cuts v0.8.0 via `/release`.
