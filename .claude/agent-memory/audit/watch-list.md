---
name: audit-watch-list
description: Verified audit findings not filed yet (5-per-pass cap, issues.md budget or ownership), first candidates for the next audit pass
metadata:
  type: project
---

Every item below was verified (or re-verified) at 9cf983e5 (iter 300; 25 of 25 held); the tag shows
the pass that first verified it. Re-check against the tree before filing; drop any a commit has
fixed.

**Why:** each survived a skeptic pass; only the cap, the 400-line issues.md budget or ownership kept
it out. **How to apply:** hand every item to the verify phase next pass; file the top ones first.
target.md/decisions items are `low` + `**Triage:** untriaged` (human-owned).

Code (CID-fixable), ranked:

- (270) Data-edge harness duplication: `iscc-lib/tests/test_data_edge.rs` vs
    `iscc-wasm/tests/data_edge.rs`; Python `tests/test_data_edge.py` vs
    `scripts/gen_data_edge_vectors.py`. Has waited 4 passes; grows with every section.
- (300) C/C++ data-edge path is ~half of every section's cost (alg_minhash_256: ~170 of ~350 harness
    lines): one struct + `row_<section>` + generator test per section; generator stack 1,904 lines.
    Fix: generic tagged-value rows. File when the next section is scoped.
- (300) Core `gen_meta_code_v0` (lib.rs:357-475) and Go `code_meta.go:81-139` repeat the encode +
    result tail in both branches; `gen_iscc_code_v0` 115 lines, inline 5-tuple.
- (300) Swift/Kotlin suites never call the streaming hashers, text utils or conformanceSelftest
    though their specs list them as verification criteria.
- (300) Error wording: napi typed-param errors vs `opt_number` text; .NET "Hasher already finalized"
    (others name the type); minhash.rs:271 wrapper-vs-inner test cannot fail.
- (300) tests/: `normalise_whitespace` etc. copied in test_napi/ruby_gem/wasm_docs.py.
- (290) Go conformance: nine identical `run*Case` (conformance.go:291-433) + five per-generator
    tests re-running them (audio/image/video/data/iscc `_test.go`).
- (290) Rust one-shot `gen_data/instance_code_v0` (lib.rs:889-935) still copy the hashers' result
    tails (streaming.rs:40-138); Go delegates. The iter-289 issue on this was closed by review.
- (290) Go errors: ~15 public sites drop the `iscc:` prefix; `JsonToDataUrl` "invalid JSON in meta".
- (290) Result type names: Ruby `IdCodeResult`; `DecodeResult` vs `IsccDecodeResult`.
- (290) Python `*_flat` video functions exported/typed but unused and untested.
- (290) FFI `iscc_alloc`/`iscc_dealloc` "WASM allocator" exported, unused, untested.
- (290) gen_sum_code_v0 has no shared vector; four wrapper suites check result shape only.
- (290) Hasher wrapper shells x16 across bindings (FFI data/instance twins, lib.rs:1420-1597).
- (280) napi/WASM six identical narrowing helpers (decision rejected sharing at ~5 lines).
- (280) C++ narrow int params wrap silently; no `-Wconversion`.
- (270) napi `Vec<f64>` rejects typed arrays WASM accepts; alg_cdc_chunks utf32 default only .NET;
    Ruby gen_instance_code_v0 bits doc; JNI hand-written non-negative checks; .NET null handling.
- (270/280) Tests that cannot fail (WASM decode fields, napi cdc, Rust import-path, C++/.NET
    `alg_simhash([[]])`, JNI constants, uniffi 32-bit); (290) multihash/SumHasher refactor tests
    compare a path with itself; release-script tests leave portfile parse-failure exits untested.
- (270) Go GenMetaCodeV0 three passes, unreachable Push errors, GenIsccCodeV0 129 lines, exported
    AlgWtahash; WASM name-twice params, `tests/unit.rs` catch-all; core gen_iscc_code_v0,
    iscc_decompose, gen_meta_code_v0 long; (290) core lib.rs 2,772 lines (1,626 tests).
- (270/280) Duplication backlog: error-mapping closures, FFI Result->C, JNI new_string/throw x14,
    C++ vector overloads, JNI resource path x4, Python test loaders (no conftest; 290: 13 script
    loaders, 6 release.yml loaders), Python video x4, hex decoders, stream-read x5, bits_to_bytes,
    iscc_decode base32; (290) version_sync.py 11 get/sync regex pairs; release.yml version grep x10
    (prepare-release `version` output unused); tools/cid.py command preambles.
- (270/280) Dead: unused `serde` deps, `pytest-cov`, csbindgen outside workspace deps, `iscc_sum.c`.
- (270) Gate latency: `test_gil.py` load; pre-push builds iscc-rb + the Python extension. (290) C
    header freshness gate installs an unpinned cbindgen.

Docs (CID-fixable):

- (300) development.md Quality Gates omits 4 prek hooks (check-ci-job-table, check-spec-paths,
    standardrb x2); Python/.NET/Node runtime floors stated nowhere in README/how-to.

- (290) architecture.md: Go tests no longer read `crates/iscc-lib/tests/`; Tier 1 "MAJOR bump" vs
    SemVer-binding-from-1.0 (spec still names 0.4). c-ffi-dx spec: publish-ffi no longer creates the
    Release. napi package.json uses v2 config keys (`name`, `triples`).

- (280) C++/root README include path the tarball lacks; `cargo bench` runs iai; Implementors Guide
    gen_sum_code_v0 source + no Swift; Go README no version floor; LoaderProbe Javadoc placement.

- (270) Ruby how-to names only RuntimeError (decision says RangeError too); getting-started Go
    streaming drops errors, says Go 1.21+ (go.mod 1.26.1); `_lowlevel.pyi` mtype 0-6; development.md
    tables (besides the tracked version list).

Context drift:

- (270) Root CLAUDE.md: reference map, OIDC list omits npm, hook inventory; specs: nodejs bridge, go
    tables, rust-core signatures, ci-cd/ruby hooks, edge-vectors generator, JUnit 5.
- (270) `crates/iscc-wasm/CLAUDE.md` 500-line threshold + Vec<String> row (the target section is in
    the iter-290 WASM issue).
- (280) Human-owned: target.md v0.8.0 section is a finished rollout + dead `iscc_clean` issue ref.

Loop tooling (meta-improve/human, not the CID queue):

- (300) review.md runs Codex on `--commit HEAD` only while fan-out steps span 6-13 commits; CRAP
    gate has no local enforcing task (`mise run crap` report-only, 4+ red CI/NEEDS_WORK); cid.py
    logs timed-out runs as $0/0 turns (cid.py:281-293); audit cost $0.9 -> ~$20/pass.

- (280) review-patterns.md agent memory keeps growing; topic files unbounded.

- (280/290) 15 skipped `low` entries are inlined into define-next/review every iteration (~65% of
    issues.md); normal issues needing Titusz escalate only when the loop runs dry; meta-improve
    proposal unanswered since iter 248. This watch list itself is a shadow backlog (~60 items).

- (270/290) metrics.py: single-sample gate timings, times other commands than the gates, blind to
    C++/JS/C#/Swift/wasm tests and Go skips, counts gitignored CMake files as TypeScript.

- (290) Docs site builds only on push to main (develop ~634 commits ahead); release.yml only on
    publish; advance.md step 6 runs no ty/clippy.
