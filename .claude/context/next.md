# Next Work Package

## Step: Refresh the 97 GitHub Actions `uses:` refs in release.yml

## Goal

Bring `.github/workflows/release.yml` to the same current action majors that `ci.yml` and `docs.yml`
have run green since iteration 127, closing the last CID-doable remainder of the "Dependency review
and refresh across the project" issue. Purely mechanical: nine distinct refs move, no `if:`,
`needs:`, `with:` or step is touched — the behavioural guard change already landed as its own commit
in iteration 139, so a broken release stays bisectable.

## Scope

- **Modify**: `.github/workflows/release.yml` — bump 9 distinct `uses:` refs across 73 of the 97
    `uses:` lines
- **Reference**:
    - `.github/workflows/ci.yml` — the already-green target versions (`checkout@v7`,
        `setup-python@v7`, `setup-node@v7`, `setup-java@v5`, `setup-dotnet@v6`, `upload-artifact@v7`)
    - `.claude/context/issues.md` → "Dependency review and refresh across the project", the
        `Remaining:` paragraph (line ~122)
    - `.claude/agent-memory/define-next/release-yml-static-gates.md` — job/guard inventory and the
        five static checks

Non-doc file budget: **1** file (`release.yml`). No doc file mentions any of these refs.

## Not In Scope

- **Do not touch any `if:`, `needs:`, `with:`, `env:`, `permissions:` or `run:` block.** This step
    changes `uses:` values and nothing else. The guard invariant from iteration 139 (28 of 29 jobs
    guarded, `prepare-release` bare) must survive byte-for-byte.
- **Do not add an `astral-sh/setup-uv` step.** `release.yml` invokes no `uv`/`uvx` command at all
    (`grep -c 'setup-uv' → 0`, `grep -c 'uv '` → 0). The issue text claiming one needs a bump is
    wrong; leave the file without one.
- **Do not bump `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`, `ruby/setup-ruby@v1`,
    `PyO3/maturin-action@v1`, `nttld/setup-ndk@v1`, `oxidize-rb/actions/cross-gem@v1`,
    `rust-lang/crates-io-auth-action@v1` or `pypa/gh-action-pypi-publish@release/v1`** — all are at
    their current floating major (verified against the GitHub releases API while scoping).
- **Do not re-pin `rubygems/configure-rubygems-credentials@main` to a tag.** Moving off a floating
    branch is a supply-chain decision, not a version refresh; leave it and let review file it if it
    matters.
- Do not pin any action to a commit SHA — the repo convention is floating majors (one exception,
    `setup-uv@v9.0.0`, and it does not appear in this file).
- Do not touch the `tag: 0.9.123` input on `oxidize-rb/actions/cross-gem` — it is locked in lockstep
    with the `rb_sys` pin in `crates/iscc-rb/Gemfile`.
- Do not edit `.claude/context/specs/ci-cd.md` (its `actions/upload-artifact@v4` /
    `actions/setup-node@v4` literals went stale in iteration 127 and are a spec-wording matter), nor
    `.claude/context/issues.md` — the review agent resolves issues.
- **Do not land the guard-shape check as a `scripts/` file or prek hook.** The handoff offers it as
    a bundle; it adds a new gate and belongs in its own scoped package. Run the check inline instead
    (script below) and let review file it as a follow-up issue.
- Do not touch the deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit
    6.x) or the `jni` 0.22 / `magnus` 0.8 source rewrites.
- Do not wire Unicode boundary vectors or edit `specs/rust-core.md` — still parked on the open HUMAN
    REVIEW freeze-rule-ordering ruling.

## Implementation Notes

Nothing in this file is exercised by a CI or CID push (`workflow_dispatch` only), so every check is
static. Make the edit a pure find/replace of full `uses:` values — never reflow, reorder or
re-indent a step.

### The nine bumps (counts verified at HEAD)

| Ref                           | From  | To    | Count |
| ----------------------------- | ----- | ----- | ----- |
| `actions/checkout`            | `@v4` | `@v7` | 23    |
| `actions/download-artifact`   | `@v4` | `@v8` | 20    |
| `actions/upload-artifact`     | `@v4` | `@v7` | 11    |
| `actions/setup-java`          | `@v4` | `@v5` | 6     |
| `actions/setup-node`          | `@v4` | `@v7` | 5     |
| `actions/setup-dotnet`        | `@v4` | `@v6` | 3     |
| `actions/setup-python`        | `@v5` | `@v7` | 2     |
| `softprops/action-gh-release` | `@v2` | `@v3` | 2     |
| `actions/cache`               | `@v4` | `@v6` | 1     |

All nine floating major tags were confirmed to exist (`repos/<r>/git/ref/tags/<vN>`) during scoping.

### Why these targets are safe

- Six of the nine (`checkout@v7`, `setup-python@v7`, `setup-node@v7`, `setup-java@v5`,
    `setup-dotnet@v6`, `upload-artifact@v7`) are **already running green in `ci.yml`** with the same
    inputs this file uses (`python-version: '3.10'`, `node-version` string, `distribution: temurin`
    \+ `java-version: '17'`, `dotnet-version: '8.0'`). That is the strongest evidence available
    without a release run.
- Every intermediate major in these lines is a Node 20 → Node 24 runtime move plus an ESM
    repackaging. GitHub-hosted runners are far past the required runner `2.327.1`.
- `setup-python@v7` removed the `pip-install` input — **not used here** (both call sites pass only
    `python-version`). `setup-node@v7` removed a dummy `NODE_AUTH_TOKEN` export; the npm publish
    jobs set `NODE_AUTH_TOKEN` themselves in `env:`, so nothing changes. `setup-java@v5` keeps
    `server-id`/`server-username`/`server-password` (the Maven Central job).
- `upload-artifact` ↔ `download-artifact` **move in the same commit**. Both lines have been on
    `@actions/artifact` v4 since upload v5 / download v6, so v7 uploads and v8 downloads
    interoperate; v8 is the pair designed alongside upload v7 (it sniffs `Content-Type` before
    unzipping).
- `download-artifact@v5`'s breaking change only affects **single downloads by `artifact-ids:`** —
    this file has zero `artifact-ids:` uses; it downloads by `name:` or by `pattern:` +
    `merge-multiple:`, both still present in the v8 `action.yml`.
- `download-artifact@v8` defaults `digest-mismatch: error` (was a warning). Strict-by-default is the
    desired behaviour for a publish pipeline — do not override it back to `warn`.
- `actions/cache@v6` and `softprops/action-gh-release@v3` are Node 24 + ESM only. The single cache
    site keeps `path`/`key`/`id: xcf-cache` and the `steps.xcf-cache.outputs.cache-hit` reference;
    the two gh-release sites keep `tag_name`/`files`/`generate_release_notes`.

## Verification

Run from the repo root. `mise run format` before staging.

- `uses:` inventory is exactly the expected 18-ref histogram and the total is still 97 (no step
    added, removed or duplicated):

    ```bash
    diff <(grep -oE 'uses: [^ ]+' .github/workflows/release.yml |
    LC_ALL=C sort | uniq -c | sed 's/^ *//') - <<'EOF'
    2 uses: PyO3/maturin-action@v1
    7 uses: Swatinem/rust-cache@v2
    1 uses: actions/cache@v6
    23 uses: actions/checkout@v7
    20 uses: actions/download-artifact@v8
    3 uses: actions/setup-dotnet@v6
    6 uses: actions/setup-java@v5
    5 uses: actions/setup-node@v7
    2 uses: actions/setup-python@v7
    11 uses: actions/upload-artifact@v7
    7 uses: dtolnay/rust-toolchain@stable
    1 uses: nttld/setup-ndk@v1
    1 uses: oxidize-rb/actions/cross-gem@v1
    1 uses: pypa/gh-action-pypi-publish@release/v1
    3 uses: ruby/setup-ruby@v1
    1 uses: rubygems/configure-rubygems-credentials@main
    1 uses: rust-lang/crates-io-auth-action@v1
    2 uses: softprops/action-gh-release@v3
    EOF
    ```

    Expected output: empty diff, exit 0. The heredoc is in `LC_ALL=C sort` order (uppercase refs
    first) — the assertion is the **multiset**, so if your shell's ordering differs, re-sort both
    sides rather than editing a count. Additionally
    `grep -cE '^\s*(- )?uses: ' .github/workflows/release.yml` must print `97`.

- `grep -c 'setup-uv' .github/workflows/release.yml` → `0`

- The iteration-139 guard invariant is untouched — this script prints `release.yml guards OK` at
    HEAD and must still do so after the bump:

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

- Artifact wiring still resolves — every `download-artifact` `name:`/`pattern:` matches an
    `upload-artifact` `name:` (prints `unmatched: []` at HEAD and must still):

    ```bash
    uv run python - <<'PY'
    import fnmatch, re, yaml
    jobs = yaml.safe_load(open(".github/workflows/release.yml"))["jobs"]
    ups, downs = set(), []
    for jn, j in jobs.items():
        for s in j.get("steps", []):
            u, w = s.get("uses", ""), (s.get("with") or {})
            if u.startswith("actions/upload-artifact"):
                ups.add(str(w.get("name", "")))
            if u.startswith("actions/download-artifact"):
                downs.append((jn, str(w.get("name", "")), str(w.get("pattern", ""))))
    assert len(ups) == 11, sorted(ups)
    assert len(downs) == 20, len(downs)
    bad = [
        (jn, n, p) for jn, n, p in downs
        if p and not any(fnmatch.fnmatch(re.sub(r"\$\{\{[^}]*\}\}", "*", u), p) for u in ups)
    ]
    print("unmatched:", bad)
    assert not bad
    PY
    ```

- `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 .github/workflows/release.yml` exits 0
    with no output (module already in the Go cache; validates expression syntax YAML parsing cannot)

- `uv run prek run check-yaml --files .github/workflows/release.yml` → `Passed`

- `uv run prek run yamlfix --files .github/workflows/release.yml` → `Passed` with no
    `files were modified by this hook`

- `mise run check` → all hooks Passed, exit 0

- No source or gate side effects: `git status --porcelain` lists only
    `.github/workflows/release.yml` (plus runner-owned `.claude/context/` files). No Rust source
    moved, so no CRAP/iai baseline, `cargo deny` or semver refresh is owed.

## Done When

All verification criteria pass: `release.yml` carries the current major of all nine stale actions
with the `uses:` multiset, the 97-line total, the 28/29 guard invariant and the upload↔download
artifact wiring all intact.
