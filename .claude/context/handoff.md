## 2026-02-24 — Add Rust how-to guide

**Done:** Created `docs/howto/rust.md` with all 9 code generation sections, structured results,
streaming hashers, text utilities, algorithm primitives, conformance testing, and error handling.
Added Rust as the first entry in the How-to Guides navigation in `zensical.toml`.

**Files changed:**

- `docs/howto/rust.md`: New 351-line Rust how-to guide following the Python guide's section
    structure
- `zensical.toml`: Added `{ "Rust" = "howto/rust.md" }` as first entry in How-to Guides nav

**Verification:**

- `uv run zensical build` exits 0 — site builds with `/howto/rust/` page
- `wc -l docs/howto/rust.md` = 351 (target: 250+, approximately 280-350)
- `grep -c 'gen_.*_code_v0' docs/howto/rust.md` = 29 (target: 9+)
- `grep -c 'DataHasher\|InstanceHasher' docs/howto/rust.md` = 8 (target: 2+)
- `zensical.toml` contains `"Rust" = "howto/rust.md"` in How-to Guides
- `mise run check` — all 14 pre-commit hooks pass

**Next:** The documentation section is now closer to "met" — the Rust how-to and Python how-to are
both present. The remaining documentation gap is nav icons for top-level sections (noted in
next.md's Not In Scope). Beyond documentation, the highest-value items are the correctness issues in
issues.md (`alg_simhash` panics on mismatched digests, `sliding_window` panics on `width < 2`).

**Notes:** The Rust how-to uses the actual result struct types (`MetaCodeResult`, `TextCodeResult`,
etc.) from `types.rs` rather than the outdated `IsccResult<String>` shown in the Rust API Reference
page (`docs/rust-api.md`). The API reference page is now inconsistent with both the actual code and
the new how-to guide — updating it should be considered in a future step.
