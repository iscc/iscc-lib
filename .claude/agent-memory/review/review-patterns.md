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
- **Text-pipeline ORDER changes need a SEQUENCE differential, not per-code-point tests** (iter 133,
    Unicode 16 freeze rule). Moving a filter earlier/later in `text_clean`/`text_collapse` changes
    character *adjacency*, which silently alters context-sensitive Unicode operations. Probe recipe
    (~2 min): write `crates/iscc-lib/examples/<name>.rs` calling the public fns,
    `cargo run -q -p iscc-lib --example <name>`, print `{:04X}` per char, and compare against a
    Python transcription of the `iscc-core` pipeline — then **delete the example file**. Minimum
    sequence set: base+Cn+combining-mark (canonical composition), jamo+Cn+jamo (Hangul), Σ+Cn+cased
    (Rust `to_lowercase` applies `Final_Sigma` contextually; verify with a standalone
    `rustc /tmp/x.rs`). Confirmed divergences vs. reference: `U+00E9` vs `U+0065 U+0301`, `U+AC00`
    vs `U+1100 U+1161`, `…σ…` vs `…ς…`
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
    that gap is the part no test will ever surface.
5. **Gates to run**: `uv run ruff check`, both pre-push ruff gates (`--select S`,
    `--select C901 --force-exclude`), `ty check`, full `uv run pytest --timeout=120`,
    `mise run check`, plus the hook-scope probe (`uv run prek run <hook> --files <in-scope>` →
    `Passed`; `--files <out-of-scope>` → `Skipped`). ≈ 6 min total.

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
