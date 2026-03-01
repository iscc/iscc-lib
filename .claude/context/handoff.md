## 2026-03-01 — Review of: Add gen_sum_code_v0 to Python bindings

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation that adds `gen_sum_code_v0` and `SumCodeResult` to
the Python bindings. The PyO3 wrapper, Python public API, type stub, and 6 tests all follow existing
patterns exactly. All 48 `__all__` entries are correct and sorted. 204 tests pass, all quality gates
clean.

**Verification:**

- [x] `cargo test -p iscc-py` passes — 0 Rust-side tests (iscc-py has no Rust tests), compiles OK
- [x] `cargo clippy -p iscc-py -- -D warnings` — clean
- [x] `uv run pytest tests/ -x` passes — 204 tests (25 smoke + 179 conformance/others)
- [x] `uv run ruff check` — clean
- [x] Import verification: `from iscc_lib import gen_sum_code_v0, SumCodeResult` succeeds
- [x] `__all__` verification: both `gen_sum_code_v0` and `SumCodeResult` present (48 entries total)
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** Codex raised theoretical concerns about `os.PathLike` potentially returning bytes
(suggesting `os.PathLike[str]` would be more precise). This is a minor typing nit — `pathlib.Path`
(the standard path type) is `PathLike[str]`, and the `os.fspath()` call handles conversion
correctly. The existing pattern is consistent with common Python practice. No actionable bugs found.

**Next:** Propagate `gen_sum_code_v0` to Node.js bindings (`crates/iscc-napi/`). Follow the existing
pattern of `gen_instance_code_v0` — add napi export `genSumCodeV0(path: string)` returning an object
with `iscc`, `datahash`, `filesize` keys. Add mocha tests. Issue #15 binding tasks track the full
propagation across all 5 remaining bindings (Node.js, WASM, C FFI, Java, Go).

**Notes:** Issue #15 is progressively being resolved — Rust core tasks 1-4/7 are complete, Python
binding (task 5) is now done. 5 more binding propagations remain. The `__all__` count is now 48 (32
API + 11 result types + `__version__` + MT, ST, VS, core_opts). State.md still shows Python bindings
as "partially met" — will be updated by the next update-state agent.
