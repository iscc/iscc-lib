# Next Work Package

## Step: Restore green CI — ignore RUSTSEC-2026-0374, move macOS runners to `macos-15`

## Goal

Get CI on `develop` green again. It has two unrelated red jobs:

1. The enforcing `Audit (cargo-deny)` job fails on a new advisory, `RUSTSEC-2026-0374` (`basic-toml`
    is unmaintained). That is the only Audit error in run 44210618484.
2. The Swift job is cancelled before it starts, because GitHub retired the `macos-14` arm64 runner
    on 2026-11-02 (actions/runner-images#15226).

The runner choice is pinned `macos-15`, the arm64 successor of `macos-14`. Titusz may overrule it.

## Alternatives Considered

- **Chosen:** fix both red jobs in one step (3 files). CI red comes first, and splitting the fixes
    would keep `develop` red for another iteration.
- **Rejected:** start the next ISCC-SEQ binding (Python, or the Rust strict-base32 rejection tests).
    Feature work on red CI can't be attributed. It comes next.
- **Rejected:** `cargo update -p askama` instead of an ignore. `cargo update --dry-run` moves askama
    to 0.16.1 and uniffi to 0.32.2, but `basic-toml` stays. uniffi_bindgen 0.32.2 still enables
    askama's `config` feature, and that feature pulls in the optional `basic-toml` dependency of
    `askama_derive`. No upgrade path exists.

## Scope

- **Create**: none
- **Modify**: `deny.toml`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`
- **Reference**: `deny.toml` header (the policy for ignoring an unfixable advisory),
    `scripts/check_release_workflow.py`, the existing `[advisories] ignore` entries

## Not In Scope

- Do not run `cargo update`. Leave `Cargo.lock` untouched, because it does not fix the advisory and
    would make this diff harder to attribute.
- `uv.lock` Dependabot alerts (`soupsieve`, `setuptools`) and the Ruby `json` alert are separate
    steps.
- `tests/test_check_release_workflow.py:144/152` contains `macos-14`. It is a synthetic fixture, so
    do not touch it.
- Do not use `macos-latest` (it drifts) or `macos-15-intel` (that changes the build design). Do not
    touch `release.yml:1010`, which is already `macos-latest`.
- Do not change any other runner, toolchain, Xcode selection or action version.

## Implementation Notes

- **deny.toml:** add `"RUSTSEC-2026-0374"` to `[advisories] ignore` with a one-line justification,
    in the style of the two existing entries. Its comment block should say what the tree shows:
    `basic-toml` arrives only through the `askama_macros` proc-macro, via `askama` ←
    `uniffi_bindgen` ← `uniffi` ← `iscc-uniffi` (with `--all-features`). It is only reachable
    through uniffi's bindgen code path, `iscc-uniffi` is `publish = false`, and no upgrade drops it
    (uniffi_bindgen 0.32.2 still enables askama's `config` feature). Do **not** call it "dev-only":
    `cargo tree` shows a normal/build edge, not a dev edge. Keep the existing two-entry comment
    accurate (it is about iai-callgrind). Give the new entry its own comment.
- **Runners:** replace every `macos-14` in `.github/workflows/` with `macos-15`: `ci.yml:251` and
    the 10 `os:`/`runs-on:` sites in `release.yml`. `macos-15` is arm64, like `macos-14`, so the
    `x86_64-apple-darwin` rows still cross-compile exactly as before.
- Artifact names that include `matrix.os` (e.g. `wheels-macos-14-…`) change with the label. Any
    download that matches `wheels-*` keeps working. Run `uv run scripts/check_release_workflow.py`
    to confirm the artifact wiring still holds.
- Nothing in this step touches Rust code, so no iai or CRAP effect is expected.

## Verification

- `mise run audit` (`cargo deny check`) exits 0
- `! grep -rn 'macos-14' .github/workflows/` succeeds (no matches)
- `uv run scripts/check_release_workflow.py` exits 0
- `uv run pytest tests/test_check_release_workflow.py` passes
- `git diff --quiet HEAD -- Cargo.lock` succeeds (`Cargo.lock` unchanged)
- `mise run check` exits 0

## Done When

The advance agent is done when all six criteria pass on the working tree.

Reviewer note: only a CI run after the push can show whether the Swift job gets a `macos-15` runner.
The `release.yml` swap is checked statically only, because `release.yml` is
`workflow_dispatch`-only.
