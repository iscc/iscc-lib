# Next Work Package

## Step: Close two silent no-op gates — release.yml re-trigger guards and `.pyi` hook coverage

## Goal

Fix two gates that silently do nothing: `gh workflow run release.yml -f <registry>=true` builds
artifacts and then skips every test/publish job (issue "Fix broken single-registry re-trigger in
release.yml"), and both local ruff hooks skip `.pyi` files so the published type stub is CI-gated
only (issue "Local ruff hooks skip `.pyi` files"). **Reframed from the handoff:** the handoff asked
to bundle the release.yml `if:`-guard fix with the 97-ref GitHub Actions refresh. Those are split
here — a 19-job behavioural `if:` change and a ~10-ref mechanical bump in the same unrunnable file
would be un-bisectable if the next real release breaks. The GHA refresh stays queued as its own
step; the two-token `.pyi` fix rides along instead (the handoff explicitly offered it as a tail-end
bundle).

## Scope

- **Modify**:
    - `.github/workflows/release.yml` — add `!cancelled() && !failure() &&` guards to the 19 unguarded
        job `if:` conditions
    - `.pre-commit-config.yaml` — add the `pyi` type tag to the `ruff-check` and `ruff-format` hooks
    - `.claude/skills/release/SKILL.md` (docs) — retire the "Known bug: `-f <registry>=true`
        re-triggers skip publishing" subsection
    - `docs/development.md` and `CLAUDE.md` (docs) — the two hook-list sentences that enumerate what
        `ruff check`/`ruff format` cover now also cover `.pyi` stubs
- **Reference**: `.claude/context/issues.md` (both issue bodies carry the exact fixes),
    `.github/workflows/ci.yml` (guard style already in use), `.claude/context/handoff.md`

Non-doc file budget: **2** files (`release.yml`, `.pre-commit-config.yaml`).

## Not In Scope

- **Do not bump any `uses:` ref in `release.yml`.** The 97-ref GHA refresh (checkout v4→v7,
    `upload-artifact`↔`download-artifact` as a pair, setup-java/node/dotnet/python) is the next step
    and must land as its own bisectable commit.
- **Do not touch `prepare-release`'s `if: inputs.version != ''`.** It is correct as written —
    guarding it would make the tag-pushing job run on registry-only dispatches.
- **Do not replace `!failure()` with `always()`** anywhere. `always()` would publish after a failed
    build or test.
- Do not add `pyi` to the pre-push `ruff --select S` / `--select C901` hooks, and do not add, remove
    or re-stage any other hook.
- Do not touch the npm OIDC migration (human-gated, needs npmjs.com-side configuration first).
- Do not wire Unicode boundary vectors or edit `specs/rust-core.md` — still parked on the open HUMAN
    REVIEW freeze-rule-ordering ruling.
- Do not edit `.claude/context/issues.md` — the review agent resolves issues after verifying.
- Do not touch `~/.claude/projects/.../release-workflow.md`; it lives outside the repo.

## Implementation Notes

### Part 1 — release.yml guards (19 jobs)

Root cause (empirically verified 2026-06-18): with no `version` input, `prepare-release`
(`if: inputs.version != ''`) is skipped, and GitHub propagates that skip transitively down the
`needs` chain to every job whose `if:` lacks a `!cancelled() && !failure()` guard. The `build-*`
jobs and `publish-crates-io` already carry it; nothing downstream does.

Mechanical transformation — for each of the 19 job `if:` lines below, wrap the existing condition:

```text
if: inputs.version != '' || inputs.<registry>
→
if: ${{ !cancelled() && !failure() && (inputs.version != '' || inputs.<registry>) }}
```

exactly matching the shape already used at `publish-crates-io` (line ~128). The 19 jobs (current
line numbers): `test-wheels` 232, `publish-pypi` 261, `test-napi` 350, `publish-npm-lib` 369,
`test-wasm` 449, `publish-npm-wasm` 475, `assemble-jar` 566, `test-jni` 599, `publish-maven` 627,
`test-ffi` 769, `publish-ffi` 790, `test-gem` 864, `publish-rubygems` 884, `pack-nuget` 940,
`test-nuget` 1002, `publish-nuget` 1050, `assemble-kotlin` 1174, `test-kotlin-release` 1210,
`publish-maven-kotlin` 1276. Every one is single-registry — no condition needs restructuring.

Safety note for reviewing your own diff: `!failure()` evaluates the **job's own `needs`** status, so
a publish job still will not run when its build or test dependency failed. The guard only
neutralises the *skip* propagated from `prepare-release`. Do not touch `needs:` lists.

### Part 2 — `.pyi` hook coverage (2 tokens)

prek classifies `.pyi` as the `pyi` type, not `python`, so both hooks skip
`crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (consumer-facing — the wheel ships `py.typed` beside
it) while CI's recursive discovery checks it. Add the tag:

- `ruff-check`: `types: [python]` → `types_or: [python, pyi]`
- `ruff-format`: `types_or: [python, markdown]` → `types_or: [python, pyi, markdown]`

Probed green during scoping against an alternate prek config: both hooks then report `Passed` on the
tracked stub and leave it unmodified (it is already clean under bare `ruff check` / `ruff format`).
Keep the existing evergreen comment above `ruff-format`; extend it only if the wording becomes
inaccurate.

### Part 3 — docs

In `.claude/skills/release/SKILL.md`, the
`### Known bug: -f <registry>=true re-triggers skip publishing` subsection is now stale. Replace it
with an accurate statement: the guards are in place on all
`test-*`/`assemble-*`/`pack-*`/`publish-*` jobs so `-f <registry>=true` re-triggers reach their
publish jobs, while `gh run rerun <run-id> --failed` stays the preferred recovery path because it
reuses build artifacts. Be honest that the guard fix is verified statically and gets its first
real-world confirmation on the next release. Keep the "Never pass `-f version=<version>` to recover"
warning intact.

In `docs/development.md` (~line 137) and `CLAUDE.md` (~line 179) the ruff bullets/sentence say the
hooks cover Python and Python code blocks inside Markdown — add `.pyi` type stubs to that
enumeration. One clause each; no restructuring.

## Verification

Run from the repo root. `mise run format` before staging.

- Guard shape and preserved conditions — this script exits 0 only after Part 1 lands (it reports 19
    offenders at HEAD):

    ```bash
    uv run python - <<'PY'
    import collections, re, yaml
    jobs = yaml.safe_load(open(".github/workflows/release.yml"))["jobs"]
    shape = re.compile(
        r"^\$\{\{ !cancelled\(\) && !failure\(\) && \(inputs\.version != '' \|\| "
        r"inputs\.[a-z-]+( \|\| inputs\.[a-z-]+)*\) \}\}$"
    )
    assert len(jobs) == 29, len(jobs)
    assert jobs["prepare-release"]["if"] == "inputs.version != ''"
    bad = [n for n, j in jobs.items()
           if n != "prepare-release" and not shape.match(str(j.get("if", "")))]
    assert not bad, bad
    toks = collections.Counter(
        m for j in jobs.values() for m in re.findall(r"inputs\.[a-z-]+", str(j.get("if", "")))
    )
    assert dict(toks) == {
        "inputs.version": 29, "inputs.crates-io": 1, "inputs.pypi": 4, "inputs.npm": 6,
        "inputs.maven": 4, "inputs.ffi": 3, "inputs.nuget": 4, "inputs.rubygems": 3,
        "inputs.maven-kotlin": 4,
    }, dict(toks)
    print("release.yml guards OK")
    PY
    ```

- `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 .github/workflows/release.yml` exits 0
    with no output (module is already in the Go cache; validates expression syntax)

- `uv run prek run check-yaml --files .github/workflows/release.yml` → `Passed`

- `uv run prek run yamlfix --files .github/workflows/release.yml` → `Passed` with no
    `files were modified by this hook`

- `grep -A6 'id: ruff-check' .pre-commit-config.yaml | grep -q 'types_or: \[python, pyi\]'` → exit 0

- `grep -A8 'id: ruff-format' .pre-commit-config.yaml | grep -q 'types_or: \[python, pyi, markdown\]'`
    → exit 0

- `uv run prek run ruff-check --files crates/iscc-py/python/iscc_lib/_lowlevel.pyi` → `Passed`, and
    the output does **not** contain `no files to check`; same for `ruff-format`

- No gate weakened: `grep -c 'exclude' .pre-commit-config.yaml` → 0 and
    `grep -c 'id:' .pre-commit-config.yaml` → 22 (no hook added, removed or narrowed)

- `uv run ruff check` → `All checks passed!` and `uv run ruff format --check` → exit 0,
    `153 files already formatted`

- `mise run check` → all hooks Passed, exit 0; `git status --porcelain` afterwards shows no
    unexpected modifications

- `grep -q 'Known bug' .claude/skills/release/SKILL.md` → exit **1** (stale subsection gone), and
    `grep -q 'gh run rerun' .claude/skills/release/SKILL.md` → exit 0 (recovery path retained)

- `grep -q 'pyi' docs/development.md && grep -q 'pyi' CLAUDE.md` → exit 0

- `uv run zensical build` → exit 0, `No issues found`

## Done When

All verification criteria pass: every `test-*`/`assemble-*`/`pack-*`/`publish-*` job in
`release.yml` carries the `!cancelled() && !failure()` guard with its registry condition intact,
both local ruff hooks cover `.pyi` files, and the release skill plus the two hook-list docs describe
the new behaviour.
