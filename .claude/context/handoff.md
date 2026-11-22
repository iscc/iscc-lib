## 2026-11-22 — Review of: One section table drives Rust conformance; WASM conformance uses strict shared readers

**Verdict:** PASS

**Summary:** `conformance.rs` now has one `SECTIONS` table that drives `conformance_selftest`, the
test helpers and a table-vs-`data.json` test; the five duplicate loops left `lib.rs`. The WASM
conformance harness reads every vector through strict readers (shared ones in `tests/common/mod.rs`)
and takes case counts from the file. The work matches next.md, the public signature of
`conformance_selftest` is unchanged, and the `[audit]` issue is closed.

**Verification:**

- [x] `cargo test -p iscc-lib` (469 passed), `--no-default-features` (398),
    `--no-default-features --features text-processing` (433), 0 failed in each (counts include the
    reviewer's one added test)
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` and
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` clean (also the full workspace,
    and `--tests` under both reduced feature sets)
- [x] `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    exits 0: conformance 11, data_edge 8, unit 136
- [x] Section-name grep: lines 39–50 (the table) are the only matches before `mod tests`
- [x] Both "prints nothing" greps are empty; no other `as` cast is left in the WASM conformance
    files
- [x] Table test has teeth, re-run by the reviewer in a `/tmp` copy: without the audio row
    `test_sections_match_data_json` fails and names `gen_audio_code_v0`
- [x] `mise run check` exit 0

Reviewer probes: (1) WASM harness on a `/tmp` copy with a pixel of 256 and a text `bits` of 2^32+64,
two values the old `as u8` / `as u32` casts turned back into the original: both tests now fail with
the reader's message ("is not a list of u8", "is not a u32"), the other 9 pass. (2)
`.crap-baseline.json` compared by score with the previous commit: only `conformance.rs` rows differ
(12 new, 11 gone, `conformance_selftest` 4.08 → 5.58).

**Issues found:**

- Fixed in this review: deleting the five `lib.rs` loops removed the core's only case-count pins for
    image, audio, video, data and iscc, and an empty section passes `conformance_selftest`. Added
    `test_sections_have_cases` (every table row has at least one case, all feature sets). Exact
    counts are still pinned by the Swift suite on its gated byte-identical copy
- Not filed (nit): `assert_eq!(tested, cases.len())` in the WASM harness cannot fail, because the
    loop panics on any bad case. The floor in `section_cases` is the real guard
- Not filed (nit): the WASM harness has no section-list guard of its own; its `CLAUDE.md` says so,
    and the core's table test fails first on a new section

**Codex review:** no actionable findings. It re-ran the Rust tests under all three feature sets, the
WASM tests and clippy for both crates.

**Next:** the remaining `normal` blocker, the `[audit]` issue "Specs and API docs still describe
shipped work as planned, or leave out shipped rules" (9 files). Suggested split into two steps:

1. Docs and API comments under the audit-scope valve (7 files: `docs/c-ffi-api.md`, `IsccLib.cs`,
    `iscc.hpp`, the napi, WASM and UniFFI comments, `README.md`). A `///` edit on a `iscc-ffi` or
    UniFFI export regenerates tracked outputs (`iscc.h`, `NativeMethods.g.cs`, the UniFFI
    bindings); list them as regenerated files, which do not count against the cap.
2. The three specs (`swift-bindings.md`, `documentation.md`, `c-ffi-dx.md`). These are mechanically
    checkable facts (a checkbox that does not exist, page lists, a closed GitHub issue), so no
    escalation is needed; point to the nav and `release.yml` where a list would be copied.

**Notes:**

- `conformance_selftest`'s CRAP score rose (4.08 → 5.58, limit 30), against next.md's expectation: a
    loop adds one decision and removes covered straight-line calls, so the two unreachable error
    arms weigh more. The baseline moved in the same commit, as the gate rule requires. The
    reviewer's added test sits in the test module, which the gate does not score.
- The meta, text, mixed and instance loops in `lib.rs` stay, hard-coded counts included.
- The CRAP and Perf gates run in CI only; the first CI run on this push is the proof for the
    refreshed baseline.
