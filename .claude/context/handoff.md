## 2026-11-22 — Review of: `@iscc/wasm` docs show a Node.js setup that runs on the published web-target package

**Verdict:** PASS_WITH_NOTES

**Summary:** `docs/howto/wasm.md` and `crates/iscc-wasm/README.md` now show a Node.js setup that
works on the package npm serves, and both state that `gen_sum_code_v0` returns a
`WasmSumCodeResult`. I ran both pages' Node blocks verbatim against the real `@iscc/wasm@0.7.0` from
the registry and against a packed HEAD build: both print a code and write nothing to stderr. The
`[audit]` issue is resolved and deleted. One leftover fixed here, one sibling page filed.

**Verification:**

- [x] The handoff carries the probe — re-run: HEAD `--target web --release` build, `name` set in a
    copy, `npm pack`, tarball installed in a scratch project (Node v22.23.5). `howto_node.mjs` →
    `ISCC:EAASKDNZNYGUUF5A`, `readme_node.mjs` → `Meta-Code: ISCC:AAAT4EBWK27737D2`, exit 0, 0
    stderr bytes each. The same two files against `npm install @iscc/wasm@0.7.0`: same output
- [x] `uv run pytest tests/test_wasm_docs.py tests/test_check_docs_nav.py tests/test_check_tier1_docs.py`
    — 55 passed (56 with the test added in this review)
- [x] `git grep -nE -- '--target nodejs|Both browser and Node' -- docs README.md crates/iscc-wasm/README.md`
    — prints nothing
- [x] `grep -c WasmSumCodeResult crates/iscc-wasm/README.md` — 1
- [x] `uv run zensical build` — exit 0
- [x] `mise run check` — exit 0, before and after the step-9 fix

Extra checks (3): the registry package probe above; the `test-wasm` step body extracted with PyYAML
and run as `bash --noprofile --norc -eo pipefail` next to the packed `pkg/` (exit 0, silent); the
new test module against the previous versions of the three files (5 of 7 fail, as advance reported).
Bare `await init()` in Node fails with `TypeError: fetch failed` on the registry package, which
confirms the defect the docs now explain. `gen_sum_code_v0(…, true)` returns the four documented
fields; without `add_units`, `units` is `undefined`.

**Issues found:**

- Fixed in this review: `scripts/test_install.py` (the registry install check) still called the
    deprecated positional `init(wasmBytes)`, the sibling of the `release.yml` smoke script. Now
    `init({ module_or_path: wasmBytes })`, guarded by `test_install_script_uses_options_object`.
    `uv run scripts/test_install.py --npm --version 0.7.0` passes for both npm packages
- Filed `normal`: "`@iscc/lib` npm README says every code generator returns a string". The napi
    `gen_sum_code_v0` returns a `NapiSumCodeResult`. It is the sentence this step removed from the
    WASM README, left on the sibling npm page
- Not filed: the WASM blocks in root `README.md`, `docs/index.md` and
    `docs/tutorials/getting-started.md` show a bare `await init()`. That is correct for browsers and
    bundlers, and none of the three claims Node. A one-line pointer to the how-to's Node section
    would help a reader who runs them in Node
- Not an error: the README lists `encode_seq` and `decode_seq`, which 0.7.0 on npm does not export.
    They are unreleased additions and ship with the README in the next release
- Deviation accepted: `crates/iscc-wasm/README.md` is registered as `FULL` in
    `scripts/check_tier1_docs.py`. Filling the Codec and Encoding lines reached the gate's 30-symbol
    discovery threshold. `FULL` keeps the npm page under the parity gate

**Codex review:** No actionable findings. It ran the targeted tests, both Node examples and the
smoke script against a fresh packed web build, all green.

**Next:** The vcpkg portfile hashes (`normal` `[audit]`: a `release.yml` step plus a gate), as
define-next already planned. The napi README sentence is a small docs step (1-2 files) and can go
before or after it. `alg_minhash_256([])` and the WASM hasher `finalize()` rule each need a recorded
decision first.

**Notes:**

- The published `@iscc/wasm` is one `--target web` build with `main: iscc_wasm.js`, `type: module`
    and no `exports` map, so `require.resolve("@iscc/wasm/iscc_wasm_bg.wasm")` works. An `exports`
    map added to `build-wasm` later would break the documented Node snippet unless it exports the
    `.wasm` subpath
- `tests/test_wasm_docs.py` checks text only. Whether the snippets run is proved by the probe, so a
    step that edits either Node block must repeat it (`npm install @iscc/wasm@<version>` in an empty
    `/tmp` project, then the blocks as `.mjs`)
- No Rust change: no CRAP, iai, semver or API impact
