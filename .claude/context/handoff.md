## 2026-02-25 — Review of: Polish docs landing page code examples and Key Features

**Verdict:** PASS

**Summary:** Three targeted cosmetic fixes applied to `docs/index.md` — exactly what next.md
specified, nothing more. Key Features bullet now lists Java and Go, Rust Quick Start accesses
`result.iscc` (struct field) instead of printing as a JSON string, Python Quick Start drops the
unnecessary `import json` / `json.loads`. All changes are consistent with the README.md patterns.

**Verification:**

- [x] `uv run zensical build` exits 0 — docs site builds cleanly
- [x] `grep 'Java, Go' docs/index.md` matches — Key Features bullet lists all 7 languages
- [x] `grep -c 'json.loads' docs/index.md` returns 0 — no unnecessary json.loads
- [x] `grep -c 'import json' docs/index.md` returns 0 — no unused json import
- [x] `grep 'result.iscc' docs/index.md` matches — Rust example accesses struct field
- [x] `grep -c 'JSON string' docs/index.md` returns 0 — misleading comment removed
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention — no suppressions, no test skipping, no threshold changes

**Issues found:**

- (none)

**Next:** All documentation polish items are resolved. The project should be at full target
compliance. The natural next step is creating the PR from `develop` → `main` for the v0.0.1 release,
or running a final state assessment to confirm nothing remains.

**Notes:** This was the last cosmetic fix identified in the iteration 29 review. The docs landing
page now has correct code examples across all 6 language tabs, matching the README.md patterns.

---

## 2026-02-25 — Interactive session commits (human + Claude)

Five commits were pushed to `develop` outside the CID loop during an interactive session:

| Commit    | Type            | Description                                                                                                        |
| --------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| `52d1c88` | `fix(hooks)`    | Pre-push hooks use `always_run: true` instead of `types:`                                                          |
| `3bed859` | `style(rust)`   | Inline format arguments in `crates/iscc-lib/src/lib.rs`                                                            |
| `5461a65` | `feat(iscc-py)` | CPython C API extraction for video functions + flat variants (`gen_video_code_v0_flat`, `soft_hash_video_v0_flat`) |
| `69bb36c` | `docs`          | Command execution guidelines added to `CLAUDE.md`                                                                  |
| `c4e3657` | `chore`         | CID context files updated (API symbol count 22→23, iteration log, README)                                          |

**What changed and why:**

- **`.pre-commit-config.yaml`** — Pre-push quality gates (clippy, cargo test, ty, security,
    complexity, pytest) were filtered by file type and could be skipped when commits only touched
    non-matching files. Now they always run.
- **`crates/iscc-py/src/lib.rs`** — `gen_video_code_v0` and `soft_hash_video_v0` now accept any
    Python sequence (not just `list[list[int]]`) via direct CPython C API calls, avoiding PyO3
    object wrapping overhead. Two new `*_flat` functions accept pre-flattened byte buffers for
    numpy/array.array callers.
- **`crates/iscc-py/python/iscc_lib/__init__.py`** and **`_lowlevel.pyi`** — Updated type signatures
    to `Sequence[Sequence[int]]` and added stubs for the flat variants.
- **`CLAUDE.md`** and **`.claude/context/`** — Documentation and context bookkeeping.

**Impact on CID loop:** The next `update-state` should pick up these changes. No spec or target
conflicts — the flat variants are additive (new Tier 1 symbols, hence the 22→23 count update in
`target.md` and `specs/rust-core.md`). All pre-commit hooks pass.
