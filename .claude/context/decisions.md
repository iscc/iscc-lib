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

## 2026-11-17 — The runner passes each role's frontmatter model and effort as CLI flags

**Decision:** `tools/cid.py` reads `model:` and `effort:` from `.claude/agents/<role>.md` and passes
them as `--model`/`--effort`; the runner-wide `PRIMARY_MODEL` pin is gone. A test asserts every role
declares both. **Why:** probed on Claude Code 2.1.326 — under `claude -p --agent <role>` the CLI
ignores frontmatter `effort:` (the session took the operator's `~/.claude/settings.json`
`modelSettings`, so every role ran at `xhigh` whatever its file said), and an explicit `--model`
overrides frontmatter `model:`. Frontmatter tuning, including meta-improve's whitelisted knob, was a
no-op. **Alternatives:** keep a runner-side pin — rejected, it duplicates the agent files and still
leaves effort to whoever runs the loop. **Context:** interactive CID setup review, 2026-11-17.

## 2026-11-17 — Opus 5.5 effort re-baselined; Fable 5.2 as advisor

**Decision:** `review` stays at `xhigh` (Fable 5.2 at `high` since 2026-11-20); every other role
(`update-state`, `define-next`, `advance`, `audit`, `meta-improve`) runs at `high`. All roles stay
on `claude-opus-5-5` and get `claude-fable-5-2` as server-side advisor (review: Fable 5.2 model, no
advisor, since 2026-11-20). **Why:** human decision. Anthropic's Opus 5.5 guidance: `medium` already
beats Opus 5 at `high` on coding, Opus 5.5 thinks more per level, `xhigh` only where a gain is
measured — the quality gate keeps it. The advisor gives Fable-level judgment at decision points
without paying Fable rates for every token. It needs fetched feature flags, so the runner blanks
`DISABLE_TELEMETRY` for its own sessions; CID runs therefore send telemetry while interactive
sessions stay opted out. **Alternatives:** Fable 5.2 as the model for define-next/review — not
chosen; all-Opus without advisor — not chosen. Watch `model_costs` in iterations.jsonl: advisor
calls re-read the transcript uncached and count toward `ROLE_BUDGET_USD`. **Context:** interactive
CID setup review, 2026-11-17; supersedes the effort levels in the 2026-10-17 entry.

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

## 2026-11-19 — Steps are sized by outcome: 6-file default, fan-out by behaviour

**Decision:** the default step cap rises from 3 to 6 non-test files, and the fan-out valve covers
bringing one already-decided behaviour to N surfaces, each in its own idiom, with one commit per
surface (`define-next.md` step 5). A change to core behaviour never rides along with a fan-out.
advance and review get 2 hours and $40 per run. **Why:** over iterations 196-214 implementation was
24% of the cost of an iteration, and a step's cost barely moved with its size ($10 for under 50
changed lines, $14-16 for over 400). 5 of 119 iterations were sent back, none of the 13 largest. The
ISCC-SEQ bindings took 8 iterations because the valve required mechanically identical changes.
**Trial — kept** (Titusz, 2026-11-20): iterations 215-224 had 1 NEEDS_WORK, every fan-out push was
green in CI, and the edge vectors reached all 12 surfaces in 5 iterations at $11.1 per iteration.
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

## 2026-11-20 — A spec criterion naming a mechanism that does not exist is realigned, not escalated

**Decision:** the wasm and go "Verification Criteria" bullets "Version synced from root `Cargo.toml`
via `mise run version:sync`" became "npm package version comes from root `Cargo.toml` (wasm-pack
output patched by release.yml)" and "Module version set by `packages/go/vX.Y.Z` Git tags". Review
approved them without human escalation. **Why:** `version_sync.py` has no wasm or Go target, and
each new bullet restates what ci-cd.md ("Automatic propagation", "Go modules use git tags") and
target.md already say. A bullet that only names the real mechanism is a checkable fact, not a new
acceptance criterion. Review cut advance's "(no in-repo package-version string)" clause from the Go
bullet: it forbade something no spec forbids. **Alternatives:** revert the bullets to the false
mechanism; escalate with HUMAN REVIEW REQUESTED. **Context:** iteration 223 review.

## 2026-11-20 — `alg_simhash` rejects an empty list

**Decision:** the public `alg_simhash` (Rust core, Go port, every binding) returns an error for an
empty list instead of 32 zero bytes. The unchecked internal helpers keep returning zeros for the
generators. **Why:** human decision. A SimHash of nothing has no natural length, the 32-byte answer
was undocumented and arbitrary, an empty list is almost always a caller bug, and iscc-core fails on
it too (`IndexError`). Nothing in the library called the public function with an empty list.
**Alternatives:** keep the zeros and record them as a divergence — rejected, it would make the
arbitrary constant official. **Context:** human decision (Titusz, interactive session, 2026-11-20);
issues.md carries the rollout.

## 2026-11-20 — Review runs on Fable 5.2 without the advisor; no advisor call after a commit

**Decision:** `review` runs on `claude-fable-5-2` at effort `high` (frontmatter) and without the
advisor (`ADVISOR_ROLES` in `tools/cid.py`, the only place the advisor is set); every other role
stays on Opus 5.5 with the Fable advisor but does not call it once its deliverable is committed.
Review runs `mise run check` once on the advance work and checks its own Markdown edits with
`uv run prek run --files`. **Why:** human decision. Over iterations 196-224 the advisor was $59 of
$120 review spend (about $1.10 per call, transcript re-read uncached), 14 of 57 review calls came
after commit and push, and Codex already gives review an independent opinion. Reviews ran the full
check about 3 times each. Fable bills to plan limits on this account, not usage credits. Role
sessions start without the claude.ai connectors (`sanitize_env`): they are not wanted unattended,
and ISCC Expert bills an API key. **Context:** human decision (Titusz, interactive session,
2026-11-20); supersedes the advisor scope of the 2026-11-17 entry.

## 2026-11-20 — Comment-only edits and regenerated files do not count against the 6-file cap

**Decision:** review passed iteration 225 with 6 files of changed behaviour plus 4 binding sources
whose only edit is a doc comment (napi, py, `.pyi`, wasm) and 3 tool outputs (`iscc.h`,
`NativeMethods.g.cs`, `.crap-baseline.json`), without a fan-out or `[audit]` valve. The cap counts
files whose code changes. **Why:** the cap limits how much behaviour one review has to verify. The
alternative here was to ship four doc comments that state the removed 32-zero answer, and a
regenerated file is checked by re-running its generator, not by reading it. next.md must still list
every such file and say that it is comment-only or regenerated, and review confirms that from the
diff. **Alternatives:** NEEDS_WORK on the count (nothing for the next step to repair); a fan-out
declaration (wrong: no surface got new behaviour of its own). **Context:** iteration 225 review, an
interpretation of the 2026-11-19 sizing decision — Titusz may overrule it.

## 2026-11-20 — JNI errors map by type: a fresh `OutOfMemoryError`, pending exceptions pass through

**Decision:** `throw_and_default` takes the error itself through a private `JavaError` trait:
`JniCall(NoMemory)` throws a new `OutOfMemoryError`, every other error `IllegalArgumentException`,
and both throw helpers throw nothing while an exception is pending. A zero hasher handle is an
`IllegalArgumentException`; `IllegalStateException` stays for the finalized state. **Why:** the
issue asked for the *original* `OutOfMemoryError` via an `exception_check` guard, but jni 0.22's
catching calls clear the JVM's exception and return `NoMemory`, so the guard alone changes nothing
there. Java callers catch the class, so a fresh instance is enough. The guard matters for calls that
leave the exception pending (`new_object`): without it review measured a constructor exception
replaced by `IllegalArgumentException`. **Alternatives:** the guard only (insufficient); a class
argument per call site (90 sites to get right); a handle registry for stale handles (out of scope).
**Context:** iteration 230 review; the 2026-07-28 hand-thrown-exceptions decision is unchanged.

## 2026-11-20 — The Tier 1 doc gate matches names against a registry, with a discovery threshold

**Decision:** `scripts/check_tier1_docs.py` keeps an explicit registry. A `FULL` doc must name every
symbol derived from `lib.rs`, in any binding casing. An `EXACT` marker block must equal the set. An
`EXEMPT` path carries a reason. Any other tracked Markdown file that names 30 or more symbols must
be registered. Per-crate READMEs are partial by design (target.md asks for the gen functions and key
utilities), so they stay unregistered below that threshold. **Why:** a casing-free name match is the
one check that works across all surface idioms, and a registry turns "is this list meant to be
complete?" into a recorded choice. **Alternatives:** marker blocks in every doc (rewrites prose that
already carries the names); a lower threshold (forces entries for READMEs meant to be short); a
per-binding export parser (one more gate per surface, and not what the issue asked). **Limits:** it
checks names, not signatures, and a doc that drifted below the threshold is invisible to discovery:
register such docs by hand. **Context:** iteration 233 review.

## 2026-11-20 — `@iscc/wasm` `alg_cdc_chunks` changes its runtime shape without a transition

**Decision:** the export returns `Vec<js_sys::Uint8Array>` (`Uint8Array[]`, each chunk a copy in JS
memory) where it returned nested plain arrays. The change ships in the next 0.x release as a bug
fix, with no deprecation period and no second export. **Why:** the doc comment and the published
JSDoc always promised `Uint8Array` chunks, `@iscc/lib` returns `Buffer[]`, and every other byte
output of the package is a `Uint8Array`, so the old shape was the defect. `Uint8Array.from(chunk)`,
the documented workaround, still works on the new shape. Callers that relied on plain-array
behaviour (`Array.isArray(chunk)`, `JSON.stringify`, a widening `chunk.map`) see a difference; the
bindings carry no 1.0 stability commitment. **Alternatives:** a `js_sys::Array` return (the `.d.ts`
says `any[]`); keeping `to_value` and correcting the docs instead (keeps the one byte output that is
not a typed array). `js-sys` was already in the dependency graph. **Context:** iteration 234 review.

## 2026-11-20 — Go CI tests on the latest patch through a `toolchain` line; the `go` directive stays the floor

**Decision:** `packages/go/go.mod` keeps `go 1.26.1` and adds `toolchain go1.26.11`. CI (`setup-go`
with `go-version-file`) and the devcontainer use the toolchain version, so the test runtime carries
the current stdlib security fixes while consumers keep the 1.26.1 minimum (Go ignores a dependency's
`toolchain` line). The CI napi job moves from Node 20 to 22 on the same reasoning: `@iscc/lib`
declares no `engines` floor, so the CI version is a test runtime, not a promise. **Trade-off:** CI
no longer runs on go1.26.1 itself; the suite passed there once at this review (Codex run), and
nothing gates it afterwards. **Alternatives:** raising the `go` directive to 1.26.11 (forces every
consumer up for advisories in code they compile with their own toolchain anyway); a two-version CI
matrix (doubles the job for a floor that differs only in patch level); leaving 1.26.1 (govulncheck:
about 40 stdlib advisories in the tested toolchain). Floors stay the human's call (`ci-cd.md` →
"Dependency Freshness" → Policy). **Context:** iteration 235; next.md flagged it "Decision for
Titusz".

## 2026-11-20 — data-edge output subset: allowed keys come from `data.json`, uncompared keys are fixed per harness

**Decision:** in a `gen_*` section of `data-edge.json`, `outputs` holds `iscc` and only keys that
the same function's `data.json` vectors carry; any other key fails on every surface. A surface whose
result type lacks some allowed keys (C, C++, and the bare-string results of .NET, Java, napi and
WASM) leaves exactly those uncompared, as a set written once per harness from the result type. A
harness never intersects the expected keys with the returned ones. **Why:** the intersection passed
any vector as long as one key matched, so a result that lost a key went unseen. Deriving the allowed
set from `data.json` keeps the file's meaning independent of any one binding and needs no key list
in the spec. **Alternatives:** a per-surface `except` value carrying reduced outputs, or a top-level
metadata key (each a 12-surface change, and an `except` per subset surface breaks "a vector costs
one edit"); structured results on every binding (an API decision for Titusz, not a test-harness
one). **Context:** iteration 236; spec `edge-vectors.md` → Rules → "Output subset".

## 2026-11-20 — The Ruby source gem is staged standalone: carried core, workspace `Cargo.lock`, conditional `rb_sys`

**Decision:** the `ruby`-platform gem is built only by `scripts/build_ruby_source_gem.py`. It stages
a tree outside the repository: a generated `Cargo.toml` (concrete keys, empty `[workspace]`), the
core from `cargo package -p iscc-lib` under `core/`, and the workspace `Cargo.lock`. The gemspec
declares `rb_sys ~> 0.9` only when `core/` sits next to it. **Why:** the gem must build with nothing
but its own files, and the lock makes an install resolve the crate versions CI tested. An
unconditional `rb_sys` dependency adds a version-less line to `Gemfile.lock` ahead of the pin that
`cross-gem@v1` greps, and gives the precompiled gems a runtime dependency. **Alternatives:**
depending on the published `iscc-lib` crate (the gem could only be released after crates.io, and a
checkout could not test it); publishing no source gem (Ruby 3.4+ and unlisted platforms would need a
checkout); an unconditional dependency plus a rewritten lock-link step in `release.yml` (nothing
tests that workflow before a release). **Context:** iterations 239-240.

## 2026-11-21 — Per-crate `CLAUDE.md` files carry no file lists; a rule is stated against a file name

**Decision:** the directory trees and `File`/`Module` tables are removed from every
`crates/*/CLAUDE.md` and `packages/*/CLAUDE.md`. A fact that a list entry carried stays only when it
is a rule or the file's origin ("generated, do not edit", "must sit next to `Cargo.toml`",
"gitignored build output"), written as a sentence that names the file. **Why:** the lists repeated
`git ls-files`. Every listed path existed, so the drift was missing entries, and an existence check
would have passed while fixing nothing. **Alternatives:** extending `scripts/check_spec_paths.py` to
these files with a completeness check (needs a parser and an exemption list for three formats, and
every new file would then need a `CLAUDE.md` edit); keeping the lists unchecked (breaks the "Derived
facts are checked or not written" hygiene rule). **Context:** iteration 246.

## 2026-11-21 — Spec trees stay as partial lists with a gated note; aggregate lists are removed

**Decision:** the structure trees in `.claude/context/specs/` stay. Each is followed by a paragraph
containing "not every file", and `scripts/check_spec_paths.py` fails on a tree without it. Entries
below a `GENERATED` prefix are dropped. The version-sync target table in `ci-cd.md` is replaced by a
pointer to `TARGETS` in `scripts/version_sync.py` (review edit: it listed 10 of 21 targets and said
"all targets listed above"). The one- and two-row "Version Sync" tables in the binding specs stay:
each states where that binding's version must live and claims no completeness. **Why:** a tree
comment carries a requirement ("DO NOT EDIT", where SPM, JNA and NuGet files live) and the gate
already fails on a stale entry, so only the claim of completeness had to go. **Alternatives:**
removing all trees with the parser, its floors and about 8 tests (every requirement comment
rewritten as prose in 9 specs); a completeness check per tree; completing the version-sync table
with a parity test against `TARGETS`. **Context:** iteration 247.

## 2026-11-21 — No role uses the advisor; the audit spend ceiling is 30 USD

**Decision:** no CID role session gets the server-side advisor: `session_settings` in `tools/cid.py`
sets no `advisorModel` (and so no longer blanks `DISABLE_TELEMETRY`), and the agent files carry no
advisor guidance. The audit's `--max-budget-usd` ceiling rises from 15 to 30. **Why:** human
decision. The iteration-240 audit hit its 15 USD ceiling at 15.03 USD in the middle of verification
and filed nothing; 5.64 USD of it was the advisor, because each of the Workflow's 9 subagents
consulted it 2-5 times, re-reading its transcript uncached. The advisor was also about 40-45% of
each define-next run's cost. **Context:** human decision (Titusz, interactive session, 2026-11-21);
supersedes the advisor parts of the 2026-11-17 and 2026-11-20 entries.

## 2026-11-21 — Video frame signatures of unequal length are rejected, where iscc-core truncates

**Decision:** `soft_hash_video_v0` / `gen_video_code_v0` in the Rust core and `SoftHashVideoV0` /
`GenVideoCodeV0` in Go return an error when a frame signature's length differs from the first one,
checked in input order before deduplication. Equal lengths other than 380 are handled as before. The
Python binding keeps its own check: it flattens the frames into one buffer before calling the core,
and raises `ValueError` with the same wording. **Why:** human issue. A longer later signature
indexed out of bounds, which aborts the host process under `panic = "abort"`; a shorter one gave a
digest that depended on input order. The reference's `zip(*sigs)` truncates to the shortest
signature and returns a digest (measured at 64 bits for `[380, 381]`, `[381, 380]` and
`[380, 379]`), so a ragged input is a `diverges` case in `data-edge.json`. **Alternatives:**
truncating like the reference (hashes malformed input silently); requiring exactly 380 (changes
equal-length inputs). **Context:** iteration 250.

## 2026-11-21 — The C FFI reads `(NULL, 0)` as an empty input on every pointer+length parameter

**Decision:** Every pointer+length input of `iscc-ffi` is borrowed through `slice_or_empty` (row
arrays through `rows_or_empty`). Length 0 is an empty slice whatever the pointer is. NULL with a
non-zero length is the error "<name> must not be NULL", inner rows included (`frame_sigs[1]`). A
count of 0 therefore reaches the core, and the caller sees the core's own error or result. A NULL
`const char*` string stays an error. **Why:** audit issue. Half the exports rejected `(NULL, 0)` and
half accepted it, a NULL inner row was undefined behaviour, and because `fixed` over an empty span
and `std::vector::data()` yield NULL, each wrapper needed sentinels. **Alternatives:** rejecting
NULL everywhere (keeps sentinels in every wrapper and hides the core's error); deciding per export
(the split that existed). **Context:** iteration 255. The review made the indexed row name lazy
(`format_args!`) after measuring 2x on `iscc_alg_simhash` with 1,000 rows.

## 2026-11-21 — Out-of-range integer arguments are per-binding tests, not `data-edge.json` vectors

**Decision:** napi and WASM take every numeric parameter as a JS `number` (`f64`) and range-check it
before narrowing; JNI rejects a negative `int` for `bits`, `nbytes` and `width`. Python keeps
`OverflowError` and Ruby `RangeError`, the classes each host raises itself when an integer
conversion fails, and the docs name them. No `data-edge.json` vector covers these inputs. **Why:**
audit issue. A value such as `2**32 + 64` for `bits` cannot be passed through the typed signatures
of the Rust, C, C++, Go, .NET, Swift and Kotlin harnesses: to them it is a wrong argument type,
which `edge-vectors.md` → Rules leaves to per-binding tests. **Alternatives:** a vector with
`except` entries for seven surfaces (mostly exceptions); wrapping Python/Ruby errors into
`ValueError`/`RuntimeError` (hides the host's own class). **Context:** iteration 256.
