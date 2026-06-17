## 2026-06-16 — Add streaming `SumHasher` to the WASM bindings

**Done:** Added a `#[wasm_bindgen]` `SumHasher` streaming class to `crates/iscc-wasm/src/lib.rs`,
wrapping the shared core `iscc_lib::streaming::SumHasher` with the established `Option<inner>`
finalize-once pattern (mirroring the WASM `DataHasher`/`InstanceHasher`). Its
`finalize(bits?, wide?, add_units?)` returns the existing `WasmSumCodeResult`. This closes the only
remaining half of issue #37 (WASM bindings) — the core struct (iter 88) and Python wrapper (iter 89)
already existed.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: added `SumHasher` struct + `Default` impl + `#[wasm_bindgen] impl`
    (`new()`, `update()`, `finalize()`) after `InstanceHasher`. Pure translation layer — no
    algorithm logic; casts `filesize: u64 → f64`, maps `units: Option<Vec<String>>` directly, maps
    all errors via `JsError::new(&e.to_string())`, never panics across the boundary.
- `crates/iscc-wasm/tests/unit.rs`: added 8 `#[wasm_bindgen_test]` functions in a new `── SumHasher`
    section (matches gen_sum_code_v0, multi-update invariance vs one-shot, empty input, result
    shape, units enabled with content check, units disabled, wide-mode 128-bit differs from narrow,
    finalize-once: second finalize + post-finalize update both error). Real byte data, no mocks.
- `docs/howto/wasm.md`: added a `### SumHasher` subsection under `## Streaming` showing
    `import { SumHasher }`, chunked `update()` from a `ReadableStream`, and
    `finalize(undefined, undefined, true)` reading `result.iscc`/`result.datahash`/`result.units`.
    Updated the trailing "Both hashers" sentence to "All three hashers".
- `crates/iscc-wasm/CLAUDE.md`: bumped "2 streaming types" → "3 streaming types" in both the summary
    line and the enumeration, with a note that `SumHasher` wraps `iscc_lib::streaming::SumHasher`
    (full path, not a crate-root Tier 1 re-export) and returns the shared `WasmSumCodeResult`.

**Verification:**

- `wasm-pack test --node crates/iscc-wasm --features conformance` — **78 passed, 0 failed** (70
    prior
    - 8 new `test_sum_hasher_*`). All 8 new tests confirmed present in output.
- `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` — clean.
- `cargo clippy -p iscc-wasm --all-targets -- -D warnings` (host) — clean.
- `cargo fmt -p iscc-wasm --check` — clean (fmt auto-fix applied to one assert! in unit.rs).
- `mise run check` (15 pre-commit hooks) — all Passed.
- `grep -q "pub struct SumHasher" crates/iscc-wasm/src/lib.rs` — exit 0.
- `grep -q "SumHasher" docs/howto/wasm.md` — exit 0.
- README.md / `.claude/context/specs/rust-core.md` / `target.md` "32 Tier 1 symbols" counts
    untouched (those files were not modified; SumHasher is not promoted to Tier 1).

**Next:** Issue #37 is now fully closed across all bindings (core + Python + WASM). The reviewer
should mark/close the #37 entry in issues.md. Highest-impact remaining backlog items: npm
`optionalDependencies` fix (#38), PyO3 0.23→0.29 security migration, GIL release (#39 — when it
lands it should also cover the new `PySumHasher.update()`).

**Notes:**

- `SumHasher` is intentionally NOT a crate-root Tier 1 re-export — it is a Python/WASM streaming
    convenience reached via the full path `iscc_lib::streaming::SumHasher`. The documented "32 Tier
    1 symbols / 2 streaming types" counts in README/rust-core/target were deliberately left
    unchanged per next.md scope. Only the WASM crate's own CLAUDE.md enumeration was bumped to "3".
- The `WasmSumCodeResult` struct was reused (no new result struct introduced), so the crate's "2
    result structs" count in CLAUDE.md is unchanged.
- Test-target filtering via `wasm-pack test ... -- --test unit` does NOT work
    (wasm-bindgen-test-runner rejects `--test`; only a positional FILTER is accepted). Ran the full
    suite instead — the documented CLAUDE.md command `-- --test conformance` appears stale on
    wasm-pack 0.13.1. Left unchanged (out of scope), but worth a doc fix later.
