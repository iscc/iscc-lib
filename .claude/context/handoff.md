## 2026-11-22 — Review of: Classify every public `iscc-lib` path in one tier list and fix the false Rust API sentences

**Verdict:** PASS

**Summary:** Docs and comments only. Every public `iscc-lib` path now has a tier: Tier 1 is the 35
crate-root symbols, `IsccError`/`IsccResult` and the `types` result structs are uncounted support
types, and Tier 2 is `codec` plus `streaming::SumHasher`. Every sentence the audit issue cited now
matches `lib.rs`, and the issue is deleted. Review closed three small gaps in the wording (below).

**Verification:**

- [x] `git grep -n 'iscc_lib::api' -- ':!notes' ':!.claude'` prints nothing
- [x] `git grep -n -e 'bytes, not file paths' -e 'Import codec types' -- docs CLAUDE.md` prints
    nothing
- [x] `git grep -n 'SumHasher'` matches in each of the four files (architecture.md 1, rust-api.md 5,
    `crates/iscc-lib/CLAUDE.md` 1, rust-core.md 2)
- [x] `uv run pytest tests/test_rust_api_signatures.py`: 5 passed. `check_tier1_docs.py`: exit 0,
    "22 full and 2 exact Tier 1 docs match"
- [x] `uv run zensical build` exit 0, and `mise run check` exit 0
- [x] Extra: the `SumHasher` example compiles and runs in a `/tmp` cargo project. The same probe
    confirmed three claims in the new text: `SumHasher` equals `gen_sum_code_v0` on a file with the
    same bytes, `gen_sum_code_v0` equals `gen_iscc_code_v0` over its two units, and `types::X` /
    `streaming::DataHasher` are the crate-root items
- [x] Extra: the CI CRAP gate on HEAD. The `//!` edit moves every function in `lib.rs` down 4 lines
    and the baseline stores line numbers. Result: exit 0, "0 regressed, 33 moved". No baseline
    refresh is needed
- [x] Scope: no code change. `lib.rs` and `IsccLibTest.java` are comment-only (confirmed in the
    diff). No gate was weakened. The other unpushed commits touch `.claude/` only

**Issues found:**

- Fixed by review: root `CLAUDE.md` still said "Only Tier 1 … is bound to foreign languages", which
    the step's own classification contradicts (Python and WASM wrap `SumHasher`). It now names that
    exception
- Fixed by review: four Tier 1 functions live in `codec` (`encode_base64`, `iscc_decompose`,
    `encode_seq`, `decode_seq`), and `codec::encode_component` is a separate enum-typed Tier 2
    function. `docs/architecture.md` and `crates/iscc-lib/CLAUDE.md` now say so. The latter also
    said "`*CodeResult` structs", which left out `IsccIdResult`
- Filed `low`, untriaged: "target.md and the agent prompts name `codec` as the whole of Tier 2".
    `target.md` and the API-break rules in `.claude/agents/advance.md` and `review.md` are Titusz's
    to edit
- Nit, not filed: the "Internal Modules" table in `docs/architecture.md` has no row for the
    `pub(crate)` module `conformance`

**Codex review:** no findings. It ran the docs checks, the doctests and the new example, and found
the revised descriptions consistent with the code.

**Next:** the `normal` `[audit]` issue "One-shot gen functions re-implement their streaming
counterparts" (3 non-test files). It changes the instance hot path, so the same step must run
`mise run bench:iai:check` (needs `valgrind` and `iai-callgrind-runner@0.16.1`, see learnings →
Tooling) and refresh `.crap-baseline.json`, because both gates fail in CI only. The WASM part
(`gen_sum_code_v0` calling `SumHasher`) can be split off if the step gets too large.

**Notes:**

- Decision recorded (2026-11-22): `streaming::SumHasher` is Tier 2, and Tier 2 has the same SemVer
    contract as Tier 1. `docs/architecture.md` used to say Tier 2 "may change in MINOR releases";
    advance aligned it with `rust-core.md`, and the decision covers that change
- Until Titusz edits the review prompt, treat a signature change to `streaming::SumHasher` like a
    Tier 2 `codec` change: it needs an `**API-BREAK:**` flag
- The two issues that need a behaviour decision from Titusz are still open and still `normal`:
    `alg_minhash_256([])` and the WASM hasher `finalize()` rule. Neither is a candidate for a step
    before he decides
- A comment-only edit that shifts lines in a covered file does not trip the CRAP gate (learnings
    corrected; the old line claimed it did)
