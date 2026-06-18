## 2026-06-18 — Review of: Migrate PyO3 0.28 → 0.29 (FINAL hop — closes issue #1)

**Verdict:** PASS

**Summary:** The advance agent bumped the workspace `pyo3` pin `0.28` → `0.29` and regenerated
`Cargo.lock` (all 5 pyo3 crates → 0.29.0; `pyo3-macros-backend` dropped its internal
`pyo3-build-config` dep). No source edits were required — `crates/iscc-py/src/lib.rs` is unchanged
and the explicit `gil_used = true` is preserved. This closes the incremental 0.23→0.29 migration
arc: 0.29.0 ships both targeted RustSec advisory fixes, so the published wheel no longer carries
vulnerable PyO3 code. Clean, minimal, in-scope.

**Verification:** (every criterion from next.md)

- [x] `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.29"` with `abi3-py310`
- [x] `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.29.0"`; all pyo3 crates resolve to
    0.29.0, no pyo3 `< 0.29` entry remains
- [x] `cargo build -p iscc-py` → exit 0 (Finished)
- [x] `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean
- [x] `cargo fmt --all --check` → exit 0
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` → exit 0, `cp310-abi3` wheel installed
- [x] `uv run pytest` → 286 passed, 1 warning (the known pre-existing iscc_core Pydantic-V1/Py3.14
    UserWarning — not our code)
- [x] `cargo tree -p iscc-py -i pyo3` → single `pyo3 v0.29.0`, no duplicates
- [x] `grep -n 'gil_used = true' crates/iscc-py/src/lib.rs` → present (lib.rs:697; default not
    regressed)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (pre-push defense)
- [x] `mise run check` → all 15 pre-commit hooks Passed
- [x] `uv run prek run --all-files --hook-stage pre-push` → all 16 pre-push gates Passed (clippy,
    cargo test, ty check, Ruff S/C901, Ruby lint, pytest, mdformat)

**Silent-behavior audit:** Confirmed the macros-backend `gil_used` default handling did NOT flip
again this hop (the 0.28 default stays `false`; our explicit `gil_used = true` keeps GIL
protection). Read the 0.29.0 CHANGELOG — the one behavioral item (exception enrichment via
`PyErr::add_note` instead of replacing `TypeError`, #5349) does not affect our tests; "Remove
0.27-deprecated functionality" (#6068) is a no-op since we already migrated off
`downcast`/`downcast_into_unchecked`.

**Issues found:**

- (none affecting correctness) — diff touches only `Cargo.toml` + `Cargo.lock`, no scope creep, no
    quality-gate circumvention across all 4 unpushed commits.
- Gap (filed as new `[review]` issue, not blocking): the PyO3 advisory clearance could only be
    confirmed by the mechanical proxy "lockfile resolves a single `pyo3 0.29.0`" — `cargo deny` /
    `cargo audit` are absent from the devcontainer **and** CI, even though `notes/07` mandates
    `cargo deny check` "Run in CI" with a workspace-root `deny.toml`. No `deny.toml`, no CI job, no
    `mise` task exists.

**Issue management:**

- Deleted issue #1 ("Update PyO3 to latest release") — resolved this iteration. `[human]`-sourced,
    no `**Spec:**` field, so no spec update needed.
- Added `[review]` `normal` issue "Wire up `cargo deny`/`cargo audit` supply-chain gate" with a
    `**Spec:**` field (ci-cd.md) + HUMAN REVIEW REQUESTED — the requirement currently lives only in
    the design notes, not the CID specs.
- Stale sweep: remaining issues (#2 CRAP `--fail-above`, #3 iai-callgrind, v1.0.0 release, docs
    logos) all verified still open against state.md — none stale.

**Codex review:** Clean. "The commit only bumps PyO3 and refreshes associated lockfile entries, with
no source changes required. The updated workspace builds successfully … I did not find any
introduced correctness issues." No actionable findings.

**Next:** The PyO3 migration arc is done. **Heads-up for define-next:** all three remaining `normal`
issues are constrained — #2 (CRAP `--fail-above 30`) and the new supply-chain audit gate both carry
HUMAN REVIEW REQUESTED on the spec, and #3 (iai-callgrind) is blocked by valgrind being unavailable
in the devcontainer. The most self-contained unblocked candidate is the supply-chain audit gate: a
`deny.toml` + `Security audit` CI job + `mise run audit` task is concrete, mechanical work that
directly hardens the project and would have let this very iteration tool-confirm the advisories —
BUT it needs the spec amendment approved first (HUMAN REVIEW REQUESTED). If the human declines to
unblock any of these, the loop is at a natural pause point pending human direction (v1.0.0 cut is
explicitly human-driven). define-next should not start v1.0.0 prep or flip the `Semver` gate
autonomously.

**Notes:**

- Single CID loop confirmed running (`ps aux`: one `mise run cid:run`) — no concurrency race; the
    4-commit batch (log-104 + update-state + define-next + advance) pushes as a clean fast-forward
    (`@{upstream}...HEAD` = `0 4`).
- No core API change, no perf-path touched: iscc-py is a binding crate, pyo3 is its internal dep.
    pytest benches unchanged (iscc-lib ~3.4–4.5× faster than iscc-core on video/mixed; ~1.3× text).
- mdformat passed cleanly this cycle on both pre-commit and pre-push stages — define-next's
    `next.md`/memory were already 100-col conforming, so no context-file reformatting was needed.
- `cargo audit`/`cargo deny` verified absent (`command -v` → not found); see the new supply-chain
    issue for the wiring follow-up.
