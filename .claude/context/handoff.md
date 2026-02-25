## 2026-02-25 — Evaluate TypeScript port and add ecosystem docs page

**Done:** Researched the `branciard/iscc-core-ts` TypeScript implementation via GitHub API and
WebFetch. Created `docs/ecosystem.md` documenting official implementations (iscc-core, iscc-lib) and
the community TypeScript port with factual conformance status. Added navigation entry in
`zensical.toml` between "Explanation" and "Reference".

**Files changed:**

- `docs/ecosystem.md`: New documentation page covering ISCC ecosystem — official implementations
    (iscc-core Python reference, iscc-lib Rust polyglot) and community implementations (iscc-core-ts
    TypeScript). Includes conformance status, function coverage table, licensing, and contribution
    guidelines.
- `zensical.toml`: Added `{ "Ecosystem" = "ecosystem.md" }` nav entry after "Explanation" and before
    "Reference".

**Verification:**

- [x] `uv run zensical build` exits 0 — site builds successfully with ecosystem page at
    `/ecosystem/`
- [x] `test -f docs/ecosystem.md` — ecosystem page exists
- [x] `grep -q 'ecosystem.md' zensical.toml` — navigation entry present
- [x] `grep -q 'iscc-core-ts' docs/ecosystem.md` — TypeScript port mentioned
- [x] `grep -q 'branciard' docs/ecosystem.md` — author/repo attributed
- [x] `grep -q 'iscc-core' docs/ecosystem.md` — reference implementation mentioned
- [x] `grep -q 'icon:' docs/ecosystem.md` — YAML front matter with icon present
- [x] `mise run check` — all 14 hooks pass (no Rust changes, so clippy/tests not triggered by
    pre-commit stage, only formatting hooks run)

**Next:** The only remaining issue in issues.md (TypeScript port evaluation) is now addressed by
this ecosystem page. The CID loop is in maintenance mode — all core functionality, bindings, docs,
CI/release workflows, and ecosystem documentation are complete. Remaining work is human-gated: merge
PR #3, configure publishing credentials, and trigger a release.

**Notes:** Research findings on iscc-core-ts: version 0.3.0, Apache-2.0 license, implements all 9
gen\_\*\_v0 functions plus gen_iscc_id_v0/v1 and gen_flake_code_v0. It vendors the official
`data.json` conformance vectors (66KB) and has a dedicated `conformance.test.ts`. 263 tests pass
across 18 suites. The project is funded by NGI Zero Core (NLnet / European Commission). Author is
François Branciard. Status is "active development, not ready for production" — this is accurately
reflected in the docs page with an admonition box.
