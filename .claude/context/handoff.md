## 2026-11-21 — Gate the stale Python extension and the undiffed generated sources

**Done:** The pre-push `pytest` hook and `mise run test` now rebuild `_lowlevel.abi3.so` before
pytest runs. Both go through one mise task, `py:develop`, which runs
`uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`, the same command CI uses. CI now
diffs `NativeMethods.g.cs` after `cargo build -p iscc-ffi` in `c-ffi`. The `kotlin` job regenerates
the UniFFI Kotlin and Swift outputs into `$RUNNER_TEMP` and diffs the three committed files against
them, ignoring trailing whitespace and blank lines at the end of the file. An anchor test pins all
three wirings.

**Files changed:**

- `mise.toml`: new task `py:develop`; `test` now has `depends = ["py:develop"]`, with a comment
    giving the reason
- `.pre-commit-config.yaml`: the `pytest` entry is now
    `bash -c 'mise run py:develop && uv run pytest --timeout=120'`, with a comment explaining why it
    rebuilds instead of comparing mtimes
- `.github/workflows/ci.yml`: new `c-ffi` step "Check C# bindings freshness". New `kotlin` step
    "Check UniFFI bindings freshness": it prints the diff and an `::error::` that names the
    regeneration command. If either side is missing or empty, the step fails
- `tests/test_generated_artifact_gates.py` (new, 5 tests): checks that the `.cs` diff step comes
    after the FFI build; that the kotlin step comes after the uniffi build, uses both languages,
    names all 3 paths and leaves out `module.modulemap`; and that the hook and the `test` task run
    `maturin develop` before pytest (any `mise run X` is expanded to the task body)
- Docs: the `c-ffi` and `kotlin` rows in `specs/ci-cd.md` (kept within the old column width, so only
    those 2 lines change), the pre-push `pytest` bullet in `docs/development.md`, and the CI
    sections of `packages/{kotlin,swift}/CLAUDE.md`

**Verification:**

- `uv run pytest tests/test_generated_artifact_gates.py`: 5 passed. `mise run test` exits 0: its log
    starts with `[py:develop] $ uv run maturin develop …`, then cargo tests, then
    `822 passed in 43.91s`
- I moved `_lowlevel.abi3.so` to /tmp and ran `uv run prek run pytest --stage pre-push`: exit 0
    ("Python tests…Passed", 43 s), and the `.so` was rebuilt (new mtime 20:18). The moved copy is
    deleted
- `uv run scripts/check_ci_job_table.py` reported "OK: 22 CI jobs consistent". `mise run check`
    exited 0
- Step bodies were taken verbatim from ci.yml (PyYAML) and run with
    `bash --noprofile --norc -eo pipefail` (the GitHub default shell):
    - Real checkout, after building: `.cs` step rc=0 and UniFFI step rc=0 (green from the start)
    - Throwaway repo (`git archive HEAD | tar -x` + `git init` + commit), one change committed at a
        time:
        - `NativeMethods.g.cs` with one changed line: after the build, rc=1, diff and `::error::`
            printed. Restored and rebuilt: rc=0
        - `iscc_uniffi.kt` with the `package` line changed: rc=1,
            `::error::…iscc_uniffi.kt is stale. Regenerate: cargo run … --language kotlin …`. Restored:
            rc=0
        - `iscc_uniffiFFI.h` with `#pragma once` changed: rc=1, `::error::` names the swift command plus
            "then move iscc_uniffiFFI.h …"
        - Real regressions, using each file's own previous version (`.cs` from `3b4250d~1`, `.kt` from
            `1a39d0d~1`): `.cs` step rc=1 (56 changed lines), UniFFI step rc=1
    - The throwaway repo and the moved `.so` are deleted
- `git status --porcelain` shows no change to `NativeMethods.g.cs`, `iscc_uniffi.kt`,
    `iscc_uniffi.swift` or `iscc_uniffiFFI.h`

**Next:** the `conformance_selftest` single-runner refactor (audit), now that the gates check the
right artifacts, or the docs entries that were deferred in favour of this step.

**Notes:**

- What the generator writes compared with the committed files (cmp/od): `.kt` has 3 extra blank
    lines at the end, `.swift` has no final newline, `.h` has 1 extra blank line at the end. That is
    why the step normalises both the end of the file and trailing whitespace:
    `sed 's/[[:space:]]*$//' | awk` drops blank lines at the end and always writes the final newline
- The kotlin generator writes to `<out>/uniffi/iscc_uniffi/iscc_uniffi.kt`. The swift generator
    writes `.swift`, `.h` and `.modulemap` into one directory, so the header's regeneration message
    tells you to move the file into `Sources/iscc_uniffiFFI/`
- Probe pitfall, no CI impact: the `.cs` gate depends on csbindgen's `build.rs` actually running
    again. In a probe tree where only `NativeMethods.g.cs` changed, cargo did not rerun the build
    script (it has no `rerun-if-changed`), so I had to `touch crates/iscc-ffi/src/lib.rs`. In CI,
    the fresh checkout and rust-cache (which does not cache workspace crates) make it run again. The
    existing `iscc.h` step has the same property
- The UniFFI step shells out through `cargo run` with a relative `target/debug/libiscc_uniffi.so`.
    With a non-default `CARGO_TARGET_DIR` it fails closed (rc=1, os error 2), not false-green
- Local `uv run` prints a `VIRTUAL_ENV … does not match` warning because the CID runner sets
    `VIRTUAL_ENV`. maturin still installs into the project env (`/home/dev/.venvs/iscc-lib`)
- No core change: CRAP, iai and the feature matrix are not affected
