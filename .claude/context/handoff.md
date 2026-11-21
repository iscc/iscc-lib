## 2026-11-21 — `@iscc/wasm` docs show a Node.js setup that runs on the published web-target package

**Done:** `docs/howto/wasm.md` and `crates/iscc-wasm/README.md` now show a Node.js setup that passes
the WASM bytes to `initSync({ module })`. It was run verbatim against a packed `--target web`
release build. Both pages state that `gen_sum_code_v0` returns a `WasmSumCodeResult`. The
`test-wasm` smoke script uses `init({ module_or_path })`, and `tests/test_wasm_docs.py` guards these
claims.

**Files changed:**

- `docs/howto/wasm.md`: the Setup intro says the package is one web build. Bundler: dropped "Most
    bundlers handle … automatically". Browser: serves `node_modules/@iscc/wasm/`, no `--target web`
    build. Node: the probed `initSync` snippet. The code-generation intro says 9 return strings and
    `gen_sum_code_v0` returns `WasmSumCodeResult` (fields from `lib.rs`). Tip kept
- `crates/iscc-wasm/README.md`: "Both browser and Node.js targets" replaced by a Node block (the
    same snippet with `gen_meta_code_v0`). Return-type sentence fixed. Encoding/Codec lines filled
    from crate-root exports, `SumHasher` added to Streaming, and a Constants line added (see Notes)
- `.github/workflows/release.yml`: `test-wasm` smoke now calls
    `await init({ module_or_path: wasmBytes })`
- `scripts/check_tier1_docs.py`: `crates/iscc-wasm/README.md` registered in `FULL`
- `tests/test_wasm_docs.py` (new): 7 tests for the banned phrases, the options-object init in the
    how-to Node section and the README Node block, `WasmSumCodeResult` on both pages, no positional
    `init(x)` in either page's JS, `test-wasm` script read via PyYAML, and a regex self-test

**Verification:**

- Probe:
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release --out-dir $exp/pkg crates/iscc-wasm --features conformance`
    (exit 0, `--out-dir`, so the ignored `crates/iscc-wasm/pkg/` was not touched). Then
    `name=@iscc/wasm` set in that copy, `npm pack`, and `npm install` of the tarball into a fresh
    project (Node v22.23.5). Both pages' Node blocks were extracted by the test's own parser and run
    as `.mjs`, after mdformat's reflow:
    - `howto_node.mjs`: exit=0 stdout=`ISCC:EAASKDNZNYGUUF5A` stderr_bytes=0
    - `readme_node.mjs`: exit=0 stdout=`Meta-Code: ISCC:AAAT4EBWK27737D2` stderr_bytes=0
    - Controls: bare `await init()` → `TypeError: fetch failed` (`not implemented... yet...`), exit 1.
        Positional `init(bytes)` →
        `using deprecated parameters for the initialization function; pass a single object instead`.
        `init({ module_or_path: readFileSync(new URL(import.meta.resolve(…))) })` also works, silently
    - `gen_sum_code_v0(…, true)` → `WasmSumCodeResult ISCC:KUAB… 1e20ae17 11000 [GAA…, IAA…]`
- `test-wasm` step body extracted with PyYAML and run as `bash --noprofile --norc -eo pipefail` next
    to the packed `pkg/` → exit 0, empty stdout/stderr
- `uv run pytest tests/test_wasm_docs.py tests/test_check_docs_nav.py tests/test_check_tier1_docs.py`
    → 55 passed. The test module, run against the HEAD versions of the 3 files, fails 5 of 7 (only
    the regex self-test and no-positional-docs pass: the old docs had no positional init)
- `git grep -nE -- '--target nodejs|Both browser and Node' -- docs README.md crates/iscc-wasm/README.md`
    → no output (exit 1). `grep -c WasmSumCodeResult crates/iscc-wasm/README.md` → 1
- `uv run zensical build` → exit 0 (also re-run after the mdformat reflow)
- `mise run check` → exit 0 (the first run failed only on mdformat reflowing the JS blocks; tests
    and probe re-run on the reflowed text)
- Cleanup: `$exp` removed, `git worktree list` shows the checkout only, `crates/iscc-wasm/pkg/`
    still ignored. `/tmp/wasmprobe` and `/tmp/wasmprobe.mjs` were already there and not created here

**Next:** The vcpkg portfile hashes (`release.yml` step + gate), per the define-next alternatives.
Or the `alg_minhash_256([])` / WASM hasher `finalize()` decisions, once they are made.

**Notes:**

- Deviation from next.md, file budget 5 of 6: filling the README Codec/Encoding lines took it to 30
    Tier 1 symbols. That hits `check_tier1_docs.py`'s discovery threshold ("names 30 Tier 1 symbols
    but is not registered"). Only the 5 constants were missing, so I added a Constants line and
    registered the README as `FULL` instead of `EXEMPT`, which keeps the npm page gated
- `createRequire` (works since Node 12) was chosen over `import.meta.resolve` (Node 20.6+), since
    the README's engine floor is Node 16.4
- Out of scope, not edited: `README.md` (root, WASM block), `docs/index.md` (WASM tab) and
    `docs/tutorials/getting-started.md` show bare `await init()` with no Node claim. Run in Node,
    they fail the same way. A follow-up could add one Node pointer to the how-to
- `.claude/context/specs/wasm-bindings.md:90` keeps `--target nodejs  # For Node.js (testing)`, a
    local dev build, not a user-facing page
- No Rust change: no CRAP, iai, semver or API impact
