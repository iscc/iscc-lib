## 2026-11-21 — Review of: Docs accuracy batch — architecture.md covers every surface, no "10 standard functions" or Sum-Code-as-string claims

**Verdict:** PASS

**Summary:** Docs-only step, six pages, exactly the scope next.md set. Every new row and sentence
was checked against the source it describes (binding code, `ci.yml`, `release.yml`, test loaders,
the four added registry links) and all of it holds. Two `normal` docs issues are closed and deleted.

**Verification:**

- [x] `git grep -nE '10 .gen_' -- docs/ecosystem.md docs/architecture.md` prints nothing — exit 1,
    no output
- [x] The `All 10 .*(return a .String|…)` grep over the four pages prints nothing, and each "Code
    generation" paragraph names its Sum-Code type — Node `NapiSumCodeResult` (`index.d.ts:193`),
    Java `SumCodeResult` on both pages (`IsccLib.java:161`), C `iscc_IsccSumCodeResult` +
    `iscc_free_sum_code_result` (`iscc.h:558/573`); `gen_iscc_id_v1` returns a string on Java and C
- [x] `architecture.md` "Crate Summary", "Per-Binding Adaptation" and "Cross-Language Test Matrix"
    each have a row for Ruby, C#/.NET, C++, Swift and Kotlin; no `iscc-ffi` row says `Source`; the
    WASM row names `wasm-pack test --node` (`ci.yml:124`)
- [x] `uv run pytest tests/test_c_ffi_docs.py` — 6 passed; `uv run scripts/check_tier1_docs.py` —
    exit 0
- [x] `uv run zensical build` — exit 0, "No issues found"
- [x] `mise run check` — exit 0

Beyond next.md: the facts behind the new rows (paths in `git ls-files`; archive contents in
`release.yml:733-760`; hasher class and method names in the Ruby, .NET, C++, Swift and Kotlin
sources; vector loaders in every test suite; `tests/test_vendored_fixtures.py` covering the four
vendored copies) all match. The four registry links added to `ecosystem.md` return HTTP 200, and
both Maven artifacts (`io.iscc:iscc-lib`, `io.iscc:iscc-lib-kotlin`) exist on Maven Central.

**Issues found:**

- Fixed in this review (two table cells in `architecture.md` → "Per-Binding Adaptation", both older
    than this step): the C FFI row named `ctx_new()` / `ctx_update()` …, functions that do not exist
    (real: `iscc_data_hasher_new()` and siblings); the Java row named `DataHasher`/`InstanceHasher`
    classes, while Java has static `dataHasher*`/`instanceHasher*` methods on `IsccLib` only
- Not filed (nits): `ecosystem.md` calls Go a "binding" in the iscc-lib intro (it is a pure-Go
    port); the "Crate Summary" column is headed "Crate" but holds packages too; `IsccLibTest.java:2`
    and root `CLAUDE.md` → "Conformance" say all 10 `gen_*_v0` functions have `data.json` vectors (9
    do)
- No new issue filed

**Codex review:** no actionable findings ("Updated claims match the binding signatures and
build/test configuration").

**Next:** the `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot
run" (`normal`, 2 non-test files: `docs/howto/wasm.md`, `crates/iscc-wasm/README.md`). It needs a
probe against a packed `wasm-pack build --target web` release build before any sentence is written,
and it carries the last "All 10 … return the ISCC code as a string" sentence (`howto/wasm.md:82`).
The other two `normal` entries: the one-shot/streaming dedup (core hot path, iai baseline in play)
and `alg_minhash_256([])` (needs a behaviour decision first, then a `data-edge.json` vector).

**Notes:**

- Closed and deleted: "`docs/architecture.md` sections below the graph cover half the surfaces" and
    "Six docs pages count 10 standard functions or give Sum-Code a string result". What is left of
    the second one (`howto/wasm.md:82`) is already the third bullet of the WASM `[audit]` issue
- `git grep -nE '10 .gen_' -- docs` now shows `howto/{python,ruby,rust}.md` and `ruby-api.md`
    (library-wide counts with typed results, correct) plus `howto/wasm.md:82` (wrong, tracked)
- Remaining `normal` issues: 3, all `[audit]`. `low`: 9
- learnings.md stays at 200 lines; nothing added (routine pass)
