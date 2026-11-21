# Audit Agent Memory

Persistent knowledge for the codebase audit role: areas audited clean (with HEAD sha), watch-list
items not yet issue-worthy, and false-positive patterns. Keep under 200 lines; archive stale entries
to `MEMORY-archive.md`.

## Pass log

- iter 249 @ deb5c5e: first audit that filed anything. Filed 4 (FFI NULL/len-0 split, integer-arg
    wrap/exception classes, published-package docs drift, pytest gate latency). The 5-finder +
    5-skeptic pipeline cost about $14; 31 of 32 candidates survived.

## Clean areas

(none recorded yet; every swept dimension had verified findings at deb5c5e)

## Watch list

- [Watch list](watch-list.md) — verified findings left unfiled for lack of issues.md room; file
    these first

## False-positive patterns

- [False positives](false-positives.md) — bits>256 is reference parity; Go Skipf is backed by
    sibling Fatalf

## Operating notes

- issues.md is near its 300-line budget, so entries of about 9 lines each are what fits; run the
    prek `mdformat` hook on it before committing (wrapping adds lines).
- Grouping 9 dimensions into 5 finders, each followed by one batch skeptic (10 agents), stayed
    within the $30 budget. The iter-240 workflow, run with the advisor on, was killed.
