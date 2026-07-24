# Next Work Package

## Step: Fix CI — patch RUSTSEC-2026-0204 by bumping dev-only crossbeam-epoch to 0.9.20

## Goal

Make CI green again. The enforcing `Audit (cargo-deny)` gate is the only failing job: it correctly
caught the fresh advisory **RUSTSEC-2026-0204** against `crossbeam-epoch v0.9.18` (a dev-only
benchmark transitive dep). A patched release (`>= 0.9.20`) exists, so fix the root cause with a
lockfile bump — no code, manifest, or policy change required.

## Scope

- **Modify**: `Cargo.lock` (generated — regenerate via `cargo update -p crossbeam-epoch`, do not
    hand-edit)
- **Reference**: `deny.toml` (the enforcing policy; confirm no change is needed),
    `.claude/context/state.md` → "Next Milestone" (fix rationale + options),
    `.claude/context/learnings.md` → CI/CD `Audit (cargo-deny)` and `yanked` entries

## Not In Scope

- **Do NOT add `RUSTSEC-2026-0204` to the `deny.toml` `ignore` list.** A patched version is
    available, so the correct fix removes the vulnerable crate from the graph rather than
    suppressing the advisory. The `ignore` fallback in state.md applies only if no patched release
    existed — it does exist (`>= 0.9.20`).
- Do NOT bump `criterion`, `rayon`, or the rest of the crossbeam family — only `crossbeam-epoch`
    needs updating (the dry-run locks exactly 1 package). Leave unrelated deps at their current
    pins.
- Do NOT start any v0.6.0 work package (Python text/video GIL #41, WASM simd128 #42, Go ISCC-IDv1
    #43, aarch64 wheels #49, dependency refresh). Those come after CI is green, one per iteration.
- Do NOT touch the `Semver (cargo-semver-checks)` gate or cut v1.0.0 — both are human-held.

## Implementation Notes

- Run `cargo update -p crossbeam-epoch`. This is a semver-compatible patch bump `0.9.18 → 0.9.20`
    with **no** `Cargo.toml` change. Verified facts (define-next, iter 115):
    - RUSTSEC-2026-0204 advisory metadata: `patched = [">= 0.9.20"]` (null-pointer deref in
        `fmt::Pointer` for `Atomic`/`Shared`); `0.9.20` is the latest published and clears it.
    - `crossbeam-epoch` is **dev-only**:
        `criterion 0.5.1 → rayon → rayon-core → crossbeam-deque →   crossbeam-epoch`, reachable only
        through `iscc-lib`'s `[dev-dependencies]` (benchmarks). It never ships in any published
        artifact. `cargo tree -i crossbeam-epoch -e no-dev` is empty.
    - `cargo update -p crossbeam-epoch --dry-run` locks exactly 1 package (`0.9.18 -> 0.9.20`).
- No `deny.toml` edit is needed: the affected version disappears from the graph, so the enforcing
    advisories class passes with the two existing dev-only `ignore`s (`RUSTSEC-2025-0141`,
    `RUSTSEC-2026-0173`) untouched.
- `cargo-deny` is not preinstalled but IS installable (network permitting):
    `cargo install cargo-binstall` then `cargo binstall cargo-deny@0.19.9 --force`, then
    `cargo deny check`. cargo-deny reads `Cargo.lock` + crate metadata (not compiled artifacts), so
    a local green is authoritative for the gate. If install fails offline, the CI
    `Audit (cargo-deny)` job on the next develop push is the definitive confirmation.
- Run `mise run format` before staging so the pre-push hooks don't reject the batch.

## Verification

- `Cargo.lock` pins the patched version: `grep -A2 'name = "crossbeam-epoch"' Cargo.lock` shows
    `version = "0.9.20"`, and `0.9.18` no longer appears for that crate.
- `cargo tree -i crossbeam-epoch` shows only `v0.9.20`, still reached only via dev-dependencies.
- No advisory suppression was added: `grep -c 'RUSTSEC-2026-0204' deny.toml` returns `0`.
- Benchmarks still compile against the bumped dep: `cargo bench -p iscc-lib --no-run` succeeds.
- `cargo test -p iscc-lib` passes (sanity — a dev-dep lockfile bump changes no runtime behavior).
- Supply-chain gate clean: `cargo deny check` exits 0
    (`advisories ok, bans ok, licenses ok, sources   ok`) locally if cargo-deny is installable;
    otherwise confirmed by the `Audit (cargo-deny)` CI job going green on the next develop push.

## Done When

`cargo update -p crossbeam-epoch` has bumped the lockfile to `crossbeam-epoch 0.9.20`, the benches
compile, `cargo test -p iscc-lib` passes, no `ignore` entry was added to `deny.toml`, and
`cargo deny check` (local or the CI Audit job) reports all four policy classes clean.
