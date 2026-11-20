<!-- assessed-at: e583e06 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 3 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 237 finished the data-edge "Output subset" fan-out: the .NET, Java, napi
and WASM harnesses now compare every expected key outside a static per-section set, and that
`normal` issue is closed. 3 `normal` issues (Java how-to, Ruby precompiled gems, derived-fact drift)
still block v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 6 sections: `iscc_decode`, `encode_seq`,
    `gen_text_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `alg_simhash`; harnesses on all 12
    surfaces; the 5 tracked copies are byte-identical and match `VENDORED_COPIES`
- Python validator and C/C++ generator enforce the "Output subset" rule (allowed keys derived from
    `data.json`, unknown keys rejected, negative tests)
- `DataEdgeTests.cs`, `DataEdgeTest.java`, `data_edge.test.mjs` and WASM `data_edge.rs` each hold a
    static uncompared set (`gen_text_code_v0` → `characters`, `gen_mixed_code_v0` → `parts`), fail
    on a missing key, and carry the two negative tests plus a synthetic positive
- Not filed: the `parts` entry is untested on any surface (mixed has only an error vector); the WASM
    `compare` has no explicit non-object branch

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- No core source changed since iteration 235
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- Python 3.10 floor is past end of life (2026-10-01) — `low` issue, human-only decision

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests green
    in CI; `@iscc/lib` 0.7.0 on npm (OIDC); `ci.yml` napi job on Node `22`
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests green in CI; SIMD release build
    met
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — a changed published export shape, worth a
    v0.8.0 release-notes line

## C FFI

**Status**: met (v0.8.0 scope)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`; the generator rejects
    unknown output keys

## Java Bindings

**Status**: met (v0.8.0 scope); 1 `normal` doc issue

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections with the strict compare, green in
    CI; 0.7.0 on Maven Central
- **`normal`:** `docs/howto/java.md` `genSumCodeV0` example passes 3 args (method takes 4) and calls
    accessors on `public final` fields; it does not compile
- `low` issue: the pending-exception guard has no test

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `data_edge_test.go` runs all sections, green in CI
- `go.mod`: `go 1.26.1` floor, `toolchain go1.26.11`, `golang.org/x/text v0.41.0`

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `PinnedRows` pins all jagged-array inputs; `DataEdgeTests.cs` runs all
    6 sections with the strict compare, green in CI; 0.7.0 on NuGet
- .NET 8 floor past end of life (2026-11-10) — `low` issue, human-only

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run from the generated
    header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan

## Ruby Bindings

**Status**: partially met — 1 `normal` issue

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections
    strictly, green in CI; 0.7.0 on RubyGems
- **`normal`:** the `release.yml` cross-gem list is Ruby 3.1/3.2/3.3 (tied to the rb_sys 0.9.123
    pin), so published platform gems declare `< 3.4.dev`; Ruby 3.4/4.0 users fall back to a source
    build needing Rust. Docs promise precompiled gems without naming this limit
- Ruby 3.1 floor is EOL (`low`, human-only); `low` issue: integer list elements accept a Float

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift 30 tests (19 conformance, 11 data-edge); Kotlin `DataEdgeTest` runs all 6
    sections; both green in CI and shipped 0.7.0

## Documentation

**Status**: partially met — 1 `normal` `[human]` issue (+ the Java how-to above)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs
- Tier 1 doc gate `scripts/check_tier1_docs.py` (prek hook + pytest) checks registered docs name
    every symbol derived from `lib.rs`
- **Remaining in the `[human]` drift issue:** the gate fails open on `lib.rs` forms it does not
    parse; `docs/howto/rust.md`, `docs/howto/c-cpp.md`, `docs/rust-api.md` and
    `docs/architecture.md` are unregistered and incomplete; hand-written counts remain in specs and
    per-crate `CLAUDE.md` files

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- Security criterion met: advisory-flagged pins bumped, no CI test runtime EOL except the exempt
    floors (Python 3.10, .NET 8, Ruby 3.1 — `low` issue for Titusz). Only Rust has a standing
    advisory gate (`cargo deny`)
- **Latest CI: PASSING** on `origin/develop` = `752024f` (`github-actions` suite `success`, 21/21
    check-runs `success`): https://github.com/iscc/iscc-lib/actions/runs/44820365462
- Unpushed: only the `cid(log)` commit (`.claude/` only) — green CI covers HEAD's code

## Open Issues

11 open: 0 critical, **3 normal**, 8 low. Normal: derived-fact drift `[human]`, Java how-to
`genSumCodeV0` example, precompiled Ruby gems stop at 3.3. issues.md is at 213 of 300 lines.

## Next Milestone

Clear the 3 open `normal` issues — the remaining v0.8.0 release blockers: the broken Java how-to
example, precompiled Ruby gem coverage (docs or release matrix), and the remaining derived-fact
drift.
