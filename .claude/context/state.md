<!-- assessed-at: cf3caf071ad4c9e003c0e965fbd46bcc286c0bd6 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; CI green; 4 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 288 closed the
Rust API tier-docs issue: every public `iscc-lib` path now has a tier (Tier 2 = `codec` plus
`streaming::SumHasher`), and the docs no longer cite an `iscc_lib::api` module. 4 `normal` and 14
`low` issues remain.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section
- C/C++ read the generated `data_edge_vectors.h`. Strings longer than 4,095 bytes become static
    buffers filled by `iscc_edge_init()`
- `low`: those 4,095-byte pieces are over the 2,048 bytes that one MSVC reference page allows.
    Nobody has measured it (no MSVC in CI)
- The ragged video vector carries a `diverges` marker. For meta, napi, WASM, JNI and .NET leave
    `name`, `description`, `meta` and `metahash` uncompared. There is no `alg_minhash_256` section

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; three `normal` issues)

- Tier 1 has 35 symbols at the crate root, plus the Tier 2 `read_unit` and `SEQ_MAINTYPES`.
    `conformance_selftest` runs the seq vectors; `src/` has 315 `#[test]`
- Tier list complete: `rust-core.md` names `streaming::SumHasher` as Tier 2 under the same SemVer
    lock as `codec`; `IsccError`/`IsccResult` and `types` structs are uncounted support types
- `normal`: `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; no decision yet
- `normal`: the one-shot gen functions re-implement streaming (BLAKE3 runs twice, `meta` is parsed
    twice)
- `normal`: nine `test_gen_*_conformance` tests in `lib.rs` re-parse `data.json` beside
    `conformance.rs`, and the Rust and Go runners disagree on malformed vectors. **Postponed
    (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); committed `package-lock.json`, `npm ci` in CI and release
- `crates/iscc-napi/README.md` names all 35 Tier 1 symbols; `FULL` in the Tier 1 doc gate
- `low`: `napi build` fails in the devcontainer checkout (the 9p mount rejects `link()`); CI is
    unaffected. The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal`, two `low`)

- 35 Tier 1 symbols; SIMD release build met; scalar parameters type-checked in Rust; Node.js setup
    for the `--target web` package documented and pinned by `tests/test_wasm_docs.py`
- `normal`: hasher `finalize()` consumes the hasher when the core rejects `bits`, but not when JS
    rejects it. No cross-binding rule is recorded and no test covers it
- `low`: byte and string parameters are converted in the glue code; a list element that throws
    during the serde read kills the instance

## C FFI

**Status**: met

- 51 exports, including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- The release upload prunes unfinished assets and verifies every archive (see CI/CD)

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries; the CI `java` job tests both the CI and the `test-jni`
    layouts
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; the `go` CI job also runs as `GOARCH=386`; `GenMetaCodeV0` enforces the meta limit
- Go per-function tests re-read `data.json` beside `conformance.go` (tracked under Rust core)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` runs on .NET 8.0 and 10.0; `NativeMethods.g.cs` is diffed against a fresh
    csbindgen
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job
- `packages/cpp/portfile.cmake` pins the five v0.7.0 SHA512s; `scripts/update_vcpkg_hashes.py`
    writes them in `prepare-release` from the built archives
- `low`: the port fails one vcpkg post-build check (debug/release binary count); matters only for a
    vcpkg registry submission

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor 3.3 (gemspec, CI, and the
    `release.yml` cross-gems for 3.3 and 3.4)
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope; one `low` spec mismatch)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; 0.7.0 shipped; the `kotlin` CI job diffs regenerated UniFFI outputs
- `low`: target.md says the release workflow force-updates the tag, but the Swift spec describes
    committing the checksum and then tagging once (the target.md edit needs Titusz)

## Documentation

**Status**: partially met (two `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (22 `FULL`, 2 `EXACT`)
- `docs/architecture.md`, `docs/rust-api.md`, root and crate `CLAUDE.md` now match `lib.rs`: no
    `iscc_lib::api` reference outside `notes/`; `SumHasher` documented as Tier 2
- `low`: `target.md` and the advance/review prompts still name `codec` as the whole of Tier 2
    (Titusz's files)
- `low`: the version-sync file lists in `docs/development.md` (9 of 22 targets) and the release
    skill (13 of 22) are partial

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (CI green; one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles;
    `prepare-release` commits the vcpkg hashes with the Swift checksum before tagging; 22
    version-sync targets at 0.7.0; every ecosystem installs from a lockfile
- `publish-ffi` runs `scripts/ffi_release_assets.py stale` before the upload and `verify` after it.
    A live `starter` deletion is unproven (needs a failed upload); `release.yml` is never run by CI
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 44991356360 on the origin tip `031d6de9`: 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/44991356360
- Local HEAD adds one unpushed log commit that touches only `.claude`

## Open Issues

18 open in issues.md: 4 `normal`, 14 `low` (13 untriaged), no `critical`. Two need a behaviour
decision first (`alg_minhash_256([])`, WASM `finalize()`). None is blocked on upstream. 7 sit in
`postponed.md`.

## Next Milestone

Clear the 4 `normal` issues that block v0.8.0: the one-shot/streaming duplication, the duplicate
`data.json` parsers, and the two behaviour decisions (these need Titusz). Then Titusz cuts v0.8.0
via `/release`.
