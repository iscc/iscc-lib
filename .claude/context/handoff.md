## 2026-02-28 — Add version consistency check to CI

**Done:** Added a standalone `version-check` CI job to `.github/workflows/ci.yml` that runs
`python scripts/version_sync.py --check` to catch manifest version drift on every push. The job uses
only `actions/checkout@v4` and `actions/setup-python@v5` — no Rust toolchain, no uv, no caching —
and completes in seconds.

**Files changed:**

- `.github/workflows/ci.yml`: Added `version-check` job (10 lines) as the first job in the workflow

**Verification:**

- `python scripts/version_sync.py --check` exits 0 (versions in sync: 0.0.2)
- `grep -q 'version_sync' .github/workflows/ci.yml` exits 0 (script referenced in CI)
- `mise run check` — all 14 hooks pass
- CI on develop will validate after push (new job brings total to 9 jobs)

**Next:** The CI/CD spec lists the Go job description as stale ("cargo build --target wasm32-wasip1"
in the spec table, but Go CI is pure Go now). This is a spec maintenance issue, not a code issue.
Beyond that, check target.md for remaining partially-met sections — likely benchmark speedup
documentation or publishing configuration.

**Notes:** Minimal change — 10 lines added to ci.yml, zero risk. The version-check job runs in
parallel with all other jobs (no `needs:` dependency). Python 3.10 was chosen to match the existing
Python job. The script uses only stdlib modules so `setup-python` is the only dependency.
