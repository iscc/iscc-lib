# Handoff

## 2026-07-25 — Review of: Dependency refresh slice 6 — Go module (`go.mod`/`go.sum`) + stale Kotlin README version target

**Verdict:** PASS_WITH_NOTES

**Summary:** The Go module refresh is clean and, unusually for a dependency bump in a
Unicode-sensitive path, provably output-neutral: `TextClean`/`TextCollapse` are byte-identical
across all 1,112,032 code points under `golang.org/x/text` 0.34.0 and 0.40.0. All three modules sit
at their latest published versions, the `go 1.26.1` consumer floor and `zeebo/blake3` are untouched,
and the `packages/kotlin/README.md` version string is now managed by `scripts/version_sync.py`
instead of hand-edited. Establishing the neutrality baseline surfaced a **pre-existing, unrelated**
conformance divergence — the Rust core disagrees with both the Go package and `iscc-core` on
characters assigned after Unicode 15 — which is filed as its own issue, not held against this step.

**Verification:**

- [x] `CGO_ENABLED=0 go test -C packages/go -count=1 ./...` — exit 0 (`ok … 0.127s`, conformance
    vectors included)
- [x] `go vet -C packages/go ./...` — exit 0
- [x] `go mod tidy -C packages/go -diff` — exit 0, module graph tidy
- [x] `grep 'golang.org/x/text v0\.40\.0' packages/go/go.mod` — line 7; **not** held back, and
    independently confirmed the latest on `proxy.golang.org`
- [x] `grep 'github.com/klauspost/cpuid/v2 v2\.4\.0' packages/go/go.mod` — line 11, latest published
- [x] `grep 'github.com/zeebo/blake3 v0\.2\.4' packages/go/go.mod` — line 6, unchanged, latest
    published
- [x] `grep -c '^go 1\.26\.1$'` → 1 and `grep -c '^toolchain'` → 0 — consumer floor untouched
- [x] `grep 'packages/kotlin/README.md' scripts/version_sync.py` — line 288
- [x] `mise run version:check` — exit 0, `OK: packages/kotlin/README.md = 0.5.0` (22 targets, not 21
    as the advance handoff said)
- [x] `grep -rn '0\.3\.1' packages/kotlin/README.md` — exit 1, no matches
- [x] `mise run check` — exit 0, all 15 hooks passed, nothing rewritten (working tree clean apart
    from runner-owned `iterations.jsonl`)
- [x] `git status --porcelain crates/ .crap-baseline.json` — empty;
    `git diff --name-only   @{upstream}..HEAD` touches no file under `crates/`
- [x] Gate-integrity sweep over `@{upstream}..HEAD` — no suppressions, skips, threshold cuts, hook
    weakening, or scope exclusions

**Extra verification run by review (beyond next.md):**

- **The `x/text` bump is output-neutral, not merely vector-green.** Built a throwaway module with
    `replace github.com/iscc/iscc-lib/packages/go => <repo>/packages/go`, dumped
    `TextClean`/`TextCollapse` for every code point `0x20..0x10FFFF` (excluding surrogates), then
    re-ran under `replace golang.org/x/text => golang.org/x/text v0.34.0`: **identical, all
    1,112,032 lines.** Also identical on 13 invalid-UTF-8 byte sequences (`norm.NFKC.Bytes` /
    `NFD.Bytes`), which is where 0.40.0's internal refactor actually landed. Root cause of the
    non-event: `unicode/norm/tables15.0.0.go` and `tables17.0.0.go` are byte-identical between the
    two releases and `tables17` is gated `//go:build go1.27`, so the module compiles against Unicode
    15.0.0 tables either way.
- `golang.org/x/sys v0.47.0` is a genuine, required indirect: `cpuid/v2@v2.4.0`'s `go.mod` declares
    `require golang.org/x/sys v0.41.0`. Not cruft, not scope creep.
- Go-directive pressure re-checked from the module cache: `x/text` 0.40.0 → `go 1.25.0`, `x/sys`
    0.47.0 → `go 1.25.0`, `cpuid/v2` 2.4.0 → `go 1.24.0`. All below the 1.26.1 floor. (The advance
    handoff attributed `go 1.24.0` to `x/sys`; it belongs to cpuid. Conclusion unaffected.)

**Issues found:**

- **New (filed, `normal` `[review]`, HUMAN REVIEW REQUESTED): the Rust core diverges from
    `iscc-core` and the Go package on Unicode 16/17 characters.** Go stdlib `unicode` = 15.0.0,
    Python 3.13 `unicodedata` = 15.1.0, but Rust `unicode-general-category` 1.1.0 = 16.0.0 and
    `unicode-normalization` 0.1.25 = 17.0.0. Go's `unicode.C` includes unassigned (Cn), so
    post-Unicode-15 characters are stripped by Go and by the reference but kept by Rust. Sweep found
    5,813 `text_clean` and 5,750 `text_collapse` disagreements. Reproduced end to end with `Ɤ`
    (U+A7CB): Go and `iscc-core` both give `meta=ISCC:AAARDZ4ASOMVXBRR` /
    `text=ISCC:EAA7VW5ZOQZ3XEMT`, the Rust core gives `AAARDZ5SS6NVXBLT` / `EAA3RXNBOM77TGM5` — so
    the core, and all 10 non-Go bindings, are the outlier. No gate catches it; every vendored vector
    predates Unicode 16. **Not caused by this diff** (proven above). See issues.md for options.
- Two cosmetic inaccuracies in the advance handoff (target count 21 vs 22; the `go 1.24.0`
    attribution). Recorded above, no action needed.
- No scope, correctness, dead-code, or gate-integrity problems. Diff is 2 non-test/non-doc source
    files, well inside the 3-file cap.

**Codex review:** Ran clean — "The dependency updates are compatible with the module's existing Go
floor, tests and representative cross-builds pass, and the Kotlin README is correctly integrated
into version synchronization." No findings to action.

**Next:** The dependency-refresh issue has one CID-actionable slice left that is fully verifiable
locally: **`crates/iscc-rb/Gemfile` + `iscc_lib.gemspec`** (bundle update; note the `magnus` 0.8
migration stays its own step, and `rb_sys` in `Gemfile.lock` must keep matching the
`oxidize-rb/actions/cross-gem` Docker image tag — a mismatch causes rbconfig errors). After that the
remaining refresh work is either human-gated or deferred by design (release.yml action refs, ruff
0.16 adoption, xunit 3 / Test.Sdk 18 / Gradle wrapper 9 / JUnit 6 majors). Alternatively, take the
**ruff 0.16 adoption** step — self-contained, entirely local, and it retires a documented hold-back
pin.

**Notes:**

- **Two `normal` issues now carry HUMAN REVIEW REQUESTED** (Kotlin consumer floor from iter 128, and
    the new Unicode-version divergence). Both are support/conformance-policy calls reserved for the
    owner; neither blocks the remaining refresh slices, so the loop is not idle.
- The Unicode divergence is unstable in *both* directions: when CPython ships Unicode 16 (3.14),
    `iscc-core` will move to the Rust side and away from Go. Any fix must decide a version, not
    chase the reference. There is also an upstream angle — ISO 24138 does not pin a Unicode version.
- `scripts/version_sync.py` now manages 22 targets. With `packages/kotlin/README.md` wired in, no
    known unmanaged version string remains in the repo — a `grep -rn '0\.3\.1'` sweep of `packages/`
    and `docs/` is a cheap way for a future step to confirm that stays true.
- The differential-under-`replace` technique (dump outputs for the whole code space, swap one dep
    version, `diff`) generalizes to any Unicode/locale-table dependency and costs about two minutes.
    Recorded in learnings.md.
