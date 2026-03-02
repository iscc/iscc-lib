## 2026-03-02 — Review of: Add `units` support to `gen_sum_code_v0` in Rust core (issue #21)

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding `add_units: bool` parameter and
`units: Option<Vec<String>>` field to `gen_sum_code_v0`/`SumCodeResult`. The borrow-before-move
pattern for `gen_iscc_code_v0` is correct, all 4 binding crates compile with the new parameter, and
2 new tests validate both enabled and disabled paths. All 7 verification criteria pass.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 312 tests (258 + 31 + 22 + 1), 2 new vs prior 310
- [x] `cargo clippy -p iscc-lib -- -D warnings` — clean
- [x] `cargo build -p iscc-py` — compiles
- [x] `cargo build -p iscc-napi` — compiles
- [x] `cargo build -p iscc-ffi` — compiles
- [x] `cargo build -p iscc-jni` — compiles
- [x] `cargo bench -p iscc-lib --no-run` — compiles
- [x] `mise run check` — 14/14 hooks pass

**Issues found:**

- (none) — implementation is clean and correctly scoped

**Codex review:** Codex couldn't run tests (cross-device link error in sandbox) but performed static
analysis. It verified all binding call sites are updated, confirmed no destructuring breakage due to
`#[non_exhaustive]`, and noted docs (`docs/rust-api.md`, `docs/architecture.md`) still reference the
old 3-parameter signature. Doc updates are explicitly out of scope per next.md — they should be
included when binding APIs are exposed.

**Next:** Expose `add_units` parameter in binding APIs. Suggested order: Python first (primary
`iscc-sdk` consumer), then Node.js, C FFI, JNI. WASM has its own inline `gen_sum_code_v0`
implementation — needs separate scoping. Go needs a pure Go reimplementation. When exposing
bindings, also update `docs/rust-api.md` and `docs/architecture.md` to show the new 4-parameter
signature. Issue #21 remains open until all bindings are updated.

**Notes:** The `#[non_exhaustive]` attribute on `SumCodeResult` ensures the new field doesn't break
downstream pattern matches. When `add_units` is false, `data_result.iscc` and `instance_result.iscc`
are dropped after `gen_iscc_code_v0` borrows them — no unnecessary cloning. State.md still shows the
old 3-parameter signature at line 23 — update-state will pick this up next cycle.
