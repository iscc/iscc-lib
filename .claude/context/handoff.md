# Handoff

## 2026-07-24 — Review of: Fix CI — patch RUSTSEC-2026-0204 by bumping dev-only crossbeam-epoch to 0.9.20

**Verdict:** PASS

**Summary:** The advance agent bumped the single dev-only lockfile entry `crossbeam-epoch`
`0.9.18 → 0.9.20` via `cargo update -p crossbeam-epoch`, clearing the fresh advisory
RUSTSEC-2026-0204 that was reddening the enforcing `Audit (cargo-deny)` gate. Exactly the scoped
root-cause fix: no `deny.toml` suppression, no manifest edit, no unrelated deps touched. Locally
`cargo deny check` exits 0 and all quality gates pass.

**Verification:**

- [x] `Cargo.lock` pins the patched version — `grep -A2 'name = "crossbeam-epoch"'` shows
    `version = "0.9.20"`; `0.9.18` appears 0 times in `Cargo.lock`; diff is exactly 1 package
    (version + checksum lines only).
- [x] `cargo tree -i crossbeam-epoch` shows only `v0.9.20`, reached solely via
    `criterion → rayon → rayon-core → crossbeam-deque` under `[dev-dependencies]`; `-e no-dev`
    prints "nothing to print" — never ships in any published artifact.
- [x] No advisory suppression added — `grep -c 'RUSTSEC-2026-0204' deny.toml` returns `0`; the two
    pre-existing dev-only ignores untouched.
- [x] Benches compile — `cargo bench -p iscc-lib --no-run` built all three bench executables (lib,
    benchmarks, iai_benches).
- [x] `cargo test -p iscc-lib` — 320 tests (269+28+22+1) + doctest all pass, 0 failures.
- [x] Supply-chain gate clean — `cargo deny check` exits 0
    (`advisories ok, bans ok, licenses ok, sources ok`). cargo-deny 0.19.9 present in devcontainer.
- [x] `mise run check` — all 15 pre-commit hooks pass (no reformatting needed this cycle).
- [x] No gate circumvention across all 3 unpushed commits (`@{upstream}..HEAD`): only `Cargo.lock` +
    context/memory/handoff files changed. Every `RUSTSEC`/`continue-on-error` grep hit is
    documentation prose in context/memory files, not gate-weakening config.

**Issues found:**

- (none)

**Codex review:** Clean. "The lockfile-only dependency bump is valid, preserves the declared Rust
MSRV, and clears the advisory without changing the production dependency graph. Cargo metadata,
cargo-deny, and the full workspace all-targets/all-features check pass." No actionable findings.

**Next:** CI should go green on the next develop push (local `cargo deny check` green is
authoritative for the Audit gate — it reads Cargo.lock + metadata, not artifacts). With CI green,
the **v0.6.0 backlog opens** — one `normal` `[human]` work package per iteration, each spec'd:

- #41 Release GIL for Python text/video compute paths (`py.detach`)
- #42 Enable WASM `simd128` in the `@iscc/wasm` release build
- #43 Go bindings: experimental ISCC-IDv1 encode/decode
- #49 Restore linux/aarch64 Python wheels
- Dependency review and refresh across the project

**Recommended first pick: #41 (Python text/video GIL)** — it is the most self-contained and
lowest-risk: a single-file change in `crates/iscc-py/src/lib.rs` that mirrors the already-landed
data/instance/image/sum `py.detach` pattern, conformance-neutral (output unchanged), with a clear
documented caveat (video detach window must open strictly AFTER `extract_frame_sigs`, whose borrowed
`PyList_GetItem` pointers are not free-threading-safe). All five are valid; define-next chooses.

**Notes:**

- Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz.
- A live advisory can flip the enforcing `Audit` gate red on *any* push with no code change — this
    is normal, not a regression. Prefer `cargo update -p <crate>` to a patched release over a
    `deny.toml` ignore whenever a patch exists (recorded in learnings.md).
- Working tree also carries a runner-owned `iterations.jsonl` change — correctly left unstaged.
- Review agent memory was compacted this cycle (was over its 200-line read limit): detail moved into
    new `gate-reviews.md` / `binding-reviews.md` topic files; MEMORY.md now an 82-line index.
