# Next Work Package

## Step: Fix the `packages/go` `Final_Sigma` case-mapping defect

## Goal

Close the only `critical` issue — "`packages/go` lowercases without `Final_Sigma` context — Greek
text non-conformant" — so `TextCollapse` and every code derived from it (Text-Code, Meta-Code) agree
with the reference on ordinary Greek text. Titusz sequenced this explicitly ahead of the freeze-rule
work, and it unblocks the `Final_Sigma` boundary vector for Go.

The handoff's "no unblocked `normal` work / write `## Step: NONE`" instruction is **stale** — it
predates the `human(decide)` commit `9aa25ad`, which cleared the human backlog and filed this
critical bug. This is a user-facing correctness fix, so the tooling-cadence rule does not bite.

## Scope

- **Modify** (1 non-test/non-doc file — well inside the standard budget):
    - `packages/go/utils.go`
- **Modify** (tests, excluded from the budget):
    - `packages/go/utils_test.go`
    - optionally `packages/go/code_content_text_test.go` and/or `packages/go/code_meta_test.go` for
        the two end-to-end assertions below
- **Reference**:
    - `.claude/context/issues.md` — the `critical` issue at the top of the file
    - `crates/iscc-lib/src/utils.rs` — `text_collapse` (~line 176): NFD → `to_lowercase()` → C/M/P
        filter → NFKC. The Go port must keep exactly this order.
    - `~/go/pkg/mod/golang.org/x/text@v0.40.0/cases/cases.go` — the `Caser` doc comment (read it; see
        the concurrency trap below)

## Not In Scope

- **Do not touch `crates/iscc-lib/src/utils.rs` or the freeze rule.** The sentinel conversion
    (`filter` → `map` onto `U+FFFF`) is the *next* step and trips two hot-path gates. This step must
    leave `crates/` byte-unchanged.
- **Do not add Unicode boundary vectors to the Go suite**, and do not add the `Final_Sigma` boundary
    vector anywhere. That propagation waits for the sentinel conversion, which changes expected
    sequence outputs.
- **Do not change `decodeBase32` in `packages/go/codec.go:398`.** Its `strings.ToUpper` operates on
    base32-alphabet input and any non-ASCII input fails the decode immediately — audited during
    scoping, it is the only other case-mapping call site and it is correct. Note the audit result in
    the commit message; do not "fix" it.
- **Do not add a new Go dependency.** `golang.org/x/text v0.40.0` is already a direct require and
    ships `cases` + `language`; `go.mod`/`go.sum` must not change.
- **Do not edit `docs/howto/go.md`.** Its "Unicode tables" note (lines 292–298) is about table-
    version drift, not case mapping, and stays true. `docs/unicode.md` is parked for the sentinel
    step.
- No `strings.ToLower` sweep of other packages, no perf refactor of `TextCollapse` beyond this fix.

## Implementation Notes

### The fix

In `packages/go/utils.go`, `TextCollapse` (line 117) currently does:

```go
nfdLower := strings.ToLower(norm.NFD.String(text))
```

`strings.ToLower` applies **unconditional simple** case mapping. Replace it with
`golang.org/x/text/cases`, which implements the conditional `Final_Sigma` special case:

```go
nfdLower := cases.Lower(language.Und).String(norm.NFD.String(text))
```

Keep the NFD-then-lowercase order (the Rust core lowercases *after* NFD). Add
`"golang.org/x/text/cases"` and `"golang.org/x/text/language"` to the imports; `strings` is still
used elsewhere in the file. Update the `TextCollapse` doc comment: say the lowercasing is
context-sensitive full Unicode case mapping (`Σ` → `ς` word-finally), matching the reference.

### Trap: do NOT hoist the `Caser` to a package-level `var`

The issue text recommends "a package-level `cases.Caser` (construct once)". **That recommendation is
wrong and must not be followed** — `cases.go` line 35 states: *"A Caser may be stateful and should
therefore not be shared between goroutines."* Only `cases.Fold` is documented as concurrency-safe.
`TextCollapse` is exported and callable concurrently, so a shared `Caser` is a data race.

Measured while scoping (x/text 0.40.0, 20 000 iterations over a ~1 KB Greek string):

| variant                                   | time   |
| ----------------------------------------- | ------ |
| `strings.ToLower` (shipped)               | 235 ms |
| `cases.Lower(language.Und)` **per call**  | 386 ms |
| shared package-level `Caser`              | 397 ms |
| construction alone (20 000 `Lower` calls) | 2.8 ms |

Construction is ~138 ns and hoisting it is **not faster** (it was marginally slower here). So
construct per call: correct, race-free, and free. Add a short comment saying why, so a future reader
does not "optimize" it into a shared var.

### Reference values (measured at HEAD against the Rust core, `iscc_lib.text_collapse`)

Add these as Go regression tests. Every input below is table-independent (no code point unassigned
in Unicode 15.0/16.0), so Go's older Unicode tables do not affect them and the pending sentinel
conversion will not change them.

| input            | Rust core / reference | Go today (wrong) |
| ---------------- | --------------------- | ---------------- |
| `ΛΟΓΟΣ`          | `λογος`               | `λογοσ`          |
| `ΑΣ`             | `ας`                  | `ασ`             |
| `ΣΣ`             | `σς`                  | `σσ`             |
| `ὈΔΥΣΣΕΎΣ`       | `οδυσσευς`            | `οδυσσευσ`       |
| `ΑΣΒ`            | `ασβ`                 | `ασβ` (agrees)   |
| `ΓΕΙΑ ΣΟΥ ΚΟΣΜΕ` | `γειασουκοσμε`        | agrees           |
| `İstanbul`       | `istanbul`            | agrees           |

Include the three agreeing cases too — they guard against over-correcting (a `Σ` followed by a cased
letter must stay `σ`).

### End-to-end assertions (the strongest available proof)

With `s = "ΤΟ ΓΡΗΓΟΡΟ ΚΑΦΕ ΑΛΕΠΟΥ ΠΗΔΑΕΙ ΠΑΝΩ ΑΠΟ ΤΟΝ ΤΕΜΠΕΛΗ ΣΚΥΛΟΣ"`:

- `GenTextCodeV0(s, 64)` → reference `ISCC:EAA36O3AT3YMFRFU`, `Characters: 48` (Go today returns
    `ISCC:EAA36O3IR2YMFRFU`)
- `GenMetaCodeV0("ΛΟΓΟΣ", nil, nil, 64)` → reference `ISCC:AAAXNTFLFFVJ2QUN` (Go today returns
    `ISCC:AAAX3SNL5HR3OSYB`; the `Name` and `Metahash` fields are unchanged either way, since
    `TextClean` does not lowercase)

Both reference values were produced today by the Rust core, which matches `iscc-core`. Assert them
literally.

## Verification

- `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go` passes with **≥ 171** `func Test`
    functions (165 today: `grep -c '^func Test' packages/go/*_test.go`), zero failures — all
    vendored `data.json` conformance vectors still green
- `go vet ./...` in `packages/go` exits 0
- `gofmt -l packages/go` prints nothing
- `git diff --stat -- packages/go/go.mod packages/go/go.sum` is empty (no dependency change)
- `git status --porcelain crates/ docs/ .claude/context/specs/` is empty (no core, docs or spec
    changes in this step)
- `grep -n 'strings.ToLower' packages/go/utils.go` returns **no match** (asserted directly on the
    working tree)
- `grep -n 'cases.Lower' packages/go/utils.go` returns a match, and
    `grep -c '^var .*cases\.Caser\|= cases\.Lower(' packages/go/utils.go` shows the `Caser` is
    **not** a package-level `var` (construction happens inside `TextCollapse`)
- A named test asserts `TextCollapse("ΛΟΓΟΣ") == "λογος"` and `TextCollapse("ΑΣΒ") == "ασβ"`
- A named test asserts
    `GenTextCodeV0("ΤΟ ΓΡΗΓΟΡΟ ΚΑΦΕ ΑΛΕΠΟΥ ΠΗΔΑΕΙ ΠΑΝΩ ΑΠΟ ΤΟΝ ΤΕΜΠΕΛΗ ΣΚΥΛΟΣ", 64)` yields
    `ISCC:EAA36O3AT3YMFRFU`
- `mise run check` — all hooks Passed

## Done When

`packages/go` lowercases with context-sensitive full Unicode case mapping, the Greek regression and
end-to-end tests assert the reference values above, `go test`/`go vet`/`gofmt` are clean, and
neither `go.mod`/`go.sum` nor anything under `crates/` or `docs/` changed.
