# Next Work Package

## Step: Restore linux/aarch64 Python wheels in release.yml (issue #49)

## Goal

Add a native-ARM `manylinux*_aarch64` wheel target to the Python release build so v0.6.0 ships Linux
ARM wheels again (dropped in the 0.2.0 matrix rewrite), unblocking iscc-sdk and AWS Graviton
consumers that lack a Rust toolchain. Picks up the `normal` `[human]` issue "Restore linux/aarch64
Python wheels" (#49).

## Scope

- **Modify**: `.github/workflows/release.yml` (only this file — single-file change)
- **Reference**:
    - `.claude/plans/restore-linux-aarch64-python-wheels.md` — the approved plan for this issue
    - `.claude/context/issues.md` — issue #49 text and spec pointer
    - existing `build-napi` / `build-jni` matrices in the same file (they already build
        `aarch64-unknown-linux-gnu`) for the house style, but do NOT change them

## Not In Scope

- **Dependency review/refresh** (the other open v0.6.0 issue) — separate future step; do not touch
    manifests or tooling pins here.
- **QEMU/cross-compilation** for the wheel — use the native `ubuntu-24.04-arm` runner as the plan
    specifies; do not add `docker/setup-qemu-action` or a cross-target build.
- **Docs / README platform tables** — the aarch64 wheel does not exist until the next release is
    published, so do not add forward-looking platform claims to docs; no doc file changes are needed
    for this step (verified: `docs/howto/python.md` makes no wheel-platform claims).
- **Touching other build/publish matrices** (napi, jni, ffi) or the `publish-pypi` job's registry
    logic — only the wheel build matrix and its smoke test change.
- **Bumping the workspace version or running `version:sync`** — this is a CI change, not a release.

## Implementation Notes

Two edits, both inside `.github/workflows/release.yml`:

1. **`build-wheels` matrix (around lines 172–182):** add a fourth `include` entry after the
    windows-latest one:

    ```yaml
      - os: ubuntu-24.04-arm
        target: aarch64
        interpreter: python3.10
    ```

    The existing `maturin-action` step with `manylinux: auto` will select the aarch64 manylinux
    container automatically, and the `before-script-linux` PATH prepend
    (`/opt/python/cp310-cp310/bin`) uses the same container layout on aarch64, so the abi3 wheel is
    tagged `cp310` (not `cp38`) identically. The upload step names the artifact
    `wheels-${{ matrix.os }}-${{ matrix.target }}` → `wheels-ubuntu-24.04-arm-aarch64`; no change
    needed there.

2. **`test-wheels` job (around lines 226–243):** it currently hardcodes `runs-on: ubuntu-latest` and
    downloads `wheels-ubuntu-latest-x86_64`. Matrixify it so the aarch64 wheel is import-tested on
    a real ARM runner before publish (a broken aarch64 wheel must not ship silently). Preferred
    shape — keeps `publish-pypi`'s `needs: [build-wheels, build-sdist, test-wheels]` unchanged (a
    matrixed job still completes under the single job id `test-wheels`):

    ```yaml
    test-wheels:
      name: Test Python wheel (${{ matrix.arch }})
      needs: [build-wheels]
      if: inputs.version != '' || inputs.pypi
      runs-on: ${{ matrix.os }}
      strategy:
        matrix:
          include:
            - os: ubuntu-latest
              arch: x86_64
              artifact: wheels-ubuntu-latest-x86_64
            - os: ubuntu-24.04-arm
              arch: aarch64
              artifact: wheels-ubuntu-24.04-arm-aarch64
      steps:
        - uses: actions/setup-python@v5
          with:
            python-version: '3.10'
        - name: Download wheel
          uses: actions/download-artifact@v4
          with:
            name: ${{ matrix.artifact }}
            path: dist
        - name: Install and test
          run: |
            pip install dist/*.whl
            python -c "from iscc_lib import conformance_selftest; assert conformance_selftest()"
    ```

    Keep the smoke-test command identical to the current one (`conformance_selftest()` assertion). Do
    not add the aarch64 test as a separate job unless you also add it to `publish-pypi`'s `needs` —
    the matrix approach avoids that extra edit.

Edge cases / gotchas:

- `ubuntu-24.04-arm` is a GitHub-hosted native ARM runner (free for public repos) — no QEMU, no
    `container:` override needed.
- Preserve YAML indentation exactly (the current matrix `include` entries are 10-space indented
    under `strategy:` → `matrix:` → `include:`); a mis-indent silently drops the entry.
- This change only executes during a `workflow_dispatch` release with `pypi` selected — it cannot be
    exercised by the CID loop, so verification below is static (YAML validity + presence
    assertions).

## Verification

- `uv run python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0
    (the workflow still parses as valid YAML).
- `grep -q 'ubuntu-24.04-arm' .github/workflows/release.yml` — the native ARM runner is referenced.
- `python3 -c "import sys; s=open('.github/workflows/release.yml').read(); sys.exit(0 if ('target: aarch64' in s and 'ubuntu-24.04-arm' in s) else 1)"`
    exits 0 (the `build-wheels` matrix gained an aarch64 entry).
- The `test-wheels` job resolves the aarch64 wheel artifact `wheels-ubuntu-24.04-arm-aarch64`
    (either the literal string is present, or the matrix `os`+`target`/`artifact` values compose to
    it): `grep -q 'wheels-ubuntu-24.04-arm-aarch64' .github/workflows/release.yml` OR the matrix
    carries `arch: aarch64` with an `artifact`/download that resolves to that name.
- `mise run check` passes (yamlfix/whitespace hooks accept the edited workflow; no other file
    dirtied).
- The three original `build-wheels` entries are still present verbatim — `ubuntu-latest`/`x86_64`,
    `macos-14`/`universal2-apple-darwin`, `windows-latest`/`x64` — so existing wheel outputs are
    unchanged.

## Done When

`.github/workflows/release.yml` builds and smoke-tests a `manylinux*_aarch64` abi3 wheel on a native
`ubuntu-24.04-arm` runner alongside the existing three targets, the workflow parses as valid YAML,
and `mise run check` is clean — with no other file touched.
