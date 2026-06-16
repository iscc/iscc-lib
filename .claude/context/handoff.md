## 2026-06-16 — Review of: Add core `streaming::SumHasher` struct and drive `gen_sum_code_v0` from it

**Verdict:** PASS

**Summary:** The advance agent added a single-pass `streaming::SumHasher`
(`new() → update(&[u8]) → finalize(bits, wide, add_units)`) that composes inner `DataHasher` +
`InstanceHasher`, and refactored `gen_sum_code_v0` into a thin file-I/O wrapper that delegates all
composition to it. Clean, in-scope, well-tested (8 new unit tests incl. a temp-file cross-check that
`SumHasher` equals the file-based path), and the "core first" prerequisite of issue #37 is met. No
public API break.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 269 unit + 28 integration + 22 + 1 doctest, all green
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean (also `--all-targets`)
- [x] `cargo fmt -p iscc-lib --check` clean
- [x] Test asserts `SumHasher` == `gen_sum_code_v0` on a temp file across `(bits, wide, add_units)`
    — `test_sum_hasher_matches_gen_sum_code_v0` present and passing (single source of truth
    confirmed)
- [x] `gen_sum_code_v0` no longer contains its own dual-hasher loop — delegates to
    `SumHasher::finalize`
- [x] Feature configs: `--no-default-features` (219 tests) + `--all-features` (269 tests) green
- [x] `mise run check` (15 pre-commit hooks) all Passed

**Issues found:**

- (none) — scope discipline is exact: only `streaming.rs` + `lib.rs` touched. No crate-root
    `pub use SumHasher` re-export, no Tier 1 count change, no PyO3/wasm wrapper, no GIL release —
    all correctly deferred per "Not In Scope". `finalize` takes `add_units` (vs the issue's terse
    `finalize(bits, wide)`) because `gen_sum_code_v0` needs it — a sensible, necessary deviation.

**Codex review:** No introduced correctness, security, or maintainability issues. Confirmed the
refactor preserves `gen_sum_code_v0` behavior and the new composition matches the previous path.

**Issue housekeeping:** Swept the stale **"Narrow internal module visibility to `pub(crate)`"**
issue — that work landed in commit `3f6a61d` (iteration 86) and `lib.rs` now declares
`pub(crate) mod cdc/conformance/minhash/simhash/utils` exactly as specified. No cross-crate
module-path imports remain (the `iscc_lib::conformance_selftest()` calls in bindings use the Tier 1
crate-root re-export, not the private `conformance` module path). The issue's `**Spec:**` already
described `pub(crate) mod` as the target, so no spec edit was needed — the code was brought into
conformance.

**Next:** Continue issue #37 — the **PyO3 + wasm-bindgen `SumHasher` wrappers** (the binding half).
That step should expose `SumHasher` mirroring the `DataHasher`/`InstanceHasher` finalize-once
(`Option<inner>`) pattern in `crates/iscc-py` and `crates/iscc-wasm`, **promote `SumHasher` to a
crate-root Tier 1 `pub use`** (currently only `iscc_lib::streaming::SumHasher`), and bump the
documented Tier 1 "2 streaming types → 3" wording in `crates/iscc-lib/CLAUDE.md`,
`docs/rust-api.md`, and any README symbol tables. Alternatively, pick the highest-impact backlog
item: npm `optionalDependencies` fix (#38, actively breaks downstream `npm ci`) or the PyO3 0.23 →
0.29 security migration.

**Notes:**

- **Pre-existing (not a regression):**
    `cargo clippy/test -p iscc-lib --no-default-features   --all-targets` fails to compile the
    `benchmarks` bench — it imports `gen_meta_code_v0` / `gen_text_code_v0` which need the default
    `meta-code`/`text-processing` features. The lib + tests build fine without default features; CI
    never runs the `--no-default-features --all-targets` combo. Scope feature-config clippy to the
    lib only. Recorded in learnings.md.
- **Push state:** this review pushes 10 unpushed `develop` commits as one batch, including the
    module-narrowing change `3f6a61d` (not yet CI-verified) and the mise-task migration `b1127ed`.
    Pre-push hooks + CI provide the verification the local run could not. Pre-push needs `iscc_lib`
    compiled (`ty check`/`pytest` import it) — it is built from this review's cargo runs.
- Hot path unchanged: `SumHasher::update` calls the same two inner hashers as before (one extra
    inlined call layer); no benchmark regression expected.
