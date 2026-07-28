# Handoff

> **HUMAN REVIEW REQUESTED**: the last CID-doable issue is closed, so v0.6.0's release-readiness
> criterion ("no `critical`/`normal` issue open that is not blocked on the human or on an upstream
> release") is now **met**. Every remaining backlog entry is human-gated (v0.6.0 cut, v1.0.0, the
> MSRV CI job, npm OIDC, the upstream `iscc-core#137` thread, docs logos) or trigger-gated (go1.27,
> ~Aug 2026). The loop has no actionable work; cutting v0.6.0 from PR #44 is Titusz's call.

## 2026-07-28 — Review of: Declare `iscc-uniffi`'s real rustc floor instead of inheriting a false one

**Verdict:** PASS

**Summary:** A four-line, single-file metadata change that does exactly what next.md asked:
`crates/iscc-uniffi/Cargo.toml` now declares `rust-version = "1.91"` with an evergreen comment
naming the transitive cause, replacing the false inherited `1.85`. All six next.md criteria pass, I
re-derived the floor independently, and the root workspace MSRV promise is untouched. Perfect scope
discipline — one non-test/non-doc file.

**Verification:**

- [x] `grep -n 'rust-version = "1.91"' crates/iscc-uniffi/Cargo.toml` → `8:rust-version = "1.91"`;
    `grep -c 'rust-version.workspace'` on that file → `0`. The other 7 crates still inherit
    (`ffi/lib/napi/py/wasm/jni/rb` all at `rust-version.workspace = true`)
- [x] Root floor unchanged — `grep -n 'rust-version = "1.85"' Cargo.toml` → `17:...`
- [x] `cargo +1.85.0 check -p iscc-lib --locked` → exit 0 (published crate still builds on 1.85)
- [x] `cargo check --workspace --locked` → exit 0 on stable 1.97.1;
    `git status --porcelain Cargo.lock` empty (no lockfile churn)
- [x] `mise run lint` → exit 0 (clippy `-D warnings` clean, 187 files formatted)
- [x] `mise run check` → exit 0, 18 prek hooks Passed, no file rewritten by a hook

**Probes beyond next.md (3 of 3):**

- **The declaration is effective:** `cargo +1.85.0 check -p iscc-uniffi --locked` now exits 101 with
    `iscc-uniffi@0.5.0 requires rustc 1.91`, not a bare dependency-resolution error. Intended.
- **The declared floor is exactly right, not a guess:** a `cargo metadata --locked` walk of
    `iscc-uniffi`'s 93-package **non-dev** graph gives max `rust_version` = **1.91**, from
    `cargo-platform 0.3.3` alone (next below it: askama 0.16 at 1.88, `cargo_metadata` at 1.86). So
    the comment's stated cause is accurate and the value is neither over- nor under-stated.
- **Nothing else assumes uniform inheritance:** no script, workflow, `mise.toml` task, doc page or
    README mentions `rust-version`/MSRV, so the per-crate override breaks no gate and leaves no
    stale doc claim. `specs/rust-core.md` "API Stability & Performance Invariants" states no MSRV,
    so resolving the `[review]` issue needed no spec edit.

**Issues found:**

- (none) — no gate weakening anywhere in the unpushed range (`@{upstream}..HEAD`, 4 commits; the
    only non-`.claude/` change in the whole range is this one Cargo.toml), no API break, no
    benchmarked path touched, no dead code.
- Resolved and deleted: `iscc-uniffi` no longer builds on the declared MSRV 1.85 (`normal`,
    `[review]`). Its one durable side-fact — a future MSRV CI job must use `-p iscc-lib`, never
    `--workspace` — was folded into the surviving `low` MSRV issue so it is not lost.

**Codex review:** clean. Its only statement: "The manifest now accurately declares the effective
Rust version required by the locked UniFFI dependency graph without changing the published core
crate's MSRV. No functional regressions were identified." Nothing actionable.

**Next:** There is no CID-doable step left. If the loop runs again, define-next should emit
`## Step: NONE` naming the human-gated backlog rather than manufacturing work — do **not** scope the
MSRV CI job (`low`, human-gated v1.0.0 prerequisite), the go1.27 freeze table (park until go1.27 has
a final tag), npm OIDC (deferred by Titusz for v0.6.0) or the v1.0.0 cut. For Titusz, the sequence
is: merge PR #44 `develop` → `main`, cut v0.6.0 via the `/release` skill, then decide the two v1.0.0
prerequisites (flip `cargo-semver-checks` from `continue-on-error` to enforcing, and add the 1.85
`cargo check -p iscc-lib` job).

**Notes:**

- `resolver = "2"` is not MSRV-aware, so this changed no resolution, lockfile or build behaviour on
    stable — it is a truthfulness fix to metadata, exactly as scoped.
- `cargo +1.85.0 check --workspace` fails by design now; that is the correct signal, not a
    regression. Only `-p iscc-lib` is the MSRV promise.
- CI has not yet seen this commit; the push below is its first run. The change cannot affect any CI
    job (all build on stable), so a red run would point at something else.
- The runner's `decisions.md` → `decisions-archive.md` rotation was left uncommitted in the working
    tree; it is staged with this review commit. No new decision was recorded — this step is the
    direct implementation of the 2026-07-28 "uniffi 0.32 accepted with a raised source-build floor"
    entry, not a fresh judgment call.
