# Handoff

> **HUMAN REVIEW REQUESTED**: no CID-doable `normal` work remains. All three open `normal` issues
> are now blocked on you or on an external release — uniffi 0.32 (binding regeneration) and
> criterion 0.8 (MSRV policy call), the `specs/java-bindings.md` edit (human-owned spec), and the
> go1.27 tripwire (go1.27 is still at rc2 upstream). Strict `**IDLE**` is not met, so the loop
> pauses instead. See **Next:** for the two candidate moves.

## 2026-07-28 — Review of: Pin the .NET test dependencies and commit a NuGet lock file

**Verdict:** PASS

**Summary:** The last floating dependency surface in the repo is closed: three exact pins, a
committed `packages.lock.json`, and a CI restore in `--locked-mode` with `--no-restore` on build and
test. I re-derived the gate rather than trusting it — the exact CI sequence runs green on a cold
tree, and two independent mutations (csproj drift, tampered lock entry) both red it. The Ruby doc
line now matches `crates/iscc-rb/src/lib.rs` verbatim.

**Verification:**

- [x] No floating range in the test csproj — `grep -E 'Version="[^"]*\*"'` exits 1
- [x] Lock file tracked; all three direct refs have `type: Direct` and `resolved` == csproj
    `Version` (18.8.1 / 3.2.2 / 3.1.5), and the lock's Direct set equals the csproj ref set exactly
- [x] `dotnet restore … --locked-mode` exits 0, `git status --porcelain packages/dotnet` empty; lock
    md5 unchanged. Also verified **cold** (fresh `git archive` tree, no `obj/`) — the warm run only
    prints "All projects are up-to-date for restore" and proves nothing
- [x] `cargo build -p iscc-ffi` + `dotnet test … -e LD_LIBRARY_PATH=…` → **Failed: 0, Passed: 104,
    Total: 104**, matching decisions.md 2026-07-27
- [x] `grep -n "RString::from_slice" crates/iscc-rb/CLAUDE.md` exits 1; line 108 teaches
    `ruby.str_from_slice(&bytes)` — confirmed against `src/lib.rs` (5 call sites via `Ruby::get()`)
    and magnus 0.8.2 `r_string.rs:145`
- [x] `mise run check` → exit 0, 18 hooks Passed, tracked tree unchanged

**Extra probes (3 of 3 budget):**

- **Mutation 1** (csproj drift, lock untouched, cold): NuGet errored `NU1004` — "the package
    reference xunit.v3 version has changed" — exit **1**. **Mutation 2** (tampered transitive
    `resolved` in the lock): `NU1403`, package content hash validation failed, exit **1**. Both
    halves of the gate bite.
- **Exact CI sequence replayed cold** in a throwaway tree: locked restore, then build and test with
    `--no-restore` — 0 warnings, 0 errors, 104 passed, lock byte-identical after. `--no-restore`
    does not skip compilation, and a plain (unlocked) restore does not rewrite the committed lock.
- **Action-freshness claim re-derived independently** (it drove an issues.md edit, so I did not take
    it on the handoff's word): 25 distinct `uses:` refs, `git/matching-refs/tags/v<N+1>` empty for
    all 23 versioned ones, both exact pins == their publisher's `releases/latest`. Confirmed no-op.

**Issues found:**

- (none) Scope is exactly next.md's list — 3 non-test/non-doc files, no `Not In Scope` item touched
    (`release.yml`, `specs/java-bindings.md`, `issues.md`, other ecosystems' lockfiles all
    untouched). Gate-integrity scan over all unpushed commits: no suppression, skip, threshold
    change or hook weakening; no Tier 1/2 API change; no benchmarked path.

**Issues updated:** the doc-drift entry lost its resolved Ruby bullet (now Java-spec-only, still
`HUMAN REVIEW REQUESTED`); the dependency entry lost the `release.yml` action bullet and records the
2026-07-28 probe evidence instead. Both remaining items there are human-gated.

**Codex review:** Clean — "The locked NuGet restore succeeds, and the updated no-restore build/test
flow passes all 104 .NET tests. No actionable regressions were found."

**Next:** There is no `normal` issue CID can schedule and no unmet gap in state.md that does not
route through you. Two candidate moves, in order of my recommendation:

1. **Decide the two held Rust majors** — `criterion` 0.8 needs an MSRV call (it wants rustc 1.86 vs
    the declared `rust-version = "1.85"`), and `uniffi` 0.32 needs authorization to regenerate the
    Swift and Kotlin bindings. Either decision unblocks a concrete CID step immediately.
2. **Authorize the `specs/java-bindings.md` correction** (jni 0.21 → 0.22, "~1060 lines" → 1152 at
    lines 13/22/32) — a two-minute edit that closes the last `[review]` doc-drift entry.

If neither is wanted now, the honest state is a paused loop, not idle work to invent.

**Notes:**

- **go1.27 is at rc2 upstream** (`refs/tags/go1.27rc1`, `go1.27rc2`; no final tag). When the final
    lands, the tripwire issue becomes schedulable and must land the 731-range freeze table in
    `packages/go/utils.go` in the *same* step. `go-version-file: packages/go/go.mod` means nothing
    flips without a deliberate bump — CI will not surprise you.
- The .NET lock file carries a prek-added trailing newline that NuGet never rewrites; a version bump
    must run `dotnet restore … --force-evaluate` in the same commit or CI reds with `NU1004`.
- `learnings.md` was at its 199-line ceiling; the behavioural-UCD entry (156, fully superseded by
    the vendored gated table) moved to `learnings-archive.md` to make room. No decisions.md entry —
    this was a routine reproducibility pass with no trade-off a reader could not reconstruct.
