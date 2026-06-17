## 2026-06-17 — Review of: Add streaming `SumHasher` to the WASM bindings

**Verdict:** PASS

**Summary:** The advance agent added a `#[wasm_bindgen]` `SumHasher` class to `crates/iscc-wasm`,
wrapping the shared core `iscc_lib::streaming::SumHasher` with the established `Option<inner>`
finalize-once pattern and reusing the existing `WasmSumCodeResult`. It is a pure translation layer —
no algorithm logic, output verified equal to the one-shot `gen_sum_code_v0`. Scope was clean (WASM
crate only; no core, Python, other bindings, or Tier 1 changes), and this fully closes issue #37
across all bindings (core + Python + WASM).

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` — 78 passed, 0 failed (70
    prior + 8 new `test_sum_hasher_*`, all confirmed present)
- [x] `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` — clean
- [x] `cargo fmt -p iscc-wasm --check` — clean
- [x] `grep -q "pub struct SumHasher" crates/iscc-wasm/src/lib.rs` — exit 0
- [x] `grep -q "SumHasher" docs/howto/wasm.md` — exit 0
- [x] "32 Tier 1 symbols" count unchanged in README.md / rust-core.md / target.md — SumHasher not
    promoted to Tier 1 (no crate-root re-export); only the WASM crate's own CLAUDE.md bumped to "3
    streaming types"
- [x] `mise run check` (15 pre-commit hooks) — all passed

**Issues found:**

- (none in the advance work) — implementation matches the `gen_sum_code_v0` reference exactly
    (`filesize: u64 → f64` cast, `units` mapped directly, errors via `JsError`, finalize-once via
    `inner.take()`). Tests use real byte data and assert against both `gen_sum_code_v0` and the
    individual `gen_data_code_v0`/`gen_instance_code_v0` units.
- Reviewer minor fix applied: the WASM crate `CLAUDE.md` "Test Commands" listed
    `wasm-pack test ... -- --test unit` (the runner rejects `--test` after `--`). Corrected to put
    `--test <name>` before the `--` (passed to cargo) — verified `--test unit` runs the 77 unit
    tests. The advance agent had flagged this as stale but left it out of scope.

**Codex review:** Completed, no findings. "The new WASM SumHasher binding follows the existing
streaming wrapper pattern, delegates to the core implementation, and is covered by passing WASM
tests. I did not find any introduced correctness issues."

**Next:** Issue #37 is fully closed (deleted from issues.md; `wasm-bindings.md` spec updated and its
verification box checked). Remaining `normal`-priority backlog, in suggested order:

1. **npm `optionalDependencies` fix (#38)** — confirmed downstream-breaking bug: published
    `@iscc/lib@0.4.0` declares five unpublished per-platform `optionalDependencies` that 404 and
    break `npm ci`. The fix decision is already recorded (keep bundled-package model; drop the
    `napi prepublish -t npm` injection). Well-scoped, user-facing, touches the release workflow +
    `crates/iscc-napi/package.json`.
2. **PyO3 0.23 → 0.29 security migration** — closes two RustSec advisories; six-minor-version jump,
    migrate incrementally per the PyO3 migration guide. Larger but security-relevant.
3. **GIL release (#39)** — perf; when it lands it must also cover the new `PySumHasher.update()`.

I recommend #38 next: it is a confirmed install-breaking bug for downstream consumers with a clear,
pre-recorded fix decision.

**Notes:**

- `SumHasher` remains intentionally NOT a crate-root Tier 1 re-export — it is a Python/WASM
    streaming convenience reached via `iscc_lib::streaming::SumHasher`. Do not bump the "32 Tier 1 /
    2 streaming types" counts in README/rust-core/target for it.
- 4 unpushed commits (update-state, iteration-89 log, define-next, advance) will push as one batch
    with this review — normal: the iteration-89 log commit is written by the runner after the prior
    review's push, so it trails until the next push.
- The `WasmSumCodeResult` was reused (no new result struct), so the WASM CLAUDE.md "2 result
    structs" count is unchanged and correct.
