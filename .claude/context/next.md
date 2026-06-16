# Next Work Package

## Step: Add core `streaming::SumHasher` struct and drive `gen_sum_code_v0` from it

## Goal

Add a reusable single-pass `streaming::SumHasher` to the `iscc-lib` core crate (the "core first"
prerequisite of issue #37, "Add streaming `SumHasher` to Python and WASM bindings"). This gives
streaming consumers a `new() → update(&[u8]) → finalize(...)` hasher that runs the Data-Code and
Instance-Code algorithms in a single pass, and de-duplicates the inline loop inside the path-based
`gen_sum_code_v0`.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/streaming.rs` — add a `pub struct SumHasher` plus unit tests.
    - `crates/iscc-lib/src/lib.rs` — refactor `gen_sum_code_v0` to read the file and drive a
        `SumHasher` (replacing the duplicated dual-hasher read loop).
- **Reference**:
    - `.claude/context/issues.md` → "Add streaming `SumHasher` to Python and WASM bindings" (issue
        #37) — the "Core first" bullet defines this step.
    - `crates/iscc-lib/src/streaming.rs` — existing `DataHasher` / `InstanceHasher` (pattern to
        mirror).
    - `crates/iscc-lib/src/lib.rs` ~:986 `gen_sum_code_v0`, ~:859 `gen_iscc_code_v0` (composition
        logic to port).
    - `crates/iscc-lib/src/types.rs` ~:98 `SumCodeResult` (return type).

## Not In Scope

- **No PyO3 wrapper** (`crates/iscc-py`) and **no wasm-bindgen wrapper** (`crates/iscc-wasm`) for
    `SumHasher` — those are the remainder of issue #37 and land in a dedicated follow-up step.
- **No GIL release** (`py.allow_threads`, issue #39) — separate step.
- **No crate-root `pub use streaming::SumHasher`** re-export and **no Tier 1 symbol-count change**
    (keep the documented 32 crate-root Tier 1 symbols / "2 streaming types" intact). `SumHasher` is
    reachable for the next step via `iscc_lib::streaming::SumHasher` (the `streaming` module is
    already `pub mod`). Promotion to a crate-root Tier 1 export — together with README /
    rust-core.md count updates — happens in the bindings step that actually exposes it to foreign
    languages.
- No changes to `DataHasher` / `InstanceHasher` public behavior.
- Do not touch CI, release workflows, or push/branch state.

## Implementation Notes

- `SumHasher` composes the two existing hashers — hold a `DataHasher` and an `InstanceHasher` as
    fields:

    ```rust
    pub struct SumHasher {
        data_hasher: DataHasher,
        instance_hasher: InstanceHasher,
    }
    ```

    `new()` constructs both; `update(&mut self, data: &[u8])` feeds the *same* slice to both inner
    hashers (single pass over the caller's chunk).

- `finalize` takes `(self, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>` so
    it fully replaces the body of `gen_sum_code_v0`. Port the exact composition currently in
    `gen_sum_code_v0`:

    1. `let data_result = self.data_hasher.finalize(bits)?;`
    2. `let instance_result = self.instance_hasher.finalize(bits)?;`
    3. `let iscc_result = crate::gen_iscc_code_v0(&[&data_result.iscc, &instance_result.iscc], wide)?;`
    4. `units = if add_units { Some(vec![data_result.iscc, instance_result.iscc]) } else { None };`
    5. Return
        `SumCodeResult { iscc: iscc_result.iscc, datahash: instance_result.datahash,  filesize: instance_result.filesize, units }`.

- Add `Default` impl delegating to `new()` (matches `DataHasher` / `InstanceHasher`).

- `streaming.rs` will need `crate::gen_iscc_code_v0` and `crate::types::SumCodeResult` in scope —
    extend the existing `use crate::{...}` line; `gen_iscc_code_v0` is a `pub fn` at the crate root,
    so `crate::gen_iscc_code_v0` resolves.

- Refactor `gen_sum_code_v0` (lib.rs) to: open the file, create `streaming::SumHasher::new()`, read
    in `IO_READ_SIZE` chunks feeding `hasher.update(&buf[..n])`, then
    `hasher.finalize(bits, wide, add_units)`. Keep the existing `File::open` / `read` error mapping
    to `IsccError::InvalidInput`. The existing `gen_sum_code_v0` test suite (equivalence, empty
    file, file-not-found, wide mode, bits 64/128, large data, units enabled/disabled) must stay
    green.

- New unit tests in `streaming.rs` (no file I/O — compare against the in-memory two-hasher pattern):

    - Empty input: `SumHasher::new().finalize(64, false, false)` equals the manual composition of
        `gen_data_code_v0(b"", 64)` + `gen_instance_code_v0(b"", 64)` → `gen_iscc_code_v0(.., false)`.
    - Small data single `update`.
    - Multi-`update` (split the same bytes across 2–3 `update` calls) produces an identical result to
        a single `update` of the whole slice (streaming invariance).
    - `add_units = true` yields `Some([data_iscc, instance_iscc])`; `add_units = false` yields `None`.
    - `wide = true` vs `wide = false` produce the expected differing composite codes.
    - `datahash` / `filesize` match `gen_instance_code_v0` for the same bytes.

## Verification

- `cargo test -p iscc-lib` passes (all existing tests, including the `gen_sum_code_v0` suite, plus
    the new `SumHasher` unit tests).
- `cargo clippy -p iscc-lib -- -D warnings` clean.
- `cargo fmt -p iscc-lib --check` clean.
- A test asserts `iscc_lib::streaming::SumHasher::new()` fed bytes B and finalized with
    `(bits, wide, add_units)` yields the same `iscc`, `datahash`, and `filesize` as
    `gen_sum_code_v0` run on a temp file containing B (single source of truth confirmed).
- `gen_sum_code_v0` no longer contains its own dual-hasher read loop (it delegates to `SumHasher`).

## Done When

`streaming::SumHasher` exists with `new()/update()/finalize(bits, wide, add_units)`,
`gen_sum_code_v0` is refactored to drive it, and all verification checks above pass.
