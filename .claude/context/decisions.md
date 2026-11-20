# Decision Records

Log of judgment calls that a future reader could not reconstruct from the code alone: deviations
from the iscc-core reference or a spec, design trade-offs between real alternatives, accepted
performance regressions, justified suppressions, API-shape choices. Written by the review agent when
approving such a call; human entries welcome. Distinct from learnings.md: learnings capture
operational gotchas, decisions capture design rationale.

**A recorded decision is binding.** CID agents must not re-open, re-derive or re-litigate a
trade-off settled here. Superseding one takes a new fact and a new dated entry that says what
changed.

**Size.** One entry per review, at most 12 lines. The file has a 400-line budget; `tools/cid.py`
rotates the oldest entries into `decisions-archive.md` when it is exceeded (oldest-first, no
judgment applied), so entries are written to be read now, not to be permanent. Rotation does not
retire a ruling: archived **titles** stay in every agent's context alongside the live ones, and an
archived decision binds exactly as much as one still in this file. Only the bodies leave.

Entry format:

```markdown
## <YYYY-MM-DD> — <title>

**Decision:** <what was decided>
**Why:** <the reasoning, including constraints that forced it>
**Alternatives:** <what was rejected and why>
**Context:** <iteration N / commit sha / discussion reference>
```

<!-- Append entries below this line -->

## 2026-07-27 — Agent prompts state no token costs, and separate finding from filing

**Decision:** two standing rules for every CID prompt and context pack. (1) Never frame inlined
content as a cost, a billing event, or a context-limit risk — state the fact and leave accounting to
`tools/cid.py`. (2) Instructions that narrow *what to look for* are forbidden in a finding pass; the
severity bar belongs at the filing step. **Why:** the Claude Fable 5 guide documents that surfacing
token budgets makes a model trim its own work, and the Opus 5 guide documents that a review prompt
saying "be conservative" is followed literally and reports less — "ask it to report everything and
filter in a separate pass instead". Both misfires were live: `review` was told it "runs closest to
the context limit … Guard it" (it is the quality gate and the costliest role, 103 turns /
$7.89 at iteration 161), and `audit` suppressed nits in the finder prompt *and* had its
skeptics refute them by default. **Evidence:** audit iterations 130/140/150/160 filed zero issues
for $11.53. **Alternatives:** leave the audit alone and read 4/4 clean passes as real — rejected,
the double suppression makes a clean pass uninformative either way. **Context:** interactive
session, 2026-07-27, against the platform prompting guides for Opus 5 and Fable 5.

## 2026-07-27 — The xunit v3 test project stays in VSTest runner mode

**Decision:** `packages/dotnet/Iscc.Lib.Tests` moves to `xunit.v3` 3.x + `Microsoft.NET.Test.Sdk`
18.x but keeps the classic VSTest runner (`xunit.runner.visualstudio` 3.x, no
`TestingPlatformDotnetTestSupport` / `UseMicrosoftTestingPlatformRunner`). The only csproj change
beyond the package swap is `<OutputType>Exe</OutputType>`, which v3 requires because its test
projects are stand-alone executables. **Why:** the `dotnet` CI job passes the native library path
with `dotnet test -e LD_LIBRARY_PATH=…`, and `-e` is a VSTest feature; Microsoft.Testing.Platform
mode would force a CI-invocation rewrite for no test-coverage gain. VSTest mode also kept the
migration to zero test-source edits. **Alternatives:** adopt MTP runner mode now — rejected, it
changes the CI contract and the P/Invoke library-path mechanism in the same step as a framework
major. **Context:** iteration 164; v2 and v3 both report exactly 104 results.

## 2026-07-28 — JNI exports return `#[repr(transparent)]` wrappers and keep hand-thrown exceptions

**Decision:** the 33 `extern "system"` functions in `crates/iscc-jni/src/lib.rs` return jni wrapper
types (`JString`, `JObject`, `JByteArray`, `JObjectArray`) instead of the raw `jstring`/`jobject`
aliases, and `throw_and_default`/`throw_state_error` keep calling `env.throw_new` themselves,
returning `Ok(T::default())` into `.resolve::<ThrowRuntimeExAndDefault>()`. **Why:** jni 0.22's
`EnvOutcome::resolve` requires `T: Default`, which raw pointer aliases do not implement; the
wrappers are `#[repr(transparent)]`, so the ABI Java sees is unchanged. The policy's `on_error`
calls `Env::exception_check` first (jni-0.22.4 `src/errors/policy.rs:188`) and returns the default
without throwing when an exception is already pending, so the `IllegalArgumentException` /
`IllegalStateException` contract asserted by six Java tests survives untouched and the policy only
fires on a genuine panic or unhandled `Err`. **Alternatives:** a custom `ErrorPolicy` per exception
class — rejected, it would move the exception choice away from the call site that knows which one
applies, for no behavioural gain. **Context:** iteration 168, jni 0.21 → 0.22 migration.

## 2026-07-28 — uniffi 0.32 accepted with a raised source-build floor on unpublished `iscc-uniffi`

**Decision:** the workspace pins `uniffi = "0.32"` even though its default graph
(`cargo_metadata 0.23.1` → rustc 1.86, `cargo-platform 0.3.3` → 1.91) makes `iscc-uniffi`
unbuildable on the inherited `rust-version = "1.85"`. **Why:** `iscc-uniffi` is `publish = false`
and exists only to emit the Swift/Kotlin bindings; the published crate still passes
`cargo +1.85.0 check -p iscc-lib --locked`, every CI job builds on stable, and no gate or consumer
promise moves. Holding at 0.31 to protect a floor nothing verifies would have blocked the last
authorized v0.6.0 dependency item. **Alternatives:** raise the workspace `rust-version` — rejected,
that is the published MSRV promise and Titusz's call; pin the transitives back — rejected, uniffi
0.32 requires them. **Context:** iteration 172; the now-false inherited declaration is filed as a
`normal` issue proposing an explicit per-crate `rust-version` on `iscc-uniffi`.

## 2026-07-29 — JS-number ISCC-IDv1 surfaces take all params as `f64` and validate before narrowing

**Decision:** on the two JS-number surfaces (napi, wasm) `gen_iscc_id_v1` takes `timestamp`,
`hub_id` and `realm` all as `f64` — not the core's `u16`/`u8` — and a per-file `checked()` rejects
non-finite / non-integral / negative / out-of-range values before narrowing. **Why:** napi and wasm
coerce a JS `number` into the declared integer type (ToUint32 / truncation) *before* the wrapper
runs, so integer params make the low and non-integral end unvalidatable — an out-of-contract input
would silently mint a valid-but-wrong ID (a collision hazard). Only `f64` preserves the original
value for a range check. **Alternatives:** keep integer params and rely on core — rejected, core
only guards the high end and never sees the pre-truncated value; a shared validation crate —
rejected, ~5 duplicated lines beat a new crate boundary. **Scope:** JS-number surfaces only;
ffi/jni/rb/uniffi are typed-int with no coercion hazard and must not copy the pattern. **Context:**
iteration 181, #43 fan-out; resolves the napi coercion issue for both surfaces at once.

## 2026-07-29 — `bench_iscc_code.four_units` iai baseline raised 11,968 → 13,400 (accepted regression)

**Decision:** the committed `.iai-baseline.json` four_units baseline is raised from 11,968 to 13,400
Ir, accepting the +11.71% measured cost (13,369 Ir) of routing codec input through the
reference-correct `codec::iscc_clean` (iters 191/192). **Why:** the pre-191 baseline was set for
`strip_prefix("ISCC:")`-only cleaning that diverged from the reference (`codec.py:644`); 191/192
traded that for conformance and the residual is intrinsic scanning cost (two std SIMD `memchr`
scans), not removable waste — a scalar single-pass measured *worse* (13,499 Ir) and iter 193's `Cow`
change already removed the allocation component. Absolute cost is negligible: 13,369 Ir against 7M+
for real hashing. **Alternatives:** add the `memchr` crate for a single-pass `memchr2` — rejected, a
new dependency + audit-gate surface for a micro-saving; revert 191/192 — rejected, reopens the codec
cleaning divergence. **Context:** iteration 193, human-gated (Titusz picked option A) after the loop
halted on the enforcing Perf gate.

## 2026-07-30 — Unicode 16.0.0 pin removed; version-drift measures postponed pending upstream

**Decision:** All v0.6.0 Unicode-version compatibility measures are removed (human decision,
Titusz): the `U+FFFF` sentinel freeze rule and vendored unassigned/case tables in the core, frozen
`Final_Sigma` lowercasing, boundary vectors and their tests in all 11 surfaces, the differential
sweep gate (script, mise task, CI job), and `docs/unicode.md`. `text_clean`/`text_collapse` now
apply the reference's steps with whatever Unicode tables the dependencies ship. **Why:** the pin's
complexity raised the risk of faulty third-party implementations and cost per-char table lookups,
while the reference itself drifts across CPython versions — compatibility semantics belong upstream
(iscc/iscc-core#137) and will be revisited once upstream decides. **Alternatives:** keep the pin —
rejected by the human; pin without freeze trickery — rejected, still diverges from the unpinned
reference. **Supersedes** the 2026-07-25 to 07-27 freeze-rule rulings. Those entries and the agent
memories describing the freeze design were deleted on 2026-11-19; git history keeps them. Do not
reintroduce any freeze mechanism without a new human decision. **Context:** interactive session
2026-07-30, this commit.

## 2026-10-17 — npm publishes via OIDC trusted publishing, direct publish (not staged)

**Decision:** `@iscc/lib` and `@iscc/wasm` publish through npm trusted publishing: one trusted
publisher per package on npmjs.com (GitHub Actions, `iscc/iscc-lib`, workflow `release.yml`, no
environment, "Allow npm publish" on). The two publish jobs carry no `NPM_TOKEN`, run Node 24 (npm

> = 11.5.1) and set no `registry-url` in `setup-node`. **Why:** granular npm write tokens expire
> after at most 90 days and the token broke or blocked the v0.5.0 and v0.7.0 releases; OIDC has
> nothing to expire, matching crates.io/PyPI/RubyGems. `registry-url` is omitted because
> `setup-node@v7` no longer exports a dummy `NODE_AUTH_TOKEN`, so its generated `.npmrc` would
> reference an unset variable. **Alternatives:** keep rotating tokens — rejected (recurring release
> failures); staged publishing (CI stages, a human approves each version on npmjs.com with 2FA) —
> npm's recommended mode, deferred because it adds a manual step to every release; it can be
> switched on later without touching the workflow's auth. **Context:** interactive session
> 2026-10-17, first validated by the v0.7.0 publish. Changing a trusted-publisher entry needs the
> human's npm 2FA passkey.

## 2026-11-17 — The runner passes each role's frontmatter model and effort as CLI flags

**Decision:** `tools/cid.py` reads `model:` and `effort:` from `.claude/agents/<role>.md` and passes
them as `--model`/`--effort`; the runner-wide `PRIMARY_MODEL` pin is gone. A test asserts every role
declares both. **Why:** probed on Claude Code 2.1.326 — under `claude -p --agent <role>` the CLI
ignores frontmatter `effort:` (the session took the operator's `~/.claude/settings.json`
`modelSettings`, so every role ran at `xhigh` whatever its file said), and an explicit `--model`
overrides frontmatter `model:`. Frontmatter tuning, including meta-improve's whitelisted knob, was a
no-op. **Alternatives:** keep a runner-side pin — rejected, it duplicates the agent files and still
leaves effort to whoever runs the loop. **Context:** interactive CID setup review, 2026-11-17.

## 2026-11-17 — Opus 5.5 effort re-baselined; Fable 5.2 advises every role

**Decision:** `review` stays at `xhigh`; every other role (`update-state`, `define-next`, `advance`,
`audit`, `meta-improve`) runs at `high`. All roles stay on `claude-opus-5-5` and get
`claude-fable-5-2` as server-side advisor (`ADVISOR_MODEL` in `tools/cid.py`). **Why:** human
decision. Anthropic's Opus 5.5 guidance: `medium` already beats Opus 5 at `high` on coding, Opus 5.5
thinks more per level, `xhigh` only where a gain is measured — the quality gate keeps it. The
advisor gives Fable-level judgment at decision points without paying Fable rates for every token. It
needs fetched feature flags, so the runner blanks `DISABLE_TELEMETRY` for its own sessions; CID runs
therefore send telemetry while interactive sessions stay opted out. **Alternatives:** Fable 5.2 as
the model for define-next/review — not chosen; all-Opus without advisor — not chosen. Watch
`model_costs` in iterations.jsonl: advisor calls re-read the transcript uncached and count toward
`ROLE_BUDGET_USD`. **Context:** interactive CID setup review, 2026-11-17; supersedes the effort
levels in the 2026-10-17 entry.

## 2026-11-18 — Base32 decoding stays strict: no padding, zero spare bits

**Decision:** Every codec surface rejects base32 with `=` padding or with non-zero unused trailing
bits in the last character, although iscc-core 1.5.0 accepts both. The Python ISCC-SEQ differential
test asserts this divergence explicitly instead of matching the reference's reject set, and the
pure-Go port must follow (issues.md). **Why:** human decision. No encoder emits these strings:
IEP-0001 defines the canonical form as RFC 4648 base32 without padding, and RFC 4648 §3.5 lets
decoders reject non-zero pad bits. The reference accepts them only because it re-pads and calls
Python's `b32decode`; its own docstring says "without padding" and its `iscc_validate` regex rejects
`=`. Spare bits make real codes ambiguous: a 2-unit ISCC-CODE has one, so
`ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` decodes to the same bytes as `…CNGI`, and two strings for one
ISCC break keys, deduplication and signatures. Rust has rejected both since the codec landed
(`b23aee2`), so every release already behaves this way; matching the reference would be the
behaviour change. Same reasoning as the archived 2026-07-24 entry on rejecting aliasing strings.
**Alternatives:** a lenient decoder for `encode_seq` only (two decoders in one codec — rejected);
match the reference codec-wide (widens the Tier 1 input domain and reintroduces aliasing —
rejected). **Context:** escalated by review in iteration 195; interactive session 2026-11-18.

## 2026-11-18 — macOS CI and release jobs pin `macos-15` (arm64), not `macos-latest`

**Decision:** Every macOS job in `ci.yml` and `release.yml` runs on `macos-15`, the arm64 successor
of the retired `macos-14` (actions/runner-images#15226); the `x86_64-apple-darwin` rows keep
cross-compiling from arm64 exactly as before. **Why:** a like-for-like swap keeps the build design
unchanged and the label explicit. **Alternatives:** `macos-latest` — rejected, it drifts silently to
a new OS/Xcode (the existing `release.yml` `macos-latest` job is left as is); `macos-15-intel` —
rejected, it would turn the x86_64 rows into native builds, a design change. The `release.yml` half
is verified statically only (`workflow_dispatch`-only). **Context:** iteration 196, CI-restore step
chosen by define-next; Titusz may overrule.

## 2026-11-18 — Codec input is upper-cased ASCII-only, `encode_seq` included

**Decision:** `decode_base32`, the `iscc_normalize` prefix check and `encode_seq` upper-case ASCII
letters only (Go: `asciiUpper`), so `ı`/`ſ`/`ß`/`ﬁ` fail as invalid base32 symbols. For
`iscc_decode` this matches iscc-core (`b32decode` rejects non-ASCII). For `encode_seq` it diverges:
iscc-core calls Unicode `unit.upper()` first (`codec.py:503`) and accepts all four. **Why:** extends
"Base32 decoding stays strict" (2026-11-18), which rejected a lenient `encode_seq`; these strings
alias canonical codes. Unicode case mapping is also unportable: Go's `strings.ToUpper` maps `ſ`/`ı`
but not `ß`/`ﬁ`, so no Unicode rule gives Rust and Go one reject set. **Alternatives:** Unicode
upper-casing in `encode_seq` only (reference-exact, but two casing rules in one codec — rejected).
ASCII-only also keeps the codec independent of each runtime's Unicode case tables, the version drift
parked on 2026-07-30. **Context:** iteration 201 review; confirmed by Titusz in an interactive
session, 2026-11-18, and written into `rust-core.md` "Base32 input is strict".

## 2026-11-18 — `encode_seq` reads a multibase-ambiguous string as a bare unit, like iscc-core

**Decision:** a bare string that is both lowercase multibase (`b` + base32 of `cc01` + unit) and,
once upper-cased, a valid unit (`bzqaq4yamaaqaaaaaaaaaaaaaaaaaaaa` → 20-byte `0e6008…`) encodes as
the bare unit on every surface. **Why:** iscc-core 1.5.0 `encode_seq` returns the same bytes (it
upper-cases before `iscc_clean`), the spec mandates upper-casing first so `faaa-aaaaaaaa` works, and
Rust (195) and Go (202) agree. **Alternatives:** detect a complete multicodec-wrapped unit before
case folding (Codex, iteration 202) — rejected: a reference divergence with no observed real input,
to be mirrored on 11 surfaces. **Context:** iteration 202 review; Codex will re-flag it per surface.

## 2026-11-18 — Go text utilities drop the `strings.Fields` / `TrimSpace` ASCII fast path

**Decision:** `TextRemoveNewlines` / `TextTrim` call `strings.FieldsFunc` / `strings.TrimFunc` with
`isPyStripChar` on every input. Measured on go1.26.1 (iteration 203 review): a 6-word title takes
about 2× as long (502 → 1000 ns/op). Allocations stay equal up to 32 fields, then rise (40 fields: 2
→ 3, 512 fields: 2 → 6), because `FieldsFunc` keeps a 32-entry span buffer. **Why:** this gives
parity with iscc-core `str.split()` / `str.strip()` on U+001C–U+001F. The cost is under a
microsecond on a Meta-Code path dominated by NFD/NFKC and hashing, and the Go port has no perf gate.
**Alternatives:** keep `strings.Fields` when no U+001C–U+001F is present and fall back otherwise
(Codex, iteration 203). Rejected: two code paths for a sub-µs gain on an ungated path. Re-open only
with a Go benchmark showing a regression at the Meta-Code level. **Context:** iteration 203 review;
next.md prescribed `FieldsFunc` verbatim.

## 2026-11-19 — Embedded NUL is rejected at the .NET and C++ wrapper boundary

**Decision:** the .NET and C++ wrappers reject any string input containing an embedded NUL before
marshalling, instead of letting `CStr::from_ptr` truncate it. The C FFI ABI stays NUL-terminated.
**Why:** a wrong ISCC returned as success is worse than an error; a C string cannot hold a NUL, so
the wrappers are what breaks the contract; the check is one choke point per wrapper. Accepted cost:
.NET and C++ refuse NUL-bearing text that Rust and Python accept. **Alternatives:** strings with a
length across the FFI. Rejected: it breaks every string-taking C function for a rare input.
**Context:** human decision (Titusz, interactive session, 2026-11-19); issues.md has the evidence.

## 2026-11-19 — Edge-case vectors are collected in `data-edge.json` and go upstream later

**Decision:** edge-case inputs on which surfaces, or iscc-lib and iscc-core, disagree are collected
in one shared file that every conformance harness runs (`specs/edge-vectors.md`). Titusz signals
when the file is contributed to iscc/iscc-core; nobody opens an upstream PR or issue before that.
**Why:** `data.json` has only well-formed inputs, and every cross-surface disagreement found so far
(strict base32 in Go, case folding, embedded NUL, empty lists) was on an input it does not cover.
**Alternatives:** per-binding negative tests only, the status quo. Rejected: eleven hand-written
copies drift. **Context:** human decision (Titusz, interactive session, 2026-11-19).

## 2026-11-19 — Review-found old defects are filed `low` until the human promotes them

**Decision:** the review files a defect as `normal` only when the step caused it, ordinary
documented use hits it, or an outside signal raised it (CI, a security advisory, an upstream
release, a user report). Everything else is `low` with `**Triage:** untriaged`, and Titusz promotes
what matters (`review.md` step 7). **Why:** `normal` blocks the release, and 10 of 16 open issues
were review-filed, so the reviewer was setting the release bar from inside the loop.
**Alternatives:** keep `normal` as the default. Rejected for that reason. **Context:** human
decision (Titusz, interactive session, 2026-11-19), applied the same day to four open entries (napi
`Buffer` leak, JNI pending exception, Ruby negative Bignum, `iscc-ffi` file split).

## 2026-11-19 — A step that introduces a pattern on a surface covers all its call sites

**Decision:** when a step introduces or changes how one surface does something (a guard, a helper,
an error mapping, a conversion), it covers every sibling call site on that surface in the same step
(`define-next.md` step 5, checked by the review). **Why:** minimal steps left the old form next to
the new one and filed the rest as an issue: three .NET pin loops with two behaviours (iteration
206), a NUL guard on 1 of 16 C++ call sites (207). **Alternatives:** keep minimal steps and track
leftovers as issues. Rejected: the leftover is the defect. **Context:** human decision (Titusz,
interactive session, 2026-11-19).

## 2026-11-19 — Steps are sized by outcome: 6-file default, fan-out by behaviour (trial)

**Decision:** the default step cap rises from 3 to 6 non-test files, and the fan-out valve covers
bringing one already-decided behaviour to N surfaces, each in its own idiom, with one commit per
surface (`define-next.md` step 5). A change to core behaviour never rides along with a fan-out.
advance and review get 2 hours and $40 per run. **Why:** over iterations 196-214 implementation was
24% of the cost of an iteration, and a step's cost barely moved with its size ($10 for under 50
changed lines, $14-16 for over 400). 5 of 119 iterations were sent back, none of the 13 largest. The
ISCC-SEQ bindings took 8 iterations because the valve required mechanically identical changes.
**Trial:** Titusz evaluates after iteration 224 (issues.md). Revert the commit that introduced this
entry if more than 2 of iterations 215-224 end NEEDS_WORK or a fan-out push turns CI red.
**Context:** human decision (Titusz, interactive session, 2026-11-19).

## 2026-11-20 — .NET jagged-array inputs: null check first, padded pin arrays

**Decision:** the six .NET jagged-array methods pin through one private `PinnedRows` helper. A null
array or element throws `ArgumentNullException` (parameter name and `name[i]`) before any UTF-8
conversion or pinning, so a null element wins over an embedded NUL anywhere in the input. That is a
small change for `EncodeSeq`, which checked element by element. An empty input pins pointer and
length arrays of `max(Count, 1)` slots and passes the real count, so `[]` reaches the core's error.
**Why:** one validation order and one code path for all six, no `count == 0` branch at a call site.
**Alternatives:** a stack sentinel per call site (next.md's suggestion; six extra branches);
interleaving the null and NUL checks per element (keeps the old `EncodeSeq` order, splits the
helper); `IsccException` for null (not the .NET idiom). **Context:** iteration 220 review.
