# Next Work Package

## Step: `@iscc/wasm` docs show a Node.js setup that runs on the published web-target package

## Goal

Fix the `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot run":
the how-to and the npm README show a Node path that works on the `--target web` package that
`release.yml` publishes, and both pages state what `gen_sum_code_v0` returns.

## Alternatives Considered

- **Chosen:** WASM Node.js docs — a user-facing gap on the npm registry page, 2 files, no behaviour
    decision, and the handoff's pick.
- **Rejected:** vcpkg portfile hashes — also user-facing, but it needs a `release.yml` step plus a
    gate. It is a good next pick. `alg_minhash_256([])` and WASM `finalize()` need a behaviour
    decision first.

## Scope

- **Create**: `tests/test_wasm_docs.py`
- **Modify**: `docs/howto/wasm.md`, `crates/iscc-wasm/README.md`, `.github/workflows/release.yml`
    (only the `test-wasm` smoke script)
- **Reference**: `.github/workflows/release.yml` (`build-wasm` around :427, `test-wasm`),
    `crates/iscc-wasm/src/lib.rs` (`WasmSumCodeResult`, `SumHasher`, crate-root exports),
    `tests/test_ruby_gem_docs.py` (pattern for a docs-claim test)

## Not In Scope

- Changing the build target, adding a second `--target nodejs` npm package, or adding an `exports`
    map to the package.json in `build-wasm`.
- The `low` WASM issues (byte conversion, serde getter unwinding) and the `normal` hasher
    `finalize()` rule: leave the `wasm.md` streaming and error-handling text as it is.
- `crates/iscc-wasm/CLAUDE.md`: its `--target nodejs` lines describe local dev builds and stay.

## Implementation Notes

- Facts: `release.yml` builds with `wasm-pack build --target web --release`. The published 0.7.0
    `package.json` (`npm view @iscc/wasm@0.7.0 --json`) has `main: iscc_wasm.js`, `type: module`,
    and no `exports`, so `import … from "@iscc/wasm"` resolves in Node, and so does the subpath
    `@iscc/wasm/iscc_wasm_bg.wasm`. Called with no argument, `init()` builds
    `new URL('iscc_wasm_bg.wasm', import.meta.url)` and `fetch`es it, and Node's `fetch` rejects
    `file:` URLs.
- wasm-bindgen (0.2.126 in Cargo.lock) takes one options object: `init({ module_or_path })` and
    `initSync({ module })`. A positional argument still works but logs
    `using deprecated parameters for the initialization function; pass a single object instead`. The
    `test-wasm` smoke script calls `init(wasmBytes)` positionally: switch it to the object form.
    Source: https://github.com/wasm-bindgen/wasm-bindgen/blob/main/crates/cli-support/src/js/mod.rs
- The Node example to probe and then document (hypothesis, adjust to what the probe shows):
    `import { readFileSync } from "node:fs";` `import { createRequire } from "node:module";`, then
    `initSync({ module: readFileSync(createRequire(import.meta.url).resolve("@iscc/wasm/iscc_wasm_bg.wasm")) })`.
    An `import.meta.resolve` variant is fine if it runs on Node 22.
- **Probe (required, evidence in the handoff):** build as `build-wasm` does (`RUSTFLAGS` simd128,
    `--target web --release --features conformance`). Set `name` to `@iscc/wasm` in a COPY of `pkg/`
    and `npm pack` it. `npm install` the tarball into a fresh `/tmp` project, and run the Node
    snippets from both pages verbatim as `.mjs` files. They must print a code and log no deprecation
    warning. Clean up `/tmp` and leave `crates/iscc-wasm/pkg/` untracked as it is.
- `wasm.md` → Setup: the published package is one web-target build. Bundler: keep `await init()` and
    drop "Most bundlers handle WASM initialization automatically". Browser: it can serve the
    installed package, so drop "build with `--target web`". Node: the probed snippet. Keep the
    "Prefer @iscc/lib" tip.
- `wasm.md:82` and README "All code generators return ISCC strings directly": 9 return a string,
    `gen_sum_code_v0` returns a `WasmSumCodeResult` (`iscc`, `datahash`, `filesize`, plus `units`
    when asked; read the fields from `lib.rs`). README Streaming line: add `SumHasher`. While in
    that list, fill the Codec/Encoding lines from the crate-root exports in `lib.rs`.
- Replace README "Both browser and Node.js targets are supported." with a Node quick-start line or
    block (the same snippet). Run `git grep -n -- '--target nodejs'` and
    `git grep -n 'Both browser and Node'` repo-wide: no user-facing page (`docs/`, READMEs) may keep
    the claim. `notes/` and `crates/iscc-wasm/CLAUDE.md` are exempt.
- `tests/test_wasm_docs.py` (plain pytest functions): neither page contains `--target nodejs` or
    "Both browser and Node.js targets"; the `wasm.md` Node section and the README use `initSync(` or
    `init({`; both pages name `WasmSumCodeResult`; the `test-wasm` script in `release.yml` uses no
    positional `init(wasmBytes)`. Read `release.yml` with PyYAML, as other tests do.
- Docs/workflow-only step: no Rust change, so no CRAP, iai or semver effect.

## Verification

- The handoff carries the probe: `npm pack` of the web release build, installed in a scratch
    project, runs both pages' Node snippets and prints an ISCC code with no deprecation warning
- `uv run pytest tests/test_wasm_docs.py tests/test_check_docs_nav.py tests/test_check_tier1_docs.py`
    passes
- `git grep -nE -- '--target nodejs|Both browser and Node' -- docs README.md crates/iscc-wasm/README.md`
    prints nothing
- `grep -c WasmSumCodeResult crates/iscc-wasm/README.md` is at least 1
- `uv run zensical build` exits 0
- `mise run check` exits 0 (includes the release-workflow static check)

## Done When

All six verification criteria pass, and the Node snippet on both pages has been run against a packed
`--target web` release build.
