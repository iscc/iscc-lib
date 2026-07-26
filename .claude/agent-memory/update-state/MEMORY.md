# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `quality-gates.md`, `lint-tooling.md`. **Size budget: under 140 lines** —
archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap instead of calling CI green-for-HEAD (bit at 148).
- **ALWAYS `tail -5 .claude/context/iterations.jsonl`** — the ONLY place a crashed role shows up
    (also the `audit` role, every 10th iteration). **A non-OK status does NOT mean no work:
    corroborate with `git log` for the `cid(<role>):` commit** — infra crash vs benign overrun and
    the crashed-review fingerprint → `MEMORY-archive.md`.
- **Verify a Go-package claim in ~60s**: `/tmp` module requiring
    `github.com/iscc/iscc-lib/packages/go v0.0.0` + a `replace` to the repo, then
    `GOFLAGS=-mod=mod CGO_ENABLED=0 go mod tidy && CGO_ENABLED=0 go run main.go`.
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 153): crate/pkg READMEs & CLAUDE.md 12 each (**scope the glob to
    `crates/*/ packages/*/`** — bare `grep README.md` gives 14); pytest-benchmark 18; UniFFI 32;
    docs pages **23** + `includes/`; `docs/howto/*.md` 11; speedups 1.3x-158x; ffi extern **47**
    (`'#\[unsafe(no_mangle)\]'`; bare `no_mangle` gives 48); iscc-lib `#[test]` **335**
    (`grep -rc --include="*.rs"`, sum); `packages/go` `^func Test` **177**.
- **GOTCHA — `git ls-files 'docs/**/*.md'` returns 13, NOT 24**: it misses top-level `docs/*.md`.
    Use both globs, or run `uv run scripts/check_docs_nav.py` (prints the authoritative page count).
- **version_sync TARGETS** = **21** (`scripts/version_sync.py --check | grep -c '^OK'`) = the
    authoritative set of version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir, `.iai-baseline.json` 16 entries); coverage + CRAP
    (`.crap-baseline.json` **100** entries — `--fail-regression` is **CI-ONLY**, so a green
    `mise run check` proves nothing); cargo-deny; docs page-list parity (no dedicated CI job — prek
    hook + a pytest in `python-test`).
- **Can go red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml
    static gate — check the JOB LOG for zero `warning: skipped`. **Informational:**
    cargo-semver-checks (`continue-on-error` until v1.0.0, HELD).

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests and `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the freeze rule.
- `ci.yml` — **20 YAML job entries → 21 jobs → 22 check names**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR on `needs.python-test.result`; `push:` under
    `on:` is NOT a job. `release.yml` — 8 registry toggles; the Swift XCFramework step lives in
    `prepare-release` and is NOT a toggle.
- **Unicode = 16.0.0 + SENTINEL freeze rule → read `unicode-contract.md` before ANY Unicode call.**
    Crit 1+2 MET (148); crit 3 at **6 of 11 native surfaces** (Rust, Python, WASM, Ruby, napi, Java)
    plus the pure-Go port, leaving **5 surfaces + 4 sibling `data.json`**; crit 4 (differential
    sweep) has no harness. `decisions.md` keeps 2 SUPERSEDED designs that must NOT be implemented.
- **Per-surface fixture-loading plumbing table → `unicode-contract.md`** (canonical-path vs
    vendored-copy vs no-JSON-parser; read it before costing any propagation slice). One landmark
    worth inlining: **surefire's default CWD is the pom basedir**, NOT the `mvn -f` invocation dir —
    that is why the Java suites' `../../iscc-lib/tests/…` paths resolve.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
    `iscc-py` has **12** `.detach(` sites. `benches/`: 12 criterion (0.7) + iai 0.16 (11 fns, 16
    cases). **Inline `# held:` = authoritative pin rationale** — root `Cargo.toml` has **4**;
    `pyproject.toml` zero.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; prek/CI parity holes CLOSED
    (138, 139) — local prek is a strict SUPERSET. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory, GHA histograms + slice history** → `dep-refresh-survey.md`. All GHA
    refs CURRENT (slice 9, 140); release.yml keeps **97 `uses:`**, with
    `rubygems/configure-rubygems-credentials@main` (L895) the ONLY unpinned ref. No
    Dependabot/Renovate; **`rb_sys` pinned in THREE places that move together**.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change.
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede` (losing designs are logged chronologically). At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute binding claims yourself** — cheap probes beat inherited text (confirmed a
    `critical` bug at 147, its fix at 148; refuted a "checked-in stale napi `.node`" claim across
    151-153 — it is gitignored). Deliberate deviations from a fix sketch are legitimate: read the
    in-source comment first.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.
    Spec *prose* also rots: at 149 `rust-core.md` still described a defect fixed 2 iterations back.
- **Cost-rank a propagation slice on THREE axes** (153, extended 154): fixture-reading plumbing
    (parses JSON? canonical relative path vs vendored copy?) × target-API coverage × **local
    buildability** (`command -v`). Ranking on API coverage alone put C++ at "cheap" for two
    iterations; the third axis is what demotes Kotlin/Swift (no toolchain here → CI-only proof).
- **Oracles (149):** a "must NOT be" value means nothing unless the DESIGN producing it is named,
    and a fixture can be structurally incapable of gating what it claims to. Ask what a test must
    *distinguish*.

## Current State (assessed-at: 2c175f2, iter 154)

- **IN_PROGRESS — CI GREEN AND COVERING ALL CODE.** `origin/develop` == `32fbfc4`: 43 check-runs, 22
    names, 0 non-success. HEAD `2c175f2` is ONE `cid(log)` commit ahead but the non-`.claude` diff
    vs origin is EMPTY. ~2x runs because PR **#44 (develop→main) is OPEN** ("Release 0.6.0" — NOT
    shipped; version **0.5.0**). Iteration 153 ran clean (4 roles `OK`, verdict `PASS`).
- **Statuses:** Rust-core partially met (crit 3 at **6 of 11** surfaces + the pure-Go port; crit 4
    not started); all else met except CI/CD (partial). All 6 gated suites ride EXISTING CI jobs.
    Per-section evidence lives in state.md — don't duplicate it here.
- **Invariant to re-check every propagation slice:**
    `git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** paths, matching
    `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` (the 152 drift gate: table +
    `git ls-files` set check + `>= 5` floor + ASCII guard, rides `python-test`). Register every new
    copy; keep canonical basenames.
- **Next = slice 4, recommended C#** (alone or + C FFI): only remaining surface green on all three
    axes. Prefer a csproj `<Content Include>` at the canonical fixture + `Link="testdata\…"` to
    avoid a tracked copy. **Defer Kotlin/Swift** (0 text tests AND unbuildable here); C++ last.
- **`specs/rust-core.md` is STALE on Go `Final_Sigma`** (present tense, L149-157) though fixed at
    147; crit-1/crit-3 boxes unchecked though met — human-owned, CID doesn't edit specs.
- **Issues: 8** (4 `normal`, 4 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`), unchanged at 153.
    **Human backlog CLEARED.** AUTHORIZED for CID: the rubygems `@v2.1.0` pin; major dep bumps **one
    per step** (magnus 0.8 / jni 0.22 = source rewrites; xunit 3.x, Test.Sdk 18.x, Gradle wrapper,
    JUnit 6.x); exhaustive `specs/ci-cd.md` job table. DEFERRED: npm OIDC.
- **Don't re-flag as DONE**: napi+Java 153; drift gate 152; WASM+Ruby 151; Python+Go 150; sequence
    vectors 149; sentinel conversion 148; Go `Final_Sigma` 147 (earlier → `MEMORY-archive.md`).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` bind mount): a mode-only commit
    shows nothing in `git status`/`diff` (`--stat` = a 0-line file). Use `git ls-files -s <path>`.
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Grep binding APIs CASE-INSENSITIVELY** (near-miss 152): napi exports are **snake_case**, so a
    `textClean` grep wrongly cleared napi; Java/Kotlin/Swift are camelCase, C# PascalCase. Also
    `crates/iscc-napi/__tests__` (two underscores).
- **Container toolchains (re-probed 154):** `dotnet`, `mvn`, `gcc`, `java`, `go`, `ruby`, `node`,
    `wasm-pack` present; **`cmake`, `swift` and `gradle` are ABSENT** → `packages/{cpp,swift}` are
    CI-verified only, and Kotlin has only the Gradle *wrapper* (network download + bind-mount
    flakes). Probe with `command -v` — never assume from a package dir existing. Other env gotchas →
    `MEMORY-archive.md` (metrics.jsonl counts gitignored artifacts; `proc-macro-error2`
    future-incompat warning is dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
