<!-- assessed-at: 1767b8646fede63c0d8b505e5f50fa2ccdb7200a -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 3 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 295 made Go's
public `EncodeComponent` reject a digest shorter than `bit_length/8`, as every Rust-backed surface
does, and recorded the rule in decisions.md. The Go `normal` issue was rewritten to what is left: a
shared `encode_component` vector. 3 `normal` and 16 `low` issues are open; two of the `normal` need
a behaviour decision from Titusz.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met (one `normal` asks for a new section)

- `crates/iscc-lib/tests/data-edge.json` has 9 sections; the 5 tracked copies are byte-identical;
    all 12 harnesses run every section
- C/C++ read the generated `data_edge_vectors.h`. Strings longer than 4,095 bytes become static
    buffers filled by `iscc_edge_init()`
- `normal`: no `encode_component` section, so no shared vector pins the short-digest rule (decided
    2026-11-22); needs a short-digest error row and an exact-length code row on all 12
- `low`: the 4,095-byte pieces exceed the 2,048 bytes one MSVC reference page allows; unmeasured

## Rust Core Crate

**Status**: partially met (v0.8.0 features met; one `normal` issue)

- Tier 1 has 35 symbols at the crate root, plus the Tier 2 `read_unit` and `SEQ_MAINTYPES`.
    `conformance_selftest` runs the seq vectors; `src/` has 324 `#[test]`
- `conformance.rs` is the only file in `src/` that parses `data.json` (decision 2026-11-22)
- `normal`: `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises; no decision yet
- `low` (new, upstream too): gen functions accept `bits` of 288 or more and return a code that
    `iscc_decode` rejects; iscc-core behaves the same
- **Postponed (human):** `>= 1.0.0`, MSRV, `no_std`

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check; CI 3.10/3.15
- `low`: `crates/iscc-py/CLAUDE.md` still lists test files one by one

## Node.js Bindings

**Status**: met (v0.8.0 scope; one `low`)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); committed `package-lock.json`, `npm ci` in CI and release
- `low`: `napi build` fails in the devcontainer checkout (the 9p mount rejects `link()`); CI is
    unaffected. The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal`, two `low`)

- 35 Tier 1 symbols; SIMD release build met; scalar parameters type-checked in Rust;
    `gen_sum_code_v0` feeds one core `streaming::SumHasher`; every snippet calls `await init();`
- `normal`: hasher `finalize()` consumes the hasher when the core rejects `bits`, but not when JS
    rejects it. No cross-binding rule is recorded and no test covers it
- `low`: byte/string conversion in the glue code; a list element that throws during the serde read
    kills the instance

## C FFI

**Status**: met

- 51 exports, including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`

## Java Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 9 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries; the CI `java` job tests both the CI and `test-jni` layouts
- `low`: each bundled load leaves an empty `iscc-jni-*` temp directory behind

## Go Bindings

**Status**: met (v0.8.0 scope; the shared-vector `normal` sits under data-edge)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green on all 9
    sections; the `go` CI job also runs as `GOARCH=386`; 311 `func Test`
- Public `EncodeComponent` (`codec.go:480`) rejects a digest shorter than `bit_length/8`; gen and
    decompose paths call the clamping `encodeComponentInternal`, as Rust uses `codec::`
- `conformance.go` follows the same wrong-type rule as Rust (`parseSection`, `containsNull`)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 9 sections strictly; 0.7.0 on NuGet;
    `dotnet-test` on .NET 8.0 and 10.0; `NativeMethods.g.cs` diffed against a fresh csbindgen
- .NET 8 reached end of support on 2026-11-10; raising the `net8.0` floor is the human's call

## C++ Bindings

**Status**: met (v0.8.0 scope; one `low`)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job;
    `portfile.cmake` SHA512s written in `prepare-release` by `scripts/update_vcpkg_hashes.py`
- `low`: the port fails one vcpkg post-build check (debug/release binary count)

## Ruby Bindings

**Status**: met (v0.8.0 scope; three `low`)

- `encode_seq` / `decode_seq`; data-edge runs strictly; floor 3.3 (gemspec, CI, release cross-gems
    for 3.3 and 3.4)
- `low`: source gems up to 0.7.0 do not build; `release.yml` loads only the 3.3 binary before
    publishing; Ruby 4.0 gets no precompiled gem (needs rb_sys newer than 0.9.123)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope; one `low` spec mismatch)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 9 data-edge sections; 0.7.0 shipped; the `kotlin` CI job diffs regenerated UniFFI outputs
- `low`: target.md says the release workflow force-updates the tag; the Swift spec commits the
    checksum and tags once (the target.md edit needs Titusz)

## Documentation

**Status**: met (two `low`)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (22 `FULL`, 2 `EXACT`)
- `low`: `target.md` and the agent prompts name `codec` as the whole of Tier 2 (Titusz's files); the
    version-sync file lists in `docs/development.md` (9 of 22) and the release skill (13 of 22) are
    partial

## Benchmarks

**Status**: met (one `low`)

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`
- `low`: `.iai-baseline.json` holds the pre-dedup `bench_instance_code` rows (about 2× measured), so
    the +10% gate cannot catch a returning second BLAKE3 pass. A refresh is a `cid(human)` call

## CI/CD and Publishing

**Status**: met (one `low`)

- `ci.yml` has 22 job keys; `release.yml` is `workflow_dispatch`-only with 8 registry toggles;
    `prepare-release` commits the vcpkg hashes with the Swift checksum before tagging; 22
    version-sync targets at 0.7.0; every ecosystem installs from a lockfile
- `scripts/version_sync.py` fails on a stale occurrence, missing target or unmatched pattern (19
    tests)
- `low`: no CI job runs a test suite on Windows; `iscc-ffi`, napi and WASM sit outside iai/CRAP
- **Latest CI: PASSING.** Run 45004014806 on the origin tip `11092f1a`, 24/24 check-runs green:
    https://github.com/iscc/iscc-lib/actions/runs/45004014806
- Local HEAD adds 1 unpushed log commit that touches only `.claude`

## Open Issues

19 open in issues.md: 3 `normal`, 16 `low` (15 untriaged), no `critical`. Two `normal` need a
behaviour decision from Titusz (`alg_minhash_256([])`, WASM `finalize()`); the third (shared
`encode_component` vector) is decided and loop-workable. None is blocked on upstream. 7 sit in
`postponed.md`.

## Next Milestone

Clear the `normal` issues that block v0.8.0. The shared `encode_component` vector is the only one
the loop can move without Titusz; `alg_minhash_256([])` and the WASM `finalize()` rule need his
decision. After that, Titusz cuts v0.8.0 via `/release`.
