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
    `next.md` (155): adopt-and-verify but re-derive every numeric invariant (not a bounce — say so).
- **A "remaining item" in an issue body can already be done — probe the external source first**
    (170: a `gh api` loop showed zero refs behind for a still-listed `release.yml` refresh; 165:
    read a remembered version range out of the tool's own artifact → ledger).
- **Any upstream-API claim in Implementation Notes must be read out of
    `~/.cargo/registry/src/*/<crate>-<ver>/`, with enough context to see which item an attribute
    belongs to** (168: a misattributed `#[deprecated]` cost iter 169; manifests carry
    `rust-version`).
- **A fix touching surfaces no test exercises → scope fix + missing tests as ONE step** (169; tests
    are outside the 3-file budget). **Run an expensive probe while scoping when it can invert the
    plan** (155/157 sweep reordered milestones; 161 Swift fetch killed a 10-iter-old veto).
- **Generated/tool-output files (Cargo.lock, bindings) and docs don't count toward the 3-file
    limit** — batch related small changes (version sync + docs; several fixes in one crate).
- **IDLE is valid** only when all target sections met and just `low` issues remain — an
    already-specced, locally-verifiable target gap or infra/release fix is NOT idle work. **HUMAN
    REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** (a gate
    amending spec/notes needs human sign-off first, 106/112).
- **next.md/MEMORY.md are sensitive files** — Write needs a prior Read; the prek mdformat hook
    reflows and can inject a double space inside inline code spans — reword to avoid.

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports (33 once
    `gen_iscc_id_v1` lands), bound in all languages. `SumHasher` at `iscc_lib::streaming::SumHasher`
    is NOT Tier 1.
- Go bindings are pure Go (no CGO/WASM). `gen_iscc_code_v0` vectors have no `wide` — pass `false`;
    `"stream:<hex>"` prefix = hex-encoded byte data.

## Dev Environment Constraints

- **No shellcheck** (CI-only). `cmake` via `uv run --with cmake cmake …` (160); **Swift runs here**
    (swift.org debian12 tarball, ~5 min, no sudo — recipe → propagation ledger).
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`; SPM module
    MUST be `iscc_uniffiFFI`; no `const`/`usize`/borrowed/generic exports.
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
    hashes all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson groupId `com.google.code.gson`; "10 gen fns" vs "9 conformance
    fns" — never blanket 9→10 (learnings.md). Doc-example name styles: Ruby/Rust `snake_case`;
    C#/Go/Java `PascalCase`; JS/Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u`.
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
- **A dep major can leave a *declaration* false without breaking anything** (172→173: uniffi 0.32's
    `cargo-platform` pushed `iscc-uniffi`'s real floor to 1.91). Declaring the true floor on an
    **unpublished** crate is CID-doable; the **root/published** `rust-version` stays Titusz's call.
- **2026-07-28 (174): out-of-loop commit `2c4e487` REOPENED scope AFTER a review IDLE/NONE handoff**
    → target grew to 33 Tier 1 symbols + experimental **ISCC-IDv1** on core + all 11 surfaces (#43).
    Trust fresh state.md over an IDLE handoff when a non-`cid()` commit post-dates the last review.
- **2026-07-28 (177): state.md conflated a Semver check-run "failure" with the workflow conclusion**
    — it declared "CI RED on develop", but `gh run list --branch develop` shows tip `a961c75` (=HEAD
    code) both runs `conclusion:success`. The `Semver` job is `continue-on-error: true` (ci.yml
    L355), so its red check-run does NOT fail the workflow. CRAP was fixed at 176 → CI is GREEN.
    Verify workflow *conclusion* at the source, never trust a state.md "red" derived from check-run
    status. So 177 skipped a phantom "fix CI" step and went straight to #43 Part 2.
- **#43 Part 2 core step (177): `gen_iscc_id_v1(timestamp:u64, hub_id:u16, realm:u8)` +
    `IsccIdResult` in `iscc-lib` core.** Reference `iscc_id.py`: `body=(ts<<12)|hub`, `to_be_bytes`,
    then `encode_component(Id, SubType::try_from(realm)?, V1, 64, &digest)` (helper at codec.rs:490,
    exactly what the reference uses). Validation order normative: ts>=2^52, hub>=2^12, realm∉{0,1}.
    Golden `gen_iscc_id_v1(1751831876325218,1,0)=="ISCC:MAIGHFECJMOPMIAB"`. cargo tests = round-trip
    (via `iscc_decode`) + validation ordering only; Python differential test belongs with the wheel
    step. Adding a fn shifts lib.rs lines → CRAP re-baseline in the SAME step. Then bindings fan
    out.
- **ISCC-IDv1 (#43) = Part 1 (codec accept V1) then Part 2 (`gen_iscc_id_v1` minting)** — spec-
    mandated order; full detail in `specs/rust-core.md`. Part 1 adds `V1`+`#[non_exhaustive]` to
    `codec::Version` (semver-major `enum_marked_non_exhaustive` → advance writes `**API-BREAK:**`
    note; semver job `continue-on-error`) and gates V1 on `Id`; **one core `codec.rs` edit flips 10
    of 11 surfaces** (Go is a separate port, already accepted out-of-loop). Part 2 sig
    `(timestamp:u64, hub_id:u16, realm:u8)`, **no `decode_iscc_id_v1`** (Titusz), diff test Python-
    only, no new JSON fixture. **Part 1 landed 174 NEEDS_WORK**: widening decode re-exposed a latent
    `decode_header` truncation (`as u8` narrows a multi-nibble varnibble before `TryFrom` →
    `262`→`6`=`Id`, so a malformed header canonicalizes). 175 scoped the `critical` codec.rs fix
    (`u8::try_from` for the three casts + `"MDFZAAAAAAAAAAAAAA"` rejection test); prerequisite for
    Part 2. **175 pushed the whole batch → first CI run went RED on TWO check-runs, ONE real blocker
    (176):** `Coverage + CRAP` (no `continue-on-error`) fails because `decode_header`'s fix took CC
    12→16 (100% covered, crap 16 < 30) and `--fail-regression` rejects any rise vs
    `.crap-baseline.json` → remedy is `mise run crap:baseline`, NOT a refactor. The `Semver` red is
    `continue-on-error: true` (verified ci.yml L355) so it does NOT fail the workflow conclusion —
    the `#[non_exhaustive]` break is expected+informational pre-1.0; don't "fix" it. Verified at
    source: run `3cdb00a` overall CI conclusion=failure driven by coverage only.
- **Closed phases: 115–123 (features) + dep slices 124–137 + 162–172** → MEMORY-archive.md +
    [dep-refresh ledger](dep-refresh-ledger.md). Still-biting: CRAP regression gate is **CI-only**;
    **never move a consumer floor** (MSRV, `go` directive, `required_ruby_version`, a published
    binding's compiler) in a refresh slice (128); preserve a major's behavioural contract with a
    built-in option, never by editing tests (168).
- **Lint/formatter tool bumps + hook-config changes**:
    [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact file/finding
    count a pass/fail criterion** — it drifts with CID's own commits; use the exit code.
- **Unicode chain DONE (147–161)**: sentinel freeze, `Final_Sigma` freeze, `mise run unicode:sweep`
    gate, boundary vectors on all 11 surfaces; never revive the superseded category override or a
    15.1.0 declared version. A 13th vector costs 12 suites — any fixture edit is its own slice.
    Details → [unicode-freeze-facts](unicode-freeze-facts.md) + propagation ledger.
- **Gate/checker steps in `scripts/`**: [gate scripts playbook](gate-scripts-playbook.md). **In a
    file no CI push exercises (`release.yml`), split behavioural from mechanical edits** (139/140):
    [release.yml static gates](release-yml-static-gates.md).
- **Text-hot-path steps trip two gates**: CI-only CRAP `--fail-regression` baseline +
    `.iai-baseline.json` 10% Ir gate. A `tests/`-only step usually trips **neither** — say so in
    next.md.
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep
    ([ty gate trap](define-next-ty-generator-scripts.md)). `uv run zensical build` (exits 0, "No
    issues found", ~8s) verifies any docs-only step.
- **Recurring**: cargo-deny goes red on fresh RustSec advisories vs dev/bench deps — CI-red-first;
    prefer `cargo update -p <crate>` over a `deny.toml` ignore. Tooling-cadence flag in state.md: 3
    of 4 recent iterations CI/lint/workflow → prefer a user-facing item once (143).
- **Binding artifacts are cheap probes, but each has its own age** — probe a *discriminating* input
    via a path leaving no tree diff (`/tmp` clone, gitignored build dir; freshness → propagation
    ledger).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (159). Pair the no-op with a pytest case proving the gate fires on a mutated
    source — unless a third-party generator emits non-hook-clean bytes (172).
- **"Not buildable in this container" claims decay — re-probe before they veto a slice** (Kotlin
    154, C++ 160, Swift 161 each ran in minutes). Ladder: `$PATH` → `uv run --with <tool>` → cached
    toolchain dir → upstream tarball. Trap: a runner can report success **without executing**
    (gradle UP-TO-DATE) — force it. **When state.md names the file to edit, re-derive it** (160: it
    pointed at the public `iscc` INTERFACE for a test-only include dir vcpkg/conan consumers
    inherit).
