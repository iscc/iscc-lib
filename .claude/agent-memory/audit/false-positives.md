---
name: audit-false-positives
description: Audit finding patterns that look like defects but are reference parity or handled elsewhere — do not file
metadata:
  type: feedback
---

Do not file these:

- **`gen_*` accept bits > 256 and emit codes `iscc_decode` rejects** (text/data/instance/audio/meta;
    image caps at 256). iscc-core returns the identical string for `gen_text_code_v0('Hello', 512)`,
    so this is reference parity, not an iscc-lib defect. If it is raised at all, it goes upstream.
- **Go conformance tests `t.Skipf` when data.json is missing**
    (`packages/go/codec_test.go:1104,1174`): sibling tests in the same package read the same path
    and `t.Fatalf`, so the monorepo CI still fails.

**Why:** finders surface these with plausible evidence; checking against the reference or the
sibling tests refutes them. **How to apply:** before verifying a "surprising output" finding,
compare with the reference output first.
