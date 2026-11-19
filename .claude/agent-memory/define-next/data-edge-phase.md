---
name: data-edge-phase
description: v0.8.0 data-edge.json slice plan (iter 215 onward), probed seed outcomes, packaging traps
metadata:
  type: project
---

Second v0.8.0 criterion (added by the human 2026-11-19): `crates/iscc-lib/tests/data-edge.json` run
by Rust + 11 bindings.

**Why:** spec `specs/edge-vectors.md` orders it: file + Rust + Python harnesses → one fan-out for
JSON-parsing bindings → C/C++ (generated table, recover `scripts/gen_ffi_boundary_vectors.py` from
commit `ea69169`).

**How to apply:**

- 215 scoped the seed + Rust/Python harnesses + schema test; deferred the vendored copies
    (Go/.NET/Kotlin/Swift dirs next to data.json) to the fan-out.
- `iscc_decode` output encoding was fixed in 215: `[mt, st, vs, length_index, "bytes:<hex>"]`.
- Packaging trap: `crates/iscc-lib/tests/` ships in the .crate (`cargo package --list`), so the file
    needs `exclude` in Cargo.toml and a runtime read (not `include_str!`) in the Rust test.
- Probed 2026-11-19 (iscc_core 1.5.0): the ref ACCEPTS padding/spare-bit forms on `iscc_decode` too
    (not only `encode_seq`); ref `gen_mixed_code_v0([])` raises AssertionError; ref seq NUL raises
    binascii.Error. gen_text NUL agrees (`ISCC:EAASKDNZNYGUUF5A`, 10 chars).
- Fan-out needs per-surface `except`: .NET/C++ `error` on NUL strings, C `skip`.
- 215 NEEDS_WORK: Rust test helpers score CRAP at 0% coverage (cc²+cc) → cc ≤ 5 per fn, else CI-only
    `--fail-above` red. 216 = repair (section fn table, split check_vector/check_doc; Python
    `pytest.raises` on raw call only; `REJECTIONS` += `IndexError`, proven by an `iscc_decode("")`
    vector). Any harness `tests/*.rs` step must put CI's two CRAP lines in Verification.
- Fan-out (after 216 passes): vendor next to the 4 data.json copies, fill `VENDORED_COPIES`, add
    sdist/gem "not shipped" checks (review asked).
