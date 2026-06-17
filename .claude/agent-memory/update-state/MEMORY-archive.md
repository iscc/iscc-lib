# Update-State Agent Memory — Archive

Archived iteration-by-iteration findings from completed project phases. Moved here to reduce
per-invocation context loading. Full history preserved in git.

See MEMORY.md for current active entries. See git log for the complete historical record of all
archived entries.

## Closed milestones

- **Core SumHasher (iteration 88, `3fc44d2`)**: `pub struct SumHasher` in `streaming.rs:157`
    (new/update/finalize(bits,wide,add_units)/Default). `gen_sum_code_v0` (lib.rs:997) drives it.
    Reachable as `iscc_lib::streaming::SumHasher` but NOT a crate-root re-export (only
    `DataHasher`/`InstanceHasher` are, lib.rs:24). SumHasher NOT promoted to Tier 1 — counts stay
    32\.
- **Module visibility (iteration 86, `3f6a61d`)**:
    `cdc/conformance/dct/minhash/simhash/utils/wtahash` = `pub(crate) mod`; only
    `codec/streaming/types` = `pub mod`. Issue swept.
