---
name: env-gotchas
description: Container/tooling gotchas that silently produce wrong answers during state assessment — absent toolchains, invisible exec bits, case-sensitive binding APIs, build-script side effects
metadata:
  type: project
---

# Environment gotchas (update-state)

Split out of [[MEMORY]] to keep the index under budget. These are the traps that make an exploration
command return a *confidently wrong* answer rather than an error.

## Toolchains present / absent in this container

`dotnet`, `mvn`, `gcc`, `g++`, `java`, `go`, `ruby`, `node`, `wasm-pack`, `rustc` are **present**.
`cmake`, `swift`, `gradle`, `ninja` are absent from `$PATH` — but **absent from `$PATH` ≠
unbuildable**, and that mistake shaped scoping for ~6 iterations:

- **Kotlin IS locally testable** via `./gradlew … --offline` (warm cache) despite no `gradle`.
- **MSRV claims are locally verifiable** (found by `review` at 172): `rustup` here has the **1.85.0
    toolchain installed**, so `cargo +1.85.0 check -p <crate> --locked` settles any "does it still
    build on the declared MSRV" question without a CI job. Always scope it `-p iscc-lib` (the only
    published crate), never `--workspace` — `iscc-uniffi` is `publish = false` and now floors at
    1.91.
- **C++ IS locally testable, two ways** (established 160-161): `uv run --with cmake cmake …` pulls
    the PyPI cmake wheel (4.4.0) and runs the full ASAN suite; *or* skip cmake entirely and compile
    `packages/cpp/tests/test_iscc.cpp` with plain `g++ -std=c++17 -Wall -Wextra -Wpedantic` plus
    `-I packages/cpp/include -I crates/iscc-ffi/include -I crates/iscc-ffi/tests`,
    `-L target/debug -liscc_ffi -lpthread -ldl -lm`, then run under `LD_LIBRARY_PATH=target/debug` →
    `69 passed, 0 failed`. The g++ route proves the *test code*; only the cmake route proves the
    `target_include_directories` wiring. Configure cmake into a **fresh** `build-*/` —
    `packages/cpp/build*/` holds 5 stale gitignored dirs, one with an incompatible cmake 3.25 cache.
- **Swift IS locally testable — the "genuinely absent" claim was REFUTED at 161.** No `swift` on
    `$PATH` and no PyPI substitute (`uv --with swift` installs an unrelated OpenStack package), but
    a hand-fetched toolchain lives at `/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin` and
    survives across iterations (verified present at 162). Run with an explicit
    `--scratch-path /tmp/…` so nothing lands in the tree; `.build/` is gitignored. If `/tmp` is
    cleared, the re-fetch recipe (784 MB, ~5 min, no `sudo`) is in `packages/swift/CLAUDE.md`.
    Nothing was added to the image, `mise.toml`, the devcontainer or CI.

**No surface is CI-proof-only any more.** Always probe with `command -v` **and** consider a
PyPI/wrapper fallback or a `/tmp` toolchain before calling one unbuildable — that mistake shaped
scoping for ~6 iterations. Other env details → `MEMORY-archive.md`.

## Commands that lie

- **Exec-bit changes are INVISIBLE** (`core.fileMode=false`): a mode-only commit shows nothing in
    `git status` / `git diff`. Use `git ls-files -s <path>` and read the mode digits.
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Grep binding APIs CASE-INSENSITIVELY** (learned 152): napi exports are **snake_case**,
    Java/Kotlin/Swift camelCase, C# PascalCase. A case-sensitive grep for `textClean` misses four
    surfaces. The napi test dir is `crates/iscc-napi/__tests__` (two leading underscores).
- **`git ls-files 'docs/**/*.md'` returns 13, NOT 24** — it misses top-level `docs/*.md`. Use
    `uv run scripts/check_docs_nav.py` for the authoritative page count.

## Build-time side effects

- **csbindgen runs on every `cargo build`** (`crates/iscc-ffi/build.rs`), regenerating the C#
    P/Invoke bindings — so a "modified" C# binding file can be a build artifact, not an edit.
- **UniFFI is a proc-macro** (no separate codegen step to look for).
- **Kotlin uses JNA** — running its suite needs BOTH `java.library.path` AND `jna.library.path`.
- **Release-workflow internals** (guards, artifact flow, registry toggles) → `MEMORY-archive.md`.

## Cheap probe recipes

- **Verify a `packages/go` claim in ~60s**: create a `/tmp` module requiring
    `github.com/iscc/iscc-lib/packages/go v0.0.0` plus a `replace` pointing at the repo, then
    `GOFLAGS=-mod=mod CGO_ENABLED=0 go mod tidy && CGO_ENABLED=0 go run main.go`.
