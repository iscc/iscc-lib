# Handoff

## 2026-07-28 — Declare `iscc-uniffi`'s real rustc floor instead of inheriting a false one

**Done:** Replaced `rust-version.workspace = true` in `crates/iscc-uniffi/Cargo.toml` with an
explicit `rust-version = "1.91"` plus a three-line evergreen comment naming the transitive cause
(uniffi → cargo_metadata → cargo-platform 0.3.3, floor 1.91), the `publish = false` status, and the
unchanged published `iscc-lib` MSRV of 1.85. No other file touched.

**Files changed:**

- `crates/iscc-uniffi/Cargo.toml`: `rust-version = "1.91"` with explanatory comment, replacing
    workspace inheritance (line 5 → lines 5-8)

**Verification:** All six next.md criteria pass, evidence from this session:

- `grep -n 'rust-version = "1.91"' crates/iscc-uniffi/Cargo.toml` → `8:rust-version = "1.91"`;
    `grep -c 'rust-version.workspace'` on the same file → `0`
- `grep -n 'rust-version = "1.85"' Cargo.toml` → `17:rust-version = "1.85"` (root floor unchanged)
- `cargo +1.85.0 check -p iscc-lib --locked` → exit 0 (run with `set -o pipefail`, exit code
    captured explicitly)
- `cargo check --workspace --locked` → exit 0 on stable 1.97.1; `git status --porcelain Cargo.lock`
    → empty (no lockfile churn)
- `mise run lint` → exit 0 (187 files formatted, clippy clean, "All checks passed!")
- `mise run check` → exit 0, all 18 prek hooks Passed (incl. `check toml` + TOML formatting — taplo
    left the key in place as intended)
- Bonus (next.md's expected-behaviour claim confirmed):
    `cargo +1.85.0 check -p iscc-uniffi --locked` now fails with cargo's explicit message
    `iscc-uniffi@0.5.0 requires rustc 1.91` / `cargo-platform@0.3.3 requires rustc 1.91` instead of
    a confusing resolution error

**Next:** Per the previous review handoff, this was the last CID-doable `normal` issue for v0.6.0
release readiness. Remaining issues.md entries are `low` (human-gated) or trigger-gated (go1.27,
~Aug 2026) — the loop is likely at or near IDLE / human-handoff.

**Notes:** Pure metadata change — `resolver = "2"` is not MSRV-aware, so no resolution, lockfile, or
build behaviour changed on stable. The review agent can close the `normal` `[review]` MSRV issue in
issues.md on this evidence.
