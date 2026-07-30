---
name: define-next-ty-generator-scripts
description: How to scope a checked-in Python generator script that needs a pinned external package, without polluting dev deps or tripping the ty pre-push gate
metadata:
  type: project
---

# Checked-in generator scripts with external pins

Rule: a generator script under `scripts/` that needs a package the project does not depend on gets
**PEP 723 inline metadata + `uv run --script scripts/x.py`** — never a `[dependency-groups]` entry.

**Why:** the pre-push hook runs bare `uv run ty check` (no path filter, whole project;
`[tool.ty.src] exclude` currently lists only `packages/cpp/conanfile.py` with the comment "conan is
not a project dependency"). An import of a package absent from the project env is an
`unresolved-import` **error**, so the script reddens a gate it has nothing to do with. Adding the
package to dev deps instead makes it a permanent hold-back item in every future dependency-refresh
slice (a pinned Unicode table package must never be "refreshed").

**How to apply:** scope the script with inline metadata; then tell the advance agent that *if*
`uv run ty check` reports `unresolved-import`, the sanctioned fix is one more entry in the existing
`[tool.ty.src] exclude` list with a comment mirroring the conanfile.py precedent — and that an
inline `# ty: ignore` is not acceptable (the review agent's gate-integrity check reads inline
suppressions as weakening). Budget `pyproject.toml` as a source file in `next.md` up front so the
conditional edit does not blow the 3-file cap.

Related: [[unicode-freeze-facts]] (the first step to use this pattern).
