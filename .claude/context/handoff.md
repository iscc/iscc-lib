## 2026-11-22 — Classify every public `iscc-lib` path in one tier list and fix the false Rust API sentences

**Done:** Every public `iscc-lib` path now has a tier in the docs. Tier 1 is the crate-root symbols.
`IsccError`/`IsccResult` and the `types` result structs are crate-root support types of Tier 1 and
are not counted among the 35. Tier 2 is `codec` plus `streaming::SumHasher` (Python and WASM wrap
it). `streaming::DataHasher`/`InstanceHasher` and `types::X` are the same items as the crate-root
re-exports. I fixed the false sentences the issue cited. Only docs and comments changed.

**Files changed:**

- `docs/architecture.md`: added a support-types paragraph under Tier 1. Tier 2 now lists `codec` and
    `streaming::SumHasher`, with the SemVer sentence aligned to rust-core.md. The streaming-pattern
    sentence now names `gen_sum_code_v0` as the one function that takes a path.
- `docs/rust-api.md`: added `### SumHasher` under Streaming (Tier 2, `impl` block copied from
    `streaming.rs`, plus an example). `IsccError` is now presented as a crate-root type, and a
    `### Codec types` subsection carries the Tier 2 framing that used to sit above it.
- `CLAUDE.md` (root): the `iscc_lib::api` bullet now says crate-root re-exports. The conformance
    paragraph now says which functions have vectors: every `gen_*_v0` with a `data.json` section.
    `gen_sum_code_v0` has none.
- `crates/iscc-lib/CLAUDE.md`: added a "Tier 1 support types" block. Tier 2 is now `codec` plus
    `streaming::SumHasher`.
- `crates/iscc-lib/src/lib.rs`: `//!` crate doc only. Tier 1 = crate-root symbols, Tier 2 = `codec`
    and `streaming::SumHasher`.
- `.claude/context/specs/rust-core.md`: the Tier 2 paragraph after `tier1:end` and the
    API-compatibility bullet now name `streaming::SumHasher` beside `codec`.
- `crates/iscc-jni/.../IsccLibTest.java`: comment only. The file has no `gen_sum_code_v0` vectors;
    it checks `genSumCodeV0` by equivalence (lines 352-390).

**Verification:**

- `git grep -n 'iscc_lib::api' -- ':!notes' ':!.claude'` prints nothing
- `git grep -n -e 'bytes, not file paths' -e 'Import codec types' -- docs CLAUDE.md` prints nothing
- `git grep -c SumHasher` on the four files gives rust-core.md 2, crates/iscc-lib/CLAUDE.md 1,
    architecture.md 1 and rust-api.md 5
- `uv run pytest tests/test_rust_api_signatures.py`: 5 passed (this covers the printed
    `SumHasher::finalize` signature)
- `uv run python scripts/check_tier1_docs.py`: "OK: 22 full and 2 exact Tier 1 docs match"
- `uv run zensical build`: exit 0, "No issues found"
- `mise run check`: exit 0. The first two runs failed only because mdformat reflowed my new lines; I
    reread the reflowed text before the third run.
- The new `SumHasher` example, the `use iscc_lib::codec::{MainType, SubType}` import and the Error
    Handling example compiled and ran in a `/tmp` cargo project with a path dep (printed
    `Generated: ISCC:EAASKDNZNYGUUF5A`, asserts passed). The `/tmp` project is deleted.

**Next:** the one-shot/streaming dedup that next.md rejected for this step: WASM `gen_sum_code_v0`
copies the `SumHasher` body. It is a hot path, so it needs `mise run bench:iai:check` and a CRAP
baseline refresh in the same step.

**Notes:**

- Design call, for review to record in decisions.md: `streaming::SumHasher` is Tier 2 (public Rust
    API, under SemVer, not part of the every-surface contract). It is not a crate-root re-export.
- Changed beyond the literal sentence list: `architecture.md` Tier 2 used to say "May change in
    MINOR releases". That contradicts rust-core.md's API-compatibility bullet (Tier 2 is under the
    semver-checks lock), and the spec wins. It now says Tier 2 has the same SemVer contract as Tier
    1, with `cargo-semver-checks` informational until v1.0.0 (`ci.yml:462-463`,
    `continue-on-error: true`).
- `target.md`'s Tier 2 line names only `codec`. With this classification it is incomplete, so review
    may file a `low` issue for Titusz. I did not edit target.md.
- Other "all 10" lines from the sweep are correct and were left alone:
    `crates/iscc-ffi/src/lib.rs:3` ("Exposes all 10") and `benches/benchmarks.rs:1`, whose benches
    include `gen_sum_code_v0` (5 references). The how-to guide lines describe calling conventions.
- No Rust code changed, so the CRAP, iai and semver gates are unaffected.
