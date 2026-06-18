# Review Agent Memory — Archive

Stale or niche entries moved out of `MEMORY.md` to keep it under the 200-line budget. Not loaded by
the agent automatically — reference for humans and occasional lookup only.

## Python Benchmark Review (archived iter 93)

- pytest-benchmark tests in `tests/test_benchmarks.py`: 18 benchmarks (9 fn x 2 impls), ~11s
    collection overhead (`--benchmark-disable` optimization noted in learnings-archive.md)
- Review shortcut: `mise run check` + ruff check/format + clippy (Python-only shortcut)

## Swift Package Review (archived iter 94 — Swift bindings fully complete)

- `packages/swift/` — SPM package: iscc_uniffiFFI (C header + modulemap) + IsccLib (generated Swift)
- Two `Package.swift` files: root (SPM consumers, binaryTarget) + `packages/swift/Package.swift`
    (CI/local dev, linkedLibrary). Root omits testTarget. Both coexist without conflict
- Root `Package.swift` uses Ferrostar-style variable toggle: `useLocalFramework` (bool),
    `releaseTag`, `releaseChecksum`. Default `false` → remote binaryTarget from GitHub Releases
- `scripts/build_xcframework.sh`: 5 Rust targets → lipo fat binaries → xcodebuild → ditto zip →
    swift package compute-checksum. Cannot test on Linux — `bash -n` syntax check only
- Review shortcut: `cargo build/test/clippy -p iscc-uniffi` + `mise run check` (no `swift test` on
    Linux). Swift tests structurally validated only — execution needs macOS CI
- Swift CI job (`swift:`) on `macos-14`: `dump-package` (root) → `cargo build -p iscc-uniffi` →
    `swift build` → `swift test`

## JNA Android ARM32 (archived iter 97)

- **JNA Android ARM32 resource path**: JNA canonicalizes ARM32 arch to `arm` (not `armv7`). Correct
    prefix is `android-arm/`, not `android-armv7/`. Verified via bytecode decompilation. Filed as
    spec issue with HUMAN REVIEW REQUESTED.

## C++ Wrapper Review (archived iter 107 — C++ wrapper fully complete)

- C++ wrapper in `packages/cpp/` — header-only, no Rust crate. CMake + INTERFACE library
- Review shortcut: `cargo build -p iscc-ffi` + CMake configure/build/test + ASAN rebuild + clippy +
    `mise run check`
- CI `cpp` job: cmake + ASAN + test on ubuntu-latest
- `iscc.hpp` bundled in FFI release tarballs (flat, alongside `iscc.h`); pkg mgrs vcpkg/conan in
    `packages/cpp/`
- **C++ cmake build**: use `cmake -B build -DFFI_LIB_DIR=../../target/debug` from `packages/cpp/`

## Semver gate review (archived iteration 108, stable)

- **Semver gate review** (iter 93): `semver` job is informational (`continue-on-error: true`)
    pre-1.0 — it adds a gate, doesn't weaken one. Verify locally:
    `cargo semver-checks check-release -p iscc-lib` (~7s; expect "2 major checks failed" = the
    post-0.4.0 `pub(crate)` narrowing, intended). Piping to `tail`/`head` masks the non-zero exit —
    read the "Summary … N checks failed" line.

## Perf gate (iai-callgrind) full verify recipe (archived iter 111 — gate complete & CI-verified)

- VERIFY: `mise run bench:iai` → 16 `.out`, then `python3 scripts/iai_regression.py --check` → 16
    within 10% exit 0 (committed `.iai-baseline.json` is CI-sourced; local 1.96.0 agrees ≤1.66%).
    Script logic test in /tmp: `--update` from synthetic `summary:` dirs, self-check exit 0, tamper
    one baseline −50% → exit 1, missing baseline → exit 1, `.out.old` excluded.
- RACE: `--check` immediately after `cargo bench` exits can match 9/16 mid-flush — re-run; CI runs
    it as a separate step so unaffected.
- FALSE-GREEN EDGES HARDENED (iter 110): `check_regressions(run, baseline, allow_missing=False)`
    FAILS on any shared bench with current Ir 0 (zero-count guard, independent of `--allow-missing`)
    AND on a disappeared baselined bench (unless `--allow-missing` downgrades to warning);
    `only_run` new-bench still warns only.

## Concurrent CID loops (iter 97, resolved iter 98)

Spurious `mise run check` "files were modified by this hook" on a file the advance never touched
(e.g. `standardrb-fix` flagging when NO `.rb` is dirty) + a working-tree `state.md`/context change
appearing mid-review = a SECOND CID loop racing the branch. Confirm with
`ps aux | grep -E 'cid:run|claude -p CID iteration'` (two `mise run cid:run` trees / two different
`iteration N` agents). Flag HUMAN REVIEW REQUESTED, do NOT push, do NOT kill processes yourself.
RESOLUTION: a later review re-checks `ps aux` — once a SINGLE `mise run cid:run` remains, the
duplicate is gone and the unpushed backlog pushes as a fast-forward
(`git rev-list --left-right --count origin/<b>...HEAD` = `0 N`); scan ALL `@{upstream}..HEAD`
commits for gate circumvention before that batch push.
