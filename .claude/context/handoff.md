## 2026-03-21 — Swift documentation and README integration

**Done:** Created Swift howto guide (`docs/howto/swift.md`) covering all 10 gen functions, streaming
hashers, codec operations, text/encoding utilities, algorithm primitives, constants, conformance
testing, and error handling. Created `packages/swift/CLAUDE.md` with package role, file layout,
build commands, test patterns, CI notes, and common pitfalls. Added Swift to README.md
(installation, quick start, Key Features polyglot list), zensical.toml nav, and gen_llms_full.py
ORDERED_PAGES.

**Files changed:**

- `docs/howto/swift.md`: New file — full Swift howto guide (25 sections, matches dotnet.md
    structure)
- `packages/swift/CLAUDE.md`: New file — per-package agent docs for Swift bindings
- `README.md`: Added Swift installation (SPM Package.swift snippet), quick start example, and
    "Swift" to polyglot language list
- `zensical.toml`: Added `{ "Swift" = "howto/swift.md" }` after C / C++ in How-to Guides nav
- `scripts/gen_llms_full.py`: Added `"howto/swift.md"` after `"howto/c-cpp.md"` in ORDERED_PAGES

**Verification:** All 9 verification criteria pass:

- `test -f docs/howto/swift.md` — PASS
- `test -f packages/swift/CLAUDE.md` — PASS
- `grep -q 'Swift' zensical.toml` — PASS
- `grep -q 'howto/swift.md' zensical.toml` — PASS
- `grep -q 'howto/swift.md' scripts/gen_llms_full.py` — PASS
- `grep -q 'swift' README.md` — PASS
- `grep -q 'genMetaCodeV0' docs/howto/swift.md` — PASS
- `grep -q 'DataHasher' docs/howto/swift.md` — PASS
- `grep -c '##' docs/howto/swift.md` → 25 (≥ 10) — PASS
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- `mise run check` — 15/15 hooks pass

**Next:** Swift documentation is complete. The remaining Swift gap is version sync integration
(`Constants.swift` + `scripts/version_sync.py`), which was explicitly out of scope. After that, the
Swift issue can be closed and Kotlin bindings work can begin.

**Notes:** Swift API uses all free functions (not methods on a class), Data type for binary data,
and getter functions for constants. The howto guide accurately reflects the actual UniFFI-generated
API signatures from `iscc_uniffi.swift`. The `InstanceCodeResult` struct has `datahash` and
`filesize` fields (unlike the C#/.NET version which only has `iscc`) — this matches the actual
UniFFI binding definition.
