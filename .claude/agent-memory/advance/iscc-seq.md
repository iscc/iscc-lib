---
name: iscc-seq
description: ISCC-SEQ (IEP-0020, #45) Rust core implementation facts — raw header helpers, read_unit, conformance runners, known divergences, oracle recipe for binding slices
metadata:
  type: project
---

# ISCC-SEQ (#45)

ISCC-SEQ #45 Rust core DONE (iter 195):
`codec::{read_unit, encode_seq, decode_seq,     SEQ_MAINTYPES}` (seq fns re-exported at crate root;
Tier-1 count sweep 33→35 deferred until all surfaces bind). Built on private RAW header helpers
`decode_header_raw` (returns `tail_start`, `#[inline]`) / `encode_header_raw` (u32s, no typed
gates); typed `decode_header`/`encode_header` are thin wrappers (iai unchanged). `read_unit` uses
`data.get(offset..).unwrap_or(&[])` → out-of-range offset = "Malformed" like ref. Seq conformance
runners in conformance.rs (`decode_bytes` for `bytes:<hex>`). Known divergence: `decode_base32`
rejects non-zero trailing bits (`GAAAAAAAAB`), iscc-core accepts — affects every decode, not
seq-specific. Bindings: test oracle values via
`uv run --no-project --with iscc-core==1.5.0 python -c ...`

Binding slices (next): each surface wraps `iscc_lib::encode_seq(&[&str])` / `decode_seq(&[u8])`;
conformance harnesses must decode `bytes:<hex>` in inputs AND outputs (outputs are a string / list,
not objects). Then the Tier-1 33→35 doc/count sweep (like IDv1 iter 190).
