# Next Work Package

## Step: Remove hand-written counts and line numbers from the specs and per-crate CLAUDE.md files

## Goal

Delivers the count and line-number part of "Derived facts in guidance and docs are written by hand
and drift" (`normal`, the last v0.8.0 blocker). Every count, test tally, file size or `path:NN`
reference in `.claude/context/specs/` and the 12 per-crate/package `CLAUDE.md` files that repeats
what the code, the vectors or a workflow matrix already says is removed or reworded so it cannot go
stale.

## Alternatives Considered

- **Chosen:** counts + line numbers in both directories. It is one class rule applied to plain text,
    docs-only, and it removes known-wrong facts: `ci-cd.md` says "9 sync targets" (there are 21),
    `java-bindings.md` says "65 tests", the Python spec says "9 typed result classes" where
    `iscc-py/CLAUDE.md` says 10.
- **Rejected:** file lists first (the indentation trees in the `CLAUDE.md` files). No gate parses
    that tree format, so the step would need a remove-or-gate decision and probably a change to
    `scripts/check_spec_paths.py`. That is a different kind of change and gets its own step next.

## Scope

**Size rule:** docs-only. Only `.md` guidance files are touched, and no source, script or test file,
so the 6-file cap does not apply.

- **Modify**: `.claude/context/specs/*.md` (every spec with a hit) and `crates/*/CLAUDE.md`,
    `packages/*/CLAUDE.md` (every file with a hit)
- **Reference**: `.claude/context/README.md` → "Hygiene rules" → "Derived facts are checked or not
    written"; the registry in `scripts/check_tier1_docs.py` (`FULL` / `EXACT`: the symbol *names* in
    `rust-core.md`, `crates/iscc-lib/CLAUDE.md`, `crates/iscc-wasm/CLAUDE.md`,
    `crates/iscc-ffi/CLAUDE.md` and `packages/go/CLAUDE.md` are gated; keep every name, drop only
    the numerals)

## Not In Scope

- File lists and directory trees in the `CLAUDE.md` files and specs: the next step
- Versions (JUnit 5 vs 6, .NET 8, C++17, Kotlin 2.3) even where stale; `target.md` (human-owned);
    root `CLAUDE.md`; `docs/`, READMEs, `notes/`
- Adding a new gate or hook that bans counts: that would be new policy
- Editing `issues.md`: the drift issue stays open, and review rewrites it to the file-list part

## Implementation Notes

**Remove, never correct.** Replacing "9 sync targets" with "21 sync targets" leaves a number that
drifts again. Write "every sync target", "all Tier 1 symbols", "every binding", "every release
platform", "every `gen_*_v0` function with `data.json` vectors".

**The grep in Verification is a floor, not the worklist.** Counts split across a line break ("all 9"
then "gen\_\*\_v0" on the next line, `python-bindings.md:4`) and spelled-out counts ("All four
functions", "Five codec functions", `rust-core.md` sections) escape it. Read each file with a hit
against the class rule below.

**Remove (in scope):**

- Tier 1 totals and per-category numerals: "35 public symbols", "4 text utilities:", "10 gen
    functions:", "(+ the …)". Keep the symbol lists themselves. In `rust-core.md` drop the
    `Total: 10 gen functions + … = 35` arithmetic, or replace it with one line saying that
    `scripts/check_tier1_docs.py` keeps the list above in parity with `lib.rs`
- Counts of bindings or surfaces ("all 11 bindings", "the other 10 bindings")
- `gen_*_v0` counts ("all 10", "all 9", "9 of 10"). The 9-versus-10 split (`gen_sum_code_v0` has no
    `data.json` vectors) is said in words, never as a number
- Vector, test and pass tallies: "65 tests", "135 mocha tests", "286 passed", "50 vectors", "4 + 4
    vectors", "19 test methods", "85 Rust + 65 C tests", "7 … sites", "12 total `py.detach` sites".
    A Verified-when bullet keeps its behavioural claim and loses the tally
- Platform and target counts: "5 platforms", "all 8-9 platforms", "5 desktop + 3-4 Android", "5 Rust
    targets … 3 platform slices". Where the platforms are named next to the count, keep the names
    and drop the numeral
- Sync-target, job and check counts: "all 9 sync targets", "the 21 job keys surface as 22 check
    names" (`check_ci_job_table.py` gates the table rows, not this prose). Keep the sentence about
    the `python-test` matrix and its aggregator, without the numbers
- File sizes: "~3200 lines", "~400–500 lines", "~100 lines of thin wrapper"
- Line numbers: `crates/iscc-napi/src/lib.rs:97`, `packages/go/iscc_id_test.go:160` (keep the path,
    drop `:NN`; `check_spec_paths.py` still checks the path) and "line 58 in extensiontask.rb" in
    `iscc-rb/CLAUDE.md` (name the method or call instead)

**Keep (out of scope):**

- Algorithm and format parameters fixed by the standard or the code's contract: 64 KiB, 256
    permutation pairs, 380 int32 values, 52/12 bit packing, hub_id 0–4095, 600,000 units, 16-bit
    header, `4 * (ln + 1)` bytes
- Split-threshold policy ("do not split unless the file exceeds ~1500 lines"): a rule, not a fact
- Quoted error text ("at least 2 ISCC unit codes"), JNI signature descriptions next to the literal
    ("four ints + byte array"), external-tool facts ("wasm-pack supports three build targets"),
    issue numbers, dates, iteration numbers, "two strategies" or "three policy classes" when the
    items are listed in the same paragraph
- `rust-core.md` text that names the count rule in general terms ("N Tier 1 symbols on every
    surface")

mdformat reflows shortened lines. Run `mise run format` before committing. A double space can appear
inside an inline code span; reword rather than fight it.

## Verification

- `git grep -nE '\.(rs|py|go|cs|kt|swift|rb|java|hpp|h|c|js|ts|toml|yml|json).?:[0-9]+' -- .claude/context/specs 'crates/*/CLAUDE.md' 'packages/*/CLAUDE.md'`
    prints nothing

- This command prints nothing:

    ```sh
    git grep -nE '\b[0-9]+ (Tier 1|.?gen_|gen |conformance|vectors?\b|tests?\b|passed\b|sync targets|platforms?\b|platform targets|desktop|Rust targets|bindings|binding surfaces|surfaces|job keys|check names|typed|streaming|text util|algorithm|encoding|codec|constants|diagnostic|soft hash|ISCC-SEQ|helper functions|hasher classes|ISCC-IDv1|public symbols|test methods|methods|mocha|Rust \+|C tests|.?py\.|new\b|one-shot)' -- .claude/context/specs 'crates/*/CLAUDE.md' 'packages/*/CLAUDE.md' | grep -v 'JUnit [0-9]'
    ```

- `git grep -nE '~ ?[0-9][0-9,–-]* lines|\bline [0-9]+ in' -- .claude/context/specs 'crates/*/CLAUDE.md' 'packages/*/CLAUDE.md' | grep -v exceeds`
    prints nothing

- `uv run scripts/check_tier1_docs.py && uv run scripts/check_spec_paths.py && uv run scripts/check_ci_job_table.py`
    exits 0

- The advance commit touches only `.claude/context/specs/*.md`, `crates/*/CLAUDE.md` and
    `packages/*/CLAUDE.md` (`git show --stat HEAD`)

- `mise run check` exits 0

## Done When

All six verification criteria pass, and a read of each touched file finds no remaining count of the
in-scope classes, including spelled-out and line-wrapped ones.
