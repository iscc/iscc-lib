# Handoff

## 2026-07-28 — Review of: Bump uniffi 0.31 → 0.32 and regenerate the Swift + Kotlin bindings

**Verdict:** PASS_WITH_NOTES

**Summary:** A clean, pure regeneration: I re-ran both documented bindgen commands myself and the
committed Swift/Kotlin/header bytes are content-identical to uniffi 0.32's output (differences are
only the trailing-whitespace/EOF normalization the prek hooks apply), so there are no hand edits and
no `crates/iscc-uniffi/src` change. All six next.md criteria pass and both binding suites are green
locally. One real defect surfaced that next.md did not ask about and Codex flagged independently:
the bump raises the *source-build* floor of the unpublished `iscc-uniffi` crate above the inherited
`rust-version = "1.85"` — accepted with a decisions.md entry and filed as a `normal` issue.

**Verification:**

- [x] Pin/lock/MSRV/wording — `Cargo.toml:55` = `uniffi = "0.32"`, `Cargo.lock` resolves `0.32.0`,
    `rust-version = "1.85"` at L17, no "pending bump"/"not verifiable locally" text survives (the
    only repo hit is stale prose in `state.md`, which update-state rewrites)
- [x] Regeneration is a no-op — **modulo hygiene normalization**, as the handoff disclosed. My own
    regen into `/tmp/regen` is byte-identical to the committed files after `sed 's/[[:space:]]*$//'`
    \+ EOF trim, for all three files (`.swift`, `.h`, `.kt`); raw bindgen output is not hook-clean,
    so literal `git status --porcelain` emptiness is unachievable
- [x] Kotlin — `cargo build -p iscc-uniffi` + `./gradlew clean test` BUILD SUCCESSFUL;
    `ConformanceTest.xml` `tests="9"`, `UnicodeBoundaryTest.xml` `tests="13"`, 0 skipped/failed
- [x] Swift — swift.org 6.1.2 Debian 12 toolchain: `Executed 12 tests, with 0 failures`, 0 skips
- [x] `mise run lint` exit 0 (clippy `-D warnings` clean, 187 files formatted); `mise run test` exit
    0 (`cargo test --workspace` all ok incl. iscc-uniffi 21 passed; 441 pytest passed)
- [x] `mise run audit` exit 0 (`advisories ok, bans ok, licenses ok, sources ok`), no `deny.toml`
    edit; also `mise run check` 18 hooks Passed, no gate config touched anywhere in the push range

**Issues found:**

- **MSRV (filed, `normal` `[review]`):** uniffi 0.32 pulls `cargo_metadata 0.23.1` (rustc 1.86) and
    `cargo-platform 0.3.3` (rustc 1.91) into `iscc-uniffi`'s **default non-dev** graph, so
    `cargo +1.85.0 check -p iscc-uniffi --locked` now fails; the pre-bump graph topped out at 1.83.
    Consumers unaffected — `iscc-uniffi` is `publish = false` and
    `cargo +1.85.0 check -p iscc-lib --locked` still passes. Accepted deliberately (decisions.md).
- Public API of both bindings is unchanged (diffed declaration by declaration); Kotlin checksum
    externals widened `Short` → `Int` and Swift closures now bind `uniffiCallStatus` instead of `$0`
    — upstream template changes, runtime-verified by the passing checksum init in both suites.
- Nit (next.md, not the advance agent): the step touched 4 non-test/non-doc files without the
    `**Fan-out:**` line the scope cap wants. Three are machine-generated (`Cargo.lock` + the two
    bindings) from a single one-line pin, so this is a formatting gap, not a scope breach.
- Fixed inline: `specs/rust-core.md` no longer claims `packages/go` lowercases context-insensitively
    (verified — the 7 Go `Final_Sigma` tests pass) or that it is tracked in issues.md (it is not);
    `specs/kotlin-bindings.md` no longer pins a uniffi version in prose.

**Codex review:** one P1, and it was correct: "UniFFI 0.32's default feature graph selects
`cargo_metadata 0.23.1` (Rust 1.86) and `cargo-platform 0.3.3` (Rust 1.91), so
`cargo +1.85.0 check -p iscc-uniffi --locked` now fails … while the crate still advertises
`rust-version = "1.85"`." I reproduced both halves and confirmed the published crate is unaffected.
No other findings.

**Next:** the only actionable non-trigger issue left is the MSRV one — a one-line
`rust-version = "1.91"` on `crates/iscc-uniffi/Cargo.toml` so the crate states its real floor
instead of inheriting a false one, verified with `cargo +1.85.0 check -p iscc-lib --locked` (must
still pass) and `cargo check --workspace` on stable. Everything else in issues.md is `low`
(human-gated) or trigger-gated (go1.27, ~Aug 2026), so if that step is declined the loop is close to
IDLE.

**Notes:**

- **Outstanding confirmation:** the macOS `swift` CI job on the pushed commit. I ran the suite on
    Linux with swift 6.1.2 and the C header/modulemap are unchanged, so risk is low — but a red
    there is the trigger to re-file the dependency issue (deleted this iteration: uniffi 0.32 was
    its last item and all authorized majors have now landed).
- `rustup` in this devcontainer has the **1.85.0 toolchain installed** — MSRV claims are locally
    verifiable (`cargo +1.85.0 check -p <crate> --locked`); no need to defer them to a CI job.
- Kotlin compile still emits `Expression is unused` at `iscc_uniffi.kt:1082` (uniffi-intended);
    bindgen still warns about missing `swift-format`. Both benign, do not hand-fix generated code.
- The `proc-macro-error2 v2.0.1` future-incompat warning remains pre-existing and dev-only.
