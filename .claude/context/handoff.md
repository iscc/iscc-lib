## 2026-11-19 — Review of: npm packages ship a license and a plain-text description

**Verdict:** PASS

**Summary:** `crates/iscc-wasm/Cargo.toml` now inherits `license` (Apache-2.0) and sets a
browser-specific `description`, so wasm-pack writes both into `pkg/package.json`.
`crates/iscc-napi/package.json` has its plain-text description. It is a two-file manifest change,
within scope. The `publish = false` comment moved to its own line because taplo would otherwise push
it out to column 119; the value is unchanged (`cargo metadata` `publish: []`). The `[human]` npm
licence issue is resolved and deleted.

**Verification:**

- [x] `wasm-pack build --target web --dev crates/iscc-wasm` exits 0. I ran a fresh build with
    wasm-pack 0.13.1, version 0.7.0. It printed only the informational "no LICENSE file" note
- [x] `node -e` on the fresh `pkg/package.json`: `license` is `"Apache-2.0"` and `description` is
    the exact WASM string (`assert.strictEqual`)
- [x] `node -e` on `crates/iscc-napi/package.json`: `description` is the exact napi string and
    `license` is still `"Apache-2.0"`
- [x] `cargo metadata --no-deps --format-version 1` parses; `iscc-wasm` has license `Apache-2.0` and
    the description
- [x] `mise run check` exits 0, and every hook passed
- [x] Extra checks:
    - `mise run version:check` exits 0 with 21 `OK:` lines (the napi `package.json` is one of the
        targets)
    - `cargo deny check licenses` reports `licenses ok`
    - the wasm-pack installer that release.yml downloads (`init.sh`, fetched today) pins `UPDATE_ROOT`
        to v0.13.1, the same version verified locally
- I confirmed by reading release.yml's patch step (and Codex agrees) that it changes only `name`,
    `repository` and `version`, so `license` and `description` survive. I did not re-run it; the
    advance agent ran it on a copy

**Issues found:**

- Spec drift, not caused by this step: `specs/wasm-bindings.md` lists a committed
    `crates/iscc-wasm/package.json` (in the crate tree and the Version Sync table). That file does
    not exist, and `version_sync.py` has no WASM target. Filed as `normal` `[review]`, no escalation
    needed (it is a file-list fact)
- Nit: the npm tarball still ships no LICENSE file. next.md excluded copying one, and the `license`
    field is what scanners read
- Nit: wasm-pack logs "missing `repository`". release.yml adds it at release time, so it is harmless

**Codex review:** No actionable findings. Codex confirmed that Cargo resolves the inherited
Apache-2.0 licence, that both descriptions are correct, and that the release patch step keeps both
fields.

**Next:** Fix the Ruby negative-Bignum truncation (`normal`; `RBignum::to_i32` in
`crates/iscc-rb/src/lib.rs`, test in `test_array_args.rb`). It was the runner-up this time. Other
small `normal` candidates:

- the Python `encode_seq` docstrings (`[human]`)
- the wasm-bindings spec drift filed this iteration
- the Dependabot refresh of `soupsieve` and `setuptools`

**Notes:**

- `license.workspace` resolution is proven only for wasm-pack 0.13.1. release.yml installs wasm-pack
    with a remote `init.sh`, so its version is controlled outside this repo. If a later release
    shows an empty `license`, switch to the literal `license = "Apache-2.0"`
- The fix reaches npm with the next release; the published 0.7.0 packages are unchanged
- `learnings.md` is at 200/200 lines and `decisions.md` at 400/400, so every addition has to be
    offset by a cut
