## 2026-03-04 — Review of: Create Ruby API reference page

**Verdict:** PASS

**Summary:** The advance agent created `docs/ruby-api.md` (780 lines) documenting all 32 Tier 1
public symbols with signatures, parameter tables, return types, and code examples. Navigation entry
added to `zensical.toml`. The doc is well-structured, follows the Java API reference pattern adapted
for Ruby conventions, and the site builds cleanly. Two minor accuracy issues (Codex-caught) were
fixed during review.

**Verification:**

- [x] `test -f docs/ruby-api.md` exits 0 — file exists
- [x] `grep -q 'ruby-api.md' zensical.toml` exits 0 — nav entry added after Java API
- [x] `grep -c 'gen_.*_code_v0' docs/ruby-api.md` returns ≥10 — got 46 (each function referenced
    multiple times across headings, signatures, result class table)
- [x] `grep -c 'IsccLib' docs/ruby-api.md` returns ≥30 — got 78
- [x] `uv run zensical build` succeeds — "Build finished", `/ruby-api/` in output
- [x] `wc -l < docs/ruby-api.md` between 400 and 800 — got 781 (one line added by review fix)
- [x] `mise run check` — all 15 hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (fixed) `iscc_decode` return description said "length" — should be "length_index" (the encoded
    length code, not the actual bit length). Fixed to match the howto guide's correct description
- (fixed) `alg_cdc_chunks` parameter table said "default 1024" for `avg_chunk_size` — this is a
    required positional argument in the Ruby API (no wrapper provides a default). Fixed to say
    "required"

**Codex review:** Two actionable P2 findings, both valid and fixed during review: (1) misleading
`length` vs `length_index` in `iscc_decode` return description; (2) false "default 1024" for
required `avg_chunk_size` parameter in `alg_cdc_chunks`.

**Next:** With `docs/ruby-api.md` now complete, the Ruby bindings section should reach "met" status
(all spec requirements fulfilled except human-action items like RubyGems account setup). The
remaining `normal` priority work is now fully resolved. The Ruby bindings issue can be considered
complete. Possible next steps: (1) update-state to reflect Ruby "met", (2) address any remaining
non-`low` gaps or move to maintenance mode.

**Notes:** The Ruby bindings implementation is now feature-complete from a CID perspective. The only
remaining item is RubyGems account setup (human action). All `normal` and `critical` priority issues
are resolved. The remaining issues in issues.md are all `low` priority (C#, C++, Swift, Kotlin
bindings, CDC validation edge case, logo additions) which CID skips by policy.
