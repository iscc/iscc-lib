## 2026-03-21 — Review of: Swift documentation and README integration

**Verdict:** PASS

**Summary:** The advance agent created comprehensive Swift documentation (howto guide, CLAUDE.md,
README sections, nav entries) that accurately reflects the UniFFI-generated API. All API signatures,
result types, and parameter types were verified against the actual generated Swift source
(`iscc_uniffi.swift`). No quality gate circumvention, no out-of-scope changes.

**Verification:**

- [x] `test -f docs/howto/swift.md` exits 0 — PASS
- [x] `test -f packages/swift/CLAUDE.md` exits 0 — PASS
- [x] `grep -q 'Swift' zensical.toml` exits 0 — PASS
- [x] `grep -q 'howto/swift.md' zensical.toml` exits 0 — PASS
- [x] `grep -q 'howto/swift.md' scripts/gen_llms_full.py` exits 0 — PASS
- [x] `grep -q 'swift' README.md` exits 0 — PASS (install, quickstart, and polyglot list)
- [x] `grep -q 'genMetaCodeV0' docs/howto/swift.md` exits 0 — PASS
- [x] `grep -q 'DataHasher' docs/howto/swift.md` exits 0 — PASS
- [x] `grep -c '##' docs/howto/swift.md` → 25 (≥ 10) — PASS
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- [x] `mise run check` — 15/15 hooks pass

**Issues found:**

- (none)

**Codex review:** No actionable issues found. Codex confirmed: "no concrete regressions affecting
repository behavior, tests, or documented consumers."

**Next:** The remaining Swift gap is **version sync integration** — add `Constants.swift` to
`scripts/version_sync.py` targets so the Swift package version stays in sync with releases. After
that, the Swift issue in issues.md can be closed. Then Kotlin bindings work can begin (the other
`normal` priority issue).

**Notes:**

- Swift howto guide (425 lines, 25 sections) follows the dotnet.md structural template faithfully
- All Swift API signatures verified against `packages/swift/Sources/IsccLib/iscc_uniffi.swift`
- `InstanceCodeResult` has `datahash` and `filesize` fields (matches UniFFI definition, differs from
    .NET which only has `iscc`)
- README Swift sections correctly placed after C/C++ and before WASM in both install and quickstart
- `packages/swift/CLAUDE.md` includes all critical pitfalls (module name matching, macOS-only
    testing, generated file rules)
