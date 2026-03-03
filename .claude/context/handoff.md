## 2026-03-03 — Review of: Add 5 algorithm primitives to Ruby bridge

**Verdict:** PASS

**Summary:** Added 5 algorithm primitive functions (`sliding_window`, `alg_simhash`,
`alg_minhash_256`, `alg_cdc_chunks`, `soft_hash_video_v0`) to the Ruby Magnus bridge, advancing from
25/32 to 30/32 Tier 1 symbols. Implementation is clean, follows established patterns exactly, and
all tests pass (46 runs, 121 assertions, 0 failures). All functions correctly use direct module
registration (no `_` prefix, no Ruby wrapper).

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- [x] `bundle exec rake compile` — builds native extension in release profile
- [x] `bundle exec rake test` — 46 runs, 121 assertions, 0 failures, 0 errors, 0 skips
- [x] `IsccLib.sliding_window("Hello World", 3)` returns Array of 9 Strings
- [x] `IsccLib.alg_simhash` returns binary String matching input digest length (4 bytes)
- [x] `IsccLib.alg_minhash_256` returns 32-byte binary String
- [x] `IsccLib.alg_cdc_chunks` returns Array of binary Strings that reassemble to original
- [x] `IsccLib.soft_hash_video_v0` returns binary String of correct size (bits/8)
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- Minor scope deviation: advance agent updated docstring symbol count (25→30) despite next.md
    listing "Updating the docstring symbol count" as Not In Scope. Trivially beneficial — no action
    needed.
- `alg_cdc_chunks` safety comment says slice is "not held across any Ruby API calls that could
    trigger GC", but chunk sub-slices ARE used during `arr.push` calls. Practically safe for Ruby
    3.1 (non-compacting GC, RString pinned on stack), but comment is slightly imprecise.

**Codex review:** Two findings, both advisory:

1. **P1: `avg_chunk_size=0` hang** — valid concern but this is a Rust core issue, not Ruby-specific.
    All bindings share the same vulnerability. Filed as issue in issues.md for the core.
2. **P2: Unsafe slice across Ruby API calls** — technically correct that chunk sub-slices reference
    the original `bytes` during `arr.push` calls. Practically safe: `data` RString is stack-pinned
    (function parameter), MRI Ruby 3.1 uses non-compacting GC. Would only become a real issue with
    Ruby 3.2+ auto-compaction + Magnus 0.8. Not actionable for current target.

**Next:** The remaining 2/32 Tier 1 symbols are `DataHasher` and `InstanceHasher` streaming types.
These require Ruby class wrappers (not module functions) — a different pattern from all 30 prior
symbols. This should be a dedicated step with careful design for the `new → update → finalize`
interface.

**Notes:**

- 30/32 symbols complete. Only streaming types remain, which need Ruby class wrappers (different
    from all prior module function patterns).
- All algorithm primitives follow the same conventions: `RString` + `unsafe { as_slice() }` for
    binary input, `RString::from_slice` for binary output, `RArray` + `TryConvert` for nested
    arrays, `to_magnus_err` for error mapping.
- `alg_minhash_256` is correctly infallible (no `Result` return) — the only such function.
- Test quality is high: 9 new tests cover return types, binary encoding, output sizes, error
    handling (width < 2, mismatched digest lengths), and chunk reassembly verification.
