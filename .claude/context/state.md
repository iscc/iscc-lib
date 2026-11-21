<!-- assessed-at: 8c3f8305959488bd7445f998bb10f4fa12d35964 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 16 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 270 raised the
Ruby floor to 3.3 (closing that `[human]` issue), and the iteration-270 audit filed 5 more `normal`
issues. CI is green on the pushed tip `0fcdae3`, which covers all code at HEAD. 22 issues are open:
0 `critical`, 16 `normal`, 6 `low`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    C/C++ read the generated `data_edge_vectors.h`; all 12 harnesses run every section
- The ragged video vector carries a `diverges` marker; napi, WASM, JNI and .NET leave `name`,
    `description`, `meta`, `metahash` uncompared for meta
- No vector from 2^29 up, on purpose (decision of 2026-11-21); no lone-surrogate vector (serde_json
    cannot load it; Go unit tests cover it); no `alg_minhash_256` section (see Rust core)

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; four `normal` issues)

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

**Status**: partially met (one `normal` issue)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` 8.0/10.0 legs
- `normal`: remove the `PinnedRows` padding (and C++ `safe_data`) — decided
- .NET 8 reached end of support 2026-11-10; the `net8.0` floor is the human's call

## C++ Bindings

**Status**: partially met (one `normal` issue, shared with .NET)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `normal`: remove the redundant `safe_data` padding

## Ruby Bindings

**Status**: partially met (two `normal`, three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor is 3.3 (gemspec `>= 3.3.0`, CI `ruby`
    job 3.3, `release.yml:850` cross-gems 3.3 and 3.4, parity-tested); minitest 6.1.0
- `normal`: a rebuilt devcontainer cannot run the Ruby gates (`post-create.sh` bundles under system
    Ruby 3.1; nothing pins Ruby in `mise.toml`)
- `normal`: document the implicit Float→Integer `to_int` conversion
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 precompiled binary
    before publish; Ruby 4.0 gets no precompiled gem (needs rb_sys past 0.9.123)

## Swift and Kotlin Bindings

**Status**: partially met (one `normal` issue)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; shipped 0.7.0
- `normal` (audit): `text_trim` casts `nbytes as usize`, truncating on 32-bit Android ABIs, and its
    doc comment (copied into the generated Swift/Kotlin) says "word boundary"

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
    `_lowlevel.abi3.so` under pre-push pytest; `NativeMethods.g.cs` and UniFFI outputs undiffed)
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING** on `origin/develop` = `0fcdae3`:
    https://github.com/iscc/iscc-lib/actions/runs/44935056667 — first run of the `ruby` job on 3.3;
    the unpushed commits touch `.claude/` only

## Open Issues

22 open in issues.md: 0 `critical`, 16 `normal`, 6 `low`; none blocked on the human or upstream (the
`alg_minhash_256` entry needs a decision). The Ruby floor issue was closed; review filed the
devcontainer `normal` and two Ruby `low`; the audit filed 5 `normal`. 7 sit in `postponed.md`.

## Next Milestone

Clear the 16 `normal` issues that block v0.8.0 — loop/tooling health (devcontainer Ruby gates,
undiffed generated artifacts) first, then the code defects (UniFFI `text_trim`, FFI padding, JNI
loader fallback, `alg_minhash_256` decision, duplication) and the docs/spec accuracy entries. Then
Titusz cuts v0.8.0 via `/release`.
