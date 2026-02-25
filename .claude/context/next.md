# Next Work Package

## Step: Fix Python ruff format CI failure

## Goal

Fix the `ruff format --check` failure in CI that is blocking the Python job (and thus pytest) from
passing. The `_lowlevel.pyi` file has a single line-length formatting violation introduced by the
interactive session commit `5461a65` (CPython C API optimizations for video functions).

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
- **Reference**: (none — the fix is mechanical formatting)

## Not In Scope

- Modifying any Rust code (`lib.rs`, `src/*.rs`)
- Modifying `__init__.py` (already passes `ruff format --check`)
- Investigating or changing the CPython C API logic in `crates/iscc-py/src/lib.rs`
- Any other Python source changes beyond formatting
- Tagging v0.0.1 or creating PRs (separate future step)

## Implementation Notes

Run `uv run ruff format crates/iscc-py/python/iscc_lib/_lowlevel.pyi` to apply canonical formatting.
The only change is splitting the `gen_video_code_v0` function signature across multiple lines to
satisfy the line-length limit:

```diff
-def gen_video_code_v0(frame_sigs: Sequence[Sequence[int]], bits: int = 64) -> dict[str, Any]:
+def gen_video_code_v0(
+    frame_sigs: Sequence[Sequence[int]], bits: int = 64
+) -> dict[str, Any]:
```

After applying, run `mise run format` to ensure all pre-commit auto-fix hooks are satisfied, then
stage and commit.

## Verification

- `uv run ruff format --check crates/iscc-py/python/iscc_lib/` exits 0
- `uv run ruff check crates/iscc-py/python/iscc_lib/` exits 0 (still passes)
- `mise run check` passes — all pre-commit hooks clean

## Done When

All three verification criteria pass, confirming the formatting fix restores CI-green status for the
Python job.
