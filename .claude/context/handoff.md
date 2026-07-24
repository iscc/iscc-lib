# Handoff

## 2026-07-24 — Restore linux/aarch64 Python wheels in release.yml (issue #49)

**Done:** Added a native-ARM `manylinux*_aarch64` wheel target to the Python release build. The
`build-wheels` matrix gained a fourth entry (`ubuntu-24.04-arm` / `aarch64` / `python3.10`), and the
`test-wheels` job was matrixified (x86_64 on `ubuntu-latest` + aarch64 on `ubuntu-24.04-arm`) so the
aarch64 wheel is install- and selftest-verified on a real ARM runner before publish. Single-file
change, exactly as the plan and next.md specified.

**Files changed:**

- `.github/workflows/release.yml`: (1) `build-wheels` matrix `include` gained
    `os: ubuntu-24.04-arm / target: aarch64 / interpreter: python3.10` after the windows entry; (2)
    `test-wheels` now uses `runs-on: ${{ matrix.os }}` with a 2-entry `include` matrix
    (`arch`/`artifact` per row), downloads `${{ matrix.artifact }}`, job name shows the arch. The
    smoke-test command (`conformance_selftest()` assertion) is unchanged.

**Verification:**

- `uv run python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` — exit 0
    (valid YAML).
- `grep -q 'ubuntu-24.04-arm'` and the `target: aarch64` presence check — both pass.
- `grep -q 'wheels-ubuntu-24.04-arm-aarch64'` — pass (the matrix `artifact` value is the literal
    string, matching what `build-wheels` uploads as `wheels-${{ matrix.os }}-${{ matrix.target }}`).
- Structural check via parsed YAML: `build-wheels` has exactly 4 include entries with the original
    three verbatim (`ubuntu-latest`/`x86_64`, `macos-14`/`universal2-apple-darwin`,
    `windows-latest`/`x64`); `test-wheels` matrix has exactly the 2 planned rows; `publish-pypi`
    `needs: [build-wheels, build-sdist, test-wheels]` is untouched (a matrixed job still completes
    under the single `test-wheels` job id).
- `mise run check` — all pre-commit hooks Passed (yamlfix accepted the edit as-is; no other file
    dirtied).
- Not runtime-verified: the change only executes during a `workflow_dispatch` release with `pypi`
    selected, so the actual aarch64 wheel build/smoke-test will first run at the next release (per
    next.md, static verification is the acceptance bar for this step).

**Next:** The remaining CID-doable v0.6.0 issue is the **project-wide dependency review/refresh**
(root `Cargo.toml`/lock, `pyproject.toml`/`uv.lock`, binding manifests, tooling pins) — mind the
documented pinning constraints (PyO3/gil_used, rb_sys/cross-gem version match, abi3-py310,
quality-gate tool pins bump together with their baselines). Do NOT cut v1.0.0 or flip Semver to
enforcing (human-held).

**Notes:**

- The `before-script-linux` PATH prepend (`/opt/python/cp310-cp310/bin`) is shared by both Linux
    matrix rows; the aarch64 manylinux container uses the same `/opt/python` layout, so the abi3
    wheel will be tagged `cp310` identically (per the plan's analysis — will be confirmed at the
    next release).
- `maturin-action` with `manylinux: auto` selects the aarch64 manylinux container automatically on
    the ARM runner; no QEMU or `container:` override added (explicitly out of scope).
- No docs touched: the aarch64 wheel doesn't exist until the next release publishes, and
    `docs/howto/python.md` makes no wheel-platform claims (verified by next.md).
- `.claude/context/iterations.jsonl` is modified in the working tree (runner-managed); left unstaged
    per protocol.
