# Define-Next Agent Memory

Scoping decisions, estimation patterns, architectural knowledge across CID iterations. **Keep under
140 lines**; archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next". Then `critical` issues, regardless of feature trajectory.
- **`tools/cid.py` never pushes — review pushes the whole batch on PASS**, so "get unpushed commits
    under CI" is never a step; it only affects *sizing* — scope a small diff when HEAD carries
    unseen commits so a red CI run is attributable (162).
- **Verify claimed gaps by reading actual files** — state.md and handoff "IDLE" both go stale. **A
    `human(...)`/non-`cid()` commit newer than the last `cid(review)` invalidates the handoff
    wholesale** (147) — check `git log` before trusting "Next".
- **A suggested fix (issue or handoff) is a hypothesis, not a spec — probe it** (147: the
    `cases.Caser` hoist was unsafe and no faster); cheap probes turn `## Implementation Notes` into
    measured facts.
- **A crashed review role (`iterations.jsonl` FAIL/turns:1, no `cid(review)` commit) means the prev
    step has NO verdict** — re-verify from the tree. A `define-next` TIMEOUT leaves an uncommitted
    `next.md` (155): adopt-and-verify, re-deriving every numeric invariant.
- **A "remaining item" in an issue body can already be done — probe the external source first**
    (170: a `gh api` loop showed zero refs behind for a still-listed refresh; 165: read a range out
    of the tool's own artifact).
- **Any upstream-API claim in Implementation Notes must be read out of
    `~/.cargo/registry/src/*/<crate>-<ver>/`, seeing which item an attribute belongs to** (168: a
    misattributed `#[deprecated]` cost iter 169; manifests carry `rust-version`).
- **A fix touching surfaces no test exercises → scope fix + missing tests as ONE step** (169; tests
    are outside the 3-file budget). **Run an expensive probe while scoping when it can invert the
    plan** (155/157; 161 Swift fetch killed a 10-iter-old veto).
- **Generated/tool-output files (Cargo.lock, bindings) and docs don't count toward the 3-file
    limit** — batch related small changes (version sync + docs; several fixes in one crate).
- **IDLE is valid** only when all target sections met and just `low` issues remain — an
    already-specced, locally-verifiable target gap or infra/release fix is NOT idle work. **HUMAN
    REVIEW override is for BUG fixes, not NEW policy gates** (gate amending spec/notes needs
    sign-off).
- **next.md/MEMORY.md are sensitive** — Write needs a prior Read; the prek mdformat hook reflows
    (wraps at 100, can inject a double space in inline code spans) — reword to avoid.

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **33** crate-root re-exports (`gen_iscc_id_v1` landed
    in core 177; doc/count text still reads 32 until the sweep step). `SumHasher` at
    `iscc_lib::streaming::SumHasher` is NOT Tier 1.
- Go bindings are pure Go (no CGO/WASM). `gen_iscc_code_v0` vectors have no `wide` — pass `false`;
    `"stream:<hex>"` prefix = hex-encoded byte data.

## Dev Environment Constraints

- **No shellcheck** (CI-only). `cmake` via `uv run --with cmake cmake …` (160); **Swift runs here**
    (swift.org debian12 tarball, ~5 min, no sudo). `uniffi-bindgen`:
    `cargo run -p iscc-uniffi   --features bindgen --bin uniffi-bindgen`; SPM module MUST be
    `iscc_uniffiFFI`; no `const`/`usize`/borrowed/generic exports.
- `dotnet` 8.0.423 on `$PATH`, nuget.org reachable → .NET restore/test locally verifiable.
    cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner installed;
    cargo-deny via `cargo binstall cargo-deny@0.19.9 --force`.
- **rustup carries stable (1.97.1) *and* 1.85.0** → every MSRV claim is locally verifiable:
    `cargo +1.85.0 check -p <crate> --locked`. Never defer an MSRV assertion to a CI job.
- **`mise run lint` is only `cargo fmt --check` + clippy + ruff**; taplo/mdformat/hygiene run in
    prek (`mise run check`). Never credit a TOML/Markdown format check to `lint`.

## CI/Release, Docs, Gotchas

- Release: `workflow_dispatch` per-registry checkboxes; version_sync.py manages **21** targets;
    `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in Rust CI); XCFramework cache key
    hashes build inputs. Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need
    an entry per new howto guide (template `docs/howto/dotnet.md`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; "10 gen fns" vs "9 conformance fns" — never blanket 9→10. Doc-example
    casing: Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/Swift/Kotlin `camelCase` — but napi
    and WASM export snake_case (`gen_iscc_id_v1`, not camel); Swift/Kotlin take named `bits`/`u`.
- **4 quality gates**: Semver (informational until human-gated v1.0.0) + Coverage/CRAP + Perf +
    Audit (enforcing). **Baselines move only via deliberate reviewed `mise run` commits**, never
    from CI; don't widen `--epsilon`. `cargo deny check` is authoritative locally (reads
    Cargo.lock).

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released; 12 bindings meet core criteria; v0.6.0 features + Unicode chain DONE (161); 169
    closed JNI debt, 170 `packages/dotnet` lockfile. **2026-07-28 human unblocked v0.6.0** (active):
    171 (`criterion` 0.8) + 172 (`uniffi` 0.32) closed the dep issue →
    [dep-refresh ledger](dep-refresh-ledger.md). Trigger-only: go1.27 + Go freeze table (~Aug 2026).
    HELD `low`: v1.0.0 + Semver-enforcing, MSRV job, npm OIDC.
- **A dep major can leave a *declaration* false without breaking anything** (172→173: uniffi 0.32
    pushed `iscc-uniffi`'s real floor to 1.91). Declaring the true floor on an **unpublished** crate
    is CID-doable; the **root/published** `rust-version` stays Titusz's call. **Trust fresh state.md
    over an IDLE handoff when a non-`cid()` commit post-dates the last review** (174: out-of-loop
    `2c4e487` reopened scope to 33 symbols + ISCC-IDv1 on all 11 surfaces, #43).
- **2026-07-28 (177 AND AGAIN 178): state.md conflates a Semver check-run "failure" with the
    workflow conclusion** — it declares "CI RED on develop", but
    `gh run list --branch develop --json   conclusion` shows the develop tip (=HEAD code)
    `conclusion:success` on BOTH the push run and the open develop→main PR run. The `Semver` job is
    `continue-on-error: true` (ci.yml L355), so its red check-run does NOT fail the workflow. This
    phantom recurs every iteration the semver check-run is red — verify workflow *conclusion* at the
    source each time, never trust a state.md "red" derived from check-run status. 177 and 178 both
    skipped a phantom "fix CI" step and went to #43 fan-out.
- **ISCC-IDv1 (#43): core DONE (33 symbols); each surface its OWN step (tech wraps differently)** →
    [idv1 fan-out facts](iscc-idv1-fanout.md) has golden/validation + per-surface slice recipes.
    Done: 178 Python, 179 Go rename, 180 napi, 181 wasm+JS-number validation, 182 ffi, 184 jni, 185
    rb, 186 uniffi (Swift+Kotlin, one step), 187 dotnet C#, **188 cpp (LAST minting surface)**.
    **After 188: ALL 11 minting surfaces done — ONLY the Tier-1 32→33 doc/count sweep remains** (its
    exact stale sites incl. 3 non-Markdown are enumerated in issues.md #43 + the fanout file).
- **Closed phases: 115–123 (features) + dep slices 124–137 + 162–172** → MEMORY-archive.md +
    [dep-refresh ledger](dep-refresh-ledger.md). Still-biting: CRAP regression gate is **CI-only**;
    **never move a consumer floor** (MSRV, `go` directive, `required_ruby_version`, a published
    binding's compiler) in a refresh slice (128); preserve a major's behavioural contract with a
    built-in option, never by editing tests (168).
- **Lint/formatter tool bumps + hook-config changes** →
    [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact file/finding
    count a pass/fail criterion** — use the exit code.
- **Unicode chain DONE (147–161)**: sentinel freeze, `Final_Sigma` freeze, `mise run unicode:sweep`
    gate, boundary vectors on all 11 surfaces; never revive the superseded category override or a
    15.1.0 declared version. A 13th vector costs 12 suites — any fixture edit is its own slice.
    Details → [unicode-freeze-facts](unicode-freeze-facts.md) + propagation ledger.
- **Gate/checker steps in `scripts/`** → [gate scripts playbook](gate-scripts-playbook.md) +
    [release.yml static gates](release-yml-static-gates.md) (split behavioural from mechanical in
    the CI-unexercised `release.yml`, 139/140).
- **Text-hot-path steps trip CI-only CRAP `--fail-regression` + `.iai-baseline.json` 10% Ir; a
    `tests/`-only or binding-only step trips neither** — say so in next.md.
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep
    ([ty gate trap](define-next-ty-generator-scripts.md)). `uv run zensical build` (exits 0, "No
    issues found", ~8s) verifies any docs-only step.
- **Recurring**: cargo-deny reds on fresh RustSec advisories vs dev/bench deps — CI-red-first,
    prefer `cargo update -p <crate>` over a `deny.toml` ignore. When 3 of 4 recent iters are
    CI/lint/workflow → prefer a user-facing item once (143).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (159); pair the no-op with a pytest case proving the gate fires on a mutated
    source — unless a third-party generator emits non-hook-clean bytes (172).
- **"Not buildable in this container" claims decay — re-probe before they veto a slice** (Kotlin
    154, C++ 160, Swift 161 each ran in minutes). Ladder: `$PATH` → `uv run --with <tool>` → cached
    toolchain dir → upstream tarball. Trap: a runner can report success **without executing**
    (gradle UP-TO-DATE) — force it. **When state.md names the file to edit, re-derive it** (160: it
    pointed at the public `iscc` INTERFACE for a test-only include dir vcpkg/conan consumers
    inherit).
