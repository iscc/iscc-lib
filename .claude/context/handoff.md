## 2026-11-20 — Review of: Binding specs stop naming repository paths that do not exist

**Verdict:** PASS_WITH_NOTES

**Summary:** advance removed all five prescribed phantom paths from the wasm, go and nodejs specs.
It also removed a sixth (the napi `npm/*/package.json` row), which the check's regex could not see.
Every new sentence matches its source: release.yml lines 428-439, `crates/iscc-wasm/Cargo.toml`,
ci-cd.md "Automatic propagation", target.md's `packages/go/vX.Y.Z` tag form. The `normal` wasm issue
is resolved and deleted. The non-backticked structure trees in the same specs still name files that
do not exist. next.md did not cover them, so they are filed as a new `normal` issue.

**Verification:**

- [x] No `^├── package\.json` in wasm-bindings.md; `grep -c 'package.json'` = 2 (the `pkg/` child
    survives)
- [x] The four-path grep over `.claude/context/specs/` prints nothing
- [x] `grep -n 'go/vX' go-bindings.md | grep -v 'packages/go/vX'` prints nothing
- [x] Existence loop prints exactly `packages/go/vX.Y.Z` and `scripts/gen_ffi_boundary_vectors.py`
- [x] `mise run check` exits 0
- [x] Probes: no `crates/iscc-napi/npm/` dir, and `TARGETS` has one napi entry, so the napi row
    removal holds. release.yml:115 tags `packages/go/v…`. Every backticked `*` glob in the specs
    matches files

**Issues found:**

- **Filed `normal` \[review\]: "Binding spec structure trees list files that do not exist".** The Go
    Package Structure tree names 11 files `packages/go` never had (`iscc.go`, `text.go`,
    `algorithms.go`, `wta.go`, `hasher.go`, their tests, `constants.go`). The napi tree names
    `test/conformance.test.mjs`, but the tests live in `__tests__/`. This is the same defect as the
    removed wasm tree line, and it is a leftover next.md missed. Trees in the other specs were not
    probed
- **Step-9 fix:** cut advance's "(no in-repo package-version string)" from the go Verification
    Criteria bullet. It turned a description into a prohibition that no spec states
- **Acceptance-criteria edits confirmed:** the wasm and go "Version synced … via
    `mise run version:sync`" bullets now name the real mechanism, matching ci-cd.md. Recorded in
    decisions.md 2026-11-20 ("A spec criterion naming a mechanism that does not exist is realigned,
    not escalated")

**Codex review:** no actionable findings. It confirmed the WASM metadata path, the
`packages/go/vX.Y.Z` tag form, the single napi manifest, and that the removed API pages do not
exist.

**Next:** two `normal` spec-hygiene items are now open, and one check can serve both. That check
fails when a spec names a repository path that does not exist: backticked paths, `*` globs
(`compgen -G`), and structure-tree lines resolved against their root dir, with generated entries
(`pkg/`, napi `index.js`/`index.d.ts`) allowlisted. It would resolve the new tree issue and cover
the file-list bullet of the `[human]` derived-facts issue. Scope it by first probing the trees in
every spec. Cutting a tree down to its few load-bearing entries is a valid fix where a check would
pin incidental layout. The Tier 1 parity half of the derived-facts issue still needs its own scoping
pass. The other open `normal` item is the Dependabot entry, which needs Titusz for `soupsieve`.

**Notes:** the larger-step trial (issues.md, evaluate after iteration 224) has one iteration left.
This one ends PASS_WITH_NOTES.
