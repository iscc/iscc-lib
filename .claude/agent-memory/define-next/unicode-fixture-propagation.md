---
name: unicode-fixture-propagation
description: Ledger for propagating unicode_boundary.json into the 11 binding surfaces — loader taxonomy, which binding artifacts are stale, measured Go pass/fail, slice order
metadata:
  type: project
---

# Propagating `unicode_boundary.json` to the bindings (issue remainder **(b)**)

The Rust fixture (`crates/iscc-lib/tests/unicode_boundary.json`, 7 `text_clean` + 5 `text_collapse`
cases, pure ASCII) is finished and design-discriminating since iter 149. What remains is wiring it
into each binding's test suite.

**Why:** the sentinel freeze rule is a cross-cutting conformance contract; until every surface gates
it, 11 language surfaces ship ungated. **How to apply:** scope one language group per step, cite the
measured facts below instead of re-probing.

## Loader taxonomy — decides how much plumbing a slice needs

- **Canonical-path readers** (no file copy needed; a second fixture is a path constant + new test):
    Python `tests/test_conformance.py`, napi `crates/iscc-napi/__tests__/conformance.test.mjs`, WASM
    `crates/iscc-wasm/tests/conformance.rs` (`include_str!("../../iscc-lib/tests/data.json")`), Ruby
    `crates/iscc-rb/test/test_conformance.rb`.
- **Vendored-copy surfaces** (need a byte-identical `cp` next to their `data.json`):
    `packages/go/testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`. JNI/Java reads
    `data.json` from `crates/iscc-jni/java/src/test/.../IsccLibTest.java` — check its path shape
    before assuming.
- Go is a hybrid: its **per-function `*_test.go` conformance tests read
    `../../crates/iscc-lib/tests/data.json` by relative path**, while `testdata/data.json` exists
    only so the *public* `ConformanceSelftest()` can `//go:embed` it. Either shape is defensible;
    iter 150 chose the embedded copy (no skip-on-missing branch, works from a downloaded module).

## Binding artifact freshness (measured iter 150)

- **Python editable install: CURRENT** — passes all 12 vectors, no rebuild needed.
- **napi `crates/iscc-napi/iscc-lib.linux-x64-gnu.node`: STALE (pre-iter-133)** — returns `aSb` for
    `text_clean("a" + U+A7F1 + "b")` and `eŚ` for the U+A7F1 sequence. A napi slice must rebuild the
    addon first; budget for it in the step.
- Probe rule: U+0378 rows do **not** discriminate a stale artifact (U+0378 is `Cn` in every Unicode
    version, so even a no-freeze-rule build gets them right). Only **U+A7F1** rows do.

## Go — measured results on all 12 vectors (iter 150, Go 1.26.1)

9 pass, exactly 3 fail:

| section         | case                                    | why it fails                        |
| --------------- | --------------------------------------- | ----------------------------------- |
| `text_clean`    | `test_0000_u1fae9_assigned_so_retained` | U+1FAE9 is `Cn` in Go's 15.0 tables |
| `text_clean`    | `test_0001_u113c5_assigned_mc_retained` | U+113C5 is `Cn` in Go's 15.0 tables |
| `text_collapse` | `test_0000_u1fae9_assigned_so_retained` | same cause                          |

- **All four sequence vectors PASS** — the iter-147 `Final_Sigma` fix is live.
- `text_collapse/test_0001_u113c5_assigned_mc_mark_dropped` **passes** (Go drops the mark as `C`,
    the expectation drops it as `M`; both give `ab`). So the skip list is **per case, not per code
    point** — a naive "skip both Unicode-16 code points everywhere" over-skips by one.
- Ruling: `decisions.md` 2026-07-26 — skip with a tracking note until go1.27 (~Aug 2026); never
    vendor the 5,813-code-point 15.0→16.0 delta.

## Slice order used / planned

1. **iter 150** — Python (canonical path) + Go (vendored copy + 3 skips). Establishes both plumbing
    patterns in one step; both are runnable in this container with no artifact rebuild.
2. napi + WASM (both need a build: `napi build`, `wasm-pack test --node`).
3. Ruby (needs `libclang`; check it is installed before scoping) + JNI/Java (maven present, no
    gradle in this container).
4. dotnet + Kotlin + Swift (**no Swift toolchain and no gradle here** — those are CI-only checks, so
    scope them with static verification plus a CI-green criterion, and keep them last).

## Standing hazards

- Copy vendored fixtures with `cp`, never Write/Edit, then verify with `cmp` — a tool payload can
    decode `\uXXXX` into literal UTF-8 and silently break an ASCII-escaped file (observed iter 149;
    *not* reproduced in iter 150, so treat it as transport-dependent and always verify).
- Do **not** replicate the `delete_filter_output` oracles into binding tests. They live in the Rust
    `SEQUENCE_VECTORS` const and the issues.md table; equality against `outputs.result` already
    catches a delete-filter regression, and copying oracles 11× re-opens the iter-149 mislabel
    hazard. See [[unicode-freeze-facts]] for the corrected row-3 value.
- No vendored-copy drift gate exists (not for the five `data.json` copies either). Adding one is a
    separate step; per-slice `cmp` is the interim guard.
