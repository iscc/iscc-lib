# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files. A `low`
entry with `**Triage:** untriaged` is a review finding the human has not weighed yet: promote it to
`normal`, or delete the line to park it. The human moves an entry that does not matter for the
current release to `postponed.md`, which the loop never acts on.

**This is a defect tracker, not a work queue.** An entry states the remaining problem and how to
tell it is fixed, in **40 lines or fewer**, and is deleted once nothing is left to fix. Never record
progress inside an entry — no per-iteration checklists, no "✅ slice N done", no plan of remaining
sub-steps. Git history records what was done; a step queue here removes the gap analysis that
define-next exists to perform. If only part of a problem remains, rewrite the entry to describe that
smaller problem. Design rationale belongs in `decisions.md`, operational gotchas in `learnings.md`,
and user-facing behaviour in `docs/`.

<!-- Add issues below this line -->

## Ruby 4.0 gets no precompiled gem `normal` [review]

Every user on Ruby 4.0, the current stable series, installs the source gem and needs a Rust
toolchain and `libclang-dev`; the docs say so. The 0.8.0 source gem from rubygems.org builds and
passes `conformance_selftest` on Ruby 4.0.10 (measured 2026-11-23).

The cross-gem image is pinned to rb-sys 0.9.123, whose newest cross Ruby is 3.4.5. Ruby 4.0 needs
rb_sys 0.9.124 or newer, and the hold comment in `crates/iscc-rb/Gemfile` says that version breaks
`RbSys::ExtensionTask`. Unverified since the hold was written.

Resolved when the cross-gem list includes 4.0, `test-gem` loads the 4.0 gem, and the docs name the
range, or a decisions.md entry keeps 4.0 on the source gem.

## NuGet publishes with a long-lived API key `normal` [human]

`publish-nuget` in `release.yml` pushes with the `NUGET_API_KEY` secret. The key expired during the
v0.8.0 release (403 "The specified API key has expired"), and the upload had to be re-run with a new
key. nuget.org offers trusted publishing for GitHub Actions: `NuGet/login@v1` exchanges the job's
OIDC token (`id-token: write`) for a short-lived key, checked against a policy that names the
repository owner, the repository and the workflow file
(https://learn.microsoft.com/nuget/nuget-org/trusted-publishing). crates.io, PyPI, RubyGems and npm
already publish this way here.

Creating the nuget.org policy (`iscc` / `iscc-lib` / `release.yml`) is Titusz's step, and so is
naming the nuget.org profile for the action's `user:` input. The workflow change is CID work.

Resolved when `publish-nuget` authenticates through `NuGet/login`, nothing in `release.yml` reads
`NUGET_API_KEY`, the release skill says NuGet uses trusted publishing, and the secret is deleted.
