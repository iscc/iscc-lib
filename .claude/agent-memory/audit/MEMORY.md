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
- iter 280 @ db1fe55: filed 5 (Go meta limit unenforced, vcpkg v0.2.0 hashes, data.json parsers
    beside the runners, WASM finalize consumption, Rust API tier docs). 4 finders + 4 batch skeptics
    \+ 1 watch-list skeptic = 9 agents, about $19; 59 of 61 survived, 23 of 24 watch items held.
- iter 290 @ b3869b91: filed 3 (version_sync check blind spots, Go EncodeComponent short digest,
    WASM init/target drift). 4 finders piped into 4 batch skeptics + 1 watch skeptic = 9 agents,
    about $19.6; 38 of 38 survived, 30 of 31 watch items held. Conan entry cut for the line budget.

## Clean areas

(none: every dimension had verified findings at 34303a6)

## Watch list

- [Watch list](watch-list.md) — verified findings left unfiled (5-per-pass cap); data-edge harness
    duplication and napi/wasm narrowing helpers go first

## False-positive patterns

- [False positives](false-positives.md) — reference parity, decided behavior (IDv1 binding range
    checks, ruff S/C901 hooks, Go TextTrim), superseded decisions

## Operating notes

- issues.md budget is 400 lines (391 after iter 290: room for ~2 entries, so file the sharpest two
    or three); run the prek `mdformat` hook on it before committing (wrapping adds lines).
- Downloading a published artifact settles a packaging claim cheaply (iter 280: vcpkg hashes matched
    the v0.2.0 tarball, not v0.7.0).
- Handing the watch list to the skeptics works: it refuted 4 stale items (incl. the 2-pass IDv1
    item) by decision. Workflow output JSON nests results under `.result`.
- One batch skeptic per finder is lenient (62/63 survive): spot-check the top candidates yourself
    with `sed -n` before filing; refute-by-decision is what it catches.
- Budget: the 9-agent workflow cost ~$19-20 of $30; leave ~$8 for spot-checks, filing and memory.
    Batch skeptics refuted 0 of 38 in iter 290: they confirm facts, not significance; judge that
    yourself and check the watch-list skeptic for "tracked by an open issue" refutations.
- Grouping 9 dimensions into 5 finders, each followed by one batch skeptic (10 agents), stayed
    within the $30 budget. The iter-240 workflow, run with the advisor on, was killed.
