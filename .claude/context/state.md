<!-- assessed-at: da761cae4c9c951c49b630fa17e0a0d01874d6e4 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; CI green; 7 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 284 committed the
napi `package-lock.json` and switched CI and release to `npm ci`, which turned CI green again (24/24
on the origin tip). Review filed two `low` follow-ups: the locked napi cli cannot build on the
devcontainer's 9p mount, and two version-sync file lists are partial.

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

**Status**: partially met (v0.8.0 features met; four `normal` issues)

- Tier 1 has 35 symbols at the crate root, plus the Tier 2 `read_unit` and `SEQ_MAINTYPES`.
    `conformance_selftest` runs the seq vectors; `src/` has 315 `#[test]`
- `normal`: `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; no decision yet
- `normal`: the one-shot gen functions re-implement streaming (BLAKE3 runs twice, `meta` is parsed
    twice)
- `normal`: nine `test_gen_*_conformance` tests in `lib.rs` re-parse `data.json` beside
    `conformance.rs`, and the Rust and Go runners disagree on malformed vectors
- `normal`: the public `streaming::SumHasher` and `IsccError` appear in no tier list, and the docs
    and `CLAUDE.md` cite an `iscc_lib::api` module that does not exist. **Postponed (human):**
    `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: partially met (one `normal`, one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- `crates/iscc-napi/package-lock.json` is tracked (pins `@napi-rs/cli` 3.10.12); the `nodejs` CI job
    and both `release.yml` napi jobs run `npm ci`; `version_sync.py` owns the lock's two root
    version fields
- `normal`: `crates/iscc-napi/README.md:52` says every code generator returns a string, but
    `gen_sum_code_v0` returns a `NapiSumCodeResult`
- `low`: `napi build` fails in the devcontainer checkout (the locked cli hard-links its outputs and
    the 9p mount rejects `link()`); CI is unaffected
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal`, two `low`)

- 35 Tier 1 symbols; SIMD release build met; scalar parameters type-checked in Rust
- `docs/howto/wasm.md` and the crate README show a working Node.js setup for the `--target web`
    package, pinned by `tests/test_wasm_docs.py`; the README is registered `FULL` in the doc gate
- `normal`: hasher `finalize()` consumes the hasher when the core rejects `bits`, but not when JS
    rejects it. No cross-binding rule is recorded and no test covers it
- `low`: byte and string parameters are converted in the glue code; a list element that throws
    during the serde read kills the instance

## C FFI

**Status**: met

- 51 exports, including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`

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

**Status**: partially met (one `normal`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job
- `normal`: `packages/cpp/portfile.cmake` still pins the five SHA512s of the v0.2.0 tarballs while
    the URL follows `vcpkg.json` 0.7.0, so the vcpkg port cannot install; no gate checks it

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

**Status**: partially met (two `normal`, one `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (21 `FULL`, 2 `EXACT`)
- `normal`: napi README result-type sentence (see Node.js); Rust API tier docs (see Rust core)
- `low`: the version-sync file lists in `docs/development.md` (9 of 22 targets) and the release
    skill (13 of 22) are partial

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- The iai baseline reads 3–8% high on 9 of 16 benches (toolchain drift); a refresh is a human call

## CI/CD and Publishing

**Status**: partially met (CI green; one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 22
    version-sync targets at 0.7.0 (the napi lockfile joined); every ecosystem now installs from a
    committed lockfile
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 44967270838 on the origin tip `cac7c1f5`: 24/24 check-runs green. The
    code commit `33d9b437` was green too (run 44966745935):
    https://github.com/iscc/iscc-lib/actions/runs/44967270838
- Local HEAD adds one unpushed log commit that touches only `.claude`

## Open Issues

19 open in issues.md: 7 `normal`, 12 `low` (11 untriaged), no `critical`. Two need a behaviour
decision first (`alg_minhash_256([])`, WASM `finalize()`). None is blocked on upstream. 7 sit in
`postponed.md`.

## Next Milestone

Clear the 7 `normal` issues that block v0.8.0. The vcpkg portfile hashes and the napi README are the
user-facing ones; the Rust API tier docs, the duplicate `data.json` parsers, the one-shot/streaming
duplication and the two behaviour decisions follow. Then Titusz cuts v0.8.0 via `/release`.
