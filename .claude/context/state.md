<!-- assessed-at: 24ae20ac2957ce015d3b4a10085b452bb37b9fed -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 13 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 273 removed the
C++ `safe_data` sentinel and the .NET `PinnedRows` padding, closing that issue. CI is green on the
pushed tip `621bd77`, which covers all code at HEAD. 19 issues are open: 0 `critical`, 13 `normal`,
6 `low`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    C/C++ read the generated `data_edge_vectors.h`; all 12 harnesses run every section
- The ragged video vector carries a `diverges` marker; napi, WASM, JNI and .NET leave `name`,
    `description`, `meta`, `metahash` uncompared for meta
- No vector from 2^29 up, on purpose (decision of 2026-11-21); no lone-surrogate vector (serde_json
    cannot load it; Go unit tests cover it); no `alg_minhash_256` section (see Rust core)

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; three `normal` issues)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 311 `#[test]` in `src/`
- `normal` (audit): `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; five
    per-surface tests pin it and no decision covers it
- `normal` (audit): `conformance_selftest` repeats one runner loop nine times (Rust and Go)
- `normal` (audit): one-shot gen functions re-implement streaming (double BLAKE3, double `meta`
    parse, WASM `SumHasher` copy); specs and crate `CLAUDE.md` files are stale
- **Postponed (human):** crate `>= 1.0.0`; MSRV verification; `no_std` support

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `specs/python-bindings.md` still says text/video paths hold the GIL (part of the spec issue)

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

**Status**: met (v0.8.0 scope; shares two audit issues)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; `go` CI job also runs as `GOARCH=386`
- Shares the `conformance_selftest` duplication and the spec-staleness issue (`io.Reader` claim)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs
- `PinnedRows` sizes arrays to `Count` (no `Math.Max` padding); `ArrayInputTests` covers an empty
    inner row
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `safe_data` is gone (no match in `packages/`, `tests/`, `docs/`); `iscc.hpp` passes `.data()`, so
    empty vectors reach the FFI as `(NULL, 0)`; `tests/test_cpp_safe_data.py` deleted

## Ruby Bindings

**Status**: partially met (one `normal`, three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor is 3.3 (gemspec `>= 3.3.0`, CI `ruby`
    job 3.3, `release.yml` cross-gems 3.3 and 3.4); minitest 6.1.0
- Development Ruby pinned in root `mise.toml` (`ruby = "3.4.13"`); both standardrb hooks run
    `mise exec --`; guarded by `tests/test_ruby_gem_docs.py`
- `normal`: document the implicit Float→Integer `to_int` conversion
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 precompiled binary
    before publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0
- `text_trim` saturates `nbytes` via `usize::try_from(..).unwrap_or(usize::MAX)`; the doc comment
    matches the core and is regenerated into both bindings; 30 `#[test]` in iscc-uniffi
- The 32-bit wrap test only bites on a 32-bit target, which CI does not run for Rust (settled)

## Documentation

**Status**: partially met (six `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; Sum-Code comments show `KAA`;
    docs overstate language coverage and ISO 24138 scope; WASM Node.js setup (see WASM)
- `normal`: `docs/development.md` "Publish" says a tag push triggers the release
- `normal` (audit): spec/`CLAUDE.md` staleness

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (one `normal`, one `low`; release criteria otherwise met)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- `normal` (audit): gates test generated/built artifacts nothing refreshes or diffs (stale local
    `_lowlevel.abi3.so` under pre-push pytest; `NativeMethods.g.cs` and UniFFI outputs undiffed —
    UniFFI outputs are in a known-clean state as of `06238f6`)
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING** on `origin/develop` = `621bd77`:
    https://github.com/iscc/iscc-lib/actions/runs/44944695726 — github-actions suite `success`;
    HEAD's only unpushed commit is `.claude/` only

## Open Issues

19 open in issues.md: 0 `critical`, 13 `normal`, 6 `low`; none blocked on the human or upstream (the
`alg_minhash_256` entry needs a decision). The C++/.NET padding issue was closed this iteration. 7
sit in `postponed.md`.

## Next Milestone

Clear the 13 `normal` issues that block v0.8.0 — the code defects (JNI loader fallback, Ruby Float
coercion docs, `alg_minhash_256` decision, duplication), the undiffed generated-artifacts gate, and
the docs/spec accuracy entries. Then Titusz cuts v0.8.0 via `/release`.
