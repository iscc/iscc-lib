# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

### Tab order inconsistency across doc pages

**Priority:** low | **Source:** [review]

**Spec:** `specs/documentation.md` — "Standard tab order: Python, Rust, Java, Node.js, WASM"

Tab order differs across pages: spec says "Python, Rust, Java, Node.js, WASM" (no Go), landing page
uses "Rust, Python, Node.js, Java, Go, WASM" (Rust first), tutorial uses "Python, Rust, Node.js,
Java, Go, WASM" (Python first, includes Go). The spec should be updated to include Go and a single
canonical order should be applied consistently. `HUMAN REVIEW REQUESTED` — spec change needed to add
Go to the standard tab order.
