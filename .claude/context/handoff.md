## 2026-06-18 — Wire up the `cargo-deny` supply-chain audit gate

**Done:** Added a workspace-root `deny.toml` (advisories + licenses + bans + sources policy), an
enforcing `Audit (cargo-deny)` CI job running `cargo deny check`, and a `mise run audit` task.
Installed cargo-deny 0.19.9 locally and iterated `cargo deny check` to exit 0 against the real
workspace graph; this surfaced two genuine supply-chain issues that I fixed (see Notes).

**Files changed:**

- `deny.toml` (new): `[graph] all-features = true`; `[advisories] yanked = "deny"` with two dev-only
    unmaintained advisories in `ignore` (justified); `[licenses] allow = [...]` (13 standalone
    licenses incl. `MPL-2.0`, `BSL-1.0`, `Unicode-3.0`, `Zlib`) + `private = { ignore = true }`;
    `[bans] multiple-versions = "warn"`; `[sources] unknown-registry/unknown-git = "deny"`.
- `.github/workflows/ci.yml`: appended the `audit` job after `coverage` — checkout → rust-toolchain
    → rust-cache → `taiki-e/install-action` `cargo-deny@0.19.9` → `cargo deny check`. No
    `continue-on-error` (enforcing).
- `mise.toml`: added `[tasks.audit]` (`run = "cargo deny check"`) under a new "Supply-chain audit"
    block.
- `Cargo.lock`: bumped the wasm-bindgen family (js-sys, wasm-bindgen{,-futures,-macro,
    -macro-support,-shared,-test,-test-macro,-test-shared}, web-sys) off the **yanked**
    `wasm-bindgen 0.2.111` / `js-sys 0.3.88` to current versions (`cargo update -p` only — no
    manifest changes). Required to make `yanked = "deny"` green.
- `.claude/context/specs/ci-cd.md`: flipped the line-448 Audit "verified when" box to `[x]` (local
    `cargo deny check` exits 0 — see Notes on the local-vs-CI flip decision).

**Verification:**

- `cargo deny check` exits **0**: `advisories ok, bans ok, licenses ok, sources ok`. 6 non-failing
    `multiple-versions = "warn"` duplicate warnings (libloading, rustc-hash, thiserror,
    thiserror-impl, windows-sys, winnow) — expected, do not fail the gate.
- `python3 -c "import tomllib; tomllib.load(open('deny.toml','rb'))"` → ok; traps grep (`MPL-2.0` +
    `BSL-1.0` + `private`) → ok.
- `mise tasks | grep audit` → registered; `uv run python` yaml assertion → audit job is enforcing,
    runs `cargo deny check`, name `Audit (cargo-deny)`.
- `mise run check` → all 15 pre-commit hooks pass. Pre-push stage
    (`prek run --all-files   --hook-stage pre-push`) → all pass incl. Rust clippy, Rust tests,
    pytest, Ruby lint, ty check (confirms the wasm-bindgen bump breaks nothing).

**Next:** This was the last autonomous v1.0.0-hardening work package. Remaining issues are
human-gated: cut v1.0.0 + flip the `Semver (cargo-semver-checks)` gate to enforcing (held by
Titusz), plus 2 `low` `[human]` docs-logo items. The loop should flag **HUMAN REVIEW REQUESTED**
(per the human-handoff-vs-IDLE learning) rather than manufacture churn. A reasonable follow-up
(separate work package) is the deferred supply-chain extras: Dependabot, `cargo audit`/`npm audit`
steps, signed tags — all explicitly out of scope here.

**Notes:**

- **Cargo.lock is a 4th changed file beyond the 3 source files (deny.toml, ci.yml, mise.toml).** It
    is a generated lockfile, not hand-edited source, and the update was *forced by the gate*: with
    `yanked = "deny"`, `wasm-bindgen 0.2.111` and `js-sys 0.3.88` are yanked. The clean fix is
    `cargo update` to non-yanked versions, not loosening `yanked` to `warn`.
    `wasm-bindgen-test   0.3.61` pinned `wasm-bindgen =0.2.111` exactly, so the whole family had to
    move together. Diff is confined to those 10 crates; no manifest/version requirements changed. I
    treated this like the committed `.crap-baseline.json` / `.iai-baseline.json` artifacts —
    necessary generated state, not a source-scope violation.
- **Two unmaintained advisories ignored, with justification:** `RUSTSEC-2025-0141` (bincode 1.x) and
    `RUSTSEC-2026-0173` (proc-macro-error2) both come *only* from `iai-callgrind` — a dev-only bench
    dependency (the perf gate). No safe upgrade is available and neither ships in any published
    artifact, so exposure is nil. This follows the next.md instruction to `ignore` specific
    unfixable advisory IDs rather than loosen the whole advisory class (`unmaintained` defaults to
    deny under cargo-deny config v2). If `iai-callgrind` later drops these, the `ignore` entries can
    be removed.
- **Schema:** used the modern cargo-deny config v2 (NOT the outdated `notes/07` sketch). Under v2,
    vulnerabilities/unmaintained deny by default and unlisted licenses deny by default — so there is
    no `vulnerability =`/`unlicensed =` key; only `yanked = "deny"` needed an explicit bump.
- **Checkbox flip decision (heads-up for review):** the previous reviewer's handoff assumed
    cargo-deny could NOT be installed in the devcontainer and said to flip the box only after CI
    green. That turned out false — `cargo install cargo-binstall` +
    `cargo binstall cargo-deny@0.19.9` works in the devcontainer (binstall compile ~5.5 min, then
    prebuilt binary). next.md's Scope explicitly authorizes flipping the box "ONLY if you verify
    `cargo deny check` green locally", which I did against the real Cargo.lock. cargo-deny reads
    Cargo.lock + crate metadata (not compiled artifacts), so the CI rustc version is irrelevant to
    the advisory/license/source verdict. If you prefer strict post-push confirmation, the box can be
    reverted to `[ ]` until the green CI run, but the local result is authoritative for this gate.
- Pure CI/config/lockfile + doc change — no Rust source, no public API, no hot path touched, so
    benches were correctly skipped.
