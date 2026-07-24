# Handoff

## 2026-07-24 — Rust direct-pin evaluation — bump criterion to 0.7, document held-back majors

**Done:** Completed slice 3 of the "Dependency review and refresh" issue: bumped the `criterion`
workspace pin from 0.5 to 0.7 (dev-only bench dependency), migrated the single deprecated
`criterion::black_box` import in `benchmarks.rs` to `std::hint::black_box`, and added inline
`# held:` comments documenting the deliberate hold-back reason next to each of the four held
workspace pins (`criterion` 0.8, `jni` 0.22, `magnus` 0.8, `uniffi` 0.32) plus a `# note:` on `pyo3`
about re-verifying `gil_used`/`py.detach` (issue #41) on future bumps.

**Files changed:**

- `Cargo.toml` (root): `criterion = { version = "0.7", ... }` (was `0.5`); 4 `# held:` comment
    blocks adjacent to the criterion/jni/magnus/uniffi pins; 1 `# note:` block above pyo3. No other
    pin text changed.
- `crates/iscc-lib/benches/benchmarks.rs`: import line only — `black_box` removed from the
    `use criterion::{...}` list, `use std::hint::black_box;` added (sorted before `std::io::Write`).
    All 30 `black_box(...)` call sites byte-identical.
- `Cargo.lock` (generated): criterion 0.5.1 → 0.7.0, criterion-plot 0.5.0 → 0.6.0; removed
    hermit-abi 0.5.2, is-terminal 0.4.17, itertools 0.10.5 (subtree shrank by 3 crates).

**Verification:** (all commands run this session, exit codes captured)

- `grep -A1 '^name = "criterion"' Cargo.lock` → `version = "0.7.0"`; no 0.5.x criterion remains.
- `grep -c '# held' Cargo.toml` → 4, each adjacent to its pin (criterion, jni, magnus, uniffi).
- `grep -c 'use std::hint::black_box' benchmarks.rs` → 1;
    `grep -c 'criterion::{[^}]*black_box' benchmarks.rs` → 0.
- `cargo bench --no-run` exit 0 (compiles `benchmarks.rs` + `iai_benches.rs` under criterion 0.7).
- `mise run lint` clean — clippy `--workspace --all-targets -D warnings` passes, no deprecation
    warnings; ruff "All checks passed!", 24 files already formatted.
- `mise run test`: `cargo test --workspace` exit 0 (15 suites, 0 failures — 270 iscc-lib unit +
    conformance, 85 ffi, 28+22 integration, 21 uniffi) + **314 pytest passed**.
- `mise run audit` exit 0 — "advisories ok, bans ok, licenses ok, sources ok" against the refreshed
    lockfile (criterion 0.7's clap 4.6/criterion-plot 0.6/itertools 0.13 subtree passed cargo-deny).
- `mise run bench:iai:check` exit 0 against the **unmodified** `.iai-baseline.json` — 16/16 within
    10% (max delta +1.96% on bench_mixed_code.two_codes; baseline files untouched per git status).
- `mise run check` exit 0 — all 15 pre-commit hooks pass, no file rewrites left in tree (taplo
    preserved all `# held:` comments).
- `git status --porcelain crates/iscc-lib/src crates/iscc-rb/src crates/iscc-jni/src` — empty; no
    library source touched.

**Next:** Continue the dependency-refresh issue with slice 4: per-binding manifests (napi
`package.json`, rb `Gemfile`/gemspec — rb_sys must match the `oxidize-rb/actions/cross-gem` Docker
tag, jni `pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`) — one small step each.
Then tooling pins (`mise.toml`, `.pre-commit-config.yaml`, GHA actions). The dedicated ruff 0.16
adoption step also remains open.

**Notes:**

- criterion resolved to 0.7.0 (latest in the 0.7 line). 0.8.2 exists but requires rustc 1.86 vs our
    declared `rust-version = "1.85"` — held and documented; MSRV raise is a human decision for the
    v1.0.0 cut.
- The `cargo bench --no-run` / `cargo test` future-incompat warning about `proc-macro-error2 v2.0.1`
    is **pre-existing** (verified: `git diff Cargo.lock` contains no proc-macro-error2 lines — it
    comes from the magnus/rb-sys subtree, untouched this slice). Worth a look in the Ruby slice.
- valgrind + iai-callgrind-runner + cargo-deny were already present in this container — no reinstall
    needed this session.
- `.claude/context/iterations.jsonl` shows as modified in the working tree (CID runner's log) — not
    staged per protocol.
