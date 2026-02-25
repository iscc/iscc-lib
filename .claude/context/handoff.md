## 2026-02-25 — Polish docs landing page code examples and Key Features

**Done:** Applied three cosmetic fixes to `docs/index.md`: added Java and Go to the Key Features
multi-language bullet, fixed the Rust Quick Start to access `result.iscc` (struct field) instead of
printing as a JSON string, and removed the unnecessary `import json` / `json.loads` from the Python
Quick Start.

**Files changed:**

- `docs/index.md`: Three edits — Key Features bullet (line 34), Rust code example (line 63), Python
    code example (lines 72-77)

**Verification:** All seven criteria pass:

- `uv run zensical build` exits 0 — docs site builds cleanly
- `grep 'Java, Go' docs/index.md` matches line 34
- `grep -c 'json.loads' docs/index.md` returns 0
- `grep -c 'import json' docs/index.md` returns 0
- `grep 'result.iscc' docs/index.md` matches line 63
- `grep -c 'JSON string' docs/index.md` returns 0
- `mise run check` — all 14 pre-commit hooks pass

**Next:** All documentation polish items from the iteration 29/30 review are now resolved. The
project should be at full target compliance. Consider creating the PR from `develop` → `main` for
the v0.0.1 release.

**Notes:** Straightforward text-only changes, no surprises. The Python Quick Start now matches the
README.md pattern (dict-style access without json.loads). The Rust Quick Start now matches the
README.md pattern (struct field access).
