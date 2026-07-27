---
name: unicode-fixture-propagation
description: Ledger for propagating unicode_boundary.json into the 11 binding surfaces — loader taxonomy, which binding artifacts are stale, measured Go pass/fail, slice order
metadata:
  type: project
---

# Propagating `unicode_boundary.json` to the bindings (issue remainder **(b)**)

The Rust fixture (`crates/iscc-lib/tests/unicode_boundary.json`, 7 `text_clean` + 5 `text_collapse`
cases, pure ASCII) is finished and design-discriminating since iter 149. What remains is wiring it
into each binding's test suite.

**Why:** the sentinel freeze rule is a cross-cutting conformance contract; until every surface gates
it, 11 language surfaces ship ungated. **How to apply:** scope one language group per step, cite the
measured facts below instead of re-probing.

## Loader taxonomy — decides how much plumbing a slice needs

- **Canonical-path readers** (no file copy needed; a second fixture is a path constant + new test):
    Python `tests/test_conformance.py`, napi `crates/iscc-napi/__tests__/conformance.test.mjs`, WASM
    `crates/iscc-wasm/tests/conformance.rs` (`include_str!("../../iscc-lib/tests/data.json")`), Ruby
    `crates/iscc-rb/test/test_conformance.rb`.
- **Vendored-copy surfaces** (need a byte-identical `cp` next to their `data.json`):
    `packages/go/testdata/`, `packages/swift/Tests/IsccLibTests/`. **dotnet and kotlin are NOT** —
    each can be turned into a canonical-path reader by a one-line build-config indirection (see
    slice 5 below), so a tracked copy + `VENDORED_COPIES` registration is avoidable there.
- **JNI/Java is a canonical-path reader** (verified iter 153): `IsccLibTest.java` does
    `Files.readString(Path.of("../../iscc-lib/tests/data.json"))` — maven's basedir is
    `crates/iscc-jni/java`, so the same relative shape reaches the boundary fixture. No copy needed.

**Cost has two axes, not one** (the single "already calls text functions" axis misled iters
151-152): fixture-reading plumbing *and* text-function coverage. C FFI (`tests/test_iscc.c`) and C++
have **neither a JSON parser nor a vector file** — most expensive to open, but once the C header
exists C++ reuses it for near-zero cost. Swift has plumbing but adds a *tracked copy* (→
`VENDORED_COPIES` registration); C# and Kotlin looked the same until iter 154 found the build-config
indirection that avoids it.

**`cmake` is NOT a blocker (measured iter 160).** `cmake` is absent from `$PATH`, but the PyPI wheel
installs on demand: `uv run --with cmake cmake …` → **4.4.0**, ~29 MB download, accepts the
project's `cmake_minimum_required(VERSION 3.14)` with no deprecation error. Full flow from the repo
root, no `cd`, gitignored build dir:
`uv run --with cmake cmake -S packages/cpp -B packages/cpp/build-uv -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=$PWD/target/debug -DSANITIZE_ADDRESS=ON`
→ `--build` → `LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-uv/tests/test_iscc` = **54
passed, 0 failed**. Do not reuse `packages/cpp/build/` (its cache is cmake 3.25). `swift` remains
genuinely absent with no equivalent escape hatch.

- Go is a hybrid: its **per-function `*_test.go` conformance tests read
    `../../crates/iscc-lib/tests/data.json` by relative path**, while `testdata/data.json` exists
    only so the *public* `ConformanceSelftest()` can `//go:embed` it. Either shape is defensible;
    iter 150 chose the embedded copy (no skip-on-missing branch, works from a downloaded module).

## Binding artifact freshness (measured iters 150-151)

- **Python editable install: CURRENT** — passes all 12 vectors, no rebuild needed.
- **napi `crates/iscc-napi/iscc-lib.linux-x64-gnu.node`: STALE (pre-iter-133)** — returns `aSb` for
    `text_clean("a" + U+A7F1 + "b")` and `eŚ` for the U+A7F1 sequence. It is **untracked**
    (`crates/iscc-napi/.gitignore:4`), so a napi slice must rebuild it; budget for it in the step.
- **Ruby `crates/iscc-rb/lib/iscc_lib/iscc_rb.so`: was STALE, rebuilt at iter 151.**
    `bundle exec rake compile` (~1.5 min, cached cargo) flipped `text_clean("a"+U+A7F1+"b")` from
    `"aSb"` to `"ab"`. The `.so` is gitignored (`crates/iscc-rb/.gitignore:3`) → rebuilding leaves
    no tree diff. Baseline suite: **111 runs / 299 assertions / 0 failures**, `standardrb` clean.
- **JNI `target/debug/libiscc_jni.so`: STALE (measured iter 153)** — dated Jul 25, older than the
    sentinel commit `7acf0fa` (2026-07-26). Rebuild with `cargo build -p iscc-jni`; surefire's
    `argLine` already points `-Djava.library.path` at `target/debug`.
- **C FFI `target/debug/libiscc_ffi.so` and UniFFI `libiscc_uniffi.so`: both rebuilt iter 154**
    (`cargo build -p iscc-ffi` / `-p iscc-uniffi`, ~15 s each, **no tree diff** — the build.rs
    regeneration of `NativeMethods.g.cs` and the checked-in `iscc_uniffi.kt` are both byte-stable).
    Post-rebuild FFI probe via `ctypes` on `iscc_text_clean` returned the exact sentinel values for
    all three sequence vectors.
- **WASM has no artifact age** — `wasm-pack test --node` recompiles the core every run.
- Probe rule: U+0378 rows do **not** discriminate a stale artifact (U+0378 is `Cn` in every Unicode
    version, so even a no-freeze-rule build gets them right). Only **U+A7F1** rows do.

## Runner facts (measured iter 151, offline)

- `wasm-pack test --node crates/iscc-wasm --features conformance` → **exit 0 in ~2 min**; wasm-pack
    0.13.1 with `wasm-bindgen`/`wasm-opt` already in `~/.cache/.wasm-pack` (no download).
    `--features conformance` only gates the `conformance_selftest` test in `unit.rs`;
    `conformance.rs` is ungated, so new boundary tests need no `cfg`.
- **`#[wasm_bindgen_test]` files compile on the host but run as 0 tests there** —
    `cargo test -p iscc-wasm` reports `0 passed` for every target. So host `cargo test` verifies
    only compilation; `wasm-pack test --node` is the only executing runner. Say this in next.md or
    advance will chase a phantom failure.
- CRAP/coverage are `-p iscc-lib` only (`mise.toml [tasks.coverage]`) and `.crap-baseline.json` has
    **zero** `iscc-wasm` entries → binding-test slices never move a baseline.
- Ruby: `rake test` loads every `test/*.rb` into ONE process, and `test_conformance.rb` defines
    top-level `DATA_JSON` / `CONFORMANCE_DATA` — a new file must use fresh constant names or Ruby
    warns "already initialized constant". `standardrb` is CI-only, **not** a prek hook.

## Go — measured results on all 12 vectors (iter 150, Go 1.26.1)

9 pass, exactly 3 fail:

| section         | case                                    | why it fails                        |
| --------------- | --------------------------------------- | ----------------------------------- |
| `text_clean`    | `test_0000_u1fae9_assigned_so_retained` | U+1FAE9 is `Cn` in Go's 15.0 tables |
| `text_clean`    | `test_0001_u113c5_assigned_mc_retained` | U+113C5 is `Cn` in Go's 15.0 tables |
| `text_collapse` | `test_0000_u1fae9_assigned_so_retained` | same cause                          |

- **All four sequence vectors PASS** — the iter-147 `Final_Sigma` fix is live.
- `text_collapse/test_0001_u113c5_assigned_mc_mark_dropped` **passes** (Go drops the mark as `C`,
    the expectation drops it as `M`; both give `ab`). So the skip list is **per case, not per code
    point** — a naive "skip both Unicode-16 code points everywhere" over-skips by one.
- Ruling: `decisions.md` 2026-07-26 — skip with a tracking note until go1.27 (~Aug 2026); never
    vendor the 5,813-code-point 15.0→16.0 delta.

## Slice order used / planned

1. **iter 150 ✅** — Python (canonical path) + Go (vendored copy + 3 skips). Establishes both
    plumbing patterns in one step; both runnable in-container with no artifact rebuild.
2. **iter 151 ✅** — WASM + Ruby, both canonical-path readers, both runnable in-container (~2 min
    each). No skips: both execute the Rust core, so all 12 vectors must pass.
3. **iter 152 ✅** (interleaved) — the vendored-copy byte-identity drift gate
    (`tests/test_vendored_fixtures.py`), landed before the first slice that adds new copies.
4. **iter 153 scoped** — napi + JNI/Java, both canonical-path readers, both runnable offline
    in-container, no new vendored copy. Both need a native rebuild first
    (`npx napi build --platform` from `crates/iscc-napi`; `cargo build -p iscc-jni`). JNI/Java has
    **zero** text-function tests, so its loop is new plumbing — but gson + the canonical-path
    `readString` already exist in `IsccLibTest`. `mvn` resolves **fully offline**
    (`mvn -o -B test-compile` = BUILD SUCCESS in 2.4 s against a 54 MB `~/.m2`);
    `crates/iscc-jni/java/target/` is gitignored.
5. **iter 154 scoped** — C# + Kotlin, **both locally runnable** and both avoiding a tracked copy:
    - C#:
        `<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json" Link="testdata\unicode_boundary.json">`
        - `PreserveNewest` — probed in a throwaway `/tmp` project at equal depth; lands in
            `bin/Debug/net8.0/testdata/`, which is gitignored. `dotnet` 8.0.423 present, NuGet restore
            works (network reachable, ~4 s); baseline suite **91 passed in ~190 ms** with
            `LD_LIBRARY_PATH=$PWD/target/debug`.
    - Kotlin: one
        `systemProperty("iscc.fixtureDir", "${rootProject.rootDir}/../../crates/iscc-lib/tests")` in
        the existing `tasks.withType<Test>` block (`rootProject.rootDir` = `packages/kotlin` locally
        *and* in CI, which sets `working-directory: packages/kotlin`).
    - **state.md's "Kotlin/Swift cannot be built here" is only half true.** `~/.gradle` has the
        unpacked `gradle-8.12.1-bin` dist + populated caches since iter 128, so
        `./gradlew cleanTest test --offline` really runs (**9 tests, 0 fail, 6 s**). **`swift` is
        genuinely absent.** Trap: bare `./gradlew test` prints `BUILD SUCCESSFUL` while the task is
        UP-TO-DATE and executes nothing — always `cleanTest test` and check the `TEST-*.xml` mtime.
6. **iter 159 scoped — C FFI**, ahead of Swift/C++ because it is the last surface that both builds
    *and runs* here. Design (measured while scoping, all proven end-to-end in `/tmp`):
    - PEP 723 generator `scripts/gen_ffi_boundary_vectors.py` (`dependencies = []`, stdlib `json`)
        renders the canonical fixture into a **tracked, ASCII**
        `crates/iscc-ffi/tests/   unicode_boundary_vectors.h` of
        `static const struct {name,input,expected}` arrays plus
        `ISCC_TEXT_{CLEAN,COLLAPSE}_VECTOR_COUNT` / `ISCC_UNICODE_DATA_VERSION` macros.
    - **Octal escapes (`\%03o`), never `\xHH`** — C hex escapes are greedy/unbounded, so
        `"a\xF0\x9F\xAB\xA9b"` mis-parses; octal stops after 3 digits.
    - The header sits **next to `test_iscc.c`**, so a quoted `#include` needs **no new `-I`** and
        `ci.yml`'s `c-ffi` gcc line stays byte-identical. Verified by compiling with the exact CI
        command.
    - A generated header is derived, not byte-identical → it must **NOT** go in `VENDORED_COPIES`;
        its equivalent is a pytest gate asserting `render(fixture) == tracked header` (+ a mutation
        case so the gate provably fires).
    - Measured: `cargo build -p iscc-ffi` = 14.8 s, **no tree diff** (`NativeMethods.g.cs` is
        byte-stable). The local `.so` was stale (pre-156 `Final_Sigma`); after rebuild a `ctypes`
        probe gives **12/12 vectors OK, zero skips** — the C FFI wraps the Rust core, so no skip map.
        Existing C suite baseline: **65 passed, 0 failed** → 80 with 12 vectors + a 3-assertion
        metadata guard.
7. **iter 160 scoped — C++**, promoted ahead of Swift because the `uv run --with cmake` escape hatch
    (above) makes it locally runnable *and* the artifact it needs already exists: reuse
    `crates/iscc-ffi/tests/unicode_boundary_vectors.h` as-is — no second generator, no
    `VENDORED_COPIES` entry. Measured while scoping with a standalone `/tmp` probe: `-Wall -Wextra`
    - ASAN clean, **12/12 vectors PASS**, and the helper signature
        `void run_unicode_boundary_section(const char*, std::string (*fn)(const std::string&), const iscc_unicode_boundary_vector*, size_t)`
        compiles (taking the address of the inline `iscc::text_clean` is fine). Include dir goes on
        the **test** target in `packages/cpp/tests/CMakeLists.txt`, never on the public `iscc`
        INTERFACE target in `packages/cpp/CMakeLists.txt` (that path is inherited by vcpkg/conan
        consumers) — state.md recommended the wrong file. Baseline 54 → 69 passed (3 metadata guards
        \+ 12 vectors).
8. Swift last: the one *tracked vendored copy* (→ `VENDORED_COPIES`, SwiftPM `resources:` read via
    `Bundle.module`), and `swift` is genuinely absent here — CI-proof-only.

## Standing hazards

- Copy vendored fixtures with `cp`, never Write/Edit, then verify with `cmp` — a tool payload can
    decode `\uXXXX` into literal UTF-8 and silently break an ASCII-escaped file (observed iter 149;
    *not* reproduced in iter 150, so treat it as transport-dependent and always verify).
- Do **not** replicate the `delete_filter_output` oracles into binding tests. They live in the Rust
    `SEQUENCE_VECTORS` const and the issues.md table; equality against `outputs.result` already
    catches a delete-filter regression, and copying oracles 11× re-opens the iter-149 mislabel
    hazard. See [[unicode-freeze-facts]] for the corrected row-3 value.
- **Vendored-copy drift gate — LANDED iter 152** (`tests/test_vendored_fixtures.py`: explicit
    `(canonical, copy)` table, count floor, `git ls-files` set-equality catching an unregistered *or
    deleted* copy, ASCII guard). Tracked fixture files: **7 = 2 canonical + 5 copies**. It keys on
    the two **basenames**, so a copy renamed to anything else is invisible — keep canonical
    basenames. A canonical-path slice must leave the table untouched; say so in next.md.
- **Discovery must use `git ls-files`, never `Path.rglob`**: untracked build outputs exist in this
    container (`packages/dotnet/Iscc.Lib.Tests/bin/Debug/net8.0/testdata/data.json`,
    `packages/kotlin/build/resources/test/data.json`) and `.venv/…/iscc_core/data.json` is a
    *different* file (`md5 3690f853…` vs the canonical `4f17639ab1dd…`).
- **Only `unicode_boundary.json` is pure ASCII** (2,344 B, `\uXXXX` escapes — so an `isascii()`
    assertion is a valid corruption guard for it). `data.json` is 84,478 B and legitimately contains
    raw non-ASCII: never assert ASCII on it.
