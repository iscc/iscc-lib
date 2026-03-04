## 2026-03-04 — Review of: Add Ruby documentation (howto guide, README, root README)

**Verdict:** PASS

**Summary:** Created comprehensive Ruby how-to guide (422 lines), expanded per-crate README from
31-line stub to 93-line full guide, added Ruby install/quickstart to root README, and wired Ruby
into the docs nav. Clean documentation-only diff — no code changes, no quality gate issues. All API
examples verified against the Ruby wrapper source.

**Verification:**

- [x] `test -f docs/howto/ruby.md` — EXISTS
- [x] `wc -l docs/howto/ruby.md` — PASS (422 lines, ≥300 required)
- [x] `wc -l crates/iscc-rb/README.md` — PASS (93 lines, ≥60 required)
- [x] `grep 'gem install iscc-lib' README.md` — PASS
- [x] `grep 'ruby.md' zensical.toml` — PASS
- [x] `grep 'gen_meta_code_v0' docs/howto/ruby.md` — PASS
- [x] `grep 'DataHasher' docs/howto/ruby.md` — PASS
- [x] `grep 'conformance_selftest' docs/howto/ruby.md` — PASS
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** Codex flagged the `iscc_decode` example variable name `length` vs `length_index`.
Cross-checked: all 4 other language guides (Python, Node.js, WASM, Go) use the same `length`
variable name for the 4th return value. The Ruby guide's description text correctly says
`length_index`. This is a cross-guide naming convention, not a Ruby-specific issue.

**Next:** The Ruby bindings issue is substantially complete — all code, tests, CI, release infra,
and documentation are in place. Remaining items are lower priority: (1) Standard Ruby linting
(`standard` gem, `.standard.yml`, CI wiring), (2) `docs/ruby-api.md` API reference page (not all
languages have one). The root README Go quickstart example is stale (uses old WASM-bridge API) —
worth fixing in a separate housekeeping step.

**Notes:**

- The howto guide follows the Go guide structure with Ruby-specific patterns (keyword args,
    attribute-style access, method chaining). Coverage is comprehensive: all 10 gen functions,
    streaming, codec/diagnostics, text utilities, algorithm primitives, constants, and error
    handling.
- Ruby is now inserted after Python in the docs nav (grouping scripting languages) and after Go in
    the root README install/quickstart sections (before WASM).
- The Ruby bindings issue in issues.md still has uncompleted items (linting, ruby-api.md, account
    setup) but the core functionality and documentation are done.
