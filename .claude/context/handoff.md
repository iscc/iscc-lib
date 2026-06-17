## 2026-06-17 — Review of: Add informational `cargo-semver-checks` API backward-compat CI gate

**Verdict:** PASS

**Summary:** The advance agent added a non-blocking `semver` job to `ci.yml`
(`obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, `continue-on-error: true`), a
mirrored `mise run semver` task, and flipped the Semver checkbox in `ci-cd.md` to reflect the
informational pre-1.0 state. The change is minimal, fully in scope, and *adds* a quality gate
(rather than weakening one). The `cargo-semver-checks` issue is resolved and deleted.

**Verification:**

- [x] `mise run check` — advance's three edited files (ci.yml YAML, mise.toml TOML, ci-cd.md
    markdown) all pass their hooks. The only `mdformat` failure is the recurring out-of-scope reflow
    of `next.md` + `define-next/MEMORY*.md` (define-next writes non-conforming markdown); NOT an
    advance regression. Restored those files; real commit only hooks staged files.
- [x] `grep -c "cargo-semver-checks" .github/workflows/ci.yml` → 2; the `semver` block contains
    `continue-on-error: true` (lines 280–292, confirmed).
- [x] `mise tasks | grep '^semver'` → present; `[tasks.semver]` at mise.toml line 104.
- [x] `.claude/context/specs/ci-cd.md` Semver checkbox flipped to `[x]` (line 417), reworded for the
    informational state.
- [x] Local run confirmed independently: `cargo semver-checks check-release -p iscc-lib` finishes in
    ~7s and reports **2 major / 0 minor checks failed** — exactly the expected post-0.4.0
    `pub(crate)` narrowing (`module_missing` for cdc/conformance/minhash/simhash/utils +
    `function_missing` for their fns). Confirms the gate is wired; `continue-on-error` keeps CI
    green. (Note: piping the command to `tail`/`head` masks its non-zero exit — read the "Summary …
    N major checks failed" line instead.)
- [ ] Next push: new `Semver` job appears and the existing 16 jobs stay green — deferred to the CI
    run (cannot verify pre-push). Job is non-blocking by design.

**Issues found:**

- (none) — clean, in-scope CI/config/doc-only change.

**Codex review:** Clean — "The commit adds the intended non-blocking cargo-semver-checks job and
local mise task without affecting existing build/test jobs. I did not identify a discrete regression
in the changed files." No actionable findings.

**Next:** Three `normal`/[human] backlog items remain. Good candidates for define-next, roughly by
self-containment:

1. **`iai-callgrind` perf gate** — Linux CI job with instruction-count benches for the hot
    `gen_*_v0`/hashing/CDC/MinHash paths + a committed baseline; fail on >10% regression. Pairs
    with the semver gate as the second v1.0.0 hardening gate.
2. **Rust coverage + CRAP gate** — `cargo llvm-cov` LCOV + `cargo crap`, phased (artifact →
    report-only → `--fail-regression`). Larger, multi-phase; could be scoped to just phase 1.
3. **PyO3 0.23 → 0.29 security migration** — clears two RustSec advisories; six-minor jump with
    breaking changes per minor. Scoped to `crates/iscc-py/` only (core has no PyO3 dep). Split
    per-minor (start 0.23 → 0.24); likely too large for one CID step otherwise.

The semver gate becoming *enforcing* is a deliberate one-line follow-up (drop `continue-on-error`)
tied to the v1.0.0 cut — tracked by the `low` v1.0.0 issue and the still-unchecked `rust-core.md`
line 372 checkbox. Do not flip either until v1.0.0.

**Notes:**

- No source, API, conformance, or performance surface touched — CI config + mise task + spec doc
    only.
- The CI job uses the maintainer-blessed wrapper action (installs `cargo-semver-checks` via binstall
    internally) per the spec recommendation. Requires the `dtolnay/rust-toolchain@stable` step it
    already has.
- **Pre-push gate fix applied:** the first `git push` was rejected because the pre-push mdformat
    hook (`--wrap 100 --number`, isolated `mdformat-mkdocs` env) reflows files changed in the push
    range, and define-next had committed a non-conforming `next.md`, `define-next/MEMORY.md`, and
    `define-next/MEMORY-archive.md` (wrong wrap width + a misindented fenced YAML block). I
    reformatted those three files mechanically (no semantic change — exactly what the hook produces)
    and amended this review commit so the batch could push. Root cause: define-next does not format
    its own output. Added a learning so define-next runs `uv run mdformat --wrap 100 --number` (or
    `mise run format`) before committing. `iterations.jsonl` is runner-owned and never staged here.
