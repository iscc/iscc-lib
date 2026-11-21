# Audit Agent Memory

Persistent knowledge for the codebase audit role: areas audited clean (with HEAD sha), watch-list
items not yet issue-worthy, and false-positive patterns. Keep under 200 lines; archive stale entries
to `MEMORY-archive.md`.

## Pass log

- iter 249 @ deb5c5e: first audit that filed anything. Filed 4 (FFI NULL/len-0 split, integer-arg
    wrap/exception classes, published-package docs drift, pytest gate latency). The 5-finder +
    5-skeptic pipeline cost about $14; 31 of 32 candidates survived.
- iter 260 @ 3040298: filed 5 (Go TextTrim panic + hasher reuse, CDC param overflow, one-shot gen
    duplicates streaming, Python video exception class, spec/CLAUDE.md drift). 5 finders + 5 batch
    skeptics cost about $17.7; 62 of 63 survived.
- iter 270 @ 34303a6: filed 5 (wasm Node docs vs web build, UniFFI text_trim 32-bit + doc, minhash
    empty vs reference, conformance runner duplication, gates on stale generated/built artifacts). 5
    finders + 4 batch skeptics (watch list included) cost about $18; 54 of 58 survived.

## Clean areas

(none: every dimension had verified findings at 34303a6)

## Watch list

- [Watch list](watch-list.md) — verified findings left unfiled (5-per-pass cap); WASM finalize
    consumption and data-edge harness duplication go first

## False-positive patterns

- [False positives](false-positives.md) — reference parity, decided behavior (IDv1 binding range
    checks, ruff S/C901 hooks, Go TextTrim), superseded decisions

## Operating notes

- issues.md budget is 400 lines (373 after iter 270: file at most 1-2 next pass); run the prek
    `mdformat` hook on it before committing (wrapping adds lines).
- Handing the watch list to the skeptics works: it refuted 4 stale items (incl. the 2-pass IDv1
    item) by decision. Workflow output JSON nests results under `.result`.
- One batch skeptic per finder is lenient (62/63 survive): spot-check the top candidates yourself
    with `sed -n` before filing; refute-by-decision is what it catches.
- Grouping 9 dimensions into 5 finders, each followed by one batch skeptic (10 agents), stayed
    within the $30 budget. The iter-240 workflow, run with the advisor on, was killed.
