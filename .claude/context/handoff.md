## 2026-11-20 — Review of: Pre-release security pass — bump advisory-flagged pins and EOL test runtimes

**Verdict:** PASS

**Summary:** The step bumps rustix to 1.1.6 and `golang.org/x/text` to v0.41.0, tests Go on
go1.26.11 through a `toolchain` line while the `go 1.26.1` floor stays, moves the napi CI job to
Node 22, and adds a Runtimes row to the `ci-cd.md` freshness table. The diff matches next.md
exactly, no floor was raised and no core code changed. The security issue is closed; two follow-ups
are filed.

**Verification:**

- [x] `Cargo.lock` locks `rustix` at `1.1.6`, no rustix `1.1.4` entry; `cargo deny check` exits 0 —
    cargo-deny 0.19.9 (prebuilt, as in CI): "advisories ok, bans ok, licenses ok, sources ok"
- [x] `go.mod` reads `go 1.26.1`, `toolchain go1.26.11`, `golang.org/x/text v0.41.0`;
    `CGO_ENABLED=0 go test ./...` ok, `go vet ./...` clean, govulncheck prints 0 `Found in:` lines
    ("No vulnerabilities found", go1.26.11 stdlib + x/text@v0.41.0)
- [x] `ci.yml` has no `node-version: '20'`; the napi job has `node-version: '22'`
- [x] Dockerfile has `ARG GO_VERSION=1.26.11`; the `curl -fsIL` on the go1.26.11 tarball exits 0
- [x] `ci-cd.md` has no `` `mise.toml` tool pins `` and names `.devcontainer/Dockerfile` in the
    table; `uv run scripts/check_spec_paths.py` exits 0
- [x] `mise run check` exits 0

Beyond next.md: `diff -rq` of x/text v0.40.0 against v0.41.0 differs only in `secure/precis` and
`go.mod`/`go.sum`, so the Unicode tables did not move. setup-go v7 reads `go-version-file` before it
sets `GOTOOLCHAIN=local`, so CI installs the `toolchain` version. osv-scanner over Cargo, uv,
Gemfile, Maven, .NET and Go reports only the three unmaintained crates that `deny.toml` already
ignores. `go mod tidy -diff` and `go mod verify` are clean.

**Issues found:**

- Closed: "Dependencies and language runtimes are not checked for security releases before v0.8.0".
    Its third condition (CI green on the refreshed set) can only be read after this push:
    update-state must confirm it, and the `go` job log must show go1.26.11
- Filed `normal` `[review]`: "Precompiled Ruby gems stop at Ruby 3.3". The published platform gems
    declare `required_ruby_version < 3.4.dev` (rubygems.org), so Ruby 3.4 and 4.0 users get a source
    build that needs a Rust toolchain. Titusz may demote it
- Filed `low` `[review]`, untriaged: "Three supported-version floors are past end of life" (Python
    3.10, .NET 8, Ruby 3.1, with dates). This is the list the closed issue asked for; it is a
    decision for Titusz and CID must not raise a floor
- Fixed here (stale facts, no behaviour): `docs/development.md` tool table named Python 3.12,
    Node.js 20 and Go 1.24 (the Dockerfile has 3.13, 22 and 1.26.11) and said mise holds tool
    versions; `ci-cd.md` → "Tool Management" said the same about mise; the Dockerfile comment now
    names the `toolchain` line
- Not filed: `release.yml` has one `macos-latest` in a smoke matrix (line 1010) beside the pinned
    `macos-15` jobs. Pre-existing, outside this step

**Codex review:** No actionable findings. Codex ran the Rust core tests, the UniFFI build check, the
napi tests on Node 22, and the Go tests on both go1.26.11 and the go1.26.1 floor; all passed.

**Next:** Three `normal` items from the previous handoff are still open, plus the new Ruby one:

1. The output-subset rule in `edge-vectors.md` with a decisions.md entry. It is the design step that
    the four strict data-edge compares (.NET, Java, napi, WASM) wait on
2. The Java how-to `genSumCodeV0` example, a small docs fix that can ride with another docs step
3. The Tier 1 doc-gate holes (docs below the discovery threshold, fail-open crate-root forms)
4. Precompiled Ruby gems: the cheap resolution is to state the covered Ruby versions in the how-to
    and README; extending the cross-gem list needs an rb_sys bump with static evidence only

**Notes:**

- Decision recorded: Go CI tests on the `toolchain` line, the `go` directive stays the consumer
    floor (decisions.md, 2026-11-20). CI no longer runs on go1.26.1 itself
- Whether to add an advisory gate beyond `cargo deny` (osv-scanner, govulncheck) is a policy call
    for Titusz and is not filed. `cargo deny` was green with rustix 1.1.4 while osv listed the
    advisory, so the two sources differ. learnings.md has the one-line osv-scanner command
- npm, Gradle and the GitHub Actions pins are not covered by osv-scanner here; they rest on
    define-next's probes in next.md (fresh `npm audit` 0, Kotlin and Actions pins above the affected
    ranges)
- No core code changed, so CRAP, iai and semver are unaffected
