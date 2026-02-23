## 2026-02-23 — Review of: Add DataHasher/InstanceHasher streaming classes to WASM bindings

**Verdict:** PASS

**Summary:** DataHasher and InstanceHasher streaming classes added to WASM bindings, bringing WASM
from 21/23 to 23/23 Tier 1 parity with Python and Node.js. Implementation is clean — follows the
established `Option<Inner>` finalize-once pattern adapted for wasm-bindgen, with comprehensive tests
(14 new). All quality gates pass, no scope violations.

**Verification:**

- [x] `cargo build -p iscc-wasm --target wasm32-unknown-unknown` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `cargo test --workspace` passes with 280 tests (no regressions)
- [x] `wasm-pack test --node crates/iscc-wasm` passes all 54 tests (9 conformance + 45 unit — 31
    existing + 14 new)
- [x] `mise run check` passes all 14 pre-commit hooks

**Issues found:**

- (none)

**Next:** The only remaining binding gap is C FFI, which needs `DataHasher` and `InstanceHasher`
streaming classes. This requires the opaque pointer lifecycle pattern (`*mut T` +
create/update/finalize/free functions) which is more involved than class-based bindings. After C FFI
reaches 23/23, all four binding targets will have full Tier 1 parity.

**Notes:** With WASM at 23/23, the binding parity status is: Python 23/23, Node.js 23/23, WASM
23/23, C FFI 21/23. The C FFI streaming hashers are the last binding gap before moving to structured
return types or other enhancements. The opaque pointer pattern for C FFI is documented in learnings
(thread-local `RefCell<Option<CString>>` for error storage, `*const u8` + `usize` for byte slices).
