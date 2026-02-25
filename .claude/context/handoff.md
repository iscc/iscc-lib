## 2026-02-25 — Create version sync tooling

**Done:** Created `scripts/version_sync.py` that reads the workspace version from root `Cargo.toml`
and syncs it to `crates/iscc-napi/package.json` and `crates/iscc-jni/java/pom.xml`. Added
`version:sync` and `version:check` mise tasks. The pom.xml was updated from `0.0.1-SNAPSHOT` to
`0.0.1` as part of the sync.

**Files changed:**

- `scripts/version_sync.py`: New script — reads workspace version via regex, updates package.json
    (json stdlib) and pom.xml (regex replacement), supports `--check` flag for validation mode
- `mise.toml`: Added `version:sync` and `version:check` tasks under a new `# --- Version ---`
    section
- `crates/iscc-jni/java/pom.xml`: Version changed from `0.0.1-SNAPSHOT` to `0.0.1` (result of
    running the sync script)

**Verification:** All 8 criteria pass:

- `uv run scripts/version_sync.py --check` exits 0
- `uv run scripts/version_sync.py` runs without error
- `mise run version:sync` executes successfully
- `mise run version:check` exits 0
- `grep -q 'version:sync' mise.toml` exits 0
- `grep -q 'version:check' mise.toml` exits 0
- `grep -q 'import re' scripts/version_sync.py` exits 0
- `mise run check` passes (all 14 pre-commit hooks clean)

**Next:** The `[normal]` version sync issue can be resolved. The remaining issues are all `[low]`
priority: TypeScript port evaluation, JNI exception mapping, and WASM CLAUDE.md staleness. The
project is ready for `v0.0.1` release — consider creating a PR from develop to main.

**Notes:** The pom.xml `-SNAPSHOT` suffix was stripped by the sync script as designed (next.md
explicitly stated "the sync script writes the bare version"). This is a real file change that should
be committed alongside the script. No third-party dependencies used — pure stdlib Python.
