# Handoff

## 2026-07-24 — Fix CI — patch RUSTSEC-2026-0204 by bumping dev-only crossbeam-epoch to 0.9.20

**Done:** Ran `cargo update -p crossbeam-epoch`, bumping the lockfile from `0.9.18` to `0.9.20`
(exactly 1 package changed, no manifest edits). This clears the fresh advisory RUSTSEC-2026-0204
that was failing the enforcing `Audit (cargo-deny)` CI gate. No `deny.toml` change was needed and no
advisory suppression was added — the vulnerable version is simply gone from the graph.

**Files changed:**

- `Cargo.lock`: `crossbeam-epoch 0.9.18 -> 0.9.20` (regenerated via
    `cargo update -p crossbeam-epoch`; dry-run first confirmed it locks exactly 1 package)

**Verification:** (all from tool output this session)

- `grep -A2 'name = "crossbeam-epoch"' Cargo.lock` shows `version = "0.9.20"`; the string `0.9.18`
    appears 0 times anywhere in `Cargo.lock`.
- `cargo tree -i crossbeam-epoch` shows only `v0.9.20`, reached solely via
    `criterion → rayon → rayon-core → crossbeam-deque` under `iscc-lib`'s `[dev-dependencies]`;
    `cargo tree -i crossbeam-epoch -e no-dev` prints "nothing to print" — the crate never ships in
    any published artifact.
- `grep -c 'RUSTSEC-2026-0204' deny.toml` returns `0` — no suppression added; the two pre-existing
    dev-only `ignore`s (`RUSTSEC-2025-0141`, `RUSTSEC-2026-0173`) are untouched.
- `cargo bench -p iscc-lib --no-run` succeeds (all three bench executables compile).
- `cargo test -p iscc-lib`: 4 test suites, all `test result: ok` (269 + 28 + 22 + 1 = 320 tests, 0
    failures).
- **Supply-chain gate green locally:** installed cargo-deny 0.19.9 via cargo-binstall in this
    session; `cargo deny check` exits 0 — `advisories ok, bans ok, licenses ok, sources ok`
    (redirected to a file first per the pager-exit-code gotcha, exit code checked directly).
- `mise run check`: all 15 pre-commit hooks pass.

**Next:** CI should be green on the next develop push (the local `cargo deny check` green is
authoritative for the Audit gate since cargo-deny reads Cargo.lock + metadata, not artifacts; a
glance at the Actions tab confirms). After that, per state.md, the v0.6.0 backlog opens: one of
Python text/video GIL (#41), WASM simd128 (#42), Go ISCC-IDv1 (#43), aarch64 wheels (#49) — one per
iteration. The v1.0.0 release cut + semver-gate flip remain human-held (Titusz).

**Notes:** No surprises. The work package's verified facts held exactly: patch-level semver bump,
dev-only reachability, single-package lock change. Working tree also carries a
`.claude/context/iterations.jsonl` modification from the CID runner — not staged (context file other
than handoff.md). Unrelated pre-existing observation: `cargo test` emits a future-incompatibility
warning for `proc-macro-error2 v2.0.1` (dev-only iai-callgrind dep, already covered by the
`RUSTSEC-2026-0173` ignore) — informational, no action taken.
