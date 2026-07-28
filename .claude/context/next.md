# Next Work Package

## Step: Bump uniffi 0.31 → 0.32 and regenerate the Swift + Kotlin bindings

## Goal

Land the last authorized dependency major — and with it the last CID-doable v0.6.0 release
criterion: pin `uniffi = "0.32"` in the workspace and regenerate both checked-in bindings as a
*pure* regeneration (no hand edits to generated code, no `crates/iscc-uniffi/src` edits).

## Alternatives Considered

- **Chosen:** uniffi 0.32 — the sole remaining item of the release-gating dependency issue; closing
    it clears the v0.6.0 gate. Both breaking-change risks were measured away while scoping (below).
- **Rejected:** correcting the stale Go `Final_Sigma` claim in `specs/rust-core.md` L149-157 — a
    mechanically checkable fact the review agent may now fix inline; it gates no release criterion.

## Scope

- **Modify**: `Cargo.toml` (workspace pin + rewrite the stale 3-line pin comment above it),
    `Cargo.lock` (generated), `packages/swift/Sources/IsccLib/iscc_uniffi.swift` (generated),
    `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h` (generated),
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (generated)
- **Reference**: `packages/swift/CLAUDE.md` (generation command + the swift.org Debian 12 tarball
    recipe under Common Pitfalls), `packages/kotlin/CLAUDE.md` (generation + Gradle),
    `crates/iscc-uniffi/src/lib.rs`, `.claude/agent-memory/advance/uniffi-swift-kotlin.md`

## Not In Scope

- Adopting any new 0.32 feature (`uniffi.toml` excludes, `--config` global config, recursive enums,
    `#[uniffi::export(rust, foreign)]`, the experimental `uniffi-bindgen-kotlin-jni` backend).
- Any edit to `crates/iscc-uniffi/src/lib.rs` or `uniffi-bindgen.rs` — if one turns out to be
    required, STOP and hand back for a re-scope (issues.md constraint on this bump).
- Moving any consumer floor: JNA 5.19.1, `kotlin("jvm") 2.4.10`, `swift-tools-version: 5.9`,
    `rust-version = "1.85"`, JDK 17. Those are Titusz's calls.
- Refreshing `.iai-baseline.json` / `.crap-baseline.json` — no `iscc-lib` core source moves here.
- Editing `issues.md` (review owns issue resolution) or `.pre-commit-config.yaml`.

## Implementation Notes

- uniffi **0.32.0** is latest and unyanked; the sparse index declares no `rust-version`, so the
    published MSRV is untouched. `uniffi::uniffi_bindgen_main()` still exists behind the `cli`
    feature → `crates/iscc-uniffi/uniffi-bindgen.rs` needs no edit.
- Verified while scoping that no 0.32 breaking change reaches this crate: the `[ByRef] bytes` change
    is UDL-only (this crate is proc-macro-only — `setup_scaffolding!()`, no `.udl` anywhere); the
    async-primary-constructor break does not apply (both `#[uniffi::constructor]`s, `src/lib.rs`
    ~441/~493, are sync); the `--config` change needs a `uniffi.toml` (none in the repo, and no
    `--config` is passed); the pipeline rework targets external bindgen authors.
- CLI shape changed benignly: `--library` is now a **deprecated boolean flag** and the cdylib is the
    positional `source` (library mode auto-detected). Both commands documented in the two
    `CLAUDE.md` files still parse — run them verbatim so the recipes stay accurate.
- Swift generation writes three files into `--out-dir`: keep `iscc_uniffi.swift` there, move
    `iscc_uniffiFFI.h` to `packages/swift/Sources/iscc_uniffiFFI/`, and **discard** the generated
    `iscc_uniffiFFI.modulemap` — the checked-in `module.modulemap` is hand-simplified (Darwin `use`
    directives break SPM) and must stay byte-identical. Module name stays `iscc_uniffiFFI`.
- Kotlin generation needs `--no-format` (no ktfmt in the container).
- Gradle reports `UP-TO-DATE` without executing anything: use `./gradlew clean test`, and do not run
    a second Gradle build concurrently (it deletes `build/reports/problems` mid-flight).
- Rebuild `cargo build -p iscc-uniffi` before every Swift/Kotlin run — a stale `libiscc_uniffi.so`
    fails the uniffi checksum check loudly, which is the intended signal, not a bug to work around.
- Run `mise run format` before `git add`. If a hygiene hook rewrites a *generated* file, record it
    in the handoff — do not hand-edit generated code to satisfy it.
- If `cargo deny` flags a new license or advisory from 0.32's added transitive crates, report it in
    the handoff; do **not** add a `deny.toml` ignore.

## Verification

- `grep -n 'uniffi = "0.32"' Cargo.toml` matches, `Cargo.lock` resolves `uniffi` to `0.32.0`,
    `grep -n 'rust-version = "1.85"' Cargo.toml` still matches, and no "pending bump" / "not
    verifiable locally" wording survives beside the pin.
- Regeneration is a no-op: after `cargo build -p iscc-uniffi`, re-running both documented bindgen
    commands (with the Swift header re-placed as above) leaves `git status --porcelain` empty.
- `cargo build -p iscc-uniffi` then `./gradlew clean test` in `packages/kotlin` exits 0, with
    `tests="9"` in `build/test-results/test/*ConformanceTest.xml` and `tests="13"` in
    `*UnicodeBoundaryTest.xml`.
- Swift suite passes locally via the `packages/swift/CLAUDE.md` tarball recipe:
    `swift test --scratch-path /tmp/swiftbuild -Xlinker -L../../target/debug -Xlinker -rpath   -Xlinker ../../target/debug`
    exits 0 with zero failures and zero skips.
- `mise run lint` and `mise run test` exit 0.
- `mise run audit` exits 0 with no `deny.toml` edit.

## Done When

All six verification checks pass on the working tree with the bindings regenerated rather than
hand-edited; the macOS `swift` CI job on the pushed commit stays the final confirmation.
