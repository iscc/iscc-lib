# Handoff

## 2026-07-28 — Bump criterion 0.7 → 0.8 (dev-dependency, bench harness only)

**Done:** Pinned `criterion = { version = "0.8", features = ["html_reports"] }` in the root
workspace manifest and refreshed `Cargo.lock` via `cargo update -p criterion` (resolved 0.8.2). The
four-line "authorized (pending bump)" comment was replaced with the evergreen dev-dependency/MSRV
rationale; the `uniffi` comment block is untouched. Zero bench-source changes were needed —
`benches/benchmarks.rs` compiles and runs unmodified under 0.8.

**Files changed:**

- `Cargo.toml`: criterion pin 0.7 → 0.8; stale authorization comment replaced with evergreen note
    (dev-dep floor ≠ published MSRV). `rust-version = "1.85"` untouched (line 17).
- `Cargo.lock`: criterion 0.7.0 → 0.8.2, criterion-plot 0.6.0 → 0.8.2; new transitives alloca 0.4.0,
    page_size 0.6.0, winapi 0.3.9 + 2 gnu shims (windows-only). 47 insertions, 4 deletions — scoped
    to criterion's resolution, nothing else moved.
- `.claude/agent-memory/advance/MEMORY.md` + `deps-refresh.md`: criterion marked DONE iter 171;
    uniffi 0.32 is the last remaining authorized major.

**Verification:** (all six Done-When checks, each from a captured exit code)

- [x] `grep 'criterion = '` → `version = "0.8"`; `Cargo.lock` entry `name = "criterion"` /
    `version = "0.8.2"`
- [x] `grep -n 'rust-version = "1.85"' Cargo.toml` → line 17 matches
- [x] `cargo bench --no-run` → exit 0 (exact CI `bench` job command)
- [x] `cargo bench -p iscc-lib --bench benchmarks -- --test` → exit 0, 18 `Testing …` / 18 `Success`
    lines — every bench body executed once under the 0.8 harness
- [x] `mise run lint` → exit 0 (clippy `--workspace --all-targets -D warnings`: no fresh
    deprecations)
- [x] `mise run test` → exit 0 (cargo tests + 441 pytest passed); `mise run audit` → exit 0
    (`advisories ok, bans ok, licenses ok, sources ok` — cargo-deny 0.19.9 via binstall, alloca's
    MIT already allow-listed, no `deny.toml` change)
- [x] `mise run check` → exit 0, 18 hooks Passed

**Next:** `uniffi` 0.31 → 0.32 — the last authorized major and the last v0.6.0 release-gating item.
Read the 0.32 changelog first; if `crates/iscc-uniffi/src` needs edits, stop and re-scope per the
issues.md constraint. Regenerate both checked-in bindings (Swift + Kotlin) as a pure regeneration;
Kotlin verifies locally via Gradle, Swift only via the `swift` CI job on the pushed commit.

**Notes:**

- No hot-path change: criterion is the measuring harness, not measured code — no `iscc-lib` source
    line moved, so `.iai-baseline.json` / `.crap-baseline.json` were correctly left alone (iai
    benches don't link criterion at all).
- The `cargo report future-incompatibilities` note that appears during builds is pre-existing and
    unrelated: `proc-macro-error2 v2.0.1` via `iai-callgrind-macros` (confirmed with
    `cargo tree -i`; already recorded in agent memory, no fixed release exists upstream).
- next.md's implementation notes held exactly: `cc` was already in the lock (blake3 build-dep), so
    the only new unix-relevant crate is alloca; `html_reports` survives under the same name; the
    async-std drop is the sole 0.8 breaking change and is unused here.
