## 2026-02-25 — Fix stale WASM CLAUDE.md documentation

**Done:** Updated `crates/iscc-wasm/CLAUDE.md` to accurately reflect that DataHasher and
InstanceHasher are fully bound as `#[wasm_bindgen]` structs, corrected the exported symbol count
from 22 to 23, and added the 2 streaming types to the export list.

**Files changed:**

- `crates/iscc-wasm/CLAUDE.md`: Updated "Exported API Surface" section — changed count from 22 to
    23, added "2 streaming types: DataHasher, InstanceHasher" to the bullet list, replaced the "not
    yet bound" paragraph with accurate documentation of the bound structs and their `Option<Inner>`
    finalize-once pattern.

**Verification:**

- `grep -c "not yet bound" crates/iscc-wasm/CLAUDE.md` → 0 (stale text removed)
- `grep "All 23 Tier 1" crates/iscc-wasm/CLAUDE.md` → matches line 117
- `grep "DataHasher" crates/iscc-wasm/CLAUDE.md` → shows lines 129, 131 (listed as bound exports)
- `grep "InstanceHasher" crates/iscc-wasm/CLAUDE.md` → shows lines 129, 131 (listed as bound
    exports)
- No Rust source files modified (only CLAUDE.md changed)
- All 14 pre-commit hooks pass

**Next:** The `[low] iscc-wasm: Stale CLAUDE.md` issue is resolved. Remaining low-priority issues
include the TypeScript port evaluation and iscc-jni Javadoc `@throws` annotation mismatch. The
project is ready for the `v0.0.1` release — consider creating a PR from develop to main.

**Notes:** mdformat re-wrapped some lines during formatting (standard behavior). No surprises or
shortcuts taken. This was a documentation-only change with no code impact.
