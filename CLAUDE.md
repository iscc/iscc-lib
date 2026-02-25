# CLAUDE.md

Guidance for Claude Code agents working on this repository.

## Project

**iscc-lib** — high-performance polyglot (Rust + language bindings) implementation of ISO 24138:2024
International Standard Content Code (ISCC). Must be compatible with the official `iscc/iscc-core`
Python reference implementation.

## Working Modes

This project uses two development modes that share the same codebase and context files:

- **Interactive session** — a human working with Claude Code directly. Used for ad-hoc tasks:
    debugging, feature requests, code exploration, and targeted changes. You are in this mode if you
    were NOT invoked with `--agent`.
- **CID loop** — autonomous Continuous Iterative Development via `mise run cid:run`. Four
    specialized agents (update-state, define-next, advance, review) cycle through
    `.claude/agents/*.md` protocols. CID agents get their context through `@` references in their
    agent definitions — they do not rely on this section.

### Branching model

- **`main`** — protected branch. Requires PRs with passing CI. Used for releases (tags) and docs
    deployment.
- **`develop`** — CID loop working branch. Agents commit and push here. CI runs on every push.
- **Merge flow:** when `develop` is stable and ready for release, create a PR from `develop` →
    `main` via `mise run pr:main` or manually with `gh pr create -B main -H develop`.
- **Agents are branch-agnostic** — they operate on the current checkout. Run the CID loop with
    `develop` checked out.
- **Release tags** are created on `main` after merging from `develop`.

### Interactive session guidelines

In an interactive session, be aware of the CID context files but do not read them all upfront. Read
selectively based on the task at hand (e.g., read `state.md` to understand project status, read
`target.md` to understand goals).

**Context files and permissions:**

| File                               | Purpose                                     | Interactive access |
| ---------------------------------- | ------------------------------------------- | ------------------ |
| `.claude/context/state.md`         | Where the project stands right now          | Read only          |
| `.claude/context/target.md`        | Desired outcome with verification criteria  | Read and write     |
| `.claude/context/next.md`          | Scoped work package for the current step    | Read only          |
| `.claude/context/handoff.md`       | Inter-agent communication and verdicts      | Read only          |
| `.claude/context/learnings.md`     | Accumulated knowledge from prior iterations | Read and append    |
| `.claude/context/issues.md`        | Tracked issues and feature requests         | Read and append    |
| `.claude/context/iterations.jsonl` | CID iteration log                           | Read only          |
| `.claude/agent-memory/<agent>/`    | Per-agent persistent memory across sessions | Read only          |

**Read only** files are managed by CID agents and overwritten each cycle — interactive edits would
be lost. **Read and write/append** files are safe to modify from interactive sessions.

**Updating the target:** If the human asks to change project goals, acceptance criteria, or
specifications, update `target.md` (or its sub-specs) directly. The next CID iteration picks up
changes automatically.

**Filing issues:** If the human identifies a bug, missing feature, or spec problem that should be
tracked for the CID loop, add it to `issues.md` following the file's format. Use source tag
`[human]`. Include a `**Spec:**` field if the issue is rooted in a target specification gap. Include
an `**Upstream:** iscc/iscc-core` field if the problem belongs in the upstream reference
implementation.

**Filing upstream issues:** When the human asks to file a GitHub issue for an upstream entry in
issues.md, use `gh issue create -R iscc/iscc-core` with the evidence from the issue description.
After filing, note the GitHub issue URL in issues.md, then delete the local entry.

**Adding learnings:** If the interactive session uncovers knowledge that would help CID agents
(e.g., a tricky API behavior, a conformance detail), append it to `learnings.md` under the
appropriate section.

**Concurrency:** A CID loop may be running in the background. Before modifying context files, check
`git log --oneline -5` — if recent commits have `cid(...)` prefixes from the last few minutes, a
loop is likely active. When in doubt, ask the human. Filing issues is always safe (append-only by
design).

## Reference Code

**`reference/iscc-core/`** — shallow clone of the Python reference implementation (gitignored,
auto-cloned by devcontainer). Read source files directly instead of using deepwiki MCP. Key paths:

- `reference/iscc-core/iscc_core/code_meta.py` — `gen_meta_code_v0` and helpers
- `reference/iscc-core/iscc_core/code_content_text.py` — `gen_text_code_v0`
- `reference/iscc-core/iscc_core/code_content_image.py` — `gen_image_code_v0`
- `reference/iscc-core/iscc_core/code_content_audio.py` — `gen_audio_code_v0`
- `reference/iscc-core/iscc_core/code_content_video.py` — `gen_video_code_v0`
- `reference/iscc-core/iscc_core/code_content_mixed.py` — `gen_mixed_code_v0`
- `reference/iscc-core/iscc_core/code_data.py` — `gen_data_code_v0`
- `reference/iscc-core/iscc_core/code_instance.py` — `gen_instance_code_v0`
- `reference/iscc-core/iscc_core/code_iscc.py` — `gen_iscc_code_v0`
- `reference/iscc-core/iscc_core/iscc_code.py` — codec functions

If the directory is missing, clone it:
`git clone --depth 1 https://github.com/iscc/iscc-core.git reference/iscc-core`

**`bio-codes/iscc-sum`** contains production-quality, optimized Rust for Data-Code and Instance-Code
(CDC, MinHash, BLAKE3, streaming). Achieved 50-130x speedup over Python. Use this code as the
foundation — adapt and extend rather than rewriting from Python. Available via deepwiki MCP.

## Architecture

Detailed design is in `notes/00-overview.md` and sub-documents (01-09). Consult these when making
design decisions — don't guess.

| Document                               | When to consult                        |
| -------------------------------------- | -------------------------------------- |
| `notes/00-overview.md`                 | Starting point, key decisions, tooling |
| `notes/01-workspace-structure.md`      | Creating crates, workspace config      |
| `notes/02-language-bindings.md`        | Adding or modifying binding crates     |
| `notes/03-async-and-streaming.md`      | Core API design, streaming processors  |
| `notes/04-api-compatibility-safety.md` | API tiers, error handling, conformance |
| `notes/05-developer-tooling.md`        | Dev environment, quality gates         |
| `notes/06-build-cicd-publishing.md`    | CI config, release workflows           |
| `notes/07-security-versioning.md`      | Security setup, release flow           |
| `notes/09-performance-benchmarks.md`   | Benchmark strategy                     |

## Core Principles

- **Sync core, streaming interface**: Rust core is synchronous. Each binding adapts async
    idiomatically. Pattern: `new() -> update(&[u8]) -> finalize() -> Result<T>`
- **Tiered API surface**: Only `iscc_lib::api` (Tier 1) is bound to foreign languages. Internal
    modules use `pub(crate)` and are never exposed across FFI.
- **Hub-and-spoke crate model**: All binding crates depend on the pure-Rust `iscc-lib` core crate.

## Design Constraints

- `workspace.dependencies` in root `Cargo.toml` for centralized version management
- Python bindings use `abi3-py310` (one wheel per platform)
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`
- No network access in tests (conformance vectors vendored as files)
- OIDC trusted publishing for crates.io and PyPI (no long-lived API keys)
- Cross-platform support required (Linux, macOS, Windows)

## Build Commands

Tool versions and tasks managed by **mise** (`mise.toml`). Python environment uses **uv**.

```bash
mise run test             # Run all tests
mise run lint             # Format checks + clippy + ruff
mise run format           # Apply formatting
mise run check            # Run all pre-commit hooks
cargo test -p iscc-lib    # Rust tests only
pytest                    # Python tests only
mise run cid:run          # Run autonomous CID loop (up to 20 iterations)
mise run cid:step         # Run one CID iteration
mise run cid:status       # Show current project state
```

## Pre-commit Hooks

Git hooks are managed by **prek** (Rust-based drop-in replacement for `pre-commit`), configured in
`.pre-commit-config.yaml`. prek is a Python dev dependency — all hook tools run via `uv run`
(Python) or `cargo` (Rust), both provided by mise.

```bash
uv run prek install --hook-type pre-commit --hook-type pre-push  # Install git hooks
uv run prek run --all-files                                      # Run all hooks manually
```

**Pre-commit stage** (fast, auto-fix on every commit): file hygiene (line endings, trailing
whitespace, YAML/JSON/TOML validation), `cargo fmt`, `ruff check --fix`, `ruff format`, `taplo fmt`,
`yamlfix`, `mdformat`.

**Pre-push stage** (thorough quality gates): `cargo clippy`, `cargo test`, `ty check`, Ruff security
scan (`S` rules), Ruff complexity check (`C901`), `pytest` with coverage enforcement.

**Pre-format before committing:** Run `mise run format` before `git add` and `git commit`. This runs
the same pre-commit auto-fix hooks (formatting, whitespace, line endings) that would fire during
commit. Applying fixes beforehand prevents the commit from failing due to hook modifications, which
would require re-staging.

## Conformance

Official test vectors from `iscc-core/data.json` are the correctness baseline. All 9 `gen_*_v0`
functions must produce output matching `iscc-core` for every vector.

## Command Execution

- Never prefix commands with `cd <project-dir> &&`. The working directory is already the project
    root.
- Run commands directly (e.g., `gh issue list ...`, `uv run pytest`, not
    `cd /path && gh issue list ...`).
- Use absolute paths for files outside the project if needed.
