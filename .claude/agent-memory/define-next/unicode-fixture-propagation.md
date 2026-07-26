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

## Binding artifact freshness (measured iters 150-151)

- **Python editable install: CURRENT** — passes all 12 vectors, no rebuild needed.
- **napi `crates/iscc-napi/iscc-lib.linux-x64-gnu.node`: STALE (pre-iter-133)** — returns `aSb` for
    `text_clean("a" + U+A7F1 + "b")` and `eŚ` for the U+A7F1 sequence. It is **untracked**
    (`crates/iscc-napi/.gitignore:4`), so a napi slice must rebuild it; budget for it in the step.
- **Ruby `crates/iscc-rb/lib/iscc_lib/iscc_rb.so`: was STALE, rebuilt at iter 151.**
    `bundle exec rake compile` (~1.5 min, cached cargo) flipped `text_clean("a"+U+A7F1+"b")` from
    `"aSb"` to `"ab"`. The `.so` is gitignored (`crates/iscc-rb/.gitignore:3`) → rebuilding leaves
    no tree diff. Baseline suite: **111 runs / 299 assertions / 0 failures**, `standardrb` clean.
- **WASM has no artifact age** — `wasm-pack test --node` recompiles the core every run.
- Probe rule: U+0378 rows do **not** discriminate a stale artifact (U+0378 is `Cn` in every Unicode
    version, so even a no-freeze-rule build gets them right). Only **U+A7F1** rows do.

## Runner facts (measured iter 151, offline)

- `wasm-pack test --node crates/iscc-wasm --features conformance` → **exit 0 in ~2 min**; wasm-pack
    0.13.1 with `wasm-bindgen`/`wasm-opt` already in `~/.cache/.wasm-pack` (no download).
    `--features conformance` only gates the `conformance_selftest` test in `unit.rs`;
    `conformance.rs` is ungated, so new boundary tests need no `cfg`.
- **`#[wasm_bindgen_test]` files compile on the host but run as 0 tests there** —
    `cargo test -p iscc-wasm` reports `0 passed` for every target. So host `cargo test` verifies
    only compilation; `wasm-pack test --node` is the only executing runner. Say this in next.md or
    advance will chase a phantom failure.
- CRAP/coverage are `-p iscc-lib` only (`mise.toml [tasks.coverage]`) and `.crap-baseline.json` has
    **zero** `iscc-wasm` entries → binding-test slices never move a baseline.
- Ruby: `rake test` loads every `test/*.rb` into ONE process, and `test_conformance.rb` defines
    top-level `DATA_JSON` / `CONFORMANCE_DATA` — a new file must use fresh constant names or Ruby
    warns "already initialized constant". `standardrb` is CI-only, **not** a prek hook.

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

1. **iter 150 ✅** — Python (canonical path) + Go (vendored copy + 3 skips). Establishes both
    plumbing patterns in one step; both runnable in-container with no artifact rebuild.
2. **iter 151 scoped** — WASM + Ruby, both canonical-path readers, both measured runnable
    in-container (~2 min each). No skips: both execute the Rust core, so all 12 vectors must pass.
3. napi (rebuild the addon first) + C FFI and/or JNI/Java — note **C FFI, JNI-Java, Kotlin and Swift
    have no text-function tests at all**, so those are new plumbing, not a copied loop; the C FFI
    fixture needs a JSON reader or a generated C table (scope deliberately). Maven is present,
    gradle is not.
4. dotnet + Kotlin + Swift (**no Swift toolchain and no gradle here** — CI-only checks, so scope
    with static verification plus a CI-green criterion, and keep them last). **Land the
    vendored-copy byte-identity drift gate immediately before this slice** — it is the only
    remaining slice that adds vendored copies (3 of them), so that is where the gate first protects
    something new. Slices 1-3 are canonical-path readers except Go, which was `cmp`-verified
    in-step.

## Standing hazards

- Copy vendored fixtures with `cp`, never Write/Edit, then verify with `cmp` — a tool payload can
    decode `\uXXXX` into literal UTF-8 and silently break an ASCII-escaped file (observed iter 149;
    *not* reproduced in iter 150, so treat it as transport-dependent and always verify).
- Do **not** replicate the `delete_filter_output` oracles into binding tests. They live in the Rust
    `SEQUENCE_VECTORS` const and the issues.md table; equality against `outputs.result` already
    catches a delete-filter regression, and copying oracles 11× re-opens the iter-149 mislabel
    hazard. See [[unicode-freeze-facts]] for the corrected row-3 value.
- **Vendored-copy drift gate — scoped iter 152** as `tests/test_vendored_fixtures.py` (explicit
    `(canonical, copy)` table, count floor, plus a "no unregistered tracked copy" test). Tracked
    fixture files at HEAD: **7 = 2 canonical + 5 copies** (4 × `data.json` + 1 ×
    `unicode_boundary.json`). Until it lands, per-slice `cmp` is the interim guard.
- **Discovery must use `git ls-files`, never `Path.rglob`**: untracked build outputs exist in this
    container (`packages/dotnet/Iscc.Lib.Tests/bin/Debug/net8.0/testdata/data.json`,
    `packages/kotlin/build/resources/test/data.json`) and `.venv/…/iscc_core/data.json` is a
    *different* file (`md5 3690f853…` vs the canonical `4f17639ab1dd…`).
- **Only `unicode_boundary.json` is pure ASCII** (2,344 B, `\uXXXX` escapes — so an `isascii()`
    assertion is a valid corruption guard for it). `data.json` is 84,478 B and legitimately contains
    raw non-ASCII: never assert ASCII on it.
