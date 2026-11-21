# Next Work Package

## Step: Gate the stale Python extension and the undiffed generated sources

## Goal

Close the `[audit]` issue "Gates test generated and built artifacts that nothing refreshes or
diffs". Three changes: local pytest runs against a freshly built `_lowlevel.abi3.so`, CI fails on a
stale committed `NativeMethods.g.cs`, and CI fails on stale committed UniFFI outputs.

## Alternatives Considered

- **Chosen:** the generated-artifacts gate. It is the review's "Next". The UniFFI outputs have
    matched the generator since iteration 272, so the gate starts green. A gate that checks the
    wrong artifact misleads every later step, so it goes before the docs entries.
- **Rejected:** the `conformance_selftest` single-runner refactor (audit, 2 files plus the CRAP
    baseline). It is a core change, which goes alone, and it needs a CRAP baseline move that only CI
    checks. Better taken when the gates are trustworthy.

## Scope

- **Create**: `tests/test_generated_artifact_gates.py`
- **Modify**: `.pre-commit-config.yaml` (pre-push `pytest` hook), `mise.toml` (`test` task, plus a
    shared build task if you add one), `.github/workflows/ci.yml` (`c-ffi` and `kotlin` jobs)
- **Modify (docs)**: `.claude/context/specs/ci-cd.md` (the `c-ffi` and `kotlin` rows of the CI job
    table), `docs/development.md` (the pre-push `pytest` bullet near line 181),
    `packages/kotlin/CLAUDE.md` and `packages/swift/CLAUDE.md` (the "CI" sections: CI diffs the
    generated files)
- **Reference**: `crates/iscc-ffi/build.rs` (csbindgen rewrites `NativeMethods.g.cs` on every
    build), the "Binding Generation" sections of `packages/kotlin/CLAUDE.md` and
    `packages/swift/CLAUDE.md`, the `python-test` job in `ci.yml` (its `maturin develop` line),
    `scripts/check_ci_job_table.py`

## Not In Scope

- `packages/swift/Sources/iscc_uniffiFFI/module.modulemap`: it is maintained by hand (Darwin `use`
    lines removed), so do not diff it against the generator output
- Do not regenerate or edit `iscc_uniffi.kt`, `iscc_uniffi.swift`, `iscc_uniffiFFI.h` or
    `NativeMethods.g.cs`. If a check finds one stale, stop and report it in the handoff. Do not fix
    it in this step
- No Windows CI leg, no change to the iai/CRAP gates, no `release.yml` edits
- Do not add a staleness check to bare `pytest` runs (a conftest that builds). Cover only the
    pre-push hook and `mise run test`

## Implementation Notes

- **Stale `.so`**: rebuild instead of comparing mtimes. A checkout resets mtimes, and a core change
    under `crates/iscc-lib/src` also makes the extension stale. Use the command CI uses:
    `uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`, run before `uv run pytest`
    in both the pre-push `pytest` hook and `mise run test`. One rule, two places: define the build
    command once if you can (for example a `mise` task that `test` depends on and the hook calls via
    `mise run`, as the standardrb hook already calls `mise exec`). Write the reason in a comment
    above the hook.
- **`NativeMethods.g.cs`**: in the `c-ffi` job, after `Build FFI crate`, add a step
    `git diff --exit-code packages/dotnet/Iscc.Lib/NativeMethods.g.cs` with an `::error::` message
    in the style of the `iscc.h` step. The build has already rewritten the file, so a diff means the
    committed copy is stale.
- **UniFFI**: in the `kotlin` job (ubuntu; it already runs `cargo build -p iscc-uniffi`), add one
    step. Run the generator with the commands documented in the two CLAUDE.md files, but write into
    a temp dir (`$RUNNER_TEMP`). Then compare three generated files with their committed copies:
    `iscc_uniffi.kt`, `iscc_uniffi.swift`, and `iscc_uniffiFFI.h` (committed under
    `packages/swift/Sources/iscc_uniffiFFI/`). Compare modulo trailing whitespace: pipe both sides
    through `sed 's/[[:space:]]*$//'` and `diff` them (learnings: the third-party generator writes
    bytes the hooks rewrite). If the files also differ in trailing blank lines at EOF, normalise
    those too and say so in the handoff. Exit non-zero with an `::error::` that names the
    regeneration command.
- **Test** (`tests/test_generated_artifact_gates.py`): an anchor test that parses `ci.yml` and
    `.pre-commit-config.yaml` with PyYAML (already a dev dep). Assert that the `NativeMethods.g.cs`
    diff step comes after the `cargo build -p iscc-ffi` step in `c-ffi`, that the `kotlin` job names
    all three generated files and runs `uniffi-bindgen` for both languages, and that the `pytest`
    hook entry (or the task it calls) runs `maturin develop` before `pytest`. Keep the functions
    small and plain, with no classes.
- **Prove each CI step on a real regression** (learnings: throwaway repo). In
    `git archive HEAD | tar -x -C /tmp/<dir>` plus `git init` and a commit, commit a copy of
    `NativeMethods.g.cs` with one line changed, build, and run the step body: it must exit non-zero.
    Do the same with one changed line in `iscc_uniffi.kt`. Then run both step bodies on the
    unmodified tree: they must exit 0. Paste the commands and exit codes into the handoff. Delete
    the temp dirs afterwards.

## Verification

- `uv run pytest tests/test_generated_artifact_gates.py` passes, and `uv run pytest` passes
- Move `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` out of the tree, then
    `uv run prek run pytest --stage pre-push` exits 0 and the `.so` exists again (then delete the
    moved copy)
- The handoff shows the throwaway-repo runs: the `NativeMethods.g.cs` step and the UniFFI step each
    exit non-zero on a mutated committed file and exit 0 on the unmodified tree
- `uv run scripts/check_ci_job_table.py` exits 0, and `mise run check` exits 0
- `git status --porcelain` shows no change to `NativeMethods.g.cs`, `iscc_uniffi.kt`,
    `iscc_uniffi.swift` or `iscc_uniffiFFI.h`

## Done When

Every verification criterion passes: the pre-push pytest rebuilds the extension, and CI diffs all
four generated sources against their committed copies.
