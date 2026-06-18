## 2026-06-18 — iai-callgrind perf gate slice 2b — committed Ir baseline + >10% regression gate

**Done:** Closed the performance-regression gate (issue #3, slice 2b): added a stdlib-only
`scripts/iai_regression.py` (check + update modes), committed a CI-sourced `.iai-baseline.json` (16
Ir entries), wired an enforcing `Check perf regression` step into the CI `perf` job, and added
`bench:iai:baseline` / `bench:iai:check` mise tasks. The `perf` job now fails when any benchmarked
path's instruction count (Ir) exceeds the committed baseline by more than 10%.

**Files changed:**

- `scripts/iai_regression.py` (new): parses each bench's `summary: <Ir> ...` from `target/iai/`,
    keyed by leaf dir name (`<bench_fn>.<bench_id>`). `--check` (default) compares the current run
    to `.iai-baseline.json` and exits 1 if any shared bench is `> baseline * 1.10`; benches only in
    the run warn but never fail. `--update` rebuilds the baseline from `--from-dir` (default
    `target/iai/`). Gates only on Ir (the deterministic metric); cache/miss counts are ignored.
- `.iai-baseline.json` (new, committed, repo root, NOT gitignored): built from the CI `iai-baseline`
    artifact of run 27746693860 (`1463edb`, the first green Perf run after the strip fix) via
    `--update --from-dir /tmp/ci-iai`, so it matches the rustc the CI gate measures with. Shape:
    `{"metric":"Ir","tolerance_pct":10.0,"benches":{<id>: <Ir>, ...}}`.
- `.github/workflows/ci.yml`: added `Check perf regression` step
    (`python3   scripts/iai_regression.py --check`) after the zero-collection guard; added
    `if: always()` to the upload step so the artifact survives a regression failure. The guard step
    is retained.
- `mise.toml`: added `bench:iai:baseline` (depends `bench:iai`, runs `--update`) and
    `bench:iai:check` (depends `bench:iai`, runs `--check`).
- `.claude/context/specs/rust-core.md`: flipped both perf checkboxes (committed baseline + >10%
    regression) to `[x]`.
- `.claude/context/specs/ci-cd.md`: flipped the Perf-job checkbox to `[x]`.

**Verification:**

- [x] `mise run bench:iai` populates `target/iai/` with non-zero `summary:` lines (16 benches).
- [x] `mise run bench:iai:baseline` regenerates `.iai-baseline.json` with exactly 16 Ir entries,
    valid JSON (verified, then restored the committed file to the CI-sourced values — `diff` clean).
- [x] Self-consistency: `--check` against a baseline built from the same local run → 0% delta, exit
    0\.
- [x] Failure path: tampering one baseline Ir down 50% → `--check` exits 1 and names the regressed
    bench (`bench_cdc_chunks.bytes_1m ... +100.00% REGRESSION`).
- [x] Missing baseline → clear error + exit 1. Bench only in run (not baseline) → warning, exit 0.
- [x] Local run `--check` against the committed CI baseline passes (max delta −0.46%, all within
    10%) — CI and local rustc counts agree closely.
- [x] `.iai-baseline.json` not gitignored (`git check-ignore` exits 1).
- [x] `perf` job has both the `Check perf regression` step and the
    `Assert non-zero instruction   collection` guard; YAML valid.
- [x] `ruff check` + `ruff format --check` + `ty check` clean on the new script.
- [x] `mise run check` — all 15 pre-commit hooks Passed.
- [x] `cargo test -p iscc-lib` — 22 tests + 1 doctest pass (no harness change).
- [ ] **CI-only (confirm next cycle):** the post-push `Perf` job's `Check perf regression` step
    passes against the committed baseline (counts within 10% on the CI runner).

**Next:** Issue #3 is now fully addressed (slice 2a + 2b). update-state should confirm the post-push
`Perf` run is green with the regression step passing, then close issue #3. Remaining `normal` issues
deliberately left untouched (CRAP `--fail-above 30` and cargo-deny/audit gate — both flagged as
human-review-required spec amendments). After CI confirmation, the natural next target area is
v1.0.0 release prep (the `semver` and crate-version checkboxes in rust-core.md stay `[ ]` until the
1.0.0 cut).

**Notes:**

- **Committed baseline is CI-sourced, not local.** Local rustc (1.96.0) and CI `stable` produce Ir
    counts that differ by < 1% here, but per next.md the committed file is built from the CI
    artifact so the gate compares like-with-like. To refresh after an accepted
    regression/improvement: download the latest green `iai-baseline` artifact and run
    `python3 scripts/iai_regression.py   --update --from-dir <dir>`, or use
    `mise run bench:iai:baseline` as a local-rustc fallback (noted in the task comment).
- **Gate is Ir-only and intersection-only.** New benches added to the harness without a baseline
    refresh will warn (not fail) until the baseline is regenerated — by design, matching how
    `--fail-regression` treats new CRAP entries. A baseline refresh after adding benches is the
    intended workflow.
- **`bench:iai:baseline` re-benches locally**, so running it overwrites the CI-sourced committed
    file with local-rustc values. That is acceptable for a deliberate reviewed refresh, but the
    preferred path for the *committed* baseline is the CI artifact (documented in the script
    docstring and the task comment). I restored the CI-sourced file after testing the task.
- The 10% tolerance lives in the baseline JSON (`tolerance_pct`) and the spec; the script reads it
    from the file (falls back to 10.0). No magic number duplicated in CI.
