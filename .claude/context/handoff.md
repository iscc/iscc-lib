## 2026-11-22 — Review of: `@iscc/lib` npm README states the `gen_sum_code_v0` result type and names every Tier 1 symbol

**Verdict:** PASS

**Summary:** The npm README no longer says that every code generator returns a string. It states
that `gen_sum_code_v0` returns a `NapiSumCodeResult` object and names all 35 Tier 1 symbols the
addon exports. The page is registered in the Tier 1 doc gate (`FULL`), and a docs test ties the
sentence to the struct in `crates/iscc-napi/src/lib.rs`. Two non-test files changed, no Rust code.

**Verification:**

- [x] `uv run pytest tests/test_napi_docs.py tests/test_check_tier1_docs.py` passes — 40 passed
- [x] `uv run python scripts/check_tier1_docs.py` exits 0 and `crates/iscc-napi/README.md` is in
    `FULL` — "OK: 22 full and 2 exact Tier 1 docs match"
- [x] `git grep -n 'return ISCC strings directly' -- ':!.claude'` prints nothing — exit 1
- [x] `grep -n NapiSumCodeResult crates/iscc-napi/README.md` prints at least one line — line 53
- [x] `uv run zensical build` exits 0 and `mise run check` exits 0 — both clean, tree unchanged

Three checks beyond next.md's list:

- The README's claims against the source: the addon has exactly 35 `#[napi]` exports and each one is
    on the page; nine generators return `String`; the struct has the four fields the sentence names;
    the constants are real `const` exports. The published 0.7.0 `index.d.ts` (`npm pack`) declares
    `NapiSumCodeResult` and `export declare const` for the five constants.
- The new test, mutated in-process on `/tmp` copies: a renamed struct field fails two tests, an
    added field fails one, a field name dropped from the sentence fails one, and the banned phrase
    wrapped over a line break is caught.
- A repo-wide grep for the same claim in other words ("all generators return", "return ISCC
    string"): every hit names its exception or describes a single function. No sibling leftover.

`uv run ty check` and the ruff `S`/`C901` pre-push rules pass on the new test file.

**Issues found:**

- Closed and deleted: "`@iscc/lib` npm README says every code generator returns a string".
- Not filed (nit): the README says "`add_units` set to `true`", while the generated `index.d.ts`
    names that parameter `addUnits` (napi-rs camel-cases parameter names). The parameter is
    positional, so no caller is misled.
- No new issues.

**Codex review:** No actionable findings. Codex confirmed that the README matches the binding's
return type and exports and that the 40 targeted tests and the Tier 1 check pass.

**Next:** The `normal` issue "`publish-ffi` skips a half-uploaded FFI archive on a rerun". It needs
a design choice between two fixes: verify after the upload that every staged archive is on the
release in state `uploaded` with the right size, or delete `starter` assets before uploading. The
issue also asks for a wiring test and a correction of two paragraphs in `docs/development.md`. The
other open `normal` issues are the Rust tier docs (4-5 files), the duplicate `data.json` parsers
(audit-sized, Rust and Go can be separate steps) and the one-shot generator duplication (core, mind
the iai baseline). `alg_minhash_256([])` and the WASM `finalize()` rule each need a behaviour
decision from Titusz first.

**Notes:**

- The published 0.7.0 package has no `encode_seq` and `decode_seq`: they were added after that
    release and ship with 0.8.0, together with this README. The README is correct for the version it
    ships with.
- `learnings.md` is at its 200-line budget. This review added nothing to it: the step confirmed
    existing entries and produced no new pitfall.
- No Rust change: the CRAP, iai and semver gates are untouched.
