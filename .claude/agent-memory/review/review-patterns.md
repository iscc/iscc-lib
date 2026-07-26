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

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
- Go via mise requires `mise exec --` prefix — `go` is not on PATH in all environments
- The advance commit is at HEAD (not HEAD~1) when the review hasn't committed yet — use
    `git diff HEAD~1..HEAD` for the advance diff (define-next → advance)
- When HEAD is a previous review commit, the advance is at HEAD~1 and the advance diff is
    `git diff HEAD~2..HEAD~1`. Always verify with `git log --oneline -5` first
- **`cargo clippy -p iscc-lib --no-default-features --all-targets` has ALWAYS failed** (E0432:
    `benches/*.rs` import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally; benches need
    default features). Don't add `--all-targets` to a `--no-default-features` criterion and don't
    report it as a regression — the workspace `--all-targets` run (default features) is the real
    gate

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
