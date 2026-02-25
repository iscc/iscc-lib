---
icon: lucide/git-pull-request
description: Setting up the development environment, quality gates, and the autonomous CID workflow.
---

# Development

A guide for contributors — setting up the development environment, running the autonomous CID
workflow, understanding quality gates, and using the task runner.

---

## Dev Container Setup

iscc-lib uses a [Dev Container](https://containers.dev/) to provide a fully configured development
environment. All tooling is pre-installed — no manual setup required.

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with Dev Containers support
- [VS Code](https://code.visualstudio.com/) with the
    [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers),
    or any Dev Containers-compatible IDE
- A GitHub personal access token (`GH_TOKEN`) for repo operations
- Claude Code credentials (`~/.claude/.credentials.json`) for the CID loop

### Getting Started

Set environment variables on your host before opening the container:

```bash
export GH_TOKEN="ghp_your_token_here"
```

Open the repository in VS Code and select **Reopen in Container** (or use the Dev Containers CLI).
The `postCreateCommand` runs automatically: trusts the mise config, installs Python dev dependencies
(`uv sync`), and sets up pre-commit hooks.

### Included Tools

| Tool             | Purpose                       |
| ---------------- | ----------------------------- |
| Rust (stable)    | Core library                  |
| Python 3.12 + uv | Python bindings, dev tools    |
| Node.js 20       | Node.js bindings, Claude Code |
| JDK 17 + Maven   | Java JNI bindings             |
| Go 1.24          | Go bindings (wazero)          |
| mise             | Tool versions + task runner   |
| Claude Code      | AI agent loop                 |
| gh               | GitHub CLI                    |

!!! tip "Git identity"

    Git identity is mounted read-only from the host `~/.gitconfig`. If git identity is not inherited
    from the host mount, configure it manually:

    ```bash
    git config user.name "Your Name"
    git config user.email "your@email.com"
    ```

## CID Autonomous Workflow

This project uses **Continuous Iterative Development (CID)** — an autonomous agent loop that
advances the codebase in small, verified increments. Four specialized agents cycle through:

```
update-state → define-next → advance → review (+ push on PASS)
```

| Agent          | Role                                           |
| -------------- | ---------------------------------------------- |
| `update-state` | Assess where the project stands                |
| `define-next`  | Scope one small, verifiable work package       |
| `advance`      | Implement the work package with tests          |
| `review`       | Verify quality, update learnings, push on PASS |

The CID orchestrator (`tools/cid.py`) streams agent progress to the terminal in real-time — you see
tool calls, text output, turn counts, and cost per agent.

### Running the Loop

```bash
mise run cid:run        # Run up to 20 iterations (stops on DONE or failure)
mise run cid:run 5      # Run up to 5 iterations
```

### Single Iteration

```bash
mise run cid:step       # Run one full iteration
```

### Individual Agents

```bash
mise run cid:status     # Show current project state (no agent invocation)
mise run cid:state      # Run update-state agent
mise run cid:next       # Run define-next agent
mise run cid:advance    # Run advance agent
mise run cid:review     # Run review agent (pushes on PASS)
```

### Context Files

The CID agents communicate through files in `.claude/context/`:

| File               | Purpose                                     |
| ------------------ | ------------------------------------------- |
| `state.md`         | Where the project stands right now          |
| `target.md`        | Desired outcome with verification criteria  |
| `next.md`          | Scoped work package for the current step    |
| `handoff.md`       | Inter-agent communication and verdicts      |
| `learnings.md`     | Accumulated knowledge from prior iterations |
| `issues.md`        | Tracked issues and feature requests         |
| `iterations.jsonl` | CID iteration log                           |

## Quality Gates

Two-stage git hook setup via [prek](https://github.com/feltroidprime/prek) (Rust-based drop-in
replacement for `pre-commit`).

### Pre-commit (fast, auto-fix on every commit)

Runs on every `git commit`. Fixes files in place — if changes are made, the commit is aborted so you
can review and re-stage.

- File hygiene (line endings, trailing whitespace, YAML/JSON/TOML validation)
- `cargo fmt` — Rust formatting
- `ruff check --fix` + `ruff format` — Python linting and formatting
- `taplo fmt` — TOML formatting
- `yamlfix` — YAML formatting
- `mdformat` — Markdown formatting

### Pre-push (thorough, run before code leaves the machine)

Runs on every `git push`. Blocks the push if any check fails.

- `cargo clippy` — Rust linting with `-D warnings` (zero warnings policy)
- `cargo test` — full Rust test suite
- `ty check` — Python type checking
- Ruff security scan (`S` rules)
- Ruff complexity check (`C901`)
- `pytest` with 100% coverage enforcement

### Manual Commands

```bash
mise run check          # Run all pre-commit hooks on all files
mise run lint           # Format checks + clippy + ruff
mise run format         # Apply all formatting (pre-commit auto-fix hooks)
mise run test           # Run all tests (Rust + Python)
```

!!! tip "Format before committing"

    Run `mise run format` before `git add` and `git commit`. This applies the same auto-fix hooks that
    fire during commit, preventing the commit from failing due to hook modifications that require
    re-staging.

### Installing Hooks

```bash
uv run prek install --hook-type pre-commit --hook-type pre-push
```

## Project Structure

```
iscc-lib/
├── Cargo.toml                  # Virtual workspace root
├── pyproject.toml              # Python project (uv)
├── zensical.toml               # Documentation config
├── mise.toml                   # Tool versions + tasks
├── .pre-commit-config.yaml     # prek hooks
├── crates/
│   ├── iscc-lib/               # Core Rust library (pure, no FFI)
│   │   ├── src/                # Algorithm implementations
│   │   └── tests/              # Conformance test vectors (data.json)
│   ├── iscc-py/                # Python bindings (maturin + PyO3)
│   │   ├── src/lib.rs          # PyO3 wrappers
│   │   └── python/iscc_lib/    # Python package + type stubs
│   ├── iscc-napi/              # Node.js bindings (napi-rs)
│   │   ├── src/lib.rs          # napi-rs wrappers
│   │   └── __tests__/          # Node.js conformance tests
│   ├── iscc-wasm/              # WebAssembly bindings (wasm-bindgen)
│   │   ├── src/lib.rs          # wasm-bindgen exports
│   │   └── tests/              # WASM integration tests
│   ├── iscc-ffi/               # C FFI bindings
│   │   ├── src/lib.rs          # extern "C" functions
│   │   └── tests/              # C test program
│   └── iscc-jni/               # Java JNI bindings
│       ├── src/lib.rs          # JNI extern "system" functions
│       └── java/               # Java package + Maven build
│           ├── pom.xml
│           └── src/             # IsccLib.java + tests
├── packages/
│   └── go/                     # Go module (pure Go, no cgo)
│       ├── go.mod
│       ├── iscc.go             # wazero WASM bridge
│       ├── iscc_test.go        # Conformance tests
│       └── iscc_ffi.wasm       # Embedded WASM binary
├── docs/                       # Documentation site (lib.iscc.codes)
├── benchmarks/
│   └── python/                 # Comparative Python benchmarks
├── notes/                      # Architecture design documents
├── reference/                  # Reference implementation (gitignored)
│   └── iscc-core/              # Shallow clone of iscc/iscc-core
├── .claude/
│   ├── agents/                 # CID agent definitions
│   └── context/                # CID state files
└── .github/workflows/
    ├── ci.yml                  # Test + lint
    ├── docs.yml                # Documentation deployment
    └── release.yml             # Publish to crates.io, PyPI, npm
```

### Crate Summary

| Crate         | Produces                         | Build Tool     | Published To  |
| ------------- | -------------------------------- | -------------- | ------------- |
| `iscc-lib`    | Rust library                     | cargo          | crates.io     |
| `iscc-py`     | Python wheel                     | maturin + PyO3 | PyPI          |
| `iscc-napi`   | Native Node.js addon             | napi-rs        | npm           |
| `iscc-wasm`   | WASM package                     | wasm-bindgen   | npm           |
| `iscc-ffi`    | Shared library (.so/.dll/.dylib) | cargo          | Source        |
| `iscc-jni`    | JNI shared library               | cargo          | Maven Central |
| `packages/go` | Go module                        | cargo + wazero | pkg.go.dev    |

## Mise Task Runner

All development tasks are defined in `mise.toml` and run via `mise run <task>`.

### Quality Tasks

| Task              | Description                                      |
| ----------------- | ------------------------------------------------ |
| `mise run check`  | Run all pre-commit hooks on all files            |
| `mise run lint`   | Format checks + clippy + ruff                    |
| `mise run format` | Apply all formatting (pre-commit auto-fix hooks) |
| `mise run test`   | Run all tests (Rust + Python)                    |

### CID Tasks

| Task                       | Description                                                            |
| -------------------------- | ---------------------------------------------------------------------- |
| `mise run cid:run [n]`     | Run CID loop up to n iterations (default: 20)                          |
| `mise run cid:step`        | Run one full iteration (update-state → define-next → advance → review) |
| `mise run cid:status`      | Show current project state from state.md                               |
| `mise run cid:stats`       | Show iteration log summary statistics                                  |
| `mise run cid:orchestrate` | CID loop using orchestrator agent (one session per iteration)          |
| `mise run cid:state`       | Run update-state agent only                                            |
| `mise run cid:next`        | Run define-next agent only                                             |
| `mise run cid:advance`     | Run advance agent only                                                 |
| `mise run cid:review`      | Run review agent only                                                  |
