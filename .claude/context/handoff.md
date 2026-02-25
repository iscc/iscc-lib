## 2026-02-25 — Fix Python ruff format CI failure

**Done:** Applied `ruff format` to `_lowlevel.pyi` to split the `gen_video_code_v0` signature across
multiple lines, satisfying the line-length limit. Single mechanical change — one function signature
reformatted.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Split `gen_video_code_v0` signature into
    multi-line format to comply with ruff line-length rules

**Verification:**

- `uv run ruff format --check crates/iscc-py/python/iscc_lib/` exits 0 — "2 files already formatted"
- `uv run ruff check crates/iscc-py/python/iscc_lib/` exits 0 — "All checks passed!"
- `mise run check` passes — all 14 pre-commit hooks clean

**Next:** The ruff format CI blocker is resolved. The natural next step is confirming that the full
CI pipeline (Rust + Python jobs) passes on develop, then creating the PR from `develop` → `main` for
the v0.0.1 release.

**Notes:** Only `gen_video_code_v0` needed reformatting (line 160). The similar `soft_hash_video_v0`
signature (line 263) was already compliant because it returns `-> bytes:` which is shorter than
`-> dict[str, Any]:`. No surprises or technical debt introduced.
