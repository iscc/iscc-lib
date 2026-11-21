## 2026-11-21 — Review of: Gate the stale Python extension and the undiffed generated sources

**Verdict:** PASS

**Summary:** The pre-push `pytest` hook and `mise run test` now rebuild the Python extension through
one mise task (`py:develop`), and CI diffs `NativeMethods.g.cs` (job `c-ffi`) and the three UniFFI
outputs (job `kotlin`) against their committed copies. I re-ran both CI step bodies against real
regressions in a throwaway copy, pushed the advance commits first, and read the job logs: CI run
44946870015 on `e24d041` is green on all 24 jobs and both steps ran their generators. The audit
issue is resolved and deleted.

**Verification:**

- [x] `uv run pytest tests/test_generated_artifact_gates.py` passes, and `uv run pytest` passes — 5
    passed; 822 passed in 21.6 s
- [x] With `_lowlevel.abi3.so` moved out of the tree, `uv run prek run pytest --stage pre-push`
    exits 0 and the `.so` exists again — exit 0 in 40 s, new mtime, moved copy deleted
- [x] The handoff shows the throwaway-repo runs, each step non-zero on a mutated committed file and
    0 on the unmodified tree — shown by advance, and re-run by me from a `git archive` copy with
    `bash --noprofile --norc -eo pipefail`:
    - `.cs` step: clean tree rc=0; one changed line after a rebuild rc=1
    - UniFFI step: clean tree rc=0; `iscc_uniffi.swift` from `1a39d0d~1` rc=1 (advance had left the
        Swift file out); trailing whitespace and extra blank lines at EOF only rc=0; one added inner
        space rc=1; an empty `iscc_uniffi.kt` rc=1; restored rc=0
- [x] `uv run scripts/check_ci_job_table.py` exits 0, and `mise run check` exits 0 — "22 CI jobs
    consistent"; all hooks passed
- [x] `git status --porcelain` shows no change to `NativeMethods.g.cs`, `iscc_uniffi.kt`,
    `iscc_uniffi.swift` or `iscc_uniffiFFI.h` — only the runner's `iterations.jsonl` is modified

Beyond next.md (3 probes):

- CI logs. `c-ffi` → "Build FFI crate" prints `Compiling iscc-ffi` before `Compiling iscc-lib` on a
    full rust-cache hit, so csbindgen's `build.rs` ran and the `.cs` diff is not vacuous. `kotlin` →
    the step ran `uniffi-bindgen generate` for Kotlin and for Swift.
- Anchor-test mutations (8, on a copy of the workflow, hook and mise files): 7 failed the test as
    they should. Dropping the UniFFI step's last line `exit "$stale"` stayed green, and without that
    line the step can never fail. Fixed here: the test now asserts the step ends with it (mutation
    re-run: 1 failed).
- Hook cost: `mise run py:develop` takes about 15 s after `cargo clippy` or `cargo test` (it
    recompiles `pyo3-ffi`, `pyo3` and `iscc-py`) and 0.3 s when repeated. The whole pre-push batch
    took 60 s.

**Issues found:**

- Fixed in this review: the anchor test did not pin the `exit "$stale"` line (see above). One
    assertion added to `tests/test_generated_artifact_gates.py`; `ty` and ruff are green on it.
- Not filed, behaviour to know: the hook and `mise run test` install a debug build, as CI does. A
    `maturin develop --release` build is replaced on every push, so rebuild with `--release` before
    running the Python benchmarks (`docs/benchmarks.md` already gives that command).
- Not filed: the UniFFI check runs `cargo run --features bindgen`, which rebuilds
    `libiscc_uniffi.so` with that feature, so the Gradle tests after it load that build. The exports
    are the same and the job is green.
- Not filed, confirmed advance's note: in a warm tree a change to `NativeMethods.g.cs` alone does
    not rerun `build.rs` (rc=0 until `crates/iscc-ffi/src/lib.rs` is touched). CI is not affected
    (fresh checkout, log evidence above). Recorded in learnings.md.
- A bare `uv run pytest` still imports whatever `.so` is there. next.md excluded that on purpose.

**Codex review:** No actionable findings. Codex ran the Rust and Python suites and both freshness
checks, and confirmed that they reject modified artifacts.

**Next:** No `critical` issue is open. Candidates among the `normal` entries, for define-next to
weigh:

- `conformance_selftest` repeats one runner loop nine times (audit, 2 files plus
    `.crap-baseline.json`). It is a core change, so it goes alone. The CRAP gate is CI-only: move
    the baseline in the same step and run the CI CRAP lines locally.
- "Specs and crate CLAUDE.md files describe shipped work as pending" (audit, 8 files, audit-scope
    step). define-next and advance read these files as truth, so this one pays back on every later
    step.
- The docs entries (`KAA` Sum-Code prefix, pkg-config version line, `docs/development.md` release
    section, `llms.txt` coverage, `@iscc/wasm` Node setup) are small and independent.

`alg_minhash_256([])` needs a decision between two behaviours before it can be scoped; if
define-next picks it, it should say which behaviour it chooses and why, or ask Titusz.

**Notes:**

- Gates are in good shape: no gate was weakened, and the scope was 3 non-test, non-doc files.
- The advance commits (`b915d78`, `e24d041`) are already on `origin/develop`; this review commit
    adds the test assertion and the context files on top.
- learnings.md stays at 200 lines: the test-framework-major note moved to `learnings-archive.md` to
    make room. Any new bullet needs an equal cut.
- No gate strengthening to flag. The other gitignored native artifacts (napi `.node`, Ruby `.so`,
    JNI `.so`) are still rebuilt by hand before local probing (learnings.md → CI/CD).
