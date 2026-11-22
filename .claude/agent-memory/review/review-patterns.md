# Detailed Review Patterns

Moved from MEMORY.md to keep it under 200 lines. Referenced from MEMORY.md.

## Documentation Review Patterns

- **C FFI type prefix**: cbindgen config has `[export] prefix = "iscc_"` for types but
    `[fn] prefix = ""` for functions. Code examples must use `iscc_FfiDataHasher`,
    `iscc_IsccSumCodeResult`, etc. The advance agent used short names — always cross-check type
    names in C code blocks against `crates/iscc-ffi/include/iscc.h`
- Always verify documented API names against actual binding source code attributes (`js_name`,
    `#[pyfunction]`, `#[napi(js_name)]`) — next.md specs may have incorrect naming that the advance
    agent faithfully reproduces
- WASM constants have `js_name = "META_TRIM_NAME"` (uppercase) despite Rust function being
    `meta_trim_name()` — this is a known divergence point
- Cross-check version requirements in docs against build config files (e.g., `pom.xml`
    `maven.compiler.source`, `go.mod` go version). Advance agents may introduce version claims that
    don't match actual build requirements (e.g., Java 11+ claimed but pom.xml requires 17+)
- Doc tab conversions: verify WASM `init()` is shown at least once. Each language's return type
    differs (Python/Rust/Go return result structs; Node.js/Java/WASM return plain strings)
- **Count-sensitive docstrings**: "10 gen functions" is correct for library-wide descriptions, but
    conformance tests, benchmarks, and type stubs that reference data.json should say "9" (no
    gen_sum_code_v0 vectors exist). Always verify the actual code scope before accepting numeric
    count changes in docstrings. Blanket find-replace of counts across all files is error-prone
- **External project claims**: When updating counts/coverage for external projects (e.g.,
    iscc-core-ts in ecosystem.md), verify against the function table in the same file. Don't assume
    all projects implement the same set of functions

## Verification Patterns

- `grep -c` counts ALL matching lines including function definitions — when next.md specifies "4
    call sites" but the function name also appears in a definition, expect count = call sites + 1.
    This is a valid pass if the arithmetic checks out
- `grep -c '---' site/llms-full.txt` does NOT reliably count page dividers — doc pages contain
    internal `---` horizontal rules. Use the script's "N pages" stdout as the authoritative check

## Issues Cleanup

- The review agent only cleans up issues resolved in the *current* iteration's advance step. It does
    NOT sweep the full issues.md backlog for stale entries resolved in prior CID loops. This led to
    issues #5-#8 persisting for 4+ iterations after their fixes landed. **Mitigation:** after
    verifying the advance work, also scan issues.md for any other entries that are now resolved
    (check state.md "met" sections against issue descriptions)

## Claim-Probing Recipes (resolved cases, still-live technique)

- **Backend-activation claims — verify the build.rs gating** (iters 117-118, #42 RESOLVED): don't
    trust "flag set → goal met". blake3 WASM SIMD is activated by the `blake3/wasm32_simd` **Cargo
    feature** (`CARGO_FEATURE_WASM32_SIMD` → `blake3_wasm32_simd` cfg → `Platform::WASM32_SIMD`),
    NOT `-C target-feature=+simd128` alone. Honest wiring proof:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` must show
    `wasm32_simd`. Counting `v128` opcodes ALONE is a FALSE POSITIVE (LLVM auto-vectorizes the
    portable path). NUANCE (118): blake3's SIMD fns carry `#[target_feature(enable = "simd128")]`,
    so a no-flag `cargo build --target wasm32-unknown-unknown` still compiles the backend — the
    global RUSTFLAGS only broadens simd128 to the whole crate. Test a no-flag wasm build before
    accepting any "the flag is needed so intrinsics compile" doc claim
- **Decode body-length must be EXACT, not `>= nbytes`** (found iter 119; Go 120, Rust core 121 —
    RESOLVED): a `len(tail) < nbytes` guard silently drops trailing base32 bytes, so `...AB` aliases
    `...ABAA` — codec-wide, NOT ID-specific. Both `IsccDecode` and `iscc_decode` now use a 2-branch
    "too short"/"too long" form (keeps the "too short" message/test intact); all 11 bindings inherit
    the Rust fix. Non-breaking for the Tier 1 symbol — rationale in `decisions.md` 2026-07-24.
    Composite `iscc_decompose` legitimately consumes trailing units — do NOT harden it. Probe
    exact-length rejection with a throwaway test whenever new decode/parse surface lands
- **Text-pipeline changes need a SEQUENCE differential, not per-code-point tests** (found iter 133,
    fixed by the sentinel map iter 148 — technique stays live for every `utils.rs` text edit). Any
    edit that changes character *adjacency* silently alters context-sensitive Unicode operations,
    and a per-code-point sweep scores such a bug **0 failures**. Bulk recipe (~3 min, replaces the
    older one-off `examples/` recipe):
    1. Python (`uv run --with unicodedata2==16.0.0`) samples ~120 code points with
        `ud2.category(chr(cp)) == "Cn"` (skip surrogates), crosses them with ≥ 8 context templates —
        base+Cn+mark, jamo+Cn+jamo, `Σ`+Cn+cased, Cn+diaeresis, ASCII, alone, compat-decomp,
        fullwidth — and dumps `/tmp/cases.json`
    2. throwaway `crates/iscc-lib/tests/tmp_probe.rs` (a `#[test]` reading `/tmp/cases.json` via
        `std::fs` + the crate's own `serde_json`, calling `iscc_lib::text_clean`/`text_collapse`,
        writing `/tmp/rust_out.json`) — `cargo test -p iscc-lib --test tmp_probe`, then **`rm` it**
    3. compare against Python transcriptions of `code_meta.py::text_clean` /
        `code_content_text.py::text_collapse` using `ud2.normalize`/`ud2.category`. Also simulate the
        *old* implementation in the same pass to prove the change was load-bearing (iter 148:
        sentinel 0/1270 vs delete-filter 504/1270, failing exactly the four adjacency-sensitive
        contexts) Caveat: `.lower()` still uses CPython's own case tables (3.13 = 15.1) even under
        `ud2` — fine for Cn code points (uncased in every version), not for cased-in-16.0 ones
- **Validate a vendored Unicode table WITHOUT the generator's own dependency** (iter 133): parse the
    generated ranges with a regex and assert every covered code point is `Cn` under the *system*
    CPython `unicodedata` (Unicode only ever assigns, so an N.0 `Cn` table must be a subset of any
    older table), then spot-check that code points assigned exactly in N.0 (U+1FAE9, U+113C5,
    U+A7CB) are **absent** — that is what proves the table is 16.0 and not 15.1. Surrogates must be
    absent (`Cs`), noncharacters present (`Cn`)
- **A data-driven fixture is self-referential — probe whether the suite guards its CONTENT** (iter
    141): vector tests assert `impl(input) == fixture.output`, so a fixture whose cases were swapped
    for ASCII no-ops stays green forever. Ask "what does this file *have* to contain?" and check
    there is an **ungated** assertion on that (code-point set / vector count / declared version) — a
    heredoc in next.md is not a gate. Add the assertion as a minor fix, then MUTATION-PROBE it: edit
    the JSON, run the single test target, `git checkout -- <fixture>`. Live example:
    `crates/iscc-lib/tests/test_unicode_boundary.rs` `BOUNDARY_CODE_POINTS`
- **Prove a Unicode vector is a LIVE guard, not a hypothetical one** (iter 141, ~20s): the Rust core
    links `unicode-normalization` 0.1.25 = **Unicode 17.0** tables, so a "unassigned in 16.0" vector
    only bites if the newer tables would actually change the output. Check with
    `uv run --no-project --with 'unicodedata2==17.0.0' python -c "…ud.category(c), ud.decomposition(c),   ud.normalize('NFKC', c)"`
    — U+A7F1 → `Lm` `<super> 0053` → NFKC `S` (leaks as `aSb` if the freeze filter ever moves
    after normalization), U+20C1 → `Sc`. U+1FAE9/U+113C5 are assigned in both
- **Docs-site rendering check** (iter 132): `uv run zensical build` reports "No issues found" even
    when an `!!! note` body is mis-indented and degrades to a plain paragraph. Grep the RENDERED
    page — `site/howto/<lang>/index.html` for `<div class="admonition note">` — after any
    admonition/tab edit. `mise run version:check`'s file list IS `scripts/version_sync.py` `TARGETS`
    (21 entries), so doc edits near a dependency line can break it

## Artifact-editing traps (moved from the MEMORY.md index, iter 172)

- **Writing `\uXXXX` through Edit/Write decodes it to literal UTF-8** (149) — edit ASCII-escaped
    fixtures via Python (`isascii()` + numeric `ord()`). The same trap bites in **bash**, so run any
    such verification criterion from a quoted heredoc before calling it failed (159).
- **next.md may task advance with an `issues.md` append (135), or task REVIEW with a spec edit (144)
    — both protocols forbid it.** advance puts the paragraph in its handoff **Notes** for the review
    agent to append. Review may edit a spec only when resolving a `[human]` issue carrying
    `**Spec:**`, or to realign a mechanically checkable FACT in a sub-spec under
    `.claude/context/specs/` (172: a falsified claim about `packages/go`, a uniffi version in prose
    — prefer deleting the drifting fact over restating it). `target.md` is never review-editable.

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
- Go via mise requires `mise exec --` prefix — `go` is not on PATH in all environments
- **Never `grep -r` over `packages/`, `docs/` or a crate dir** (230): gitignored build output
    (`packages/kotlin/build/reports/*.html`) has single minified lines of ~100 kB that land in the
    context. Use `git grep -n <pat> -- <paths>` — tracked files only
- The advance commit is at HEAD (not HEAD~1) when the review hasn't committed yet — use
    `git diff HEAD~1..HEAD` for the advance diff (define-next → advance)
- When HEAD is a previous review commit, the advance is at HEAD~1 and the advance diff is
    `git diff HEAD~2..HEAD~1`. Always verify with `git log --oneline -5` first
- **`cargo clippy -p iscc-lib --no-default-features --all-targets` has ALWAYS failed** (E0432:
    `benches/*.rs` import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally; benches need
    default features). Don't add `--all-targets` to a `--no-default-features` criterion and don't
    report it as a regression — the workspace `--all-targets` run (default features) is the real
    gate
- **zsh never word-splits an unquoted `$var`** (198):
    `for f in "" "--features x"; do cargo test $f; done` passes `--features x` as ONE argument →
    cargo errors, a red exit that is NOT a test failure. Use `${=f}` or spell each feature-matrix
    command out
- **`clippy-driver … (signal: 7, SIGBUS: access to undefined memory)` at pre-push is INFRASTRUCTURE,
    not a gate verdict** (iter 152): three parallel `clippy-driver` processes died mid-`git push`
    with SIGBUS while `cargo test` / `ty` / ruff / pytest all passed; ~14 GB RAM was free. The
    protocol's "a failed push means NEEDS_WORK, do not retry" targets a gate *rejecting* the diff —
    a crashed compiler rendered no verdict. **Diagnose before downgrading**: run the identical gate
    outside the hook (`cargo clippy --workspace --all-targets -- -D warnings > /tmp/clippy.log`,
    check `$?` on its own line — a trailing `echo` clobbers `PIPESTATUS`). Green outside the hook →
    push once more and say so in the handoff. Red, or a second SIGBUS → downgrade to NEEDS_WORK.
    Never force-push to amend an already-pushed handoff; add a follow-up commit instead

## Prek hook-scope probing (iter 138)

Reviewing any `types:` / `types_or:` change to `.pre-commit-config.yaml`:

- **Never accept `git ls-files` arithmetic as the hook's surface.** A tag is prek's own
    classification, not an extension guess. `.pyi` is tagged `pyi`, **not** `python` — so
    `types: [python]` silently skips the published `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
    that CI's bare `ruff check`/`ruff format` do cover. Found this way in iter 138 while a handoff
    claimed a "strict superset"; filed `[review]`, fix = add `pyi` to both ruff hooks (probed green
    against an alternate config).

- **Probe recipe** — write a deliberately dirty file, stage it, run the single hook:

    ````bash
    printf '# probe\n\n```py\nx=1\n```\n' > probe_gate.md && git add probe_gate.md
    uv run prek run <hook-id> --files probe_gate.md   # expect Failed / files were modified
    git rm --cached -f probe_gate.md && rm probe_gate.md
    ````

    `(no files to check) Skipped` means the tag does not match the file. Use an alternate config
    (`uv run prek run -c /tmp/probe.yaml <id> --files …`) to test a tag before recommending it.

- **Two gotchas**: prek reports `files were modified by this hook` only for **tracked** files — an
    untracked probe gets fixed but reported `Passed`. Cleanup needs `git rm --cached -f`; plain
    `--cached` errors once the hook has rewritten the worktree copy.

- **Probe the failure mode too when widening a formatter**: `ruff format` leaves a
    syntactically-invalid Markdown fence untouched and exits 0 (probed `def f(:`), so illustrative
    pseudo-code in docs cannot start failing commits.

- **Ping-pong check** when two formatters share a surface (mdformat + ruff on `.md`): run
    `mise run check` **twice** and assert `git status --porcelain` is clean after each. Hook order
    decides who wins — the mdformat repo hook is declared before the local `ruff-format`, so the
    project-pinned ruff has the last word over mdformat-ruff's unpinned copy.

- Go `go get` adds deps as `// indirect` — run `go mod tidy` after

- **CI `find` cross-arch bug**: when multiple targets extract to the same CWD with the same lib
    name, `find` patterns must include the target name (generic wildcards match all extracted dirs)

- **prek stash conflict**: untracked files with formatting issues break prek's stash/restore during
    commit. Fix: move untracked files to `/tmp` before committing, restore after

- **A mode-only commit is invisible to `git status` here** (iter 136): `core.fileMode=false` on the
    9p bind mount, so review a `chmod`-style change with `git diff HEAD~1 --summary`
    (`mode change 100644 => 100755`) and `git ls-files -s <path>`, never a content diff or
    porcelain. Confirm the blob hash is unchanged to prove it really is mode-only

- **mdformat re-wraps long inline code spans and leaves literal double spaces inside them**
    (cosmetic only — shells and TOML tolerate them, but a command looks wrong when copied). After
    `mise run format`, scan the edited Markdown for a code span containing two or more consecutive
    spaces, and prefer short spans or a topic-file pointer over an 80-char command inline in a
    bullet. Never nest backticks inside a bullet to write that grep — mdformat escapes them.

## Reviewing a NEW gate script (iter 142 — `scripts/check_release_workflow.py`)

A gate that cannot fail is worse than none, because it manufactures false confidence. Never accept
"it exits 0 at HEAD" as evidence.

1. **Run it clean**: `uv run scripts/<gate>.py` → exit 0 on the tracked target.
2. **Write your own mutations**, beyond the ones the committed test suite already covers — the
    advance agent chose those, so they prove only what it thought of. For the release-workflow gate
    the uncovered directions were: rename a *download* `name:` (the committed test renames an
    *upload*), and append a whole new job with no `if` plus a bogus `needs:` entry. Both fired.
    Recipe: `python3` heredoc writing mutated copies under `/tmp`, then run the gate on each.
3. **Dump the gate's internals** rather than reading the code and guessing — load it by path with
    `importlib.util.spec_from_file_location` in a `uv run python - <<'EOF'` heredoc and print the
    intermediate structures (for the artifact check: expanded upload names, expanded download refs,
    and any upload that collapses to a bare `*`, which would match everything).
4. **Ask what the gate structurally cannot see** and get it written down (docstring + handoff) —
    that gap is the part no test will ever surface. Three questions that have each paid off:
    - Is the check **one-directional**? (iter 144: every `with:` key passed is verified declared, but
        a newly-*required* input the workflow omits sails through.)
    - Does its **fail-open** path really catch every stdlib failure? (`except OSError` misses
        `http.client.IncompleteRead` — an `HTTPException`/`ValueError` — and `yaml.YAMLError` on an
        HTML captive-portal body, so a gate documented as "degrades to a warning" still reds CI.)
    - Is a **fully-skipped run distinguishable from a pass**? If not, say so in the handoff: the log
        is the signal, not the status badge.
5. **Prefer a REAL regression to a synthetic typo** (iter 144). Downgrading
    `actions/download-artifact@v8` → `@v3` in a temp copy of `release.yml` fired 14 errors, because
    `pattern`/`merge-multiple` genuinely did not exist in v3 — that is the failure class the gate
    exists for. A hand-typo'd key only proves the string comparison works.
6. **Gates to run**: `uv run ruff check`, both pre-push ruff gates (`--select S`,
    `--select C901 --force-exclude`), `ty check`, full `uv run pytest --timeout=120`,
    `mise run check`, plus the hook-scope probe (`uv run prek run <hook> --files <in-scope>` →
    `Passed`; `--files <out-of-scope>` → `Skipped`). ≈ 6 min total.

### Additions from iter 145 (`scripts/check_docs_nav.py`)

- **Mutate a copy of the REAL tree, not a fixture**:
    `T=$(mktemp -d); git archive HEAD | tar -x -C   "$T"`, then edit files under `$T` and call the
    gate's path-injectable entry point (`run_checks(docs, toml, script, llms)`) against them.
    Restoring `git show HEAD~1:<file>` into that copy replays the actual regression the step fixed —
    the strongest possible proof, and free. Push for path-injectable signatures in new gate scripts;
    they make review cheap.
- **Two more blind-spot questions** for the step-4 list:
    - **Set-equality gates pass vacuously on equal empty sets.** `run_checks` on an empty docs tree
        with three empty lists returned `[]` and printed `OK: 0 …`. Fix is one line in the anchor test
        (`assert len(disk_pages(DOCS_DIR)) >= 20`) — within the minor-fix bar, so just add it.
    - **Does the parser see commented-out entries?** A regex over a TOML/YAML block counts
        `# { "Kotlin" = "howto/kotlin.md" },` as live. Test every hand-rolled parser with a
        commented-out entry. (Codex found this one independently — see `codex-integration.md`.)
- **Fail-closed probes worth 60 seconds**: delete the block the regex anchors on, make the imported
    module `raise` at import, and delete an input file. All three must exit non-zero.
- **A `files:`-scoped prek hook needs a *staged* probe, and never fires on deletions.** Break an
    in-scope file → `git add` → `uv run prek run <hook>` → `Failed`; stage only an out-of-scope file
    → `(no files to check)Skipped`; `git rm --cached <page>` + move the file away → also `Skipped`
    (prek matches added/copied/modified only). Restore with `git restore --staged --worktree <file>`
    and confirm `git status --porcelain` is back to just the runner-owned `iterations.jsonl`.
- **Docs-output check ordering**: `zensical build` wipes `site/`, so run `gen_llms_full.py` after it
    (the `docs.yml` order) before asserting per-page `site/**/*.md` files exist.

### Additions from iter 146 (reviewing a gate *hardening*, not a new gate)

A hardening step closes blind spots you yourself filed, so the trap is confirmation bias: the new
branches are all green at HEAD *by construction*. Two extra questions, both cheap:

- **Is the new check non-vacuous?** Enumerate which real inputs would trigger it before believing
    "lands green". Recipe: `importlib`-load the gate, call it with the production fetcher into a
    caller-supplied cache, then print the triggering property per entry — iter 146: 4 of the 18
    `release.yml` refs declare a `required`-without-default input (`cache@v6` path/key,
    `upload-artifact@v7` path, `cross-gem@v1` platform, `setup-ndk@v1` ndk-version). Then
    `copy.deepcopy` the parsed workflow, drop each one, and confirm the exact error fires. A check
    that nothing at HEAD can trigger is a check nobody has tested.
- **Is the fix symmetric?** A one-sided normalization leaves the mirror-image false positive alive:
    skipping non-`str` `with:` keys handles a *workflow* key parsed as YAML 1.1 `True`, but not an
    *action metadata* input named `on`/`off`/`yes`/`no` compared against a quoted workflow key. Name
    the surviving half in the handoff or an issue.
- **Probe a comment stripper against the REAL config, not a fixture.** A naive `#.*$` strip breaks
    on `#` inside a quoted value (`{ "C# / .NET" = "howto/dotnet.md" }`). Mutations that must all be
    right: comment out an ordinary entry (fires), comment out the `#`-in-title entry (fires), append
    a trailing `# "ghost.md"` comment (clean), add a comment line with an *unbalanced* quote (clean
    — proves the in-string toggle is per-line).
- **A fail-open gate needs a resolved/total counter**, and the counter is itself claim-checkable:
    online → `resolved 18 of 18 … (0 skipped)` with empty stderr; behind
    `https_proxy=http://127.0.0.1:9` → `resolved 0 of 18 … (18 skipped)`, exit 0, no traceback.
- **Claim-check the wording of the summary line itself**: "the run *ends with* …" was false — the
    line is followed by the error lines and/or the final `OK:` line. Reword in review, don't
    NEEDS_WORK.

## Docs Claim-Checking (iter 143)

A docs step has no failing test to catch it — every gate is green on a page full of lies. Budget the
review time on truth, not on `zensical build`.

1. **Every number gets re-derived from the artifact, not from next.md.**
    `731 ranges / 819,533 code  points` → parse `crates/iscc-lib/src/utils/unicode16.rs` with a
    regex and sum the ranges. Table rows → dump the JSON fixture. Version/behaviour claims about a
    dependency → query the dependency (`mise exec -- go doc unicode Version`, `cargo info`).
2. **Hunt absolutes.** Grep the diff for `never|always|only|in practice|unaffected|agrees`. Each one
    is a universally-quantified claim; find one counter-example and it is dead. The counter-example
    is very often already written down in `issues.md` — read the issue the page documents *before*
    reading the page. (143: the page said "Latin text is never affected"; the issue's own repro
    character is `Ɤ` U+A7CB, a Unicode-16 **Latin** addition.)
3. **Unicode "what changed in version X" recipe** (~40 s, definitive):
    ```
    for v in 15.1.0 16.0.0; do uvx --with unicodedata2==$v python -c "
    import unicodedata2 as u
    open('/tmp/a_$v.txt','w').write('\n'.join(hex(c) for c in range(0x110000) if u.category(chr(c))!='Cn'))"; done
    ```
    then diff the sets in a `uvx --with unicodedata2==16.0.0` shell and bucket by
    `u.name(chr(cp)).split()[0]`. 15.1→16.0 = **5,185** cps: 3,995 EGYPTIAN, 7 new scripts, 19 in
    the emoji planes (7 real emoji), **32 LATIN**. Never use the system `unicodedata` as the "old"
    side — the uvx interpreter is already 3.14/Unicode 16.
4. **Scope-check agreement claims against open human-gated issues.** "X agrees with us" must be
    narrowed to the class actually proven whenever a divergence class is parked. Narrowing is a
    legal review fixup; *describing* the parked class is not (it pre-empts the human's ruling).
5. **Verdict calculus**: a false consumer-facing safety claim is not a "stale docs" nit — it meets
    the "would mislead users" bar. But if the fix is a few sentences and the correct figures are
    already measured, fixing it in review + PASS_WITH_NOTES beats NEEDS_WORK (two iterations for
    two sentences); record the correction in `decisions.md` so the wording is not "improved" back.

## Fixture / oracle review (iter 149 — `unicode_boundary.json` sequence vectors)

A "must NOT be" oracle in a test const or a docs table is a **claim about a design that no longer
exists in the tree**. Nothing compiles it, no gate re-derives it, and it is the highest-value place
for a wrong number to hide.

1. **Name the design, then re-derive the value for THAT design.** Iter 149 shipped
    `("text_clean", "e\u{A7F1}\u{0301}", "e\u{0301}", "e\u{015A}")` with the 4th field called
    `delete_filter_output`, and published `e U+015A` under a **"Delete filter would produce"** docs
    column. Wrong: a *delete filter* strips the code point pre-normalization → `NFKC("e"+U+0301)` =
    `U+00E9`. `e U+015A` is the *category-override* failure (`U+A7F1` = `<super> 0053` under
    Unicode 17, so NFKC composes `S`+acute). `utils.rs` said "category-override" correctly; next.md
    relabelled the column and forbade re-deriving. **Two superseded designs existed — always ask
    which one a rejected value belongs to.**
2. **Recover a superseded design from git, don't reason about it.**
    `git log -S '<symbol>' --oneline -- <file>` then `git show <sha>:<file>` shows whether the old
    code filtered, mapped or reordered — 20 seconds, and it settles the attribution.
3. **Cross-check a Unicode value against real tables, not glyphs**:
    `uv run --with unicodedata2==17.0.0 python -c "import unicodedata2 as u; c=chr(0xA7F1);  print(u.name(c), u.category(c), u.decomposition(c), u.normalize('NFKC','e'+c+'́'))"`.
4. **Mutate the fixture, four ways, under the FEATURE-OFF build** (only the ungated guards fire —
    that is the configuration the guard exists for). Wrong output / dropped case / swapped code
    point / duplicated case. Do it on a `cp` backup and restore with `cp` +
    `git status --porcelain  <file>`, never `git checkout --` mid-review. All four must red; if
    only some do, the guard is shape-checking, not content-checking.
5. **Ask whether the count and code-point-set assertions are derived from the CONST or from the
    FIXTURE.** Fixture-derived = self-referential = vacuous. Iter 149 derives both from
    `SEQUENCE_VECTORS` + `BOUNDARY_CODE_POINTS`, which is correct. Note that widening a
    set-equality `want` (here: adding the sequence inputs' code points) legitimately *loosens* it —
    a spurious extra case reusing an already-listed code point is no longer caught by that
    assertion alone.
6. **Verdict calculus**: a one-cell oracle error with no functional effect (both wrong values still
    fail the `assert_eq`) is PASS_WITH_NOTES + fix-in-review, *provided* the fix is re-verified
    through the full gate set. Also correct the upstream source of the claim (`issues.md` table) or
    the next propagation step copies it into 11 bindings.

**Escape-decoding trap (bit advance AND review in one iteration):** writing `\uXXXX` text through
the Edit/Write tools decodes it into literal UTF-8. For any ASCII-escaped file
(`unicode_boundary .json`, every sibling `data.json`, `issues.md`'s escape tables) edit through
Python with `"\\u"` in a non-raw string or `json.dumps(..., ensure_ascii=True, indent=2)` + trailing
newline, then assert `raw.isascii()` and check numeric `ord()` — never trust rendered glyphs.

## Reviewing a repo-state gate that reads the git index (iter 152 — `test_vendored_fixtures.py`)

A gate whose input is `git ls-files` / `git status` cannot be probed by editing files: the
interesting mutations are `git add`, `git rm`, rename. **Do not mutate the real index during a CID
iteration** (the runner commits alongside you). Build a throwaway repo instead —
`git clone /workspace/iscc-lib` FAILS in this container (`safe.directory` ownership), so use:

```
rm -rf /tmp/probe && mkdir -p /tmp/probe && git -C /workspace/iscc-lib archive HEAD | tar -x -C /tmp/probe
cd /tmp/probe && git init -q . && git add -A && git -c user.email=r@r -c user.name=r commit -qm probe
/home/dev/.venvs/iscc-lib/bin/python -m pytest tests/<file>.py -q -p no:cacheprovider -o addopts=""
```

`-o addopts=""` drops repo pytest plugins/coverage; the probe file must not `import iscc_lib`. Reset
between probes with `git reset -q --hard HEAD && git clean -qfd` (verify
`git status --porcelain | wc -l` = 0 — a half-reset probe silently poisons the next one).

**The six mutations that matter for a vendored-copy / registry gate.** Each must red, and the
*message* must name the right file:

1. one-byte drift in a copy → identity case named by `id=<copy path>`, message = the `cp` fix
2. **staged** (not committed) extra copy → unregistered-side failure. Proves the gate reads the
    INDEX, i.e. fires at the moment of `git add`, before the copy can be pushed
3. `git rm <registered copy>` → **both** the identity case (FileNotFoundError) and the `gone=[…]`
    side. This is the case a `files:`-scoped prek hook structurally cannot see — it is the evidence
    that justifies a pytest anchor over a hook, so run it explicitly rather than citing the rule
4. re-encode an ASCII-escaped fixture with `ensure_ascii=False` → the ASCII guard *and* the copy's
    identity case. Confirms the guard needs to cover only the canonical, not every copy
5. empty the registry table → the count floor reds; note the identity test degrades to pytest's
    `got empty parameter set` **SKIP**, which is exactly the vacuous pass the floor exists to catch
6. `mv .git .git-off` → must raise `CalledProcessError` (fail closed). `check=True` + no
    `pytest.skip` is the property; a gate that skips when its data source is missing is fail-open

**Blind-spot questions specific to this class:** what is the *discovery key*? (here: two hard-coded
basenames — a copy renamed to anything else is invisible; record it, do not demand tree-wide content
detection if next.md scoped that out). Does the runner see the gate on an unrelated push? (check the
prek hook is `always_run: true` + `pass_filenames: false`, else a deletion-only push skips it.) Does
checkout normalization break byte-identity cross-platform? (`.gitattributes` `*.json eol=lf` here;
also check whether the CI job is Linux-only.)

## Reviewing a vendored DERIVED-property table + a core case change (iter 156, ~25 min)

`crates/iscc-lib/src/utils/unicode16_case.rs` + `to_lowercase_unicode16`. A generated data table
plus a hot-path core edit. Run, in this order:

1. **Independent table derivation — do NOT settle for "the generator is deterministic".** A
    generator that derives a property *behaviourally* validates itself against itself. Audit the
    output against bounds computable from a **different** source. For UCD derived properties,
    `unicodedata.category` gives them free:
    - `Lu ∪ Ll ∪ Lt ⊆ Cased` (no letter is `Case_Ignorable`)
    - `Mn ∪ Me ∪ Cf ∪ Lm ∪ Sk ⊆ Case_Ignorable`
    - residual `Cased ∖ (Lu∪Ll∪Lt)` must be exactly `Other_Uppercase`/`Other_Lowercase` — at 16.0.0
        that is 130 `So` + 32 `Nl` + 2 `Lo` (U+00AA, U+00BA)
    - residual `Case_Ignorable ∖ (Mn∪Me∪Cf∪Lm∪Sk)` must be exactly the **17** UAX #29 MidLetter +
        MidNumLet + Single_Quote code points
        (`0027 002E 003A 00B7 0387 055F 05F4 2018 2019 2024   2027 FE13 FE52 FE55 FF07 FF0E FF1A`)
    - also assert the two tables are **disjoint** and that a both-properties code point (U+0345) is
        in the ignorable table only
2. **Generator determinism, properly.** Re-run it on the **committed** file and compare md5 +
    `git status --porcelain <file>` (empty). advance may only be able to show the untracked-file
    state pre-commit; review can show the real thing.
3. **Fail-closed, three ways** — not just the one next.md names: a range count, a code-point count,
    and the `unidata_version` guard. Each must exit non-zero **and** leave the output md5
    unchanged.
4. **Mutation probes must be SEMANTIC, not just truncation.** A truncation reds the shape/invariant
    test and proves nothing about the behavioural tests. Mutate one code point while **keeping the
    range count constant** (`(0x0295, 0x02AF)` → `(0x0296, 0x02AF)`) and confirm the named
    behavioural tests red, e.g.:
    - drop U+0295 from `Cased` → `test_final_sigma_{following,preceding}_context_cased_in_16` red
    - drop U+0027 from `Case_Ignorable` → `test_final_sigma_apostrophe_is_case_ignorable` red
5. **Re-run the differential sweep yourself** — it is only ~4 min of wall clock and it is the whole
    claim. `uv run maturin develop --release --manifest-path crates/iscc-py/Cargo.toml` FIRST (the
    `.so` is gitignored, so a stale one silently measures the previous commit; check its mtime
    against `src/utils.rs`), then the 8-context × 2-fn × 1,112,064-scalar probe. Expect
    `TOTAL 17793024 comparisons, 0 divergences` and exit 0.
6. **Corroborate the *pre-fix* defect without reverting.** A 6-line throwaway crate
    (`cargo new --lib /tmp/sigmaprobe`) that prints `"\u{0295}\u{03A3}".to_lowercase()` shows what
    bare std does under the *current* rustc — "ʕσ" vs the oracle's "ʕς" proves the divergence was
    real and the fix load-bearing, in seconds instead of a full revert-and-rebuild.
7. **Re-confirm every expected value against the live oracle**, comparing ref vs rust vs the literal
    written in the test source (three-way, so a test that merely mirrors the implementation shows
    up). `uv run python -c "import iscc_core, iscc_lib; ..."`.
8. Gates: `mise run check`, `cargo test -p iscc-lib` + the **full feature matrix**
    (`--no-default-features` 220, `+text-processing` 259, default/all 288 — assert the per-target
    `N passed`), clippy workspace, `mise run coverage` + the CI-exact
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (then `mise run crap:baseline` and confirm the committed baseline is md5-stable),
    `mise run bench:iai:check`, `uv run pytest -q`, and the pre-push Python trio (`ty check`, ruff
    `--select S`, ruff `--select C901`).
9. **Docs prose about a conditional algorithm is the likeliest real finding.** "Σ goes final when no
    `Cased` character follows" is WRONG — the rule is "the nearest *following* non-`Case_Ignorable`
    character is not `Cased`". Disprove it with one probe: `ΑΣ,Β` → `αςβ` (comma is not ignorable)
    while `ΑΣ.Β` → `ασβ` (period is `MidNumLet`, hence ignorable).

## Spec↔config parity gate — iteration-163 specifics

`scripts/check_ci_job_table.py` compares `ci.yml` job keys against the table in
`.claude/context/specs/ci-cd.md` (21 jobs at 163).

- **Section anchoring is load-bearing on the real file, not the fixture.** `ci-cd.md` carries 14
    backticked first-column rows under `## Version Management` (paths, mise tasks) that an
    unanchored whole-file scan would read as bogus job rows — so a green baseline run *is* the proof
    the anchor works.
- **Cheapest real regression:** `git show HEAD~1:.claude/context/specs/ci-cd.md > /tmp/old`, then
    `run_checks(real_ci, Path('/tmp/old'))`. The pre-change 14-row table yields 0 parsed job rows →
    count-floor error plus all 21 missing, i.e. the gate provably catches the drift it exists for.
- **Two set-equality blind spots:** equal *empty* sets pass (hence the ≥10 count floor), and a
    **duplicated** row passes because sets dedupe — keep row order long enough to count repeats.
    Codex found the duplicate hole; `spec_job_rows()` now preserves order and reports duplicates.
- The spec prose hardcodes "21 job keys → 22 check names"; the gate deliberately pins no count, so
    that number must be hand-corrected in whatever step adds a job.

## Moved from the index (197)

Full text of entries compacted out of `MEMORY.md`.

- **A docs-only sweep can FABRICATE a per-surface return type** (190): grep each binding's
    `src/lib.rs` for the ACTUAL return type of any newly-documented symbol, not the CLAUDE.md/howto
    prose (wasm `gen_iscc_id_v1` returns bare `Result<String,JsError>`, not an invented struct;
    Codex catches these). A `#[deprecated]`/attribute claim in next.md is a hypothesis too — read it
    in `~/.cargo/registry/src/index.crates.io-*/<crate>-<ver>/src/` (168: jni 0.22.4 never
    deprecated `Env::byte_array_from_slice`; that grep criterion cost a zero-copy path)
- **On ANY codec input-cleaning change, probe empty/garbage inputs** (191): a helper that yields
    `""` (blank/`"-"`/`"iscc:"`) makes `iscc_decompose` silently return `Ok([])`
    (`decode_base32("")` is `Ok(empty)`, not an error) — a Tier-1 silent-acceptance regression
    valid-input differential tests miss. Compare against `HEAD~1` + reference; 192 added the empty
    guard (learnings)
- **A `--no-default-features` CI step catches COMPILE breaks the default gates miss** (174): a
    `#[test]` calling a feature-gated fn fails to *compile* under
    `cargo test -p iscc-lib --no-default-features`; gate the test `#[cfg(feature = "meta-code")]`.
    Run the full CI feature matrix on any core change — `mise run check` and a default `cargo test`
    are both blind to it
- **Probe a WRAPPED multi-nibble header on any codec header-gate change** (174): `decode_header`
    casts varnibbles `as u8` before `TryFrom`, so `MDFZAAAAAAAAAAAAAA` wraps to `Id`/V1 and
    `iscc_decompose` silently rewrites it to `MAIAAAAAAAAAAAAA` (`iscc_core` rejects it) → learnings
- **No-op / human-handoff iteration (111)**: verify scope is empty, still scan `@{upstream}..HEAD`;
    HUMAN-REVIEW spec amendments + `low` left → **HUMAN REVIEW REQUESTED**, never `**IDLE**`
    (all-`low` only). Verdict still PASS. Same call when the only `normal` left is trigger-gated on
    an upstream release (173, go1.27): escalate rather than burn a `Step: NONE` cycle
- **`Step: NONE` with all-`low` issues (248, ~5 min)** → PASS + `**IDLE**`. Checks: advance diff is
    `handoff.md` only; `grep '^## ' issues.md | grep -vc '`low`'` is 0 (the `untriaged` grep also
    matches the file header: entries = count − 1); `gh run list --branch develop` green on
    `origin/develop`; one probe of a next.md claim about external state (248: the Dependabot alerts
    API vs the `develop` lockfiles). A side note for the human in next.md is lost when next.md is
    rewritten: file it as a `low` entry if it is a real defect
- **Unicode freeze rule REMOVED 2026-07-30** (decisions.md): `utils/unicode16*`,
    `unicode_boundary.json`, the sweep gate and the `U+FFFF` sentinel are gone. A diff that
    reintroduces any freeze mechanism without a new human decision is a NEEDS_WORK

## Moved from the index (204)

- **next.md can under-scope the spec's "Verified when" criterion** (197): it asked only for the seq
    base32 divergence, while `rust-core.md` demands "both sides reject the same inputs". Read the
    feature's spec checklist too; a missing cheap differential test → add it in review (step 6)
- **A metamorphic "display forms" suite: demand one form per trim/clean CALL SITE** (200: Codex saw
    the outer-only separator forms miss the 2 post-colon trims; mutation in a `/tmp` copy of
    `packages/go` — conformance tests need `../../crates`, so `-run <family>` only)
- **Scope cap: `.crap-baseline.json` is a GENERATED artifact, not a counted file** (3 src + baseline
    passed at 195 `5d2840e` and 203); a refresh with only `line:` shifts is correct, not a deviation
- **A Go `Fields`→`FieldsFunc` swap costs ~2× (ASCII fast path) and allocs only past 32 fields** —
    an `AllocsPerRun` probe needs >32 fields to see it; accepted in decisions.md (203)
- **DIFFERENTIAL gate (157/158) / vendored DERIVED-property table (156), ~25 min each**: a
    case-count pin catches a shrunken set, never a swapped one; drive `main()` in-process →
    `archive/unicode-reviews.md`
- **Repo-state gate that reads the git index (152, ~8 min)**: Python-only PLUS mutations in a
    THROWAWAY repo (`git archive HEAD | tar -x -C /tmp/x && git init`; `git clone` fails here on
    `safe.directory`) — never mutate the real index → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, 134–139)**: `mise run check`, ruff check +
    `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates, ty, pytest
- **CI-restore (advisory ignore + runner-label swap, 196, ~10 min)**: prove an ignore's "no upgrade"
    claim at crates.io (`gate-reviews.md` Audit), runner arch from the `actions/runner-images`
    README, then grep `specs/` + per-crate `CLAUDE.md` for the old label
- **A GENERATED-artifact bump** (172): regenerate into `/tmp`, never read the diff →
    `dep-refresh-reviews.md`
- **An MSRV claim is locally decidable**: `cargo +1.85.0 check -p <crate> --locked` →
    `dep-refresh-reviews.md` "Moved from the index"
- **Other bump types** (lockfile, wrapper/binary blob, test-framework major) →
    `dep-refresh-reviews.md`
- **Prek-hook-scope review (138–139)**: NEVER accept `git ls-files` arithmetic as a hook's surface —
    `.pyi` is tagged `pyi`, not `python`; a *widened* tag needs a **staged, dirty** file probe
- **Action *pin* change (162, ~6 min)**: four static gates + actionlint, then probe the pin at
    source — `git/matching-refs/tags` (does `@vN` exist?) + `compare/<tag>...main` `.ahead_by` (162:
    `@main` was 31 commits ahead) → `gha-workflow-reviews.md`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only via the
    committed gates `scripts/check_release_workflow.py` + `… --check-action-inputs` (never a
    heredoc); zero `warning: skipped` is part of the pass; `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`

## Multi-surface docs fan-out sweep (iter 211 — ISCC-SEQ docs on 11 surfaces, ~15 min)

- **next.md's stale-count greps exclude `.claude/context/`** — `learnings.md` still said "Tier 1 =
    33 / sweep pending" after a green sweep. Grep learnings + specs for the old count yourself; fix
    learnings (yours), file sub-spec drift, never touch `target.md`
- **Confirm the grep is non-vacuous**: `git grep <pat> <define-next-sha> -- <paths> | wc -l` (8 hits
    before → 0 after)
- **Compiling the C/C++ snippets takes seconds** (FFI `.so` already in `target/debug`):
    `g++ -std=c++17 -Wall -Wextra -Werror -I packages/cpp/include -I crates/iscc-ffi/include x.cpp -L target/debug -liscc_ffi -Wl,-rpath,$PWD/target/debug`.
    Ruby: `ruby -I crates/iscc-rb/lib -e 'require "iscc_lib"; …'` (check `.so` mtime > last
    `crates/iscc-rb/src` commit)
- **Rendered-HTML evidence must be case-insensitive** across casings (`EncodeSeq`/`encodeSeq`/
    `encode_seq`): `grep -ciE 'encode_?seq' site/howto/*/index.html`
- Codex on uncompiled doc examples finds missing imports (Java `Arrays`), fixture-vs-golden
    mislabels, and exception-contract overstatements (Ruby `TypeError` from Magnus conversion)

## Codec probes (moved from the MEMORY.md index, 216)

- **On ANY canonical-form/decode change, probe non-ASCII case-fold aliases** (199: `ı`/`ſ`/`ß`/`ﬁ`
    upper-case to ASCII, so Rust+Go alias codes the ASCII-only ref `b32decode` rejects; a re-encode
    compare after `ToUpper` cannot see it). Probe via the Python binding — build it first
- **"The fix matches the reference" — probe EVERY entry point it touches** (201: ref `iscc_decode`
    rejects `ı`/`ſ`, ref `encode_seq` `.upper()`s first and ACCEPTS — the issue + next.md missed it,
    Codex caught it). A new divergence → a `diverges` vector in `data-edge.json`, file a `**Spec:**`
    issue, escalate (spec exception lists are acceptance criteria)
- **Python-reference differentials: Python `strip()`/`split()` ⊋ Rust `trim`/`split_whitespace`** by
    U+001C–U+001F: codec (198/200) + text utils (203) fixed in Rust AND Go. A one-way sweep proves
    only ⊇ — demand the reverse (live `Z*`/`Cc`/`Cf` − strip set, 0.6s); `binascii.Error` ⊂
    `ValueError`

## Moved from the index (223)

- **Spec-hygiene / phantom-path step (223, ~10 min)**: next.md's backticked-path loop misses
    structure TREES — check each tree in the modified specs against `git ls-files <root>` (Go tree
    was 11/19 fictional) and globs with `compgen -G`. A criterion edit that only names the real
    mechanism another spec documents = approve + decisions.md; cut any clause that turns a
    description into a prohibition
- **issues.md at its 300-line budget**: free room by cutting history sentences, line numbers and
    counts from `[review]` entries; never reword `[human]` entries, never trim an `**Upstream:**`
    entry's repro, and never claim behaviour for a site found only by grep
- **Uncommitted HUMAN edits can appear mid-review** (212: Titusz's interactive `claude` on pts/0, no
    `--agent`, wrote decisions.md + issues.md). Not a 2nd loop: Edit targeted hunks only (never
    Write from the inlined copy), then commit his hunks verbatim + say so — leaving them unstaged
    puts them through prek's stash for the whole pre-push run while his session is live
- **Multi-surface docs fan-out (211, ~15 min)**: next.md greps skip `.claude/context/` (grep
    learnings for the old count); compile C/C++ snippets with g++ against `target/debug` in seconds
    → `review-patterns.md`

## Doc-parity gate (233 — `scripts/check_tier1_docs.py`)

- **A name-matching gate proves nothing about the sentence around the name.** 233's new WASM how-to
    line copied "array of `Uint8Array`" from the binding's own doc comment; the build returns
    `number[][]`. A source doc comment is a claim, not evidence: run every return type a docs step
    adds. Codex found this one; my source cross-check had accepted the comment
- **The local WASM `pkg/` is a web build but runs in Node**: `await import(pkg + "iscc_wasm.js")`,
    then `m.initSync({ module: readFileSync(pkg + "iscc_wasm_bg.wasm") })` from a `/tmp/*.mjs`. So
    "WASM docs come from source only, the pkg is a web build" (advance, 233) is not a limit
- **Threshold discovery is blind exactly where drift is worst**: print the per-file symbol count for
    every tracked `*.md` and read the 12–29 band (233: `docs/howto/rust.md` 26, `docs/rust-api.md`
    17 sit under 30). **Parser probes**: `pub use a::b as c;`, nested groups, `pub struct`,
    `pub static`, `pub unsafe fn` all passed silently until 242 made the parse fail closed
- **Fail-closed parser step (242 PASS, ~6 min)**: three probes were enough — (1) old script
    (`git show <define-next>:scripts/…` to `/tmp`) vs new `derive_symbols` on the real input must be
    equal; (2) one sweep script printing derive / excluded / UNRECOGNISED for ~35 forms the tests do
    not list; (3) mutants from a `git archive` + `git init` + `git add -A` copy, run with
    `/home/dev/.venvs/iscc-lib/bin/python -m pytest … --no-cov -p no:cacheprovider` (2 s each).
    Codex returned in ~3 min with nothing. An equivalent mutant (the `[<(]` after a fn name) is not
    a test gap: no valid Rust reaches it
- **Python fixture writes need `encoding="utf-8"`**: re-run a new test file once under
    `LC_ALL=C PYTHONUTF8=0 PYTHONCOERCECLOCALE=0` (233: `write_text` of `lib.rs` raised there)
- **Rewriting a part-done `[human]` issue**: keep its untouched "Resolved when" bullets verbatim and
    replace the done bullet by the measured remainder; never mark the bullet done on a threshold
- **Rust docs completion step (243 PASS, ~20 min)**: three probes carried it. (1) A `/tmp` cargo
    project (`[workspace]` table, path dep on `crates/iscc-lib`, `CARGO_TARGET_DIR=/tmp/…`) with
    every ```` ```rust ```` example block wrapped in a `#[test] fn … -> Result<(), Box<dyn Error>>`:
    15 s build, nothing lands in the tree while Codex runs. Expected failures are only the
    placeholder files (`document.pdf`, `large_file.bin`). Run it over the sibling pages too
    (`README.md`, `crates/iscc-lib/README.md`, `docs/index.md`, the tutorial). (2) Python probe of
    every `gen_*` for `isinstance(r, dict) and r["iscc"] == r.iscc`. (3) Signatures are now pinned
    by `tests/test_rust_api_signatures.py`; its mutant is the page's previous version
    (`git show HEAD~1:docs/rust-api.md` → 10 stale). A generic fn (`gen_video_code_v0<S…>(`) breaks
    any `name(` match: match on the name alone
- **A `zensical build` "page does not exist" warning during a background Codex run was a race**
    (243: both wipe `site/`); it exits 0 either way, so re-run it once Codex is done and read the
    output. **A count word hides from a digit grep**: "The three `META_TRIM_*` constants" (243)
- **C++ how-to completion step (244 PASS, ~8 min, Codex clean)**: (1) a 10-line extractor writes
    every ```` ```cpp ```` block to `/tmp`, tagged `main`/`frag`; each `main` block builds alone
    with
    `g++ -std=c++17 -Wall -Wextra -I packages/cpp/include -I crates/iscc-ffi/include f.cpp   -L target/debug -liscc_ffi`
    after `cargo build -p iscc-ffi` and runs with `LD_LIBRARY_PATH=target/debug`; pipe the output
    through `cat -A` and compare with the comments. (2) Registration mutant = the page's previous
    version in a `git archive` copy (must list exactly next.md's missing symbols). (3) A prose claim
    like "wraps the C function of the same name with an `iscc_` prefix": count `iscc_<name>(` in
    `iscc.h` per function (false for `iscc::iscc_decode` / `iscc_decompose`). (4) **Grep the touched
    page for hand-typed versions** (`git grep -nE '[0-9]+\.[0-9]+\.[0-9]+'`): `version_sync.py`
    covers only Maven/Gradle/SPM snippets (244 found `Version: 0.2.0` in the pkg-config example,
    filed `low`). In zsh a subshell's `${PIPESTATUS[0]}` is empty: use bash

## Install-claim probe (238)

A docs step that says how a package installs ("falls back to a source build", "needs X and Y") is a
claim about the published artifact, and no gate in this repo installs one. Probe it before passing:

- Ruby: `curl -s https://rubygems.org/api/v1/versions/iscc-lib.json` lists every platform gem with
    its `ruby_version` range. `gem fetch iscc-lib --version <v> --platform ruby` into an empty
    `/tmp` dir, then `GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install <file> --local --no-document`. It
    fails within seconds when the source gem is broken (238: missing `rb_sys` dependency, then
    `failed to find a workspace root`). Re-run with `gem install rb_sys` in the same `GEM_HOME` to
    see the second failure
- The same shape applies to any sdist-like fallback (Python sdist, npm source install): check what
    the package ships (`tar -t`) against what its manifest references outside the package
- Verdict pattern: the step did what next.md asked and the false sentence predates it →
    PASS_WITH_NOTES, a `normal` issue for the defect, and the old issue is replaced rather than
    kept. Codex did not catch it

### Published-package docs sweep (257, ~20 min, PASS_WITH_NOTES)

A docs-only step that rewrites install and loader claims for several registries. Green gates prove
nothing here; the probes that did:

- Java: download the Maven Central JAR into an empty `/tmp` dir, `unzip -l` for `META-INF/native/`,
    then `javac`/`java -cp` a three-line `Probe.java` kept in a separate dir, with no library path.
    Kotlin: read `<scope>` of the JNA dependency in the published POM. Swift:
    `git show v<tag>:Package.swift` (SPM resolves the TAG, not HEAD) and `sha256sum` of the release
    zip against `releaseChecksum`
- **A fallback sentence ("on other platforms, do X") is a claim about a platform you are not on.**
    Simulate it:
    `java -Dos.arch=riscv64 -Dos.name=FreeBSD -Djava.library.path=<dir with the extracted .so>`.
    257: `NativeLoader` throws before the fallback, so the new sentence was false. Codex found the
    same from the source. Fixed in step 9 (three sentences), and the loader limit filed `low`
- next.md's "optional override" for `java.library.path` was false as well: the bundled library wins
- Sweep the page a step half-fixes: the graph in `docs/architecture.md` was completed while three
    sections below it stayed partial. Filed `low`, not NEEDS_WORK, because next.md scoped only the
    graph, one sentence and one row
- A work-producing iteration that leaves only `low` issues is PASS without `**IDLE**`: the next
    `Step: NONE` iteration signals it

### npm `@iscc/wasm` Node-setup docs step (283, ~25 min, PASS)

- Strongest probe is the registry package, not only the packed HEAD build: in an empty `/tmp` dir
    `npm init -y && npm install @iscc/wasm@<published version>`, then run each page's Node block
    verbatim as `.mjs`. Extract the blocks with the docs test's own helpers (`sys.path.insert` of
    `tests/`, then `javascript_blocks(markdown_section(...))`), so the mdformat-reflowed text runs
- HEAD build:
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release   --out-dir /tmp/x/pkg crates/iscc-wasm --features conformance`
    (17 s on a warm target), set `name` in a COPY, `npm pack`, install the tarball. Run the
    `test-wasm` step body next to a `pkg/` copy
- Compare the README's export list against the published package with `import * as m` and a
    `typeof m[name]` filter: names missing there are unreleased additions, not errors
- Sibling sweep that found the leftovers: `git grep -nE 'init\(wasmBytes\)|await init\(\)'` (a
    positional init in `scripts/test_install.py`, fixed in step 9 with a guard test) and
    `git grep -n 'return ISCC strings directly'` (the napi README kept the false sentence: filed)
- `uv run scripts/test_install.py --npm --version <v>` runs both npm packages from the registry

## Moved from the index (239)

Detail of index entries that were cut to one or two lines when the index was compacted.

- **Core behaviour change with flipped binding tests (225, ~25 min)**: Rust feature matrix, Go,
    pytest, napi build+test, `wasm-pack test`, `mvn clean test`, `dotnet test`, cbindgen no-op, CRAP
    gate; run the suites from ONE logged script. grep docs for signatures the step makes relevant
    (`docs/howto/rust.md` had two missing `IsccResult`)
- **`mise run check` mdformat reformats context files** (intermittent): not an advance regression,
    but pre-push mdformat WILL reject the batch — check `git status --porcelain` right after and
    STAGE it. Never stage `iterations.jsonl`; keep inline code spans on ONE line. Never format with
    bare `uv run mdformat` (221: other wrap + list numbering than the hook) — use
    `uv run prek run mdformat --files <f>`
- **Push message "GitHub found N vulnerabilities on the default branch" is about `main`** (234: 6
    alerts, all already fixed in `develop`'s lockfiles) — compare `gh api …/dependabot/alerts`
    patched versions with `develop` before filing; they close at the `develop` → `main` merge
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop (confirm `ps aux` → HUMAN REVIEW REQUESTED, do not push or kill), or my own Edits
    landing while a backgrounded prek run held its snapshot. Never edit while prek runs
- **Spec paths are gated by `scripts/check_spec_paths.py` (224)** — a "mechanism does not exist"
    realignment is a hypothesis: grep package recipes (Conan `pkg_config_name`/`cmake_file_name`
    generate `.pc`/config at install time; 224 advance + next.md both missed it)
- **Mutate each helper the step adds, not only the previous file version** (228): the old script
    failed all 3 new tests, yet dropping `re.escape` stayed green — the "literal metachar" assert
    had no `*`, so it never built the regex. ~1 min: `sed` one call out into `/tmp/x/scripts/`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:`) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits. Green
    gates say NOTHING about truth (143). A new PAGE adds `scripts/check_docs_nav.py`
    (disk/nav/`ORDERED_PAGES`/`docs/llms.txt`, 23 pages); `zensical build` wipes `site/` →
    `gen_llms_full.py` after it
- **Python-only**: `mise run check` + `pytest`; a new `.py` file adds the pre-push `ty`/ruff gates.
    Python docstrings exist 3× (wrapper, PyO3 `///` → `_lowlevel.X.__doc__`, `.pyi`): rebuild with
    `maturin develop` before any `__doc__` test (222, ~3 min)
- **New gate script, gate HARDENING, or spec↔config PARITY gate (142–146/163, ~10 min)**: never
    accept "exits 0 at HEAD" — write your OWN mutations (cheapest real regression:
    `git show HEAD~1:<path>`), probe both set-equality blind spots (equal-empty vacuous pass,
    duplicated row), give a `files:`-scoped hook a staged probe
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (exit 0 ⇒ `git push` will not be rejected; needs `iscc_lib` built). Run it last and
    sequentially, with `run_in_background` — `--all-files` also runs `standardrb`, ~15 min total on
    the bind mount (200)
- **Codex: `--commit HEAD` reviews ONLY the last commit** — advance often ends with a handoff-only
    commit (195, 217). `git log @{upstream}..HEAD` first; for a multi-commit range use ONE run with
    `codex exec review --base <sha-before-the-range>` (217: worked with a bare sha, ~5 min, covered
    six per-surface commits) instead of per-commit runs
- **Codex: the background run executes real build/test commands in the SAME tree** (164 dotnet, 165
    \+ 172 Gradle) — a build tool failing once while Codex runs may be losing a race on
    `bin/`/`obj/`/`build/`. Re-run sequentially; hold any Gradle `clean` until it exits
- **Codex finds what a green run does not prove**: matching/parsing rules, exception contracts,
    user-facing factual claims, build-config lines (154), declared-but-ungated facts such as MSRV
    (172), and a third-party action's real behaviour (239)

## Derived-fact removal step (245 — counts out of specs + per-crate `CLAUDE.md`, ~10 min, PASS)

Docs-only guidance sweep, 24 files. The three grep criteria in next.md are a floor. What found the
rest:

- **Wrap-insensitive sweep**: per file `tr '\n' ' ' | sed -E 's/ +/ /g' | grep -oE` for a digit or
    spelled-out number before a countable noun (functions, bindings, platforms, tests, vectors,
    targets, methods, classes). Run it over the UNTOUCHED files too (`documentation.md`, jni, ffi).
- **Each reworded sentence is a new claim**: read the code it names. Found: "one typed result class
    per `gen_*_v0` function" is true but leaves out `IsccIdResult` (11 classes); a method name that
    replaced a line number (`Rake::ExtensionTask#define`) was checked in the vendored gem under
    `crates/iscc-rb/vendor/bundle`.
- **"N more than M" is still a derived number** ("one more check name than job keys"): accept only
    when the items it derives from sit in the same sentence.
- The gates that must stay green are `check_tier1_docs.py`, `check_spec_paths.py`,
    `check_ci_job_table.py`: all in `mise run check`, so no extra run is needed.
- Codex finishes a docs-only review in about 3 minutes and adds nothing here.

## File-list removal step (246 — trees + `File`/`Module` tables out of 12 `CLAUDE.md`, ~8 min, PASS)

Docs-only, no gate covers what matters. Three probes did the work, all from the diff:

- **Rule-retention sweep**: for each removed entry whose comment is a rule or origin note ("checked
    in", "DO NOT EDIT", "gitignored", "must be at", "required by"), `git grep` the file for the
    fact. A different wording counts ("never edit this file").
- **Each added sentence against its source**: `git check-ignore -v <path>` for every "gitignored"
    claim (the match may be a broad rule such as `*.so`, not the literal name the handoff quotes),
    the manifest for a lib or module name, the build file for an include path.
- **Leftover scan**: an `awk` pass for directory-looking lines inside any fence, plus `git grep` for
    the removed heading names across the repo (nothing may link to a deleted section).
- A sentence that next.md suggested word for word can still be wrong: "Tier 1 is what `lib.rs`
    re-exports" (the gen functions are defined there). Fixed in step 9.

## Spec-list gate step (247 — "not every file" note gated in `check_spec_paths.py`, ~15 min, PASS_WITH_NOTES)

- Checks that paid off: the five next.md criteria in one script; the new gate against the previous
    specs in a `git archive` repo (restore each changed spec with `git show HEAD~1:<f>`, `git add`,
    run the script: exactly the 8 unnoted trees failed); each reworded note against its source
    (`TARGET_RID` in `release.yml`, the JNA tables).
- **Before closing a list-drift issue, sweep for lists the issue text does not name.** The entry
    said "only the trees are left" (my own narrowing in 246) and was wrong: a grep over the specs
    for table rows and bullets that START with a backticked file path showed a 10-row version-sync
    table in `ci-cd.md` against 21 `TARGETS` in `scripts/version_sync.py`, with "all targets listed
    above" below it. Compare any such table with the code's own list.
- A drifted file list in a sub-spec is reviewer-fixable (file list = mechanically checkable fact):
    replace it with a pointer to the code's list, re-run the spec gate, record it in decisions.md
    and say so in the handoff. Do not write a new universal claim into the pointer sentence ("every
    hand-written version is a target" is false: the pkg-config example in `docs/howto/c-cpp.md`).
- Codex said "no actionable regressions" and did not see the table: it reviews the diff, not what
    the diff left out.

## Spec / CLAUDE.md drift realignment step (259 — release-trigger claims, ~20 min, PASS_WITH_NOTES)

- Checks that paid off: one `git grep -n -E` over `release.yml` for `if:`, `needs:`, `git tag`,
    `npm publish`, `wasm-pack build` (every condition and command on one screen), then each
    rewritten sentence read against it. The JS recipe run in Node on the built napi addon.
- **A purpose clause is an invented mechanism.** advance copied the commands correctly and then
    explained them: "force-push the tags, so a stale remote tag is overwritten". False:
    `fetch-depth: 0` makes `actions/checkout` fetch every tag (`getRefSpecForAllHistory` in its
    `src/ref-helper.ts`, read with `gh api repos/actions/checkout/contents/…?ref=v7`), so `git tag`
    fails and the push is a no-op. Read every "so that / because" in a rewritten spec as a separate
    claim. Codex found the same one.
- **Sweep the false claim repo-wide before closing the issue**, with no pathspec limit taken from
    next.md: `docs/development.md` → "Publish" still told maintainers to push a tag. next.md's grep
    covered specs and `CLAUDE.md` files only. Filed `normal`.
- A reviewer may correct a sub-spec sentence when the code states the fact (`git tag … || echo`). A
    heading or intro that names a design pattern is rationale: file it, do not rewrite it.
- `decisions.md` over budget: `cid.archive_overflow` moved two entries (422 → 396) in one call.

### Second pass (275 — eight files of an audit issue, ~15 min, PASS)

- One read-only pass per file while `mise run check` runs in the background: the code lines each
    rewritten sentence names (`.detach(` sites + the enclosing `fn`, `validate_version` callers,
    `js_name` and `serde_wasm_bindgen` greps), the spec's example output run live in Python.
- **A release-history claim is checked per tag**:
    `for t in $(git tag -l); do git grep -l '<pat>' $t -- <path>; done`. next.md said the old Go
    IDv1 symbols were "removed in 0.6.0"; no tag ever held them, and advance corrected it.
- **A sentence that generalizes from the one copied example is its own claim**: "every export keeps
    its Rust name" was false for the five constant getters (`js_name`). Grep the attribute over the
    whole file. Fixed in step 9 as a sub-spec fact.
- A leftover of the same kind outside the cited statements (the test-file list in
    `crates/iscc-py/CLAUDE.md`) is filed `low` with `**Triage:** untriaged`, not fixed: the step did
    not touch it and Titusz's rule is "document unrelated issues, do not fix them".

## Core + Go behaviour fix (250)

A step that makes the Rust core and the Go port reject an input (250: ragged video frame
signatures). PASS in about 10 minutes.

- Run every next.md criterion from ONE logged script with a `=== END <name> rc=<n>` line per step:
    `cargo test -p iscc-lib`, both clippy forms, `cargo fmt --check`, `--no-default-features`,
    `--no-default-features --features text-processing`, Go test + vet + gofmt, `cargo llvm-cov` +
    `cargo crap` with the CI flags, `mise run bench:iai:check`. Warm, the whole script took about
    one minute. Run `go test -v -run <Name>` once more to see the new Go tests `PASS` by name
- **Probe the reference for every "iscc-core does X" sentence the docs gain**, from an empty `/tmp`
    dir:
    `uv run --no-project --python 3.13 --with /workspace/iscc-lib/reference/iscc-core python -I   -c '…'`.
    In 250 next.md said a short signature raises `IndexError`; the reference returned a code for
    all three ragged pairs. Only the shipped sentence matters for the verdict, but the measured
    result belongs in the decision entry and in the handoff for the follow-up vector
- A deviation from iscc-core is a decisions.md entry. When the issue has a further "Resolved when"
    clause (a shared vector), rewrite the entry to that remainder; do not delete it
- Codex needed under 5 minutes for a diff of this size
- **Integer-width maths fix (263, ~15 min, PASS)**: after the logged script, ONE three-way
    differential on boundary values (every 2^k ± 1 and every 2^(k+0.5) ± 2, where a rounded `log2`
    flips): the reference via `uv run --no-project --with reference/iscc-core`, the Rust functions
    cut verbatim with `sed -n 'A,Bp'` into a `/tmp` file, the Go package from a `git archive` copy
    with a `zz_probe_test.go` that writes the tuples to a file. `diff` the three outputs
- **Bare `rustc` compiles edition 2015** (263): `usize::try_from` fails with E0599 and the probe
    binary is never written, so a following `diff` prints the whole reference file. Pass
    `--edition 2021 -C overflow-checks=on` and check the line count before diffing
- **Before suggesting a `data-edge.json` vector for a numeric boundary, read each binding's host
    type** (263): Java `algCdcChunks` takes a signed `int`, so 2^31 cannot be a shared vector. Also
    ask what the vector can observe: a JSON-sized input never reached the CDC masks
- `GOARCH=386 CGO_ENABLED=0 go test` runs on the amd64 host; CI has no 32-bit job (issue filed 263)
- iai readings 3 to 8% BELOW the July baseline on benchmarks the diff cannot reach are toolchain
    drift (263, rustc 1.100): say so in the handoff, do not attribute them to the step

## Go-only serializer fix (251)

A step that changes how the Go port serializes something the core also serializes (251: canonical
`meta` JSON, RFC 8785 string escaping). PASS in about 8 minutes.

- Criteria: `mise run check`, then one logged script with `go test -count=1 ./...`, `go vet`,
    `gofmt -l`, the next.md greps and `go test -v -run <Names>` to see the new tests by name
- The one probe worth its budget is a **differential against `iscc_lib`**: a Python script writes
    `{name: {in, url, code}}` to a JSON file with the core's results (`ERR` on an exception), and a
    throwaway `zz_probe_test.go` in a `git archive HEAD packages/go` copy under `/tmp` reads it and
    prints every difference. Run it with `go test -v`: without `-v` a passing test hides its
    `fmt.Printf` output and the probe looks clean
- It doubles as the check for "expected values come from the Rust core": include the committed test
    inputs as cases
- Input classes for a JSON canonicalizer: each escape rule, an escaped backslash before a real
    escape, raw U+2028 in the input, the 32 control characters, the characters in a key, astral
    characters, then the rules the step does NOT claim — key order (an emoji key next to a U+E000
    key: UTF-8 order differs from UTF-16 order), `-0.0`, `1e21`, a 17-digit integer, a lone
    surrogate escape, duplicate keys. 251 found the key order, `-0` and the lone surrogate still
    differing in Go: pre-existing, filed `low`
- Build every backslash-u sequence in the script as `chr(92) + "u2028"`. The Edit tool decoded the
    escapes in an `issues.md` edit and the edit failed to match; write such text with a Python
    script that builds the backslash the same way
- When the step fixes part of an issue, rewrite the entry to the remainder and keep its priority.
    `issues.md` sat at 298 of 300 lines: a new entry needed cuts in other `low` `[review]` entries
- 266 (structural rewrite: key order, `-0`, lone surrogates; PASS in about 15 minutes): the same
    differential with 4,000 seeded random documents added (random 64-bit doubles, integers up to
    2^70, a key pool mixing astral and high-BMP characters) gave 0 differences. Check the probe's
    power by copying `git show HEAD~1:packages/go/code_meta.go` into the `/tmp` tree and re-running:
    it must print differences. Go's `encoding/json` float output equals the ECMAScript form
- New vectors in an existing `data-edge.json` section: `node --test __tests__/data_edge.test.mjs` in
    `crates/iscc-napi` and `mvn -o -B -q test -f crates/iscc-jni/java/pom.xml` run them in seconds
    on the existing artifacts when the core did not change
- A lone surrogate escape cannot be a vector (serde_json fails on the fixture file): accept a Go
    unit test and delete the issue; the `edge-vectors.md` rule on JSON-expressible inputs covers it

## Pytest config / test-fixture speed step (254)

A step that changes `[tool.pytest.ini_options]` or where test fixtures get their bytes (254:
`addopts = "--benchmark-disable"`, Tier 1 doc tests read the repo once). PASS in about 6 minutes.

- Criteria from one logged script: full `uv run pytest --durations=10` with wall time, the changed
    test module alone, each documented pytest command as written in the docs, `zensical build`
- An ini-wide `addopts` reaches every pytest invocation under the repo root. Grep `.github`,
    `.pre-commit-config.yaml`, `mise.toml` and the docs for `pytest`, and check that each one
    installs the plugin the option belongs to (an unknown option is a usage error, exit 4)
- Run the documented command once without the new flag too: 254's exit 4 proved the doc edit was
    required, not decoration
- A fixture helper that copies less is safe when the code under test would raise `FileNotFoundError`
    on the files left out. Confirm that by reading the early-return path; no mutation test is needed
- Pre-flight `uv run ty check` and ruff `S`/`C901` on the changed test file before the push

## Moved from the index (251)

The long form of the index entries as they stood before the compaction in iteration 251.

**Common Issues**

- Verification greps false-positive: `grep -qv 'pat'` is always true on multi-line files (use
    `! grep -q`). Substring `grep -c` and exact counts in next.md are a recurring mis-spec class
    (139) — test the criterion against `HEAD~1` before believing advance broke it
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms, prose and quoted commands alike**
    (142, 143, 149; 239 a misquoted third-party grep). Re-derive every number, command and expected
    value from source → `review-patterns.md` "Claim-Probing"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix. Docs site: `https://lib.iscc.codes/`
- **A docs-only sweep can FABRICATE a per-surface return type** (190); a `#[deprecated]` claim is a
    hypothesis (168) → `review-patterns.md` "Moved from the index"
- **Codec changes**: probe empty/garbage inputs (191), `MDFZAAAAAAAAAAAAAA` on a header-gate change
    (174), case-fold aliases + every reference entry point (201) → `review-patterns.md`
- **next.md under-scoping a spec's "Verified when"**, `.crap-baseline.json` scope cap, differential
    / repo-state / lint-config / prek-scope reviews → `review-patterns.md` "Moved … (204)"
- **6-file cap counts files whose CODE changes** (225, decisions.md): comment-only binding edits +
    regenerated outputs don't count, IF next.md lists them as such — confirm from the diff
- **Core behaviour change with flipped binding tests (225, ~25 min)**: all suites from ONE logged
    script + grep docs for touched signatures → `review-patterns.md` "Moved from the index (239)"
- **Run the full CI feature matrix on any core change** — only `--no-default-features` catches a
    feature-gated `#[test]` compile break (174) → `review-patterns.md`
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`\uXXXX` decodes to literal UTF-8 through Edit/Write and bash**; who may edit `issues.md` / a
    spec is protocol-fixed (135/144/172) → `review-patterns.md` "Artifact-editing traps"
- **`mise run check` mdformat may reformat context files**: check `git status --porcelain` right
    after and STAGE it; never stage `iterations.jsonl`; format only with
    `uv run prek run mdformat --files <f>` → `review-patterns.md` "Moved from the index (239)"
- **No-op / human-handoff iteration** (111/173): verdict PASS; `low`-only + human spec work or an
    upstream-gated `normal` → HUMAN REVIEW REQUESTED, never `**IDLE**`. Verify an advance "only low
    issues remain" claim independently; all-`low` `Step: NONE` → PASS + `**IDLE**` (248, ~5 min) →
    `review-patterns.md`
- **Push message "GitHub found N vulnerabilities" is about `main`** (234), not `develop` →
    `review-patterns.md` "Moved from the index (239)"
- **Unicode freeze rule REMOVED 2026-07-30** — reintroducing any freeze mechanism = NEEDS_WORK
- **`git push` failing "index.lock: File exists" while prek stashes is a WSL2 race, NOT a hook
    rejection** (207): confirm no 2nd loop in `ps aux`, then retry once
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop, or my own Edits during a backgrounded prek run. Never edit while prek runs
- **Spec paths are gated (224)**; a "mechanism does not exist" realignment is a hypothesis. **Mutate
    each helper the step adds** (228) → `review-patterns.md` "Moved from the index (239)"
- **Spec-hygiene (223), docs fan-out (211), issues.md at budget, uncommitted human edits (212)** →
    `review-patterns.md` "Moved from the index (223)"
- **learnings.md sits AT its 200-line budget**: every new bullet needs an equal cut or a move to
    `learnings-archive.md` in the same commit (239); check `wc -l` after mdformat

**Review Shortcuts (detail in the topic files named above)**

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace (~2 s warm) + `mise run check`. Version
    sync adds `version_sync.py --check`, a shell script `bash -n <script>`
- **Core + Go behaviour-fix step (250, ~10 min)**: one logged script (tests, feature matrix, clippy,
    Go, llvm-cov + CRAP, iai) → `review-patterns.md` "Core + Go behaviour fix (250)"
- **Go-only canonicalizer / serializer fix (251, ~8 min)**: Go criteria + ONE differential against
    `iscc_lib` from a `git archive` copy, inputs per JCS rule → `review-patterns.md` "Go-only
    serializer fix (251)"
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only + full feature matrix (assert
    per-target `N passed`) + the four fixture mutations → `review-patterns.md`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` + `uv run zensical build`;
    green gates say NOTHING about truth (143) → `review-patterns.md` "Moved from the index (239)"
- **Python-only**: `mise run check` + `pytest`; a new `.py` adds pre-push `ty`/ruff; rebuild with
    `maturin develop` before any `__doc__` test (222) → same section
- **New gate script / gate hardening / parity gate (142–146/163, ~10 min)**: write your OWN
    mutations, probe equal-empty and duplicated-row blind spots → `review-patterns.md`
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed → `gate-reviews.md` Audit)
- **Bumps** (generated artifact, MSRV `cargo +1.85.0 check -p <c> --locked`, lockfile, wrapper,
    framework major) → `dep-refresh-reviews.md`
- **Security-pass / advisory-bump step (235, ~15 min)**: cargo-deny + govulncheck + ONE osv-scanner
    pass; `diff -rq` the Go module cache → `dep-refresh-reviews.md`
- **Binding-harness / data-edge / .NET wrapper / C FFI docs / generated C table (215-221)**, new
    data-edge SECTION = 12-surface script (226) → `binding-reviews.md` "Moved from the index (223)"
- **C++ wrapper-only step (227, ~4 min)**: ASAN build in an own `build-*/` dir + previous-header
    mutation + strict `g++ -Werror -fsyntax-only` → `binding-reviews.md`
- **A `[human]` issue with a `**Spec:**` field**: on closing it, the reviewer corrects the spec rows
    it names (227) — stage the spec file
- **Waiting on a background run** (bare `sleep` is blocked):
    `timeout 580 bash -c 'until grep -q "^exit" <log>; do sleep 15; done'`
- **Core text/codec change + generated data (≈12–20 min)**: Rust-only + feature matrix + CRAP +
    `bench:iai:check` + generator no-op + pre-push gates on new `.py` (203) → `gate-reviews.md`
- **JNI error-path step (230, ~12 min)**: shadow-class probe of the pending-exception guard;
    `-Xcheck:jni` is silent on `ThrowNew` over a pending exception → `binding-reviews.md`
- **Verbatim code-move step (231, ~5 min)**: prefix `diff`, de-indented body `diff`, test-name set
    `diff`; cbindgen to a temp file + `cmp` → `binding-reviews.md`
- **Test-harness-only step (232, ~8 min)**: mutate from throwaway copies; macOS proof = the `swift`
    CI job after the push → `binding-reviews.md`
- **Doc-parity gate / doc-listing step (233, ~25 min)**: restore previous docs in a `git archive`
    repo, feed the parser unlisted forms (fail-open?), RUN every return type the docs add;
    **parser-only step (242, ~6 min)**: old-vs-new derive, form sweep, mutants; **Rust docs
    completion step (243, ~20 min)**: compile every example in a `/tmp` cargo project →
    `review-patterns.md` "Doc-parity gate (233)"
- **Derived-fact removal step (245, ~10 min)**: wrap-insensitive sweep incl. untouched files; each
    reworded sentence is a new claim to check → `review-patterns.md` "Derived-fact removal step
    (245)". **File-list removal (246, ~8 min)**: rule-retention `git grep` per removed entry,
    `git check-ignore -v` per "gitignored" claim → same file, "File-list removal step (246)".
    **Spec-list gate step (247, ~15 min)**: before closing a list-drift issue, grep the specs for
    table rows starting with a backticked path and compare with the code's own list → "Spec-list
    gate step (247)"
- **WASM return-shape step (234, ~10 min)**: `#[wasm_bindgen_test]` never crosses the JS glue —
    probe the built `--target nodejs` pkg from Node + read the `.d.ts` → `binding-reviews.md`
- **data-edge rule / validator / C-generator step (236, ~8 min)**: two pytest files, regen +
    `git status` on the header, `gen.render` on a mutated fixture copy → `binding-reviews.md`
- **Harness-compare fan-out step (237, ~6 min)**: four suites from one logged script, .NET mutant
    from a `git archive` copy → `binding-reviews.md`
- **Docs install/packaging-claim step (238, ~12 min)**: install the PUBLISHED artifact in a clean
    prefix → `review-patterns.md` "Install-claim probe (238)"
- **C++ how-to / doc-example step (244, ~8 min)**: extract + `g++` each `main()` block, previous
    page as gate mutant, grep the page for hand-typed versions → `review-patterns.md` "Doc-parity
    gate (233)"
- **Ruby gem packaging step (239 NEEDS_WORK, 240/241 PASS, ~12–25 min)**: run the action's real
    unanchored lookup; criteria from one logged script with the mise Ruby 3.4 `bin/` on PATH. Codex
    caught BOTH early misses (lookup, dropped `[profile.release]`): for any generated manifest, diff
    what the root `Cargo.toml` has that the copy lacks; mutate a text-slice helper from a
    `git archive` copy (241) → `binding-reviews.md`
- **A test that shells out adds a tool to the `python-test` CI job** (240: `ruby`): check the runner
    image readme (`gh api repos/actions/runner-images/contents/images/ubuntu/Ubuntu2404-Readme.md`)
- **Search with `git grep`, never `grep -r`** over dirs with build output (230) →
    `review-patterns.md` "Gotchas"
- **Action pin / release.yml / any GHA bump**: static-verify only via the committed gates
    (`check_release_workflow.py` + `--check-action-inputs`); `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (~15 min, background, last) → `review-patterns.md` "Moved from the index (239)"
- **Per-binding commands + traps** ("not locally verifiable" claims: all FALSE so far) →
    `binding-reviews.md`; CI/Audit/Perf/Semver/CRAP gates → `gate-reviews.md`
- **decisions.md over budget → rotate with `cid.archive_overflow`**, never hand-compress (command in
    `gate-reviews.md`); stage `decisions-archive.md` too (195)

## Go API-shape step (261)

A Go-only step that changes a method signature or adds misuse guards (261: `Push` gained an `error`
return, hashers got a `finalized` flag, `TextTrim` clamps `nbytes <= 0`). About 5 minutes, PASS.

- Run the Go-only commands from the index, then the `-run '<Name>' -v` criteria with
    `grep -E '^(--- |PASS|FAIL|ok|panic)'` so a panic is visible.
- **Sibling call sites**: `git grep -n -E '\.Push\(' | grep -v _test.go` finds every caller in the
    package AND in docs. A docs example that calls the method as a statement still compiles; that is
    a style note, not an issue.
- **Go source compatibility**: adding a return value keeps statement calls compiling but breaks a
    stored method value and a caller's own interface. The advance handoff must name it; record the
    accepted break in decisions.md (0.x) and tell define-next to put it in the release notes.
- **Exported `int` parameters** (the only ones that can go negative):
    `git grep -n -E '^func .*[A-Z][A-Za-z0-9]*\(.*\bint\b' -- 'packages/go/*.go'` minus tests. At
    261 these are `TextTrim` and `SlidingWindow`, both guarded.
- "Matches every sibling surface" is a claim: read one sibling (`crates/iscc-py/src/lib.rs`,
    `inner.take()` before the core call) instead of trusting it.
- The repository has no `CHANGELOG`; a Go API break is recorded in decisions.md and the handoff.

## Docs accuracy batch (277, ~25 min)

A step that corrects false sentences on named pages. All gates were green and Codex found nothing;
the findings came from sweeping for siblings.

- **Sweep the whole tree for each corrected claim**, source comments included:
    `git grep -nE   'KA[AC]\b'` found two `///` comments (`types.rs`, wasm `lib.rs`) next.md had not
    listed, and `git grep -nE '10 .gen_' -- docs` found `ecosystem.md` and `architecture.md`. Read
    each hit: a library-wide "10 functions" count is correct, "10 from ISO" or "10 in `data.json`"
    is not (9)
- **Release-doc claims**: dump every job's `needs`, `if`, `permissions`, secrets and check-step
    names from `release.yml` with PyYAML in one script and compare the table row by row
- **Facts measured here**: `gen_sum_code_v0` and `gen_iscc_code_v0([data, instance])` return
    `ISCC:KUA…` (`wide` with 64-bit units changes nothing); iscc-core 1.5.0 has no
    `gen_sum_code_v0`; Ruby `Float::INFINITY` raises `FloatDomainError` like `NaN`
- **A human decision recorded only inside an issue dies with the issue**: write the decisions.md
    entry when closing it (the Ruby `to_int` idiom)
- A comment-only leftover in two files is a step-9 fix; prose leftovers on other pages are one
    `normal` issue (leftover of the step's own pattern)
- **Surface-coverage batch (280, ~20 min, PASS)**: every new table row checked in five greps (paths
    via `git ls-files`, `release.yml` staging steps for archive contents, hasher class/method names
    per binding, `ci.yml` run lines, each test's vector loader) plus `curl -w '%{http_code}'` on
    each added registry link. **Read the PRE-EXISTING rows of a section whose issue you close**: the
    C FFI row named `ctx_new()` (no such function) and the Java row a `DataHasher` class (static
    `dataHasher*` methods only); two cells are a step-9 fix. **Keep a replaced table cell within the
    widest row**, or mdformat re-pads every row of the table

## Binding README claim + docs-claim test (286, ~10 min, PASS)

A step that corrects one sentence on a registry README, fills its symbol list and adds a pytest that
ties the sentence to the binding source.

- **Count the binding's exports yourself** (`git grep -n -A2 '#\[napi' <lib.rs>`) against the README
    list; the Tier 1 gate checks names on the page, not that the binding exports them
- **Mutate the test in-process**, no tree edits: import the test module from `tests/`, point its
    path constants (`LIB_RS`, `README`) at `/tmp` copies with a renamed field, an added field, a
    dropped field name and the banned phrase wrapped over a line break, and call each test function
- **`npm pack <pkg>` in an empty `/tmp` dir and grep `package/index.d.ts`** shows the published type
    names in one minute. Symbols added since the last release are missing there by design (0.7.0 has
    no `encode_seq`/`decode_seq`): the README ships with the next version
- napi-rs camel-cases PARAMETER names in the `.d.ts` (`addUnits`) while `js_name` keeps function
    names snake_case; parameters are positional, so a README that says `add_units` is a nit

## API-tier classification docs step (288, ~25 min, PASS)

- A classification step rewrites sentences in several files; read each rewritten sentence against
    the step's OWN new classification. Root `CLAUDE.md` kept "only Tier 1 is bound" one paragraph
    away from "Python and WASM wrap the Tier 2 `SumHasher`"
- "Module X is Tier 2" is false for items the crate root re-exports from X: list `pub use X::…` in
    `lib.rs` and check that the text covers them (`codec` holds four Tier 1 functions, and
    `codec::encode_component` is NOT the crate-root function of the same name)
- A `//!` or `///` edit in a covered Rust file shifts line numbers in `.crap-baseline.json`. That is
    harmless: the CI gate line reported "0 regressed, 33 moved", exit 0 (coverage + gate ≈ 4 min).
    Run it once when a handoff says "comment-only, gates unaffected" and the file is `lib.rs`
- One `/tmp` cargo project with a path dep proves the doc example AND the prose claims around it
    (same-item paths by assigning `types::X` to `X`, "identical to `gen_sum_code_v0`" by comparing)
- Codex had no findings here; the two wording gaps came from the re-export listing
