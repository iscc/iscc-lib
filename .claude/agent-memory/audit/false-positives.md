---
name: audit-false-positives
description: Audit finding patterns that look like defects but are reference parity, decided, or handled elsewhere — do not file
metadata:
  type: feedback
---

Do not file these:

- **`gen_*` accept bits > 256**: was reference parity; since 2026-11-22 decided (reject) and tracked
    by a `[human]` issue in issues.md — do not file again.
- **Go conformance tests `t.Skipf` when data.json is missing**: sibling tests read the same path and
    `t.Fatalf`, so CI still fails.
- **UniFFI (Swift/Kotlin) exports have no default arguments**: declared on purpose in
    `specs/swift-bindings.md` (refuted iter 260).
- **IDv1 range checks repeated per binding** (`gen_iscc_id_v1`): the core takes u16/u8, so each
    binding narrowing a wider native int must check first; covered by a binding decision (refuted
    iter 270, after two passes on the watch list).
- **ruff `S`/`C901` pre-push hooks duplicating pyproject rules**: kept on purpose by decision so a
    config regression cannot drop the security scan (refuted iter 270).
- **Go `TextTrim` returns "" for negative nbytes**: decided and test-pinned (refuted iter 270).
- **"Reversed advisor decisions still listed"**: the 2026-11-21 entry explicitly supersedes them;
    that is how the append-only log works (refuted iter 270).

**Why:** finders surface these with plausible evidence; checking against the reference, the sibling
tests or decisions.md refutes them. **How to apply:** before verifying a "surprising behavior" or
"duplicated check" finding, grep decisions.md (live + archive) and compare with the reference.

- **Archived decision "JS-number IDv1 surfaces take f64" vs wasm `JsValue`**: wasm reads the number
    as f64 and runs `checked()` before narrowing, which is what the decision requires (refuted iter
    280).
- **C/C++ `encode_component` empty-digest tests "cannot fail"**: the C++ test fails on a NULL
    message, so it does discriminate (refuted iter 300).
