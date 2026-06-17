## 2026-06-17 — Review of: Release the GIL during Python hashing (`py.allow_threads`)

**Verdict:** PASS

**Summary:** The advance agent wrapped the pure-Rust CPU-bound compute in `py.allow_threads(...)` at
all 7 call sites in `crates/iscc-py/src/lib.rs` (4 one-shot byte-data functions + 3 streaming
`update()` methods) and added `tests/test_gil.py` with 7 deterministic concurrency-correctness
tests. The change is tightly scoped, output is byte-identical, no Python-facing signature changed,
and all verification passes. Issue #39 is fully resolved.

**Verification:**

- [x] `cargo build -p iscc-py` compiles — clean (`Ungil`/`Send` bounds satisfied at every site)
- [x] `cargo clippy -p iscc-py -- -D warnings` — clean
- [x] `cargo fmt -p iscc-py --check` — clean
- [x] `grep -c allow_threads crates/iscc-py/src/lib.rs` → 7 (matches the 7 required sites)
- [x] `maturin develop --release` succeeds; `pytest tests/` → **286 passed** (279 existing + 7 new)
- [x] `ruff check` / `ruff format --check` on `tests/test_gil.py` — clean
- [x] `ty check crates/iscc-py` — clean; `.pyi` stubs still show `update(self, data: bytes)`
    (injected `py` invisible to Python, confirming non-breaking)
- [x] `mise run check` — all hooks pass for the advance files. The only `mdformat` "Failed" is the
    recurring reformat of out-of-scope CID context files (`next.md`, define-next memory) — reverted,
    not a regression (real commits only run hooks on staged files)

**Issues found:**

- (none) — implementation matches next.md exactly, no scope creep, no gate circumvention. Soundness
    of the released `&[u8]` borrow is correct: the `__init__.py` wrapper coerces `bytearray`/
    `memoryview` to immutable `bytes` before calling the private `_lowlevel`, so no public path
    hands a mutable buffer to the GIL-released borrow.

**Codex review:** No correctness regression found. Confirmed the GIL is released only around
Rust-side work while Python object creation (`PyDict` construction in `finalize()`/one-shot dict
build) remains GIL-held. No action required.

**Next:** With #39 closed, the highest-value remaining backlog item is the **npm
`optionalDependencies` fix (#38)** — a confirmed install-breaking bug for downstream consumers with
a pre-recorded fix decision (drop the `napi prepublish -t npm` injection at `release.yml:378`, keep
the bundled-package model; `files: ["*.node"]` already bundles all binaries). Touch points:
`.github/workflows/release.yml` + verifying `crates/iscc-napi/package.json` ships no
`optionalDependencies`. After that, the **PyO3 0.23 → 0.29 security migration** (clears two RustSec
advisories) is the next normal-priority item — note it is a 6-minor-version jump and should be
staged incrementally per the PyO3 migration guide.

**Notes:**

- **Spec updated**: `.claude/context/specs/python-bindings.md` → "GIL Release During Hashing"
    checkboxes marked `[x]` (authorized — #39 is `[human]`-sourced with a `**Spec:**` field). The
    ~2× throughput criterion is mechanistically enabled by the GIL release but intentionally NOT
    CI-gated (non-deterministic on shared runners), as next.md scoped.
- **Recurring CID friction (process, not code)**: `mise run check`/`format` run `prek --all-files`,
    which reformats `next.md` and `define-next/MEMORY.md` via `mdformat` every cycle because
    define-next writes them non-conforming. This is cosmetic and does not affect real commits (hooks
    only touch staged files). Define-next could run `mdformat` on these when authoring to stop the
    churn — worth a meta-improve proposal if it keeps recurring.
- No `iterations.jsonl` staging (runner-owned). Verdict is PASS → pushing the full define-next →
    advance → review batch.

**Push note:** The first `git push` was rejected by the pre-push `mdformat` hook. All substantive
gates passed (Rust linting, Rust tests, Python type checking, Ruff security/complexity, Python tests
— all "Passed"); the only failure was `mdformat` reformatting two **pre-existing process files**
committed non-conforming by the define-next agent — `.claude/context/next.md` and
`.claude/agent-memory/define-next/MEMORY.md`. The advance code (`lib.rs`, `test_gil.py`) was not
implicated. The diff is pure line-rewrapping (17 reflowed lines, no semantic change). Per the
gate-integrity principle (address the root cause; never weaken a gate), I normalized those two files
with `mdformat` and included them in the review commit so the gate passes at the root — a
non-semantic formatting fix, not a content change to the work package, so the PASS verdict stands.
**Meta-improve candidate:** define-next should run `mdformat` on `next.md` and its own memory before
committing, to stop this recurring pre-push rejection (the non-conforming files sit in the unpushed
commit range, so leaving them risks a stuck push loop).
