## 2026-11-20 — Tier 1 doc-parity check derived from the `iscc-lib` crate root

**Done:** Added `scripts/check_tier1_docs.py`. It derives the Tier 1 set (35 symbols) from the
`lib.rs` column-0 `pub fn`/`pub const`/named `pub use` items, then checks a FULL registry (16 docs,
any binding casing), an EXACT registry (`<!-- tier1:begin/end -->` blocks in `rust-core.md` and
`crates/iscc-lib/CLAUDE.md`, set-equal and no repeats), an EXEMPT registry (`target.md`) and
fail-closed discovery (unregistered tracked `*.md` with ≥ 30 symbols). Added a prek hook and a
pytest suite, and fixed every gap the gate found.

**Files changed:**

- `scripts/check_tier1_docs.py`: the gate (Python 3.10-safe, `git ls-files`, floor on parse)
- `tests/test_check_tier1_docs.py`: 15 tests. Anchor on the real tree, synthetic `derive_symbols`,
    casing and no-substring matcher units, and fire tests that inject one defect per tree copy:
    `pub fn` added to `lib.rs`, symbol deleted from a FULL doc, extra name in an EXACT block,
    repeated name, missing marker, unregistered doc at the threshold (and one below), registered
    path untracked, collapsed parse, and `main()` exiting 1
- `.pre-commit-config.yaml`: `check-tier1-docs` hook (shape of `check-spec-paths`)
- `docs/development.md`: gate added to the pre-commit list
- `.claude/context/specs/rust-core.md`, `crates/iscc-lib/CLAUDE.md`: tier1 markers only
- `docs/java-api.md`: `META_TRIM_META` row, `SumCodeResult` class, `genSumCodeV0` section
- `docs/howto/{python,nodejs,wasm}.md`: `META_TRIM_META` (+ `core_opts.meta_trim_meta` for Py), and
    an "Algorithm primitives" section (modelled on kotlin.md, signatures taken from `.pyi`, napi
    `index.d.ts` and the WASM `lib.rs`)
- `docs/howto/{go,java}.md`, `packages/go/README.md`, `docs/c-ffi-api.md`: `META_TRIM_META` /
    `MetaTrimMeta` / `iscc_meta_trim_meta`
- `crates/iscc-ffi/CLAUDE.md`: `iscc_gen_iscc_id_v1` line. `packages/go/CLAUDE.md`: Constants
    subsection, plus `EncodeSeq`/`DecodeSeq` in the codec list. Both registered FULL, as is
    `crates/iscc-wasm/CLAUDE.md`, which was already complete (35) but unregistered

**Verification:**

- `uv run scripts/check_tier1_docs.py` → `OK: 16 full and 2 exact …`, exit 0. It also runs on
    `uv run --python 3.10 --no-project`
- `uv run pytest tests/test_check_tier1_docs.py` → 15 passed
- Real regression: HEAD's docs (git archive, with the new script and markers) → exit 1, listing
    exactly the 8 gapped docs from next.md
- `grep -c 'tier1:begin'` → 1 in each of the two EXACT files
- `mise run check` → exit 0 (the "Tier 1 doc parity" hook passed). That run skipped the two new
    files while they were untracked. At commit time ruff flagged PIE810/ISC004; both are fixed, and
    the commit hooks plus `ruff check` and `ty check` passed on them. Pre-push stage on the new
    files → exit 0, full pytest included
- `uv run zensical build` → exit 0, "No issues found". `tests/test_c_ffi_docs.py` and
    `tests/test_check_docs_nav.py` → 18 passed
- Doc examples: the Python `sliding_window`/`META_TRIM_META` output was run, and so was napi
    `sliding_window`. The local WASM `pkg` is a web build, so the WASM section comes from source

**Next:** Part 2 of the issue: sweep the unchecked counts, file lists and line numbers. Below the
threshold and unregistered, these docs should be decided in that sweep: `docs/howto/rust.md` (26/35,
missing the constants, `encode_base64`, `encode_component`, `iscc_decompose`, `soft_hash_video_v0`),
`docs/howto/c-cpp.md` (18), `docs/rust-api.md` (17) and `docs/architecture.md` → Tier 1.

**Notes:**

- `docs/howto/java.md:204` is wrong and out of scope:
    `IsccLib.genSumCodeV0("example.bin", 64,   false)` passes 3 args (the method takes 4), and
    `result.iscc()` etc. are method calls on what are `public final` fields. The gate cannot see
    this. File it or fix it in the sweep
- `crates/iscc-rb/README.md`, `packages/cpp/README.md` and `packages/dotnet/README.md` score 29, one
    symbol below discovery. One more mention will force a registry decision (FULL or EXEMPT)
- Matcher: a type symbol (PascalCase) matches any token that starts or ends with its key, so
    `DataHasherFoo` counts. This is intentional for `NewDataHasher` and `iscc_data_hasher_new`
- Deviations from next.md: `pub const fn` parses as its name, not as `fn` (none exists today). EXACT
    blocks also reject repeated names, which plain set equality would pass. Discovery skips
    `notes/`, `.claude/agent-memory/` and the agent-written `.claude/context/*.md` logs (state,
    next, handoff, issues, learnings\*, decisions\*, proposals)
- No core or binding code changed, so the CRAP, iai and semver gates are untouched
