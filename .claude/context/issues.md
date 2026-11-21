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

## A rebuilt devcontainer cannot run the Ruby gates `normal` [review]

The gem floor is Ruby 3.3 since iteration 270, and the devcontainer's system Ruby is Debian's 3.1.
`.devcontainer/post-create.sh` still runs `bundle install` in `crates/iscc-rb` under that Ruby,
which Bundler now refuses (the step is non-fatal, so setup continues). Both standardrb hooks in
`.pre-commit-config.yaml` run `mise exec ruby@3.4 -- bundle exec standardrb`: mise installs a Ruby
3.4 on demand, but nothing installs its bundle, so on a fresh container `mise run check` and every
commit that touches a `.rb` file fail until someone runs `bundle install` under the mise Ruby by
hand. Ruby is pinned in no config file: the hooks say `ruby@3.4`, `crates/iscc-rb/CLAUDE.md` and
`specs/ruby-bindings.md` say `ruby@3.4.13`. The working container of iteration 270 has the bundle,
so nothing fails there today. Introduced by the floor raise; next.md kept the devcontainer out of
that step. Also found by the Codex review.

Resolved when a fresh devcontainer build runs `mise run check` and the Ruby suite with no manual
step, and the Ruby version the hooks and docs use is named in one place (a `mise.toml` pin, or a
supported Ruby in the image).

## `release.yml` loads only one precompiled Ruby binary before publishing `low` [review]

**Triage:** untriaged. A Ruby 3.4 user (or anyone not on x86_64 Linux) would get a precompiled gem
that fails at `require` if a cross-build ever produces a broken binary for that series or platform.

`build-gem` builds Ruby 3.3 and 3.4 binaries for five platforms. `test-gem` installs the
`x86_64-linux` gem on Ruby 3.3 and runs `conformance_selftest`, so the 3.4 binary and the other
platforms are published without being loaded once. The 3.4 cross-build is new in iteration 270 and
no run has exercised it (`release.yml` is `workflow_dispatch`-only). Pre-existing in kind: the 3.1
and 3.2 binaries were published the same way.

Resolved when `test-gem` loads the gem on every series in the cross-gem `ruby-versions` list (a
matrix whose list a test ties to that input), or a decisions.md entry keeps the single smoke test.

## Ruby 4.0 gets no precompiled gem `low` [review]

**Triage:** untriaged. Every user on Ruby 4.0, the current stable series, installs the source gem
and needs a Rust toolchain and `libclang-dev`; the docs say so.

The cross-gem image is pinned to rb-sys 0.9.123, whose newest cross Ruby is 3.4.5. Ruby 4.0 needs
rb_sys 0.9.124 or newer, and the hold comment in `crates/iscc-rb/Gemfile` says that version breaks
`RbSys::ExtensionTask`. Unverified since the hold was written.

Resolved when the cross-gem list includes 4.0 and the docs name the range, or a decisions.md entry
keeps 4.0 on the source gem.

## `docs/development.md` tells maintainers to release by pushing a tag `normal` [review]

`docs/development.md` → "Publish" says to run `git tag vX.Y.Z` and `git push origin vX.Y.Z` on
`main`, and that "the tag push triggers `.github/workflows/release.yml`". The workflow has
`workflow_dispatch` as its only trigger, so the push starts nothing. It also does harm:
`prepare-release` does not move a tag that already exists (its checkout fetches every tag, `git tag`
fails and the step continues), so a later dispatch leaves the hand-pushed tag on the commit before
the `Package.swift` checksum commit. The registry table under the same heading lists five of the
packages the workflow publishes, and the checkbox sentence four of the eight checkboxes.
Pre-existing, found in iteration 259 while the same claim was removed from the specs.

In `.claude/context/specs/swift-bindings.md`, the heading "Release Process — Force-Update Tag
Pattern" and its intro paragraph still name Ferrostar's force-update approach. The Release Flow
below it creates the tag once, after the checksum commit, and moves none. Realign without escalation
(decision of 2026-11-20).

Resolved when the "Publish" section describes the dispatch with a `version` input, names every
registry chain or says the table is an excerpt, tells the reader not to push the tag by hand, and
the Swift heading and intro match the flow.

## C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection the FFI no longer has `normal` [review]

The C FFI reads a NULL pointer with length 0 as an empty input on every pointer+length parameter
(decision of 2026-11-21). `detail::safe_data` in `packages/cpp/include/iscc/iscc.hpp`, its gate
`tests/test_cpp_safe_data.py`, and the `max(Count, 1)` padding of `PinnedRows` in
`packages/dotnet/Iscc.Lib/IsccLib.cs` now only keep pointers non-null and change no result. Dropping
the padding supersedes that half of the 2026-11-20 decision on .NET jagged arrays (its null check
stays). **Decided (Titusz, 2026-11-21):** remove both, in the release whose FFI change made them
dead.

Resolved when both are removed with the C++ and .NET empty-input tests still green.

## `@iscc/wasm` converts byte inputs and traps on a non-string `low` [review]

**Triage:** untriaged. A plain-JavaScript caller who passes a string or a plain array where a
`Uint8Array` is declared gets a different valid code and no error; the `.d.ts` stops TypeScript
callers.

Measured on a `--target nodejs` release build in iteration 268. `gen_data_code_v0("abc")` returns
the code of three zero bytes, and `gen_data_code_v0([1, "2", null, 300])` the code of
`[1, 2, 0, 44]`. A number where a string is declared (`gen_text_code_v0(5)`,
`gen_meta_code_v0("a", 7)`) throws `RuntimeError: memory access out of bounds` instead of an error
that names the parameter. Pre-existing: the glue converts every `&[u8]` and `&str` parameter before
Rust runs. No `data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong argument types).

Resolved when either byte and string parameters reject a wrong type with an error that names the
parameter, or `docs/howto/wasm.md` → "Error handling" states the conversion.

## `@iscc/wasm` list inputs read through serde let a throwing element kill the instance `low` [review]

**Triage:** untriaged. Only a caller whose array holds an element getter (or a proxy) that throws,
passed several thousand times to one instance, would hit it; ordinary arrays are unaffected.

`serde_wasm_bindgen::from_value` reads array elements through js-sys getters without `catch`, so a
JavaScript exception unwinds through the Rust frames. Measured in iteration 269 from Node on release
builds, with an array whose index 1 is a throwing getter: after 5,000 to 8,200 such calls every
export fails with `RuntimeError: memory access out of bounds`. Affected: `gen_video_code_v0` and
`soft_hash_video_v0` (inner arrays), `gen_mixed_code_v0`, `gen_iscc_code_v0` and `encode_seq`.
Pre-existing: the build of `3ef83ce` behaves the same. The number-array parameters (`cv`,
`features`) are safe since iteration 269 (`number_vec` reads through `Reflect::get`). No
`data-edge.json` vector is owed (a getter is not JSON-expressible).

Resolved when a throwing element on each of these parameters gives an ordinary error and leaves the
instance usable, pinned by a test per parameter, or a decisions.md entry keeps the limit on purpose.

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

## One-shot gen functions re-implement their streaming counterparts `normal` [audit]

Verified by the iteration-249 and iteration-260 audits (3040298).

- `gen_instance_code_v0` (`crates/iscc-lib/src/lib.rs:907-908`) calls `blake3::hash(data)` and then
    `utils::multi_hash_blake3(data)`, which hashes the same bytes again (`utils.rs:164`): two full
    BLAKE3 passes per call. The multihash format is also written in `streaming.rs:39-56`.
- `gen_sum_code_v0` in `crates/iscc-wasm/src/lib.rs` copies `SumHasher::update`/`finalize`
    (`streaming.rs:172-203`), comment included; the same file already wraps `SumHasher`.
- `gen_meta_code_v0` parses `meta` twice (`lib.rs:372` via `parse_meta_json`, then `:406-408`), and
    `json_to_data_url` (`:322-328`) repeats the parse+canonicalize body with another error string.

**Why it matters:** each pair can drift apart, and the instance hot path does twice the hashing.
**Suggested fix:** derive `datahash` from the single digest (one shared multihash helper), call
`SumHasher` from the WASM one-shot function, and have `parse_meta_json` return what the data URL
step needs. **Scope estimate:** 3 non-test files; mind the iai-callgrind baseline.

Resolved when each computation exists once and the conformance and perf gates stay green.

## No CI job runs a test suite on Windows `low` [review]

**Triage:** untriaged. A Windows user of any binding would hit a defect that depends on the platform
(C `long` width, path handling, line endings), because no run before publishing can see it.

`ci.yml` has 21 jobs on `ubuntu-latest` and one on `macos-15` (the Swift job). `release.yml` builds
Windows artifacts (Python wheel, napi, JNI, C FFI, Kotlin native among them), but only `test-nuget`
runs on `windows-latest`, and it calls `conformance_selftest` alone; `test-wheels` runs the Python
wheel on Linux only. The video exception class fixed in iteration 265 differed on Windows only and
was found by reading the code. Pre-existing, measured 2026-11-21.

Resolved when either CI runs at least the Python suite on `windows-latest`, or a decisions.md entry
keeps CI on Linux and macOS on purpose.

## Specs and crate CLAUDE.md files describe shipped work as pending and name missing items `normal` [audit]

Verified by the iteration-260 audit at 3040298. CID-fixable (decision of 2026-11-20: realign).

- `specs/rust-core.md:360-371`: `Version` "has only V0" and every surface "fails on any ISCC-IDv1";
    `codec.rs:105-131` defines `V1` and admits `(Id, V1)`.
- `specs/python-bindings.md:301-304`: text and video paths "still hold the GIL";
    `iscc-py/src/lib.rs` wraps them in `.detach(` (:138, :190, :217, :241).
- `specs/go-bindings.md`: the IDv1 rename "exists only on `develop`" (it shipped in 0.6.0);
    "Streaming functions also accept `io.Reader`" and the two `io.Reader` bullets (no such API in
    `packages/go`).
- `specs/wasm-bindings.md` → "Rust Bridge Layer": the sketch shows `js_name = "genMetaCodeV0"`,
    `bits: Option<u32>` and a serde `JsValue` result; the exports are snake_case, take scalars as
    `JsValue` and return the `iscc` string.
- `crates/iscc-napi/CLAUDE.md:36,66`, `crates/iscc-py/CLAUDE.md:66`, `crates/iscc-wasm/CLAUDE.md:43`
    name `iscc_lib::Error`; the type is `IsccError`.
- `packages/go/CLAUDE.md`: :95-121 keeps a test-file list (missing three files) that the 2026-11-21
    decision removed; :146 says most packages wrap `iscc-ffi` (only C++ and .NET do); :133-135 omits
    the `packages/go/vX.Y.Z` tag the module proxy needs (`release.yml:115-116`).

**Why it matters:** define-next and advance read these as truth, so stale claims become scoped work.
**Suggested fix:** rewrite each statement in the present tense to match the code. **Scope
estimate:** 8 files, needs audit-scope step (≤8 files).

Resolved when each cited statement matches the tree.

## `@iscc/wasm` docs show a Node.js setup the published package cannot run `normal` [audit]

Verified by the iteration-270 audit at 34303a6. The npm package is built with
`wasm-pack build --target web` (`release.yml:427`). Its default `init()` builds a URL from
`import.meta.url` and calls `fetch()` on it, which Node's `fetch` refuses for `file:` URLs.

- `docs/howto/wasm.md` → "Node.js" (:58-72) imports from `"@iscc/wasm"` and calls `gen_text_code_v0`
    with no init at all, after telling the reader to build with `--target nodejs` (the reader
    installed the npm package and has no build). :42 says most bundlers initialize WASM
    automatically, which a web-target package never does.
- `crates/iscc-wasm/README.md:28-37`: the quick start's `await init()` fails under Node, and the
    next line says "Both browser and Node.js targets are supported."
- `docs/howto/wasm.md:82` and `README.md:56` say every gen function returns a string;
    `gen_sum_code_v0` returns `WasmSumCodeResult` (`lib.rs:348-353`, and the how-to's own example
    reads `result.iscc`). The README's Streaming line omits `SumHasher`.

**Why it matters:** the registry page and how-to are the first thing a Node user copies, and both
paths fail on the published artifact. **Suggested fix:** show the Node path for the web build
(`initSync({ module: readFileSync(...) })` or bytes passed to `init`), probed against a packed
release build; drop the bundler and "both targets" claims; correct the return-type sentences.
**Scope estimate:** 2 non-test files.

Resolved when the Node example runs against a `--target web` release build and both pages state
`gen_sum_code_v0`'s result type.

## UniFFI `text_trim` truncates `nbytes` on 32-bit Android and documents a word boundary `normal` [audit]

Verified by the iteration-270 audit at 34303a6 (the doc half since iteration 249).

- `crates/iscc-uniffi/src/lib.rs:330-331` passes `nbytes as usize`. `release.yml:1131-1144` ships
    the library for `armeabi-v7a` and `x86` Android, where `usize` is 32 bits, so
    `textTrim(text, 4294967301UL)` trims to 5 bytes instead of returning the text. The only test
    uses `nbytes = 5` (:643).
- The doc comment at :328 says "on a word boundary"; the core (`utils.rs:127-137`) cuts at a UTF-8
    byte boundary and strips whitespace. The wrong text is in the generated
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift` and the Kotlin `iscc_uniffi.kt`.

**Why it matters:** a silent wrong result on two shipped ABIs, and the published Swift/Kotlin API
docs describe behavior the function lacks. **Suggested fix:** saturate
(`usize::try_from(nbytes).unwrap_or(usize::MAX)`, a limit beyond the address space trims nothing),
add a `u64::MAX` test, fix the doc comment and regenerate both bindings. **Scope estimate:** 3
non-test files.

Resolved when an oversized `nbytes` returns the whole text on every target and the doc matches the
core.

## `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises, with no decision `normal` [audit]

Verified by the iteration-270 audit at 34303a6. The reference computes `min([...])` over the
features (`reference/iscc-core/iscc_core/minhash.py:14-16`), so an empty list raises `ValueError`.
The core's `minhash` falls back to `MAXH` (`crates/iscc-lib/src/minhash.rs:156-164`), so
`alg_minhash_256` returns all `0xFF`. Five hand-written tests pin the divergence
(`tests/test_algorithm_primitives.rs:146`, `iscc-ffi/src/tests.rs:784`, `IsccLibTest.java:932`,
`packages/cpp/tests/test_iscc.cpp:688`, `packages/go/minhash_test.go:79`); `data-edge.json` has no
`alg_minhash_256` section, and no decision or edge-vector rule mentions it. The closest decision
(2026-11-20) made `alg_simhash` reject an empty list, the opposite direction.

**Why it matters:** a Tier 1 function silently differs from the normative reference on one input,
and the per-surface tests freeze it without anyone having chosen it. **Suggested fix:** decide:
reject empty input in the core (as `alg_simhash` does) or keep `0xFF` with a decisions.md entry;
either way pin it with one `data-edge.json` vector instead of per-surface tests. **Scope estimate:**
1-2 non-test files plus the vector fan-out.

Resolved when the empty-input behavior is a recorded decision pinned by a shared vector.

## `conformance_selftest` repeats one runner loop nine times, in Rust and in Go `normal` [audit]

Verified by the iteration-249, -260 and -270 audits (34303a6). `crates/iscc-lib/src/conformance.rs`
has nine gen runners (:64, 128, 176, 229, 282, 342, 402, 451, 500), each repeating the section
lookup with its "section missing" message, `Ok(result) if result.iscc == expected_iscc`, and an
8-line `match` ending in "could not parse test inputs". `report_case` (:560) exists but only the two
seq runners call it (:604, :641). `packages/go/conformance.go:156-516` mirrors the layout with about
36 copied `Fprintf` lines. This is shipped library code (Tier 1 `conformance_selftest`).

**Why it matters:** about 400 lines per language where a reporting change needs nine edits, and the
copies already differ (two use `report_case`, nine do not). **Suggested fix:** one generic section
runner that does lookup, `iscc` comparison and `report_case`, so each gen function supplies only its
input-parsing closure; the same helper in Go. Move the CRAP baseline in the same step. **Scope
estimate:** 2 non-test files plus `.crap-baseline.json`.

Resolved when each language has one section runner and the selftest still passes on every surface.

## Gates test generated and built artifacts that nothing refreshes or diffs `normal` [audit]

Verified by the iteration-270 audit at 34303a6 (the pytest half since iteration 249).

- Pre-push `uv run pytest` (`.pre-commit-config.yaml:176-182`) imports the hand-built
    `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so`. No hook, `mise` task or conftest rebuilds it
    or checks it is newer than `crates/iscc-py/src`; only CI runs `maturin develop` (`ci.yml:67`). A
    Rust change pushes green against the old binary.
- CI diffs only `iscc.h` (`ci.yml:136-140`). `crates/iscc-ffi/build.rs:4-10` rewrites
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` on every build and `dotnet-test` builds against
    the fresh copy, so a stale committed file is never caught. The tracked UniFFI outputs
    (`iscc_uniffi.kt`, `iscc_uniffi.swift`) are regenerated by hand and no workflow runs
    `uniffi-bindgen` to compare them; doc drift (see the `text_trim` entry) ships unseen.

**Why it matters:** a green gate that checks the wrong artifact is worse than no gate. **Suggested
fix:** rebuild (or fail on a stale `.so`) before pre-push pytest; add `git diff --exit-code` for
`NativeMethods.g.cs` after the FFI build and a regenerate-and-diff step for the UniFFI sources
(compare modulo trailing whitespace, per learnings). **Scope estimate:** 3 non-test files.

Resolved when a stale `.so` or a stale committed generated file fails a gate.
