## 2026-02-25 — Add input validation to alg_dct and alg_wtahash

**Done:** Fixed two `[low]` correctness issues: (1) `alg_dct` now rejects non-power-of-two input
lengths using `n.is_power_of_two()` instead of the previous even-number check; (2) `alg_wtahash`
returns `IsccResult<Vec<u8>>` with guards for minimum vector length (380), valid `bits` (>0,
divisible by 8, ≤256). Updated the sole caller in `soft_hash_video_v0` to propagate the error.

**Files changed:**

- `crates/iscc-lib/src/dct.rs`: Replaced `n == 0 || (n > 1 && n % 2 != 0)` with
    `!n.is_power_of_two()`, updated error message and docstring, added 4 new test cases (lengths 6,
    10, 12 rejected; length 2 accepted)
- `crates/iscc-lib/src/wtahash.rs`: Changed `alg_wtahash` return type from `Vec<u8>` to
    `IsccResult<Vec<u8>>`, added `use crate::{IsccError, IsccResult}`, added validation guards,
    updated docstring with constraints section, updated 3 existing tests to `.unwrap()`, added 4 new
    error tests (short input, zero bits, non-divisible bits, exceeds permutations)
- `crates/iscc-lib/src/lib.rs`: Changed `Ok(wtahash::alg_wtahash(&vecsum, bits))` to
    `wtahash::alg_wtahash(&vecsum, bits)` (propagates error naturally since caller already returns
    `IsccResult`)

**Verification:** All 269 tests pass (216 unit + 31 integration algo + 22 integration text), cargo
clippy workspace-wide clean, all 14 pre-commit hooks pass. New tests confirm: `alg_dct(&[1.0; 6])`
returns `Err`, `alg_wtahash(&[0i64; 100], 64)` returns `Err`. All existing conformance tests
unaffected.

**Next:** Both `[low]` validation issues are resolved. Remaining low-priority items: iscc-py
`__version__`, iscc-py module docstring, iscc-wasm conformance feature gate, iscc-wasm stale
CLAUDE.md, iscc-jni exception mapping, TypeScript port evaluation.

**Notes:** The `alg_wtahash` return type change from `Vec<u8>` to `IsccResult<Vec<u8>>` required no
changes to binding crates because they all call through `gen_video_code_v0` (which already returned
`IsccResult`), not `alg_wtahash` directly. The single caller change in lib.rs was trivial — removing
the `Ok()` wrapper since `alg_wtahash` now returns the `IsccResult` itself.
