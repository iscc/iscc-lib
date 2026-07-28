---
name: repo-counts
description: Exact tracked-artifact counts (tests, symbols, pages, READMEs) plus the glob/grep incantation that reproduces each one without over- or under-counting
metadata:
  type: project
---

# Repo Counts and How to Reproduce Them

Baseline re-verified at iteration 161 unless noted. Each entry names the trap that makes a naive
command give a different number.

| Artifact                         | Count        | Command / trap                                                                                             |
| -------------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------- |
| crate/package `README.md`        | 12           | scope the glob to `crates/*/` + `packages/*/`; a bare grep gives 14                                        |
| crate/package `CLAUDE.md`        | 12           | same scoping trap                                                                                          |
| docs pages                       | 23           | `git ls-files 'docs/*.md' 'docs/**/*.md'` gives 24 — subtract `docs/includes/abbreviations.md` (a snippet) |
| `docs/howto/*.md`                | 11           |                                                                                                            |
| iscc-lib `#[test]`               | 342          | glob `git ls-files 'crates/iscc-lib/**/*.rs'`; `src/` alone gives 288                                      |
| pytest collected                 | 441 (at 164) | `uv run pytest --collect-only -q`                                                                          |
| pytest-benchmark fixtures        | 18           |                                                                                                            |
| `packages/go` `^func Test`       | 177          |                                                                                                            |
| iscc-ffi externs                 | 47           | `'#\[unsafe(no_mangle)\]'`; bare `no_mangle` gives 48                                                      |
| UniFFI exports                   | 32           | same Tier 1 surface as every other binding                                                                 |
| CRAP baseline `entries`          | 105          | `.crap-baseline.json`                                                                                      |
| iai baseline entries             | 16           | 11 bench fns                                                                                               |
| criterion benches                | 12           | criterion **0.8.2** since 171; one `criterion_group!` in `benches/benchmarks.rs`                           |
| version_sync targets             | 21           | `scripts/version_sync.py --check` — the authoritative set of version-synced files                          |
| tracked fixture copies           | 8            | `git ls-files -- '*data.json' '*unicode_boundary.json'` == `VENDORED_COPIES`                               |
| documented speedups              | 1.3x-158x    |                                                                                                            |
| iscc-rb `define_*` registrations | 33           | covers the 32-symbol Tier 1 surface                                                                        |
| iscc-jni natives                 | 33           | `grep -oE 'fn Java_[A-Za-z0-9_]+'`; `grep -c 'extern "system"'` gives **34** (one non-`Java_` fn)          |
| `crates/iscc-jni/src/lib.rs`     | 1168 lines   | was ~1060 pre-jni-0.22; the stale number in `specs/java-bindings.md` was fixed by hand at 171              |

## JVM test suites — annotations are NOT cases

Both JVM suites are `@TestFactory`-driven, so source annotations under-report:

- Kotlin: **9 + 3** annotations (`ConformanceTest.kt` + `UnicodeBoundaryTest.kt`) → **9 + 13**
    reported cases.
- Java: **29 + 3** annotations (`IsccLibTest.java` + `UnicodeBoundaryTest.java`) → **69 + 13 = 82**
    surefire cases.
- The dynamic totals are derivable from `data.json` vector counts: 20+5+3+5+3+2+4+3+5 = 50.
