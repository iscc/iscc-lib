# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files. A `low`
entry with `**Triage:** untriaged` is a review finding the human has not weighed yet: promote it to
`normal`, or delete the line to park it. The human moves an entry that does not matter for the
current release to `postponed.md`, which the loop never acts on.

**This is a defect tracker, not a work queue.** An entry states the remaining problem and how to
tell it is fixed, in **40 lines or fewer**, and is deleted once nothing is left to fix. Never record
progress inside an entry — no per-iteration checklists, no "✅ slice N done", no plan of remaining
sub-steps. Git history records what was done; a step queue here removes the gap analysis that
define-next exists to perform. If only part of a problem remains, rewrite the entry to describe that
smaller problem. Design rationale belongs in `decisions.md`, operational gotchas in `learnings.md`,
and user-facing behaviour in `docs/`.

<!-- Add issues below this line -->

## Ruby integer list elements accept a Float and truncate it `normal` [review]

Magnus' integer conversions apply Ruby's implicit `to_int`. So `IsccLib.gen_audio_code_v0([1.9])`
returns the code of `[1]`, and `alg_minhash_256` does the same, where Python raises `TypeError`. It
follows Ruby's own convention (`[10, 20, 30][1.9]` is `20`), so it may be acceptable as host idiom.
Pre-existing, measured in iter 229. The doc comment on `array_to_i32_vec` in
`crates/iscc-rb/src/lib.rs` says "`TypeError` for a non-Integer", which does not hold for a Float.
No `data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong-typed elements).

**Decided (Titusz, 2026-11-21):** keep the implicit conversion as Ruby host idiom; fix the docs.

Resolved when `docs/ruby-api.md` → Error Handling states the implicit conversion and the doc comment
on `array_to_i32_vec` matches it.

## The pkg-config example in the C/C++ how-to says `Version: 0.2.0` `normal` [review]

The line is typed by hand and is not a `scripts/version_sync.py` target, which covers only the
Maven, Gradle and SPM snippets in docs (0.7.0 is current). Pre-existing, found in iteration 244.

Resolved when the example shows no version that no check keeps current: a placeholder the reader
fills in, or the line is a `version_sync.py` target.

## Published Ruby source gems up to 0.7.0 do not build `low` [review]

The source gem on rubygems.org was built with a bare `gem build` up to 0.7.0: it declares no
`rb_sys` dependency and ships a workspace-inheriting `Cargo.toml` without the core (measured for
0.7.0 in iteration 238). The repository now builds a standalone source gem (`ruby-source-gem` CI
job), but published versions cannot be changed. Yanking the broken `ruby`-platform gems is a
registry action for Titusz. CID must not act on this.

Resolved when a release with the repaired source gem is on rubygems.org and it installs from the
registry into a clean gem home on a Ruby series without a precompiled gem
(`GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install iscc-lib`, then `IsccLib.conformance_selftest`).

## How-to examples show the Sum-Code prefix as `KAA` `normal` [review]

`gen_sum_code_v0` composes a Data-Code and an Instance-Code, so without `wide` its result has
SubType SUM and starts with `ISCC:KUA` (measured in iteration 238). The Sum-Code examples in the
how-tos under `docs/howto/` and in `docs/java-api.md` carry the comment `"ISCC:KAA..."`, and the
`iscc` field comment in `SumCodeResult.java` says `"ISCC:KAC..."`. Pre-existing and cosmetic. The
`gen_iscc_code_v0` examples next to them use the same comment and were not checked.

Resolved when each Sum-Code example comment shows a prefix the call can return.

## The Ruby floor is past end of life and current Rubies get no precompiled gem `normal` [human]

**Decided (Titusz, 2026-11-21):** raise the Ruby floor from 3.1 to 3.3. Ruby 3.1 reached end of life
on 2025-03-26 and 3.2 on 2026-03-31 (endoflife.date). The Python 3.10 and .NET 8 floors stay
(decisions.md, 2026-11-21).

The floor is in `crates/iscc-rb/iscc-lib.gemspec` (`required_ruby_version`), the `ruby` job in
`ci.yml` (`ruby-version: '3.1'`), the minitest hold in `crates/iscc-rb/Gemfile`, and the docs
(`crates/iscc-rb/README.md`, `docs/howto/ruby.md`, `crates/iscc-rb/CLAUDE.md`,
`specs/ruby-bindings.md`). The cross-gem step in `release.yml` builds precompiled gems for 3.1, 3.2
and 3.3 only, so a user on Ruby 3.4 or newer always compiles from source.

Resolved when the gemspec, CI and docs say `>= 3.3`, CI tests the floor, and the cross-gem list
covers every supported Ruby series from 3.3 up that the cross-gem image can build (the docs name the
list).

**Spec:** `.claude/context/specs/ruby-bindings.md`

## `ci-cd.md` describes a tag trigger the release workflow does not have `normal` [review]

`.github/workflows/release.yml` has `workflow_dispatch` as its only trigger, and `target.md` says "a
tag push does not trigger the workflow". Three places in `.claude/context/specs/ci-cd.md` say
otherwise: the Trigger cell of the Release row in "Workflow Files", the "Tag trigger as convenience"
bullet under "Release Workflow — Selective Publishing" → Design Principles, and the
`push: tags: [v*.*.*]` lines in the "Trigger Configuration" YAML block. Pre-existing. Realign the
spec without escalation (decision of 2026-11-20).

The same claim appears in four more places (iteration-249 audit, re-checked 2026-11-21):
`specs/swift-bindings.md:239` (a GitHub Release creates the tag that publishes),
`specs/c-ffi-dx.md:128` ("Tag push triggers FFI builds"), `crates/iscc-py/CLAUDE.md:134` and
`crates/iscc-wasm/CLAUDE.md:124`.

Resolved when these seven places describe `workflow_dispatch` as the only trigger.

**Spec:** `.claude/context/specs/ci-cd.md` → "Release Workflow — Selective Publishing"

## Go canonical `meta` JSON differs from the core in key order, `-0` and lone surrogates `normal` [review]

`parseMetaJSON` (`packages/go/code_meta.go`) keeps three differences from RFC 8785, measured against
`iscc_lib` in iteration 251. `encoding/json` sorts keys by UTF-8 bytes, JCS by UTF-16 code units:
for `{"\ue000":1,"\ud83d\ude00":2}` Go keeps this key order, the core swaps it. Go writes `-0.0` as
`-0`, the core as `0`. Go turns a lone surrogate escape into U+FFFD, the core rejects the input.
Pre-existing. The other numbers probed match (`1.0`, `1e21`, `1e-7`, `9007199254740993`).

Resolved when Go returns the core's result for the three inputs and each is a `data-edge.json`
vector.

## C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection the FFI no longer has `normal` [review]

The C FFI reads a NULL pointer with length 0 as an empty input on every pointer+length parameter
(decision of 2026-11-21). `detail::safe_data` in `packages/cpp/include/iscc/iscc.hpp`, its gate
`tests/test_cpp_safe_data.py`, and the `max(Count, 1)` padding of `PinnedRows` in
`packages/dotnet/Iscc.Lib/IsccLib.cs` now only keep pointers non-null and change no result. Dropping
the padding supersedes that half of the 2026-11-20 decision on .NET jagged arrays (its null check
stays). **Decided (Titusz, 2026-11-21):** remove both, in the release whose FFI change made them
dead.

Resolved when both are removed with the C++ and .NET empty-input tests still green.

## `@iscc/wasm` converts non-number values to numbers instead of rejecting them `normal` [review]

The wasm-bindgen glue applies JavaScript number conversion before the range check runs. Measured on
a `--target nodejs` build in iteration 256: `gen_audio_code_v0([1, null, 3])` returns the code of
`[1, 0, 3]`, `text_trim("abc", "2")` returns `"ab"`, `text_trim("abc", true)` returns `"a"`.
`undefined` and a non-numeric string become `NaN` and are rejected. `@iscc/lib` (napi) throws on
each of these inputs. Pre-existing, and TypeScript callers are stopped by the `.d.ts`. No
`data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong argument types).

Resolved when either each numeric parameter of `crates/iscc-wasm/src/lib.rs` rejects a non-number,
pinned by a Node test on a built package, or `docs/howto/wasm.md` → "Error handling" states the
conversion.

## JNI `NativeLoader` rejects an unknown OS or architecture before the `java.library.path` fallback `normal` [review]

`NativeLoader.load()` calls `detectOs()` and `detectArch()` first, and both throw
`UnsatisfiedLinkError` for a value they do not know, so `System.loadLibrary("iscc_jni")` is never
tried. Measured on the published 0.7.0 JAR in iteration 257 with a working library on
`java.library.path`: `-Dos.arch=riscv64` fails with "Unsupported architecture: riscv64" and
`-Dos.name=FreeBSD` with "Unsupported OS: FreeBSD", while a recognized pair without a bundled
library (Windows on aarch64) loads it. Unverified: musl Linux probably fails too, because the
bundled glibc library fails inside `System.load` and only `IOException` leads to the fallback.
Pre-existing; the Java how-to and the JNI README describe it.

Resolved when either an unknown OS or architecture reaches the `System.loadLibrary` fallback, pinned
by a Java test, and the two docs say so, or a decisions.md entry keeps the limit on purpose.

## `docs/architecture.md` sections below the graph cover half the surfaces `normal` [review]

"Crate Summary" gives `iscc-ffi` → "Source", while each GitHub release attaches prebuilt
`iscc-ffi-v*-<target>` archives. The test matrix names `cargo test` as the WASM runner, which runs
no `#[wasm_bindgen_test]` (CI runs `wasm-pack test --node`). Pre-existing.

Resolved when each of the three sections covers every surface the graph shows or says it is an
excerpt, and the `iscc-ffi` and WASM rows match the release assets and the CI command.

## Specs and the core `CLAUDE.md` contradict the code in four places `normal` [audit]

Verified by the iteration-249 audit, re-checked against the tree 2026-11-21. Agents read these files
as instructions, so each wrong statement can be implemented or "fixed back" into the code.

- `specs/python-bindings.md:274`: compute is wrapped in `py.allow_threads(...)`; the code uses
    PyO3's `py.detach(...)` (12 sites in `crates/iscc-py/src/lib.rs`)
- `specs/rust-core.md:485`: the JS recipe destructures `iscc_decode` as an array
    (`const [mt, st, …] =`); napi and WASM return an object
- `specs/swift-bindings.md:147`: "Named parameters with defaults"; the UniFFI exports in
    `crates/iscc-uniffi/src/lib.rs` declare no defaults, so Swift and Kotlin callers pass every
    argument
- `crates/iscc-lib/CLAUDE.md:7-8` lists the binding crates without `iscc-uniffi`, and its Tier 2
    list (lines 40-44) lacks `read_unit` and `SEQ_MAINTYPES`

**Suggested fix:** correct each statement against the code. **Scope estimate:** 4 non-test files.

Resolved when the four places match the code.

## Docs overstate language coverage and what ISO 24138 specifies `normal` [audit]

Verified by the iteration-249 audit, re-checked against the tree 2026-11-21. These pages are what
users and LLM tools read first.

- `docs/llms.txt:5` names 6 languages (Python, Node.js, WASM, Go, Java, C FFI) and line 9 three
    install commands; the project ships 11 surfaces (also Ruby, .NET, C++, Swift, Kotlin)
- `docs/index.md:17,26` and `docs/benchmarks.md:15` say all 10 `gen_*_v0` functions are specified
    (or defined) by ISO 24138; `gen_sum_code_v0` is an iscc-lib addition, not part of the standard

**Suggested fix:** list every surface and its install command in `llms.txt` (or point to the install
table); say 9 functions are standard and `gen_sum_code_v0` is a convenience on top. **Scope
estimate:** 3 non-test files.

Resolved when the three pages state the coverage and the standard's scope correctly.
