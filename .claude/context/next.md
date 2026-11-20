# Next Work Package

## Step: Pre-release security pass — bump advisory-flagged pins and EOL test runtimes

## Goal

Pick up "Dependencies and language runtimes are not checked for security releases before v0.8.0"
(`normal`, `[human]`). Bump the two locked dependencies that have a fixed advisory release, move the
test-only runtimes off end-of-life or patch-stale versions, and make the `ci-cd.md` freshness table
name the runtime surface. Decision for Titusz: Go CI tests on the latest 1.26 patch through a
`toolchain` line in `go.mod`, no longer on 1.26.1 itself. The `go 1.26.1` directive (the floor that
consumers see) stays as it is.

## Alternatives Considered

- **Chosen:** the security pass. It is the newest release blocker, the probes below found real
    fixed-advisory pins, and every change is small and locally checkable
- **Rejected:** strict compares in the 4 data-edge harnesses. On .NET even `TextCodeResult` carries
    only `Iscc`, and napi/WASM/Java return a bare string, so a strict compare fails
    `gen_text_code_v0/test_0001_embedded_nul` (it expects `characters`) on all four. That needs an
    output-subset rule in `edge-vectors.md` plus a decisions.md entry before any harness changes. It
    is a design step and comes later. The Java how-to fix and the doc-gate holes wait too

## Scope

- **Modify**: `packages/go/go.mod`, `.github/workflows/ci.yml`, `.devcontainer/Dockerfile`,
    `.claude/context/specs/ci-cd.md` (doc)
- **Regenerated (not counted)**: `Cargo.lock`, `packages/go/go.sum`
- **Reference**: issues.md entry above; `ci-cd.md` → "Dependency Freshness"

## Not In Scope

- Raising any floor: `go 1.26.1` directive, abi3-py310, Java 17, .NET 8 (`dotnet-version: '8.0'`,
    devcontainer `--channel 8.0`), Ruby 3.1 (CI `ruby-version: '3.1'`, devcontainer apt `ruby`),
    MSRV 1.85, the `ruby-versions: 3.1, 3.2, 3.3` cross-gem list in `release.yml`. List them for the
    human (see notes) and change none
- Adding an advisory CI gate (osv-scanner, govulncheck, npm audit job): that is new policy and needs
    Titusz's sign-off
- `deny.toml` ignores for the 3 unmaintained crates (basic-toml, bincode, proc-macro-error2). They
    have no fixed release and are already justified
- Bumping x/text past v0.41.0, or any other dependency without an advisory (freshness is not this
    step)
- Writing EOL dates into `ci-cd.md`. Hand-written dates in a spec are the drift that the
    derived-facts issue is about, so they go in the handoff only

## Implementation Notes

Probed while scoping (2026-11-20). Do not redo the clean ones:

- **Rust:** `cargo update -p rustix` moves only rustix 1.1.4 → 1.1.6 (RUSTSEC-2026-0486, xattr
    soundness; reached via tempfile ← uniffi_pipeline and dev deps). CI `cargo deny` did not flag
    it, so osv is the source here
- **Go:** `golang.org/x/text` v0.40.0 → **v0.41.0** (GO-2026-6629, a panic in `secure/precis`, which
    we don't call). A full diff of v0.40.0 against v0.41.0 changes only `secure/precis/*.go` and its
    go.mod (`go 1.25.0`). The Unicode tables are unchanged (`tables15.0.0.go` + `tables17.0.0.go` in
    both), so the text/Unicode output cannot move. Use
    `go get golang.org/x/text@v0.41.0 && go mod tidy`
- **Go toolchain:** govulncheck finds about 40 stdlib advisories in go1.26.1, all fixed by
    **go1.26.11** (latest 1.26 patch; go1.27.4 is also current). setup-go v7's README says: "If the
    `toolchain` directive is present, its version is used; otherwise … falls back to the `go`
    directive". So add `toolchain go1.26.11` to `go.mod` and keep `go-version-file` in `ci.yml`.
    Consumers ignore a dependency's `toolchain` line. Set `ARG GO_VERSION=1.26.11` in the
    Dockerfile. Local `GOTOOLCHAIN=auto` downloads go1.26.11 on the first `go` command
- **Node:** the `ci.yml` napi job uses `node-version: '20'` (EOL 2026-04-30); make it `'22'`, the
    oldest supported LTS (EOL 2027-04-30), the same as the `release.yml` smoke jobs
- **Clean, nothing to change:** npm (`package-lock.json` is gitignored and CI runs `npm install`; a
    fresh resolve of `@napi-rs/cli ^3` pulls js-yaml 5.5.0 and `npm audit` reports 0 — the local
    stale lock's js-yaml 4.x finding is not a pin), Python (`pip-audit`: none), .NET (both projects:
    `dotnet list … package --vulnerable --include-transitive` reports none), Ruby `Gemfile.lock`,
    Maven `pom.xml` deps and plugins, Kotlin (jna 5.19.1, junit 6.1.2, gson 2.14.0, kotlin 2.4.10),
    GitHub Actions pins (the osv hits on download-artifact/pypi-publish affect versions below our
    `@v8` / `release/v1`)
- **`ci-cd.md` table:** delete the stale "`mise.toml` tool pins" phrase (`mise.toml` has only
    `[tasks]`). Add a "Runtimes" row naming the setup-action `*-version` inputs in
    `.github/workflows/`, the `go.mod` `toolchain` line, and the `.devcontainer/Dockerfile` base
    image and `GO_VERSION`. Add one sentence: supported-version floors are compatibility promises
    that only the human raises
- **Handoff, for the human (EOL dates from endoflife.date, 2026-11-20):** floors that are already
    past EOL: Python 3.10 (2026-10-01), .NET 8 (2026-11-10), Ruby 3.1 (2025-03-26). Cross-gem build
    ABI Ruby 3.2 (2026-03-31). Java 17 and MSRV 1.85 are not EOL. Also worth filing: the precompiled
    gem ABIs stop at 3.3, so Ruby 3.4 / 4.0 users may get no precompiled gem (check the gemspec's
    fallback before filing)
- No core code changes, so CRAP, iai and semver are unaffected

## Verification

- `Cargo.lock` locks `rustix` at `1.1.6` and no `1.1.4` entry remains; `cargo deny check` exits 0
- `packages/go/go.mod` reads `go 1.26.1`, `toolchain go1.26.11` and `golang.org/x/text v0.41.0`. In
    `packages/go`, `CGO_ENABLED=0 go test ./...` and `go vet ./...` pass, and
    `go run golang.org/x/vuln/cmd/govulncheck@latest -show verbose ./...` prints no `Found in:` line
- `.github/workflows/ci.yml` has no `node-version: '20'` and the napi job has `node-version: '22'`
- `.devcontainer/Dockerfile` has `ARG GO_VERSION=1.26.11`, and
    `curl -fsIL https://go.dev/dl/go1.26.11.linux-amd64.tar.gz` exits 0
- `ci-cd.md` no longer contains `` `mise.toml` tool pins `` and names `.devcontainer/Dockerfile` in
    the freshness table; `uv run scripts/check_spec_paths.py` exits 0
- `mise run check` exits 0

## Done When

All six verification criteria pass, and the handoff lists the EOL floors for the human.
