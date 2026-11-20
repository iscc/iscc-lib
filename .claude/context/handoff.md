## 2026-11-20 — Review of: Tier 1 doc-parity check derived from the `iscc-lib` crate root

**Verdict:** PASS_WITH_NOTES

**Summary:** The gate (`scripts/check_tier1_docs.py`, prek hook, 15 pytest tests) works and proves
itself on real drift: with the previous doc versions restored in a throwaway repo it exits 1 and
names each gapped doc. The doc additions match the binding sources, except one WASM line that review
corrected. The `[human]` issue stays open with a smaller remainder, and two doc/binding defects
found on the way are filed.

**Verification:**

- [x] `uv run scripts/check_tier1_docs.py` exits 0 — `OK: 16 full and 2 exact …`, 0.6 s
- [x] `uv run pytest tests/test_check_tier1_docs.py` passes — 15 passed, with the 4 fire tests
    next.md asked for (each asserts the complete error list for one injected defect)
- [x] `grep -c 'tier1:begin'` — 1 in `rust-core.md`, 1 in `crates/iscc-lib/CLAUDE.md`
- [x] Hook `check-tier1-docs` runs the script and its `files:` pattern matches `lib.rs` — staged
    probe: `Passed` for `lib.rs` and for a doc, `Skipped` for `Cargo.toml`
- [x] `uv run zensical build` exits 0 ("No issues found"); `mise run check` exits 0
- [x] Review probe, previous docs + new gate: exit 1, the 8 docs from next.md plus
    `crates/iscc-ffi/CLAUDE.md` and `packages/go/CLAUDE.md`
- [x] Review probe, added doc claims vs source: Java `SumCodeResult` / `genSumCodeV0`, `.pyi`
    signatures, `core_opts.meta_trim_meta`, napi `index.d.ts`, `iscc.h`, Go constants all match
- [ ] Review probe, WASM return types (run in Node): `alg_cdc_chunks` returns `number[][]`, the new
    how-to line said "array of Uint8Arrays" — corrected in this review

**Issues found:**

- **False WASM doc line (fixed here).** `docs/howto/wasm.md` now states the measured behaviour. The
    cause is in the binding: its doc comment promises `Uint8Array` chunks. Filed `normal` ("WASM
    `alg_cdc_chunks` returns plain arrays…")
- **The gate fails open on crate-root items it does not parse** (`pub use a::b as c;`, nested
    groups, `pub struct`, `pub static`, `pub unsafe fn`): it still prints `OK`. Written into the
    `[human]` issue as part of its remainder
- **Docs below the discovery threshold are unchecked**: `docs/howto/rust.md` (26 of 35),
    `docs/howto/c-cpp.md` (18), `docs/rust-api.md` (17, and it still says the gen functions return
    `IsccResult<String>`), `docs/architecture.md` → "Tier 1". Also in the `[human]` issue
- **`docs/howto/java.md` `genSumCodeV0` example does not compile** (3 args, accessor calls on
    fields). Pre-existing, reported by advance. Filed `normal`
- Fixed here (minor): `tests/test_check_tier1_docs.py` wrote `lib.rs` without `encoding="utf-8"`
    (failed under `LC_ALL=C`), and a broken line wrap in the script docstring
- Not filed (nit): the script's module docstring describes history ("were written by hand and
    drifted") instead of the current state

**Codex review:** Two findings, both valid and both acted on. (1) The fixture write without an
explicit encoding fails on non-UTF-8 locales: fixed, the suite now passes under
`LC_ALL=C PYTHONUTF8=0`. (2) The WASM `alg_cdc_chunks` return type in the how-to was wrong:
confirmed by running the WASM build, doc corrected, binding defect filed.

**Next:** Three `normal` issues are open. Suggested order:

1. Close the gate's two holes from the `[human]` issue in one step: make the `lib.rs` parser fail
    closed (every column-0 `pub` item is derived or explicitly excluded, with a fire test), and
    register the four sub-threshold docs as `FULL` (after completing them) or `EXEMPT` with a
    reason. The Java how-to example is a three-line doc fix that fits the same step.
    `docs/rust-api.md` is the largest piece (half the symbols missing, stale return types), so it
    may need its own step
2. WASM `alg_cdc_chunks`: return `Uint8Array` chunks, add a wasm test on the element type, and
    update `docs/howto/wasm.md` → "Algorithm primitives" (`wasm-pack test --node` runs the suite)
3. Part 2 of the `[human]` issue: the sweep of unchecked counts, file lists and line numbers in the
    specs and per-crate `CLAUDE.md` files

**Notes:**

- The gate checks names only. A signature, return type or example next to a name still needs a run
    or a source read. This step's one false line came from trusting a doc comment
- The local WASM `pkg/` is a web build but runs in Node through `initSync({ module: bytes })`, so
    WASM doc claims can be checked locally
- `crates/iscc-rb/README.md`, `packages/cpp/README.md` and `packages/dotnet/README.md` name 29
    symbols, one below discovery. The next mention forces a registry entry; `EXEMPT` with the
    target.md reason is the expected answer (decisions.md 2026-11-20, Tier 1 doc gate)
- learnings.md is at its 200-line budget: two settled entries were rotated to the archive to make
    room. The next review has to rotate again before adding
- No core or binding code changed: CRAP, iai and semver gates are untouched
