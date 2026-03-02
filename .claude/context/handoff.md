## 2026-03-02 — Review of: Expose `add_units` in Python binding (issue #21)

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, correctly scoped implementation exposing `add_units: bool` parameter and `units`
return field in the Python binding. The PyO3 wrapper, Python wrapper, and type annotations all match
the Rust core API. Three well-targeted tests cover enabled, disabled, and attribute-access paths.
One issue found: the `.pyi` type stub was not updated, causing `ty check` to fail — fixed by the
review agent.

**Verification:**

- [x] `cargo build -p iscc-py` compiles without errors
- [x] `mise run test` passes — 394 Rust tests (258 lib + 31 algo + 22 utils + 82 FFI + 1 doctest)
    - 207 Python tests (including 3 new smoke tests)
- [x] `cargo clippy -p iscc-py -- -D warnings` — clean
- [x] `gen_sum_code_v0(path, add_units=True)` returns dict with `"units"` key containing 2 ISCC
    strings — confirmed via test
- [x] `gen_sum_code_v0(path)` (default) returns dict WITHOUT `"units"` key — confirmed via test
- [x] `mise run check` — 14/14 hooks pass
- [x] `ty check` — passes (after `.pyi` stub fix)

**Issues found:**

- `.pyi` stub for `_lowlevel.gen_sum_code_v0` was not updated with the `add_units` parameter,
    causing `ty check` to fail. Fixed directly by the review agent (minor fix — type stub only, no
    behavior change).

**Codex review:** Codex identified two findings:

1. **P1 (confirmed, fixed):** The `_lowlevel.pyi` stub was out of sync with the 4-argument call
    site. This was a real issue causing `ty check` failure — fixed by updating the stub.
2. **P2 (noted, no action):** `SumCodeResult.units` raises `AttributeError` when `add_units=False`
    instead of returning `None`. This follows the existing `IsccResult.__getattr__` pattern used by
    all result classes (e.g., `MetaCodeResult.description` when description is absent). The
    `list[str] | None` annotation signals optionality for IDE hints, consistent with the codebase
    convention. Dict-style `"units" in result` is the safe check.

**Next:** Expose `add_units` in Node.js binding (`crates/iscc-napi`). The NAPI pattern is documented
in learnings.md — `NapiSumCodeResult` struct needs an `Option<Vec<String>>` field for `units`, and
the `gen_sum_code_v0` function needs `Option<bool>` for `add_units`. After Node.js, continue with
remaining bindings (C FFI, JNI, WASM, Go) to complete issue #21.

**Notes:** The advance agent should always update the `.pyi` type stub when modifying Python binding
signatures. `ty check` runs in pre-push hooks and would catch this, but fixing it during advance
avoids review roundtrips. Python test count is now 207 (was 204 before this iteration).
