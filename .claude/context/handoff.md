## 2026-11-18 — Restore green CI: ignore RUSTSEC-2026-0374, move macOS runners to `macos-15`

**Done:** Added `RUSTSEC-2026-0374` (`basic-toml` unmaintained) to `deny.toml`'s
`[advisories] ignore`. It has its own comment block that traces the uniffi/askama path. Also
replaced all 11 `macos-14` runner labels in `.github/workflows/` with `macos-15`: 1 in ci.yml, 10 in
release.yml.

**Files changed:**

- `deny.toml`: one new ignore entry with its own comment. It names the path askama_derive ←
    askama_macros (proc-macro) ← askama ← uniffi_bindgen ← uniffi ← iscc-uniffi (`publish = false`,
    `--all-features`), says it is a normal/build edge and not a dev edge, and says no upgrade drops
    it. The existing iai-callgrind comment is unchanged
- `.github/workflows/ci.yml`: line 251 (Swift job) `runs-on: macos-14` → `macos-15`
- `.github/workflows/release.yml`: lines 58, 177, 309, 311, 531, 535, 700, 704, 1105, 1109 →
    `macos-15`. Line 1010 (`macos-latest`) is untouched

**Verification:**

- `mise run audit` → exit 0, `advisories ok, bans ok, licenses ok, sources ok` (only
    duplicate-version warnings). I installed cargo-deny 0.19.9, the CI pin, from the prebuilt GitHub
    release into `~/.cargo/bin`, because it was not in the container
- Negative probe: `cargo deny check --config <deny.toml minus the 0374 line> advisories` → exit 1,
    `error[unmaintained]: basic-toml is unmaintained`. So the ignore is load-bearing and is the only
    advisory error
- `cargo tree -i basic-toml --all-features -e normal,build` shows exactly the path in the comment
    (askama 0.16.0, uniffi_bindgen 0.32.0)
- `! grep -rn 'macos-14' .github/workflows/` → no matches
- `uv run scripts/check_release_workflow.py` → `OK ... passed release-workflow static checks`
- `uv run pytest tests/test_check_release_workflow.py` → 27 passed
- `git diff --quiet HEAD -- Cargo.lock` → succeeds (Cargo.lock unchanged, no `cargo update`)
- `mise run check` → exit 0
- Not verified: whether the Swift job actually gets a `macos-15` runner. Only the CI run after the
    push can show that. The `release.yml` change is checked statically only

**Next:** Confirm that both `Audit (cargo-deny)` and `Swift` are green on the pushed sha. Then go
back to ISCC-SEQ: the Python `encode_seq` / `decode_seq` binding with the iscc_core 1.5.0
differential test, staying strict on base32 per decisions.md 2026-11-18.

**Notes:**

- Spec drift (out of scope, I left these alone): `.claude/context/specs/ci-cd.md` (lines 220, 287,
    306, 307, 333, 334), `specs/swift-bindings.md` (267, 359) and `specs/c-ffi-dx.md` (107, 108)
    still say `macos-14`. Review or define-next should update them to `macos-15`
- Artifact names built from `matrix.os` change from `*-macos-14-*` to `*-macos-15-*`. No download
    step names a literal `macos-14` artifact (grep is empty), and the static wiring check passes
- No Rust code was touched, so the iai and CRAP gates are not affected
