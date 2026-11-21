---
name: v080-iteration-ledger
description: Per-iteration scoping ledger for v0.8.0 (iters 195-235) with traps, probes and the open-issue picture
metadata:
  type: project
---

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
a defect class); 215 = data-edge.json seed + Rust/Python harnesses (NEEDS_WORK: CRAP); 216 = harness
repair; 217 = data-edge fan-out G1 (6 surfaces; container crash after Go+Node → restart re-scoped
the other 4 with Go/Node criteria carried over); 218 = G2 Swift+Kotlin + Ruby key fix; 219 = G3
C+C++ generated header, see [data-edge phase](data-edge-phase.md); 220 = .NET one jagged-array pin
helper over all 6 sites (issue named 3 — finish-the-pattern widened it); 221 = C FFI docs
`iscc_`-prefixed type names (c-ffi-api.md + ffi README; issue named only the structs — `Ffi*Hasher`,
README and the doc's cbindgen cmd lacking `--config` widened it) + pytest deriving names from
`iscc.h`; 222 = Python `encode_seq` docstrings — 3 Python copies (wrapper, `.pyi`, PyO3 `///` in
`iscc-py/src/lib.rs` = `_lowlevel.__doc__`; issue named 2); 223 = phantom repo paths in specs (wasm
issue named 1 file; a backticked-path existence loop over `specs/*.md` found go `version.go`,
`go-api.md`, `nodejs-api.md` too — the loop is a reusable probe); 224 =
`scripts/check_spec_paths.py` (trees + backticked paths vs `git ls-files` — NOT disk: untracked
napi/wasm build output made a disk probe falsely green; `GENERATED`/`NOT_PATHS` dicts, no in-spec
marker; phantom tree lines often have prose twins — grep basenames outside the tree); 225 =
`alg_simhash([])` rejects in core + Go (behaviour files: simhash.rs + 4 Go; pass-through bindings
need only their zero-pinning tests flipped + comment-only docs — NOT a fan-out; CRAP baseline regen
for the new branch). 226 = `alg_simhash` data-edge section, all 12 surfaces in ONE step (a new
section is unsplittable: unhandled-section failure + generator fail-closed + vendored copies); 2
vectors (`[]` error + `[f00f,3c3c]`→`fc3f` tie, so handlers decode inputs). 227 = C++ `safe_data` on
ALL 9 vector `.data()` sites (issue named 5; FFI len-0 short-circuits made 4 non-defects, widened
anyway) via one template helper; source grep pins sites a `{}` test can't. 228 = spec gate leniency
(2 files: `*`→`[^/]*`, `**`→`.*`; `NOT_PATHS` backtick-only; replay tests on a `_known()` set +
monkeypatched `tracked_paths`). 229 = Ruby `i32` Bignum (Magnus 0.8.2: only `RBignum::to_i32` lacks
the lower check; u32/usize fine; out-of-i32 is a native-range limit → per-binding tests, NOT
data-edge). 230 = both JNI issues (null guard + error mapping): the issue's `exception_check` fix
was a no-op — jni 0.22.4 `JString::new` catches+CLEARS OOM into `JniCall(NoMemory)` ("JNI call
failed"), so map by error TYPE (trait over jni/IsccError/&str) at all 90 sites; also found JNI docs
say finalized hasher throws IAE (code: ISE). Read the dep source before trusting any issue's fix.
231 = iscc-ffi `mod tests` → `src/tests.rs` (verbatim move; csbindgen reads only `src/lib.rs`, so
exports stay put; gate = regen `iscc.h` + `NativeMethods.g.cs` → `git status` empty). 232 =
Swift+Python data-edge compare strictness (2 test files; other 9 harnesses probed strict; Swift
negatives must parse JSON TEXT — Swift literals are native Bool, not NSNumber; Darwin `as? Int`
takes `false`, `as? Bool` takes 0/1, macOS proof only in CI). 233 = Tier 1 doc-parity gate
(`check_tier1_docs.py`): source parsed from lib.rs column-0 `pub fn`/`pub const`/named `pub use`;
FULL (one-way, casing-normalised) + EXACT (marker blocks in rust-core.md + iscc-lib CLAUDE.md) +
EXEMPT registry, fail-closed discovery at ≥30 distinct symbols; it landed red: java-api lacked
`genSumCodeV0`+`META_TRIM_META` despite saying "All 35" — doc fixes rode along. 234 = WASM
`alg_cdc_chunks` → `Vec<js_sys::Uint8Array>` (wasm-bindgen 0.2.126 `Vec<T>` for JsValue-repr T,
slices.rs:546; the only `to_value` return in iscc-wasm); picked over the gate-hole step because
230–233 were all tooling. 235 = security pass (rustix lock, Go x/text v0.41.0 +
`toolchain go1.26.11` line, CI Node 20→22, devcontainer GO_VERSION, ci-cd.md runtime row); probe
kit, all read-only: `go run …/osv-scanner/v2/cmd/osv-scanner@latest scan source -r .` (skips
gitignored napi lock — fresh-resolve npm instead; `cargo deny` missed a RUSTSEC osv saw),
govulncheck `-show verbose`, `dotnet list <csproj> package --vulnerable`, osv `/v1/query` for
Maven/Actions, endoflife.date API for runtime EOLs. Data-edge strict-compare issue needs an
output-subset DECISION first: .NET `TextCodeResult` has only `Iscc` too. Next: that decision step,
gate holes (fail-closed parser + register 4 sub-threshold docs) + Java how-to fix, then part 2
sweep. 236 = data-edge output-subset rule (spec + validator + C/C++ generator, harnesses next):
C/C++ generator `gen_iscc` had the SAME silent key drop as the 4 named harnesses
(finish-the-pattern: 6 surfaces, not 4); allowed keys DERIVED from `data.json` gen outputs (no hand
list — drift issue); uncompared keys fixed statically per harness; advance can't write decisions.md
→ review records. Next: 4-harness strict fan-out (.NET/Java/napi/WASM, negative test each), then
Java how-to + Ruby gem docs line, then gate holes. Unfiled: ref truncates mismatched digest lengths,
Rust/Go reject — needs a human decision. Trap: the issue says C/C++ fail `alg_simhash([])` with the
NULL error, but `iscc_alg_simhash` short-circuits `num_digests == 0` to the core — re-probe before
restating. Still open: gate leniency, Tier 1 parity check; setuptools bump resolves to 84.2.1 via
`uv lock --upgrade-package setuptools` (dry-run 223); napi Buffer leak is `low` — skip;
Ruby/JNI/ffi-split are `normal`). **Dependabot soupsieve is human-blocked**: 3.0.2/4.x need Python
\>=3.11.5 but `uv.lock` resolves `>=3.10` — escalate, don't scope. 237 = 4-harness data-edge strict
compare fan-out (.NET/Java/napi/WASM, test-only): all four return bare ISCC strings for
text/mixed/iscc → static uncompared sets text→`characters`, mixed→`parts`, iscc→none; negatives must
be ones the OLD intersect passed (`{}` actual was already rejected → use actual `{characters}`;
key-outside-set = `parts` on text with correct iscc, defeats a global set); .NET's only object
vector has `except.dotnet` → needed a synthetic positive. Next: Java how-to + Ruby gem docs line,
then drift-issue gate holes. A 303 from a Maven/portal probe proves nothing: `curl -sL` to follow it
(220: Kotlin red was a portal outage). 238 = Java how-to `genSumCodeV0` fix + Ruby gem range wording
(docs-only, 3 files + a pytest that derives `Ruby {min} to {max}` from release.yml cross-gem
`ruby-versions` — a version range in docs is a derived fact, so it ships with a parity test).
Wording branch only; never claim 3.4/4.0 support (source build unmeasured). Next: drift-issue gate
holes (fail-closed lib.rs parser + register rust.md/c-cpp.md/rust-api.md/architecture.md), then part
3 count sweep. 239 = Ruby source gem made buildable (issue branch 1, NOT "drop the source gem":
`ruby-bindings.md` L164/L323 mandate a source gem, so dropping it amends a spec criterion → Titusz).
Facts: crates.io core can't work (HEAD uses `encode_seq`, absent in 0.7.0) → carry the core in the
gem via a staging script (`cargo package -p iscc-lib` normalizes its manifest; `conformance.rs`
needs `tests/data.json`); WRONG (239 review): cross-gem greps the FIRST line containing `rb_sys`,
unanchored, so an unconditional gemspec rb_sys dep (version-less PATH line) broke the lookup; rb_sys
0.9.130 needs Ruby >=2.7; `--local` install fails once rb_sys is a dep; never name the carried dir
`vendor/`/`tmp/` (gitignored, swept into platform gems). Next: drift gate holes, then part 3 count
sweep. 240 = repair of 239 on top of unpushed `d299106`: rb_sys gemspec dep conditional on
`File.directory?(File.join(__dir__, "core"))` (staged tree only) → lock back to `d299106~1`,
platform gems keep zero runtime deps; test models the action's REAL pipeline + fire test; `run()`
stdout-only. Lesson: model a third-party action from its `action.yml`, never from my own paraphrase.
Next: drift gate holes, then part 3 count sweep. 241 = Ruby source-gem `[profile.release]` (issue
branch 1; branch 2 = decisions.md entry advance can't write → Not In Scope). Derive by verbatim TEXT
slice of the root table, no `tomllib` (pytest imports the script in-process, `python-test` matrix
runs 3.10; same reason as `check_docs_nav.py`); fail closed on missing/empty table +
`[profile.release.*]` sub-tables; trap = `[profile.bench]` (`strip = false`, `debug = true`) right
after it. Behavioural check: `file` on the installed `.so` must lack `not stripped` (grep `stripped`
matches both). Install timing = report-only. Next: drift gate holes (fail-closed lib.rs parser +
register 4 sub-threshold docs), then part 3 count sweep. 242 = Tier 1 gate fail-closed lib.rs parse
(issue part 2): recognise by `^pub\b` OFFSET (multi-line `pub use {…}` groups), named exclusions
only (`pub(…)`, `pub mod`, `types::*`, `IsccError`, `IsccResult` — a kind-wide `::*` exclusion would
hide a new glob), group members bare idents (`{b as c}` derived junk `"b as c"` at HEAD), early
return before the floor. Next: register + complete rust.md/c-cpp.md/rust-api.md/architecture.md AND
`docs/api.md` (names 1 symbol; issue says every API reference) — 9-22 missing symbols each, maybe
split; then part 3 count sweep. 243 = drift-issue part 1, Rust-sourced group: register
`rust-api.md`, `howto/rust.md`, `architecture.md` FULL + `docs/api.md` EXEMPT (mkdocstrings
`::: iscc_lib` renders the API; source lists nothing). Probed: Python results are `dict` subclasses,
so api.md's `json.loads` example crashes; no Rust result type implements `Display`, so rust-api.md's
`println!("{result}")` / `iscc.starts_with` examples don't compile. Trap: `json_to_data_url` +
`encode_component` really return `IsccResult<String>` → never ban the string by grep. The gate exits
0 with docs unregistered, so assert registry membership directly. Next: `docs/howto/c-cpp.md` (17
missing; read c-ffi-dx.md / cpp-bindings.md to decide FULL vs EXEMPT), then part 3 count sweep in
specs + per-crate CLAUDE.md. 244 = c-cpp.md completed + registered FULL (all 17 missing symbols
exist in `iscc.hpp`/`iscc.h`; EXEMPT would leave C++ wrappers documented nowhere) + 0.4 pin folded
in as version-free `cargo add iscc-lib --no-default-features` (docs outside budget; a version_sync
target would add a derived fact). C++ constants are functions (`iscc::meta_trim_name()`); doc C++
snippets are top-level statements → compile by wrapping in `main()`; `tests/test_c_ffi_docs.py` bans
unprefixed header type names in c-cpp.md. Next: part 3 count sweep (specs + per-crate CLAUDE.md),
then v0.8.0 should have no normal blocker left. 245 = drift issue part 3a: counts + line numbers in
`specs/` and the per-crate `CLAUDE.md` files, docs-only (about 107 grep hits in 24 files, one class
rule: remove, never correct). Probed: `check_tier1_docs.py` registers `rust-core.md` and
`iscc-lib/CLAUDE.md` EXACT and the ffi/wasm/go `CLAUDE.md` files FULL, so names are gated and only
the numerals go; `check_ci_job_table.py` gates rows, not the "21 job keys / 22 check names" prose.
Trap: a backtick inside a regex in next.md breaks the inline code span (mdformat escapes `_`/`\b`).
Use `.?` or a fenced block. Next: part 3b, the file lists (indentation trees, not box-drawing) in
the `CLAUDE.md` files. No gate parses them, so remove them or extend `check_spec_paths.py`. Then
v0.8.0 has no normal blocker left. 246 = drift part 3b, CLAUDE.md half: REMOVE the trees +
File/Module tables from the 12 per-crate `CLAUDE.md` files (docs-only). Measured: zero stale listed
paths (only gitignored build output the tree labels as such); the drift is MISSING entries (go
`iscc_id.go`, napi seq/id tests, cpp conanfile/vcpkg), so an existence gate would be green and fix
nothing; completeness needs per-format universes. Removing the lists drops no Tier 1 name (probed
with `check_tier1_docs.named_symbols`). Spec half split off: `check_spec_paths.py` fails on zero
trees (floors), so removing or completeness-gating spec trees changes the script +
`tests/test_check_spec_paths.py`. Spec trees sit under `## Package/Crate Structure`, no
Verified-when names them; go lists 7 of ~50 files, so they are clearly key files, not complete
lists. Next: spec half (remove trees + drop tree floors, or mark them key files only and keep the
existence check), then v0.8.0 has no normal blocker. 247 = drift spec half: KEEP the trees and drop
the claim of completeness (the advisor agreed). The issue's Resolved-when only asks for an existence
check on a partial list, and `check_spec_paths.py` already is one. Gate the "not every file" note
per fence: keyed on `ENTRY_RE` matches, not root lines, and matched on the joined paragraph because
mdformat wraps lines. Collapse entries below a GENERATED prefix (the gate can never fail on them).
Widen `BACKTICK_RE` with `.github` for the ci-cd "Workflow Files" table. Found and left for review
to file: ci-cd.md still specifies a release tag trigger (lines ~10/173/187) against target.md and
`release.yml` (`workflow_dispatch` only). Next: if PASS, v0.8.0 has no normal blocker left. Check
the tag-trigger realignment, then NONE/IDLE. 248-252 → git log. 253 = review issue "harnesses reject
valid meta vectors": a reduced-result harness's uncompared set = every `data.json` output key minus
`iscc` (napi/WASM/JNI/.NET lacked `description`); generator test accepted only object `meta`.
Test-only, 5 files. Trap: `call_meta` (Rust test helper) sits at CRAP 30.0 — forbid any branch added
there.
