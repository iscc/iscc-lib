# Next Work Package

## Step: Declare `iscc-uniffi`'s real rustc floor instead of inheriting a false one

## Goal

Close the `normal` issue "`iscc-uniffi` no longer builds on the declared MSRV 1.85" by giving
`crates/iscc-uniffi/Cargo.toml` an explicit `rust-version = "1.91"`, so the crate states the floor
its own dependency graph imposes. This is the last CID-doable v0.6.0 release-readiness criterion.

## Alternatives Considered

- **Chosen:** the per-crate `rust-version` declaration — the only open `normal` issue that is
    neither human- nor trigger-gated, and the last thing between the tree and the human's v0.6.0 "no
    unblocked `normal` issue" criterion.
- **Rejected:** adding a guard (CI job or pytest anchor) that recomputes each crate's floor from the
    dependency graph and compares it to the declared `rust-version` — a new enforcing policy gate,
    and MSRV verification is explicitly tracked as a `low`, human-gated v1.0.0 prerequisite.

## Scope

- **Modify**: `crates/iscc-uniffi/Cargo.toml` (one line + an evergreen comment)
- **Reference**: root `Cargo.toml` L17 + L44-46 (the workspace floor and the existing
    contributor-floor comment style), `.claude/context/issues.md` (the MSRV entry)

## Not In Scope

- Touching the root `Cargo.toml` `rust-version = "1.85"`. That is the published MSRV promise and
    Titusz's call — it must read `1.85` on the working tree when this step is done.
- Any other crate's `rust-version` (all seven others correctly inherit the workspace value).
- Adding an MSRV CI job, a `rust-toolchain.toml`, or any floor-checking script/test.
- Trying to keep the floor at 1.85 by dropping uniffi default features or pinning `cargo-platform` /
    `cargo_metadata` down — the raised source-build floor was accepted in decisions.md (2026-07-28),
    so do not re-open it.
- Editing `issues.md` (review owns issue resolution) or `Cargo.lock`.

## Implementation Notes

- Replace `rust-version.workspace = true` (line 5) with `rust-version = "1.91"` and add a short
    comment above it saying the floor is set by the transitive `cargo-platform` 0.3.3 (floor 1.91)
    reached through `uniffi` → `cargo_metadata`, that the crate is `publish = false`, and that the
    published `iscc-lib` MSRV stays 1.85. Keep the key at the same position in the `[package]` table
    so `taplo fmt` leaves it alone.
- Floor verified while scoping, do not re-derive:
    `cargo tree --locked -p iscc-uniffi -i cargo-platform -e no-dev --target all` =
    `cargo-platform v0.3.3 → cargo_metadata v0.23.1 → uniffi v0.32.0 → iscc-uniffi`; registry
    manifests declare `rust-version = "1.86.0"` and `"1.91"`.
- `resolver = "2"` is not MSRV-aware, so this is a truthful metadata declaration only — no
    resolution, lockfile, or build behaviour changes. Local stable is 1.97.1, well above 1.91.
- Expected new behaviour: `cargo +1.85.0 check -p iscc-uniffi --locked` still fails, but now with
    cargo's explicit "requires rustc 1.91 or newer" message instead of a confusing dependency
    resolution error. That is the point of the change, not a regression.
- No docs surface mentions an MSRV (grep of `docs/`, `README.md`, package `CLAUDE.md` files found
    none), and `crates/iscc-uniffi/` has no `CLAUDE.md` — do not create one.

## Verification

- `grep -n 'rust-version = "1.91"' crates/iscc-uniffi/Cargo.toml` matches and
    `grep -c 'rust-version.workspace' crates/iscc-uniffi/Cargo.toml` is `0`.
- `grep -n 'rust-version = "1.85"' Cargo.toml` still matches on the working tree (root floor
    unchanged).
- `cargo +1.85.0 check -p iscc-lib --locked` exits 0 (published crate still builds on the declared
    MSRV).
- `cargo check --workspace --locked` exits 0 on stable, and `git status --porcelain Cargo.lock` is
    empty (no lockfile churn).
- `mise run lint` exits 0 (`cargo fmt --check`, clippy `-D warnings`, ruff).
- `mise run check` exits 0 (all prek hooks, incl. `taplo fmt` and TOML validation).

## Done When

All six verification checks pass on the working tree with `crates/iscc-uniffi/Cargo.toml` declaring
its own `rust-version = "1.91"` and the root workspace floor still at `1.85`.
