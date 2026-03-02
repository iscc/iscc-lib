## 2026-03-02 — Update docs for 4-parameter gen_sum_code_v0

**Done:** Updated all 5 documentation files that referenced the old 3-parameter
`gen_sum_code_v0(path, bits, wide)` signature to include `add_units` as the 4th parameter. Also
added `units` field to the C FFI struct docs and updated the free function description.

**Files changed:**

- `docs/architecture.md`: Added `add_units` to pseudocode signature on line 131
- `docs/rust-api.md`: Updated signature, added `add_units` parameter row, mentioned `units` field in
    description, updated code example
- `docs/howto/rust.md`: Added `false` 4th argument to code example
- `docs/c-ffi-api.md`: Updated C signature with `bool add_units`, added parameter row, added
    `char **units` to struct, updated free function doc
- `docs/howto/c-cpp.md`: Added `false` 4th argument to both C code examples (lines 101 and 231)

**Verification:**

- `grep -n 'gen_sum_code_v0.*bits.*wide' docs/*.md docs/howto/*.md | grep -v add_units` — no matches
    ✅
- `grep -c 'add_units' docs/rust-api.md` — 3 ✅ (≥2 required)
- `grep -c 'add_units' docs/c-ffi-api.md` — 2 ✅ (≥2 required)
- `grep 'char \*\*' docs/c-ffi-api.md` — 2 matches ✅ (≥1 required)
- `uv run zensical build` — exits 0, docs site builds cleanly ✅
- `mise run check` — 14/14 pre-commit hooks pass ✅

**Next:** Documentation section should now be fully met. Remaining open issue is #16 (feature flags
for embedded/minimal builds). The CID loop should verify state and define the next target-aligned
work package.

**Notes:** mdformat reformatted table alignment in rust-api.md and c-ffi-api.md (trailing whitespace
in table cells). This is expected — the pre-commit hook normalizes markdown table formatting. No
source code changes were needed for this step.
