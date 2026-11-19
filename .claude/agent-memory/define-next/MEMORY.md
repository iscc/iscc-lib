# Define-Next Agent Memory

Scoping decisions, estimation patterns, architectural knowledge across CID iterations. **Keep under
140 lines**; archive stale/completed-phase detail to `MEMORY-archive.md`.

**Status 2026-10-17:** v0.6.0 shipped 2026-07-30, v0.7.0 shipped 2026-10-17 (iscc-core 1.5.0
Image-Code fix, npm OIDC). Active milestone: **v0.8.0 ISCC-SEQ** (target.md). The v0.6.0 Phase
section below and the Unicode-freeze entries are history — the freeze was removed 2026-07-30. See
[ISCC-SEQ phase](iscc-seq-phase.md) for the v0.8.0 slice plan and traps (195 = Rust core; 196 =
CI-red fix; 197 = Python seq binding; 198 = core `iscc_clean` trim parity; 199 = Go strict base32;
200 = Go `isccClean`; 201 = Rust+Go case-fold alias fix; 202 = Go seq; 203 = text-utils trim parity
Rust+Go; 204 = napi+WASM seq; 205 = C FFI seq; 206 = .NET seq; 207 = C++ seq; 208 = JNI seq; 209 =
Ruby seq; 210 = UniFFI seq, last surface; 211 = 33→35 doc sweep + seq doc entries, fan-out ~30 doc
files; 212 = Ruby `RArray` fix + json bump; 213 = .NET+C++ embedded-NUL rejection (decided
2026-11-19; .NET throws `IsccException` at `ToNativeUtf8`, C++ one `detail` helper); 214 = npm
`license`/`description` for `@iscc/wasm` (Cargo.toml) + `@iscc/lib` (package.json had the same
badge-markdown description — npm derives it from the README when missing; probe sibling packages for
a defect class); 215 = data-edge.json seed + Rust/Python harnesses, see
[data-edge phase](data-edge-phase.md); Ruby Bignum/napi/JNI/ffi-split now `low` (human) — skip).

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
    are outside the file budget). **Run an expensive probe while scoping when it can invert the
    plan** (155/157; 161 Swift fetch killed a 10-iter-old veto).
- **Generated/tool-output files (Cargo.lock — moves even for a dev-dep already in the graph, 205 —
    bindings) and docs don't count toward the file limit** — batch related small changes (version
    sync + docs; several fixes in one crate).
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
- **JNI `-Xcheck:jni` = `JAVA_TOOL_OPTIONS=-Xcheck:jni mvn …`** — `-DargLine` is a no-op (208).
- **4 quality gates**: Semver (informational until human-gated v1.0.0) + Coverage/CRAP + Perf +
    Audit (enforcing). **Baselines move only via deliberate reviewed `mise run` commits**, never
    from CI; don't widen `--epsilon`. `cargo deny check` is authoritative locally (reads
    Cargo.lock).

## Phase Lessons (v0.6.0 onward; detail archived)

- **History (v0.6.0 phase, 177/178 phantom-red, IDv1 #43 closed 190, 193 iai `iscc_clean`)** →
    MEMORY-archive.md. Still-biting: a red `Semver` check-run is `continue-on-error` — verify the
    workflow *conclusion*, never a state.md "red" from check-run status; a codec/text hot-path step
    must flag `bench:iai:check` (CI-only gate).
- **Closed phases: 115–123 (features) + dep slices 124–137 + 162–172** → MEMORY-archive.md +
    [dep-refresh ledger](dep-refresh-ledger.md). Still-biting: CRAP regression gate is **CI-only**;
    **never move a consumer floor** (MSRV, `go` directive, `required_ruby_version`, a published
    binding's compiler) in a refresh slice (128); preserve a major's behavioural contract with a
    built-in option, never by editing tests (168).
- **Lint/formatter tool bumps + hook-config changes** →
    [lint tooling lessons](lint-tooling-lessons.md). Headline: **never make an exact file/finding
    count a pass/fail criterion** — use the exit code.
- **Unicode chain (147–161) — REMOVED 2026-07-30 by the human (decisions.md); history only**:
    sentinel freeze, `Final_Sigma` freeze, `mise run unicode:sweep` gate, boundary vectors on all 11
    surfaces; never revive the superseded category override or a 15.1.0 declared version. A 13th
    vector costs 12 suites — any fixture edit is its own slice. Details are in git history only.
- **Gate/checker steps in `scripts/`** → [gate scripts playbook](gate-scripts-playbook.md) +
    [release.yml static gates](release-yml-static-gates.md) (split behavioural from mechanical in
    the CI-unexercised `release.yml`, 139/140).
- **Text-hot-path steps trip CI-only CRAP `--fail-regression` + `.iai-baseline.json` 10% Ir; a
    `tests/`-only or binding-only step trips neither** — say so in next.md.
- Generator scripts needing an external pin: PEP 723 + `uv run --script`, never a dev-dep
    ([ty gate trap](define-next-ty-generator-scripts.md)). `uv run zensical build` (exits 0, "No
    issues found", ~8s) verifies any docs-only step.
- **Recurring**: cargo-deny reds on fresh RustSec advisories vs dev/bench deps — CI-red-first,
    prefer `cargo update -p <crate>` over a `deny.toml` ignore — but PROBE first (196:
    `cargo update --dry-run` + crates.io feature graph showed `basic-toml` survives every upgrade
    via uniffi_bindgen→askama `config` → per-ID ignore was the right call). Hosted-runner retirement
    (196: `macos-14` gone 2026-11-02, symptom = job `cancelled` + "configuration … not supported"
    annotation) → swap to the pinned arm64 successor at every workflow site, never `-latest`; check
    `actions/runner-images` README for live labels. When 3 of 4 recent iters are CI/lint/workflow →
    prefer a user-facing item once (143).
- **A derived/generated artifact is gated by "regenerate → `git status --porcelain` empty", not by
    `VENDORED_COPIES`** (159); pair the no-op with a pytest case proving the gate fires on a mutated
    source — unless a third-party generator emits non-hook-clean bytes (172).
- **"Not buildable in this container" claims decay — re-probe before they veto a slice** (Kotlin
    154, C++ 160, Swift 161 each ran in minutes). Ladder: `$PATH` → `uv run --with <tool>` → cached
    toolchain dir → upstream tarball. Trap: a runner can report success **without executing**
    (gradle UP-TO-DATE) — force it. **When state.md names the file to edit, re-derive it** (160: it
    pointed at the public `iscc` INTERFACE for a test-only include dir vcpkg/conan consumers
    inherit).
