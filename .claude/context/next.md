# Next Work Package

## Step: Wire up the `cargo-deny` supply-chain audit gate

## Goal

Land the last autonomous v1.0.0-hardening work package (issue "Wire up `cargo deny`/`cargo audit`
supply-chain gate", `normal` `[review]`, AUTHORIZED by Titusz in `9770332`): add a workspace-root
`deny.toml`, an enforcing `Audit` CI job running `cargo deny check`, and a `mise run audit` task.
This replaces the mechanical lockfile-inspection proxy (used to clear the PyO3 0.29 advisory bump)
with a real RustSec advisory + license + bans scan over the whole dependency graph.

## Scope

- **Create**: `deny.toml` (workspace root)
- **Modify**: `.github/workflows/ci.yml` (add an `audit` job after the `coverage` job, which ends at
    line 393); `mise.toml` (add `[tasks.audit]` after the Performance tasks block)
- **Modify (doc, excluded from 3-file limit)**: `.claude/context/specs/ci-cd.md` — flip the Audit
    "verified when" box at line 448 to `[x]` ONLY if you verify `cargo deny check` green locally
    (see Implementation Notes); otherwise leave it `[ ]` for the review agent to flip after CI green
- **Reference**: `notes/07-security-versioning.md` (deny.toml sketch — but its schema is OUTDATED,
    see notes); `.claude/context/specs/ci-cd.md` "Supply chain — `cargo-deny`" section (line 139)
    and Audit table row (line 33); `.claude/context/learnings.md` ("cargo binstall + Swatinem/
    rust-cache poisoning"); existing `perf`/`coverage` jobs in ci.yml (lines 280-393) as the install
    pattern template

## Not In Scope

- **Dependabot** (`.github/dependabot.yml`), `pip-audit`/`uv audit`, signed tags, SLSA provenance —
    `notes/07` mentions these but they are NOT in the issue or the ci-cd.md verified-when criterion.
    Separate future work.
- **`cargo audit` / `npm audit` steps** — the issue marks these "optionally"/"may complement".
    `cargo deny check` (advisories + bans + licenses) fully satisfies the mandatory criterion. Skip
    them to keep the step minimal; a follow-up can add them.
- **Enforcing duplicate-version bans** — keep `multiple-versions = "warn"` (see notes); do NOT build
    a fragile `skip`/`skip-tree` list to make duplicate-version denial pass.
- **Cutting v1.0.0 or flipping the `Semver (cargo-semver-checks)` gate to enforcing** — both
    deliberately held by Titusz until after this gate lands.
- **Adding `license.workspace = true` to the 4 unlicensed binding crates** — handle unlicensed
    workspace crates via `[licenses] private = { ignore = true }` in deny.toml instead (1 file, not
    4).

## Implementation Notes

**Install the tool locally to make this gate locally verifiable (strongly preferred).** Latest is
`cargo-deny 0.19.9` (confirmed reachable via crates.io). `cargo-binstall` is NOT preinstalled, so
either `cargo install cargo-binstall` then `cargo binstall -y --force cargo-deny@0.19.9`, or
`cargo install cargo-deny@0.19.9 --locked`. Then iterate `cargo deny check` until it exits 0 before
pushing. cargo-deny is dev/CI-only — never a shipped dependency.

**deny.toml — do NOT copy the `notes/07` sketch verbatim.** That sketch uses the pre-0.14 schema
(`vulnerability = "deny"`, `unmaintained = "warn"`, `unlicensed = "deny"`) which cargo-deny 0.18+
rejects or has removed. Write the modern schema. Target shape (verify exact keys against the
installed version — `cargo deny init` scaffolds a current template you can prune):

- `[graph] all-features = true` — so uniffi/napi/etc. binding-crate deps are included in the scan.
- `[advisories]` — vulnerabilities deny by default in v2; add `yanked = "deny"`. If a CURRENT
    unfixable RustSec advisory turns the local run red, add it to `ignore = ["RUSTSEC-YYYY-NNNN"]`
    with a one-line justification comment — do NOT loosen the whole class.
- `[licenses] allow = [...]` — the dependency graph (inspected this iteration) needs these
    STANDALONE licenses allow-listed (each appears not behind an OR alternative): `Apache-2.0`,
    `Apache-2.0 WITH LLVM-exception`, `MIT`, `MIT-0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`,
    `Unicode-3.0` (via `(MIT OR Apache-2.0) AND Unicode-3.0` in `unicode-ident`), `Zlib`
    (`foldhash`), `BSL-1.0` (`xxhash-rust` — a core CDC dep), and `MPL-2.0` (8 `uniffi*` crates).
    `CC0-1.0`/`Unlicense` are satisfiable via OR alternatives but harmless to include. Also set
    `private = { ignore = true }` — our 4 binding crates (`iscc-napi`, `iscc-py`, `iscc-rb`,
    `iscc-wasm`) carry no `license` field and would otherwise be flagged unlicensed.
- `[bans] multiple-versions = "warn"` — duplicate versions are ubiquitous in large graphs and low
    security value; enforcing denial needs a brittle skip-list and would turn CI red immediately.
    Banned-crate denial (an explicit `deny = [...]` list, leave empty for now) still enforces.
- `[sources] unknown-registry = "deny"`, `unknown-git = "deny"` — all deps come from crates.io.

**ci.yml `audit` job** (append after the `coverage` job; mirror the `perf`/`coverage` install
pattern): `runs-on: ubuntu-latest`, NO `continue-on-error` (this is enforcing), steps =
`actions/checkout@v4` → `dtolnay/rust-toolchain@stable` → `Swatinem/rust-cache@v2` → install
cargo-deny → `cargo deny check`. Install via `taiki-e/install-action@v2` with
`tool: cargo-deny@0.19.9` (it has a prebuilt-binary path; pin the version for reproducibility, same
discipline as `cargo-crap@0.2.2` / `iai-callgrind-runner@0.16.1`). If you instead use
`cargo binstall`, the `--force` flag is LOAD-BEARING against the `Swatinem/rust-cache` poisoning
gotcha (see learnings) — but `taiki-e/install-action` is cleaner here. Name the job "Audit
(cargo-deny)" for table consistency.

**mise task**: `[tasks.audit]` with a description and `run = "cargo deny check"`. CI does not use
mise — it calls `cargo deny check` directly; the task is for local reproduction only.

**Pre-commit hygiene**: run `mise run format` before staging (the pre-push mdformat/yamlfix/taplo
hooks reject the push batch otherwise — see learnings). yamlfix may fold long `run:` scalars onto
two physical lines; confirm the effective command via `python3 -c "import yaml; ..."` not raw grep.

## Verification

- `python3 -c "import tomllib; tomllib.load(open('deny.toml','rb'))"` exits 0 (deny.toml is valid
    TOML)
- `grep -q 'MPL-2.0' deny.toml && grep -q 'BSL-1.0' deny.toml && grep -q 'private' deny.toml` — the
    standalone-license + private-crate traps are handled
- `mise tasks 2>/dev/null | grep -q audit` (the `audit` task is registered) and `mise run audit`
    invokes `cargo deny check`
- `python3 -c "import yaml,sys; d=yaml.safe_load(open('.github/workflows/ci.yml')); j=d['jobs']['audit']; assert 'cargo deny check' in ' '.join(s.get('run','') for s in j['steps']); assert 'continue-on-error' not in j"`
    — the `audit` job exists, runs `cargo deny check`, and is enforcing (no continue-on-error)
- **Local gate (preferred):** with cargo-deny installed, `cargo deny check` exits 0 against the
    current graph (advisories + licenses + bans + sources all pass)
- `mise run check` — all pre-commit hooks pass (YAML/TOML/markdown valid, formatting idempotent)
- **Post-push (review agent confirms):** the `Audit (cargo-deny)` job is green on the develop tip;
    only then is the `ci-cd.md` line-448 box `[x]`

## Done When

`deny.toml`, the enforcing `Audit (cargo-deny)` CI job, and the `mise run audit` task are in place,
all local verification checks pass (ideally including a green local `cargo deny check`), and the new
CI job runs `cargo deny check` over the workspace as an enforcing gate.
