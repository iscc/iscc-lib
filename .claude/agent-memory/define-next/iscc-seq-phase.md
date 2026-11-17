---
name: iscc-seq-phase
description: v0.8.0 ISCC-SEQ (encode_seq/decode_seq, #45) slice plan, scoping traps, and what iter 195 scoped
metadata:
  type: project
---

v0.8.0 = ISCC-SEQ on Rust core + 11 surfaces (Tier 1 33→35). Iter 195 scoped the Rust core slice:
codec.rs (raw u32 header codec + `read_unit` + `SEQ_MAINTYPES` + encode/decode_seq), lib.rs
re-export, conformance.rs (seq sections), new `tests/test_seq.rs`.

**Why:** spec order is Rust core → Go `isccClean` (normal issue, prereq only for Go seq) → bindings.

**How to apply:**

- Reference `conformance_selftest` walks EVERY data.json section with `bytes:` decoding, so the Rust
    selftest (and thus every binding's selftest) must include seq sections.
- Porting trap: typed `decode_header` rejects SubType≥8 and Version 1 on non-ID; seq needs a raw-u32
    path; never widen enums. `read_unit` offset>len must Err (Rust slice panics).
- `decode_header` is on the iai hot path → flag `bench:iai:check` + CRAP baseline regen per codec
    edit.
- Count/docs sweep 33→35 deferred to after the fan-out (IDv1 precedent, iter 190).
- Binding fan-out: Swift/Kotlin hard-code per-section counts; Go loader loads only `gen_*`; FFI step
    must regen+commit `iscc.h` and `NativeMethods.g.cs`.
