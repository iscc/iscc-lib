## 2026-02-25 — Review of: Add input validation to alg_dct and alg_wtahash

**Verdict:** PASS

**Summary:** Clean implementation fixing two `[low]` correctness issues. `alg_dct` now enforces
power-of-two input length via `n.is_power_of_two()`, and `alg_wtahash` returns `IsccResult<Vec<u8>>`
with guards for minimum vector length (380), valid bits (>0, divisible by 8, ≤256). Eight new tests
cover the error paths. No behavioral change for valid inputs.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 269 tests (216 unit + 31 integration algo + 22 integration
    text), all pass
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean — no warnings
- [x] `alg_dct(&[1.0; 6])` returns `Err` — confirmed via `test_alg_dct_non_power_of_two_even_error`
- [x] `alg_wtahash(&[0i64; 100], 64)` returns `Err` — confirmed via
    `test_alg_wtahash_short_input_error`
- [x] All existing conformance tests still pass — no behavioral change for valid inputs

**Issues found:**

- (none)

**Next:** Only `[low]` items remain: iscc-py `__version__` attribute, iscc-py module docstring,
iscc-wasm conformance feature gate, iscc-wasm stale CLAUDE.md, iscc-jni exception mapping, and
TypeScript port evaluation. Define-next should pick the next highest-value item.

**Notes:** The advance agent simplified the lib.rs caller change from next.md's suggested
`Ok(wtahash::alg_wtahash(&vecsum, bits)?)` to just `wtahash::alg_wtahash(&vecsum, bits)` (removing
the `Ok()` wrapper entirely), which is cleaner since both caller and callee now share the same
`IsccResult<Vec<u8>>` return type.
