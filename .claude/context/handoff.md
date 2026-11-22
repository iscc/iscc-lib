## 2026-11-22 — Review of: Specs and API docs state shipped facts and the shipped error rules

**Verdict:** PASS_WITH_NOTES

**Summary:** The three specs now state what ships, the short-digest rule of `encode_component` is
documented on the seven surfaces that lacked it, `docs/c-ffi-api.md` says a failed first
`_finalize()` consumes, and the README list names Swift. The diff is comments, docs, specs and
regenerated outputs only; no code line changed. Every documented claim was checked against the code,
and the `[audit]` issue is closed. The reviewer corrected one sentence the step added to
`specs/documentation.md`, because the tree contradicted it.

**Verification:**

- [x] The three greps (`#16`, `includes .swift. checkbox`,
    `Python, Rust, Java, Node.js, WASM, C/C++`) print nothing; `check_spec_paths.py` exits 0 ("15
    specs")
- [x] The `encode_component` doc block in each of the 7 files states the short-digest rejection
    (`bit_length / 8` or `bitLength / 8`, one hit per file); `docs/c-ffi-api.md`'s streaming warning
    says a failed first `_finalize()` consumes; `README.md`'s "What is iscc-lib" list names Swift
- [x] Generated outputs are fresh: after a forced `iscc-ffi` rebuild and the CI cbindgen command,
    `git diff --exit-code` on `NativeMethods.g.cs` and `iscc.h` exits 0; the UniFFI freshness step
    body from `ci.yml`, run with `bash -e`, exits 0
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean;
    `cargo test -p iscc-uniffi -p iscc-ffi`: 121 + 31 passed, 0 failed
- [x] `uv run zensical build` exits 0 ("No issues found"); `mise run check` exits 0, tree clean

Beyond next.md: the short-digest rule is pinned on all 12 surfaces by the `encode_component` rows in
`data-edge.json`; `iscc_*_hasher_finalize` takes the state before the core call (read);
`cargo tree -p iscc-lib --no-default-features` shows no Unicode crate and no
`serde_json_canonicalizer`, and still shows `serde_json`, as `c-ffi-dx.md` §5 now says.

**Issues found:**

- Fixed by the reviewer (`specs/documentation.md`): the step wrote "other pages follow the same
    order" for tabs. `docs/tutorials/getting-started.md` uses Python, Rust, Node.js, Java, Go, WASM,
    while `docs/index.md` puts WASM and Go before Java. The sentence now says the landing page has
    one tab per entry of the README's Installation section and that `docs/index.md` sets the order
    ("C / C++" has one tab, "C++", so "every language" was not exact either)
- Filed `low`, untriaged: `encode_component` docs on five surfaces (C FFI, Java, Python stub, Ruby,
    Go) give the `bit_length` range without the ISCC-ID case (64 to 96 in steps of 8; measured on
    the Python binding). Advance reported it
- Filed `low`, untriaged: `specs/documentation.md` says every code example is tabbed and that all
    how-to guides use tabs; the eleven how-to pages have none. Pre-existing
- Not filed (nits): the UniFFI comment says "Returns `IsccUniError`", and the generated Kotlin doc
    carries that name although Kotlin throws `IsccUniException`; the napi comment writes
    `bit_length` where the generated `.d.ts` names the parameter `bitLength`

**Codex review:** No actionable findings. It confirmed that the documented error behaviour matches
the runtime checks and that the regenerated bindings match their sources.

**Next:** No `critical` or `normal` issue is left in issues.md: all 17 entries are `low`, 16 of them
untriaged. state.md named the closed audit issue as the last blocker of v0.8.0, so if update-state
finds no unmet gap, the right next.md is `Step: NONE`, and the release is Titusz's call
(`/release`). Do not scope a `low` entry.

**Notes:**

- CI on this push runs the `c-ffi` and UniFFI freshness steps on the regenerated files; both passed
    locally with the step bodies taken from `ci.yml`
- Gate integrity: no suppression, skip or threshold change in the unpushed commits. No API break:
    only `///` and `<exception>` lines changed in code files, and the UniFFI checksum of
    `encode_component` moved (11010 → 629) because the doc comment is part of it
- learnings.md is at its 200-line budget and decisions.md at 396 of 400: a new entry in either needs
    an equal cut
