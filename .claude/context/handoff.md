## 2026-11-18 — Review of: Restore green CI — ignore RUSTSEC-2026-0374, move macOS runners to `macos-15`

**Verdict:** PASS

**Summary:** Advance added one justified `deny.toml` advisory ignore and swapped all 11 `macos-14`
runner labels for `macos-15`: 1 in `ci.yml`, 10 in `release.yml`. The diff is exactly the 3 files in
scope, and `Cargo.lock` is untouched. All six criteria pass. The ignore follows deny.toml's own
policy for an advisory that has no fix. I confirmed at the source that no upgrade path exists, so it
does not weaken the gate.

**Verification:**

- [x] `mise run audit` exits 0 → `advisories ok, bans ok, licenses ok, sources ok`
- [x] `! grep -rn 'macos-14' .github/workflows/` succeeds (no matches)
- [x] `uv run scripts/check_release_workflow.py` exits 0
    (`OK … passed release-workflow static checks`)
- [x] `uv run pytest tests/test_check_release_workflow.py` → 27 passed
- [x] `git diff --quiet HEAD -- Cargo.lock` succeeds. It is also unchanged against `HEAD~1`
- [x] `mise run check` exits 0 (all hooks passed)
- Probe 1, the "no upgrade drops it" claim: crates.io shows uniffi's latest is 0.32.2.
    `uniffi_bindgen` 0.32.2 depends on `askama ^0.16` with `features: [config, derive, alloc]`.
    `askama` 0.16.1 `config` → `askama_macros/config` → `askama_derive` `dep:basic-toml`. So the
    advisory cannot be fixed by an update and the ignore is justified
- Probe 2: `cargo tree -i basic-toml --workspace --all-features -e no-dev --target all` has a single
    root, `iscc-uniffi`. No published crate pulls `basic-toml`
- Probe 3: the `actions/runner-images` README lists `macos-15` as arm64, so the x86_64-darwin rows
    still cross-compile from arm64, as before. It also confirms that every `macos-14` image was
    retired on 2026-11-02

**Issues found:**

- Spec and doc drift, which advance flagged. I fixed it as review bookkeeping, since a runner label
    is a fact that can be checked against the code:
    - `macos-14` → `macos-15` in `specs/ci-cd.md`, `specs/swift-bindings.md` (including the criterion
        at line 440, where only the label changed and the requirement did not) and
        `specs/c-ffi-dx.md`, plus `packages/swift/CLAUDE.md`
    - `specs/ruby-bindings.md` had stale drift unrelated to this step: its gem table listed
        `macos-14`/`windows-latest` runners, but `build-gem` cross-compiles every platform on
        `ubuntu-latest`. I dropped the OS column and stated that once in the heading
- `notes/06-build-cicd-publishing.md:133,170` still says `macos-14`/`macos-13`. Those are
    illustrative design-time YAML examples, not a description of the current workflow, so I left
    them alone
- Nit: the `deny.toml` comment leans on `publish = false`, but the compiled `iscc-uniffi` libraries
    do ship in the Swift and Kotlin releases. The stronger reason is that `basic-toml` sits inside a
    proc-macro, so it runs only at compile time on the build host and is never linked into a shipped
    artifact. Reword it if the file is touched again; not worth its own step
- Learnings: I corrected the Audit bullet, which still said "never a `deny.toml` ignore". It now
    says to try `cargo update` first and to ignore only when the edge that pins the crate is proven
    at the latest upstream

**Codex review:** Clean. It says the runner swap keeps targets and artifact wiring, and the ignore
is limited to `basic-toml`. The offline cargo-deny run, the static release check and the 27 tests
passed. It notes the macOS jobs still need a hosted run.

**Next:**

1. **CI is green on the reviewed sha `008d6f4`** (run 44281783958, confirmed after the push):
    `Swift` passed on `macos-15` and `Audit (cargo-deny)` passed. Re-check the latest develop sha
    before scoping, as usual
2. Return to ISCC-SEQ: the Python `encode_seq`/`decode_seq` binding with the iscc_core 1.5.0
    differential test. It must stay strict on base32 (padding and spare bits), asserting the
    divergence explicitly per decisions.md 2026-11-18. The Go `decodeBase32` spare-bit fix plus the
    Rust strict-base32 tests (issues.md, `normal` `[human]`) is the other ready candidate
3. Smaller ready `normal` items: the `uv.lock` Dependabot bump (`soupsieve`, 2 high; filed in this
    review), the Ruby `json` Dependabot bump, the Ruby locale fix, `@iscc/wasm` package metadata,
    and the Go `isccClean` port

**Notes:**

- **Decision recorded:** decisions.md 2026-11-18 pins `macos-15` (arm64) and rejects both
    `macos-latest` (it drifts) and `macos-15-intel` (it would turn the x86_64 rows into native
    builds). Titusz may overrule. The `release.yml` half is static-only, because the workflow is
    `workflow_dispatch`-only, so the first real check is the next release
- Release artifact names built from `matrix.os` change from `*-macos-14-*` to `*-macos-15-*`. No
    download step names a literal label, and the static wiring gate passes
- cargo-deny is not in the devcontainer. Advance installed the prebuilt 0.19.9 (the CI pin) into
    `~/.cargo/bin`
- decisions.md is at 396/400 lines. The runner rotates it automatically once it goes over
