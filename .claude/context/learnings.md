# Learnings

High-signal pitfalls, patterns, and verified conventions accumulated during CID iterations. The
review agent maintains this file — append new entries, prune stale ones, archive completed-phase
entries to `learnings-archive.md`.

**Size budget:** Keep under 200 lines. When this file exceeds 200 lines, move entries about
fully-met target sections to `learnings-archive.md`.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → binding crates (py, napi, wasm, ffi, jni, rb,
    uniffi). Each depends only on `iscc-lib`, never on another binding. `packages/go` is NOT a
    binding — it is an independent pure-Go reimplementation (see the Unicode entry below)
- Tier 1 API (32 symbols) exposed via `pub use` at crate root. Tier 2 is `pub(crate)` — internal
    only, never crosses FFI. Core is sync; each binding adapts async idiomatically

## Reference Implementation

- Reference code lives in `reference/iscc-core/` (shallow clone, gitignored). Read source files
    directly — do not use deepwiki MCP
- When porting from Python reference, verify against Rust `crates/iscc-lib/src/` first — the Rust
    implementation is the authoritative source for this project
- **`iscc-core` output is not stable across CPython versions** (5,185 code points differ 3.13 vs
    3.14 — `text_clean`/`text_collapse` strip category `C` incl. unassigned `Cn`, so the result
    tracks `unicodedata.unidata_version`). Always name the interpreter, and check both via
    `uv run --python 3.13 --no-project --with iscc-core python -c …`. Upstream: iscc-core#137
- Any dependency shipping DATA TABLES (Unicode, locale, tz) must be proven output-neutral by a
    **differential sweep** over all 1,112,032 code points (~2 min; recipe → `learnings-archive.md`),
    never by a green vector suite — every vendored conformance vector predates Unicode 16

## Tooling

- `mise` manages tool versions and tasks. Python env uses `uv`. Hooks via `prek`
- Never use `mise` in CI — call tools directly
- Pre-push-**only** gates: clippy `-D warnings`, cargo test, pytest, `ty check`. The ruff `S`/`C901`
    scans also run pre-commit since iter 134 (they are in the default select now)
- **PyO3 is `0.29`** (iscc-py only): keep `#[pymodule(name = "_lowlevel", gil_used = true)]`
    explicit. Per-hop upgrade recipe → `learnings-archive.md`
- **`_lowlevel.pyi` stub bodies are docstring-only — no trailing `...`** (iter 131). The wheel ships
    `py.typed`, so the stub is consumer-facing: check changes against `mypy 1.18 --strict` +
    `pyright 1.1.407` too, not just `ty` (0.16 double-reports docstring + `...` as `PIE790` *and*
    `PYI048` on one line, so N findings collapse to N/2 deletions)
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (iter 133,
    `scripts/gen_unicode16_unassigned.py` + `unicodedata2==16.0.0`): inline `# /// script` metadata,
    run with `uv run --script <path>`, and add the path to `[tool.ty.src] exclude` with a comment
    (the `packages/cpp/conanfile.py` precedent). Keeps `uv.lock` free of a dep that would need a
    permanent hold-back in every dependency refresh. Generated Rust must be data-only + rustfmt
    stable (one tuple per line, 4-space indent, trailing commas) so regeneration is a no-op diff
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer** (iter 124):
    `mise run bench:iai:check` dies with "No such file or directory" until
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`
    (MUST match the `iai-callgrind` pin) — CI mirrors this per-run; not a bug, do not file an issue

## ISCC Algorithm Knowledge

- **Unicode data version — declared 16.0.0 with a freeze rule** (divergence found iter 129, decided
    2026-07-25, **Rust core implemented iter 133**): Go stdlib/`x/text` = 15.0.0, Python 3.13 =
    15.1.0, Rust `unicode-general-category` = 16.0.0 / `unicode-normalization` = 17.0.0, and Go's
    `unicode.C` includes unassigned `Cn` — so 5,813 post-15 code points are stripped by
    Go/`iscc-core` but kept by the Rust core (repro `Ɤ` U+A7CB); no vector catches it. Freeze rule:
    `text_clean`/`text_collapse` remove code points unassigned in Unicode 16.0.0 *before* any
    normalization or category lookup (vendored 731-range table, regen
    `uv run --script scripts/gen_unicode16_unassigned.py`). Deltas: 15.1→16 = 5,185 category + 56
    normalization; 16→17 = 4,803 + 1 — all new assignments, zero changes to assigned characters.
    Never "fix" one binding to match another; remaining steps → `issues.md`
- **The freeze rule changes ADJACENCY, so it diverges from `iscc-core` on sequences even with
    identical Unicode data** (measured iter 133 review). Removing a `Cn` code point *before*
    normalization unblocks contextual transforms that `iscc-core` (remove *after*) still blocks:
    `text_clean("e\u{0378}\u{0301}")` → `U+00E9` vs reference `U+0065 U+0301`;
    `text_clean("\u{1100}\u{0378}\u{1161}")` → `U+AC00` vs `U+1100 U+1161`;
    `text_collapse("\u{391}\u{3A3}\u{378}\u{392}")` → `…σ…` vs `…ς…` (Rust `to_lowercase` applies
    Final_Sigma using the *stripped* context). A per-code-point sweep cannot see this class — any
    differential sweep must include multi-code-point sequences (base+Cn+mark, jamo+Cn+jamo,
    Σ+Cn+cased)
- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing
- Conformance vectors: `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data. Empty
    after prefix = empty bytes. 50 total vectors (v1.3.0): 20+5+3+5+3+2+4+3+5
- **Per-algorithm internals** (nibble interleave, MinHash, audio SimHash, mixed grouping,
    `encode_units`, DCT), settled API-parameter facts (trim/decode checks, pixel layout, MainType
    Ord, JCS, `bits`/`wide`) and **ISCC-IDv1** are archived → `learnings-archive.md`
- `conformance_selftest` masks truncated codes bitwise — never compare full strings below 256 bits
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`**: a
    `< nbytes` guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`).
    Enforced in Go `IsccDecode` (iter 120) + Rust core `iscc_decode` (iter 121, two-branch). NOTE
    composite `iscc_decompose` legitimately consumes trailing units — do NOT harden it
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID (C FFI: length index for 64-bit codes is 1, not 0)

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern** (9 registry inputs → build → smoke test → publish; `version_sync.py`
    manages **21** targets) archived iter 133 → `learnings-archive.md`
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like every other release job), so the `--ref main` re-trigger breaks for Swift
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (enforcing at v1.0.0;
    `rust-core.md` box stays `[ ]`), `coverage` enforcing. `mise run semver` / `mise run coverage`
- **CRAP gate (ci-cd.md)**: ENFORCING Phase 3 (`cargo crap --fail-regression --fail-above`, thresh
    30.0, max ~22.3), `.crap-baseline.json` COMMITTED (regen `mise run crap:baseline`),
    `.cargo-crap.toml` excludes `benches/**`. **CI-ONLY guard gap**: NOT in `mise run check`/
    pre-commit — a source change adding a branch/loop to a covered fn lands green locally but reds
    CI unless the baseline is refreshed in the SAME step (never widen epsilon/threshold). Mechanics
    → `learnings-archive.md`
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped binary → all benches `summary: 0` false-green) → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING (ci-cd.md)**: root `deny.toml` (config v2,
    `yanked = "deny"`, two dev-only iai-callgrind advisories ignored) + `audit` CI job
    (`cargo-deny@0.19.9`) + `mise run audit`. cargo-deny reads Cargo.lock + metadata (NOT artifacts)
    so green locally is authoritative (`cargo binstall cargo-deny@0.19.9`). A yanked crate OR fresh
    RustSec advisory reds the gate on ANY push with no code change — not a regression. Fix with
    `cargo update -p <crate>` (confirm dev-only reach via `cargo tree -i <crate> -e no-dev` =
    empty), NOT a `deny.toml` ignore — ignore ONLY when no patched release exists
- **Dependency refresh is sliced per-ecosystem** (v0.6.0 `[human]` issue; per-slice status lives in
    `issues.md` — all 7 ecosystem slices done; ruff 0.16, `release.yml` refs and deferred majors
    remain). Verify each Rust slice with the 4-gate set (`test`/`lint`/`audit`/`bench:iai:check`);
    `cargo-deny` is the main risk (new transitive license/advisory). **Hold-back reasons are inline
    `# held:` comments** beside the pin — confirm the stated reason from registry metadata, not
    prose: `cargo info <crate>@<ver>` (`rust-version`), `gem specification <gem> -v <ver> --remote`
    (transitive pins), `https://rubygems.org/api/v1/versions/<gem>.json` (`ruby_version`; the v2
    endpoint returns `null`). GOTCHA: `criterion` > 0.5 deprecates `criterion::black_box`, fatal
    under `-D warnings` → use `std::hint::black_box`
- **ruff 0.16 adoption is sliced by decision type, not by file** (iters 125/131/134/135/136): run
    the unpinned version with `uvx ruff@0.16.0 check .` — it never touches `uv.lock`, so `ruff<0.16`
    stays in `pyproject.toml` until the tree is clean. 104 → **0** across sub-slices A–D; only the
    pin drop (E) is left. Rules go in `[tool.ruff.lint] extend-select` — **never `select`**, which
    drops ruff's `E4`/`E7`/`E9`/`F` defaults; the pre-push `--select S`/`--select C901` hooks stay
    as deliberate redundancy naming the failing gate. isort needs BOTH
    `[tool.ruff] src = [".", "crates/iscc-py/python"]` (makes `iscc_lib` first-party) and
    `[tool.ruff.lint.isort] combine-as-imports = true` (else the `_lowlevel` re-export block
    shatters into ~60 statements). **Never `ruff@0.16 check --fix .`** — it deletes the load-bearing
    `# noqa: S603/S607` in `tools/`+`scripts/`; use the pinned ruff + `--select`
- **A file-mode change needs `git update-index --chmod=+x`, not just `chmod`** (iter 136, ruff
    `EXE001` on `tools/cid.py`): `core.fileMode=false` here, so a plain `chmod +x` satisfies local
    tools but is invisible to git. Run both; prove it with `git ls-files -s <path>` → `100755`
- **Two ruff invocation gotchas**: (1) an unused `# noqa` is invisible unless `RUF100` is selected
    (in `extend-select` since iter 135) — before deleting any directive prove it dead with
    `uv run ruff check --select <rule> --ignore-noqa`; `S603` never fires on a fully static
    list-literal argv, only on dynamic argv (`["git", "add", rel]`). (2) the pre-commit `ruff-check`
    hook passes *filenames* and no `--force-exclude`, so hook-mode can disagree with `ruff check .`
    — re-probe per-file whenever a path-sensitive setting (`src`, `exclude`, isort) changes
- **A binding-toolchain bump can silently raise the *consumer* floor** (iter 128 Kotlin, documented
    iter 132): KGP 2.1.10→2.4.10 stamps `mv=[2,4,0]` into the published jar and
    `kotlin-stdlib:2.4.10` into the POM; a `mavenLocal` consumer proved 2.1.10/2.2.21 fail, 2.3.21
    passes. Treat compiler/toolchain bumps in a *published* binding as support-policy changes
    reserved for Titusz, not pins. The floor ("Kotlin 2.3 or newer") now lives in the root README,
    `packages/kotlin/README.md`, `docs/howto/kotlin.md` and `specs/kotlin-bindings.md` — any future
    bump that moves it must update all four in the same step
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) archived iter 129 →
    `learnings-archive.md`. Read it before touching `pom.xml` / `build.gradle.kts` or before calling
    a Gradle error a test failure
- **`cargo tree -i <crate>` prints "nothing to print" for proc-macro / target-specific deps** — add
    `--target all`. The `proc-macro-error2 v2.0.1` future-incompat warning emitted on every
    `cargo test`/`cargo bench` traces to `iai-callgrind-macros` (dev-only), **not** magnus/rb-sys;
    no fixed release exists (iai-callgrind 0.16.1 is latest), so it stays a warning for now
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** —
    `gh api repos/<o>/<r>/releases/latest` proves a release exists, not that `@vN` resolves. Always
    confirm with `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN`.
    `astral-sh/setup-uv` stopped publishing floating majors after `v7` → pin `@v9.0.0` with a
    `# exact tag:` comment; it recurs in `release.yml`. Current majors (2026-07-24) →
    `.claude/agent-memory/advance/deps-refresh.md` (`upload-pages-artifact` + `deploy-pages` pair)
- **ci.yml sets `cancel-in-progress: true` per ref** — pushing a follow-up develop commit cancels
    the in-flight run of the previous sha (its check-runs conclude `cancelled`, not `failure`). When
    a step's Done-When needs a green CI on a specific sha, let that run conclude before pushing
    again. Each develop commit also triggers TWO runs (push + `pull_request` from the open
    develop→main PR), so check-run totals are ~2× the job count

## CID Process

- Never force-push to `develop` during a CID loop — agents commit incrementally (branching model
    itself is in CLAUDE.md)
- **Feature flags**: fully met, archived → `learnings-archive.md` (read before touching
    `[features]`)
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — frequently stale. Verify independently against the source (`cargo search`,
    `npm view`, Maven Central API, `pip index versions`, Go module proxy, `gh api`)
- **Context growth**: learnings.md and agent memory grow monotonically — archive completed-phase
    entries periodically to prevent token bloat
- **Detect concurrent CID loops** (iter 97): context files changing mid-review, or `mise run check`
    reporting spurious "files were modified by this hook" on a file advance never touched, means a
    race. Confirm with `ps aux | grep -E 'cid:run|claude -p CID iteration'`, then flag HUMAN REVIEW
    REQUESTED — do NOT kill processes yourself, and do NOT push
- **Human-handoff vs IDLE (iter 111)**: when autonomous work runs out but `normal` issues remain
    that are all `HUMAN REVIEW REQUESTED` spec amendments, strict `**IDLE**` (all issues `low`) is
    NOT met. Flag `**HUMAN REVIEW REQUESTED**` (runner "pause") instead of `**IDLE**` (which runs
    meta-improve, reserved for all-`low`). Don't manufacture churn to avoid the pause
- **Pre-push mdformat blocks on non-conforming context files**: the pre-push hook runs mdformat
    (`--wrap 100 --number`, isolated `mdformat-mkdocs[recommended]` env) on every file in the push
    range — incl. `next.md` and per-agent `MEMORY*.md`. A non-conforming file rejects the whole
    batch push even though staged-only `git commit` passed. define-next MUST run `mise run format`
    before committing; review can unblock by reformatting + amending (match hook args exactly)
- **next.md must never task advance with editing `issues.md`** (iter 135): advance's protocol
    forbids writing it and review owns issue progress/resolution. A slice-progress ledger paragraph
    belongs in the handoff Notes for review to append — advance correctly refused and quoted it
- **Role model assignment (2026-07)**: `advance` runs on Claude Fable 5 (`model: fable`,
    `effort: xhigh`, runner timeout 3600s); all other roles on `opus`. Deliberate diversity — Fable
    implements, Opus reviews, Codex is the second opinion. Do not "unify" onto one model
