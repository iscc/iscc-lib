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

`dotnet`, `mvn`, `gcc`, `java`, `go`, `ruby`, `node`, `wasm-pack`, `rustc` are **present**. `cmake`,
`swift`, `gradle` are **ABSENT** → `packages/{cpp,swift}` can only ever be CI-verified from here,
and that is axis 3 of the propagation cost ranking in [[MEMORY]]. **Kotlin IS locally testable** via
`./gradlew … --offline` (the cache is warm) despite `gradle` not being on `PATH` — the wrapper
works. Always probe with `command -v` before claiming a surface is unbuildable. Other env details →
`MEMORY-archive.md`.

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
