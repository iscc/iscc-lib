# Handoff

## 2026-07-24 — Review of: Refresh Python `uv.lock` (dependency-refresh slice 2)

**Verdict:** PASS

**Summary:** The advance agent regenerated the root Python `uv.lock` via `uv lock --upgrade` (40
packages bumped — notably iscc-core 1.2.2→1.3.0, ty 0.0.18→0.0.63, maturin 1.14.1, prek 0.4.11,
pytest 9.1.1, zensical 0.0.51) with a single documented `ruff<0.16` hold-back in `pyproject.toml`.
No Rust or Python source file was touched. All quality gates (test, lint, check, pre-push, docs) are
green on the refreshed lockfile; the hold-back is legitimate scope discipline, not gate weakening.

**Verification:**

- [x] `uv lock --check` exits 0 — "Resolved 91 packages", committed lock consistent with
    `pyproject.toml`.
- [x] `uv sync --group dev` exits 0 and
    `uv run maturin develop --manifest-path   crates/iscc-py/Cargo.toml` builds the abi3 extension
    (maturin 1.14.1). NOTE: `uv sync` uninstalls the editable `iscc-lib` — the develop step is
    required before pytest can import.
- [x] `mise run test` passes — Rust workspace + **314 pytest** (incl. conformance vs `data.json` and
    iscc-core 1.3.0 comparative benches).
- [x] `mise run lint` clean — `ruff check` "All checks passed!", `ruff format --check` "24 files
    already formatted", cargo fmt/clippy unchanged.
- [x] `mise run check` — all 15 pre-commit hooks pass, no file rewrites left in the tree.
- [x] Pre-push hooks (`uv run prek run --all-files --hook-stage pre-push`) all pass — **"Python type
    checking" green on ty 0.0.63** (the biggest risk of the 0.0.18→0.0.63 jump), plus clippy, Rust
    tests, Ruff S/C901, pytest.
- [x] `uv run zensical build` exits 0 (0.0.51) and `uv run python scripts/gen_llms_full.py` exits 0
    (22 pages, 268195 bytes).
- [x] Change set is `uv.lock` + `pyproject.toml` (single documented hold-back) + context/memory only
    — **no Rust or Python source file modified**. (Review added one minor doc-anchor fix — see
    Issues found.)

**Gate integrity:** Clean. The `ruff<0.16` pin is NOT gate weakening — it holds back a tool version
to defer adoption of ruff 0.16's *new* default lint rules; ruff 0.15.22 enforces the exact same rule
set previously locked (0.15.2). No lint rule disabled, no test skipped, no threshold lowered, no
hook removed. **Verified genuine**, not a mask: `uvx ruff@0.16.0 check .` reproduced exactly 104
errors (72 in `_lowlevel.pyi`), matching the handoff claim precisely. The pin carries an inline
`# held:` comment and a deferred-adoption follow-up. next.md pre-authorized this exact contingency
for a lockfile slice.

**Issues found:**

- **Fixed during review (minor doc):** the pre-existing broken intra-page anchor at
    `docs/howto/c-cpp.md:8` — `[C++ section](#c-wrapper-iscc-hpp)` — had an extra hyphen; zensical
    0.0.51's new anchor checker surfaced it (non-fatal, exit 0). The actual generated slug is
    `c-wrapper-iscchpp`. Corrected the link; `zensical build` now reports "No issues found". This
    was a pre-existing defect (not caused by the lockfile refresh), fixed as a behavior-neutral
    minor fix.

**Codex review:** Confirms the refreshed lockfile is consistent with `pyproject.toml`, the ruff
hold-back is correctly represented, and linting, type checking, docs generation, and the full Python
test suite (3.10 + 3.14) pass. No actionable findings.

**Next:** Continue the "Dependency review and refresh" issue with the next slice. Slices 1 (Rust
`Cargo.lock`) and 2 (Python `uv.lock`) are done. Remaining, in rough order of value:

1. **Rust direct-pin evaluation** — review workspace `Cargo.toml` majors (uniffi, pyo3, criterion,
    iai-callgrind, magnus, jni, napi); bump the safe ones, document each deliberate hold-back next
    to its pin (pyo3 stays pinned per #41 `gil_used`/`py.detach`; uniffi 0.32 needs Swift/Kotlin
    regen).
2. **Per-binding manifests** — napi `package.json`, rb `Gemfile`/gemspec (rb_sys must match the
    `oxidize-rb/actions/cross-gem` Docker tag), jni `pom.xml`, kotlin `build.gradle.kts`, dotnet
    `.csproj`, go `go.mod` — one small step each.
3. **Tooling pins** — `mise.toml`, `.pre-commit-config.yaml`, GHA action versions.

Separately, a small dedicated step should **adopt ruff 0.16**: `ruff check --fix` (60 auto-fixable),
hand-fix the rest (mostly `_lowlevel.pyi` stub-style PIE790/PYI048/RUF022), then drop the
`ruff<0.16` pin. Do NOT cut v1.0.0 or flip the `Semver` gate — both held by Titusz.

**Notes:**

- **iscc-core 1.3.0 matches the vendored conformance vectors** (`data.json` v1.3.0); comparative
    tests confirm zero output drift. ISO 24138 is frozen, so this is expected.
- **Guard for future refresh/source slices:** the CRAP regression gate is CI-only (not in
    `mise run check`) — any source change adding a branch/loop to a covered function must refresh
    `.crap-baseline.json` in the same step. (Not triggered this iteration — no source touched.)
- **Verification gotcha (recorded in review memory):** `uv sync` uninstalls the editable `iscc-lib`
    extension; `uv run maturin develop` must run before pytest or the import fails.
- Watch the enforcing `Audit (cargo-deny)` gate — a fresh live advisory can flip it red on any push
    with no code change (prefer `cargo update -p <crate>` over a `deny.toml` ignore).
