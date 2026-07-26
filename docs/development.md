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
| Go 1.24          | Go bindings                   |
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

| Agent          | Role                                                                                 |
| -------------- | ------------------------------------------------------------------------------------ |
| `update-state` | Assess where the project stands                                                      |
| `define-next`  | Scope one small, verifiable work package                                             |
| `advance`      | Implement the work package with tests                                                |
| `review`       | Verify quality, update learnings, push on PASS                                       |
| `meta-improve` | Improve the loop itself (only on IDLE / on-demand, under runner-enforced guardrails) |

A fifth role, `meta-improve`, runs only when the loop reaches IDLE or via `mise run cid:improve`. It
may auto-apply at most one whitelisted low-risk change to the loop per cycle (prose/model/effort of
the non-gate agents, or `learnings.md` hygiene); `tools/cid.py` hard-reverts anything outside that
whitelist and rolls back changes whose metrics regress. Everything else it writes to
`.claude/context/proposals.md` for the human.

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
- `ruff check --fix` + `ruff format` — Python linting and formatting (includes the security `S`,
    complexity `C901`, import-sorting `I`, sorted-`__all__` `RUF022` and unused-`# noqa` `RUF100`
    rules via `extend-select` in `pyproject.toml` — imports are auto-sorted at commit time); both
    hooks also cover `.pyi` type stubs, and `ruff format` additionally covers Python code blocks
    inside Markdown, matching the `ruff format --check` step in CI
- `taplo fmt` — TOML formatting
- `yamlfix` — YAML formatting
- `mdformat` — Markdown formatting
- `scripts/check_release_workflow.py` — static checks for `.github/workflows/release.yml` (guard
    shape, artifact wiring, `needs:` graph); the release workflow is `workflow_dispatch`-only, so
    these invariants are never exercised by CI runs. Also enforced in CI via
    `tests/test_check_release_workflow.py`. A fourth check, `--check-action-inputs`, fetches each
    action's published `action.yml` over the network and validates `with:` keys (docker actions
    additionally accept the GitHub-native `args`/`entrypoint` overrides), required inputs without a
    default, and `steps.<id>.outputs.<x>` reads; it runs only in the dedicated `release-workflow` CI
    job (prek and pytest stay network-free). Transport and parse failures degrade to a stderr
    warning instead of a failure, and the check prints an
    `action-inputs: resolved <R> of <T> action refs` summary line so an all-skipped run is visible
    in the job log
- `scripts/check_docs_nav.py` — network-free parity check asserting that the `docs/**/*.md` pages on
    disk (minus `includes/` partials), the `nav` in `zensical.toml`, `ORDERED_PAGES` in
    `scripts/gen_llms_full.py`, and the links in `docs/llms.txt` are the same set. Nav parsing is
    comment-aware: a commented-out nav entry counts as missing, while a `#` inside a quoted title
    (`"C# / .NET"`) is kept. Adding a docs page means updating all three lists together. Also
    enforced in CI via `tests/test_check_docs_nav.py`, whose anchor test also catches page deletions
    that the prek hook's `files:` matching cannot see

### Pre-push (thorough, run before code leaves the machine)

Runs on every `git push`. Blocks the push if any check fails.

- `cargo clippy` — Rust linting with `-D warnings` (zero warnings policy)
- `cargo test` — full Rust test suite
- `ty check` — Python type checking
- Ruff security scan (`S` rules) — focused re-run of rules already enforced by `ruff check`
- Ruff complexity check (`C901`) — focused re-run of rules already enforced by `ruff check`
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
│       ├── codec.go            # ISCC codec, constants, types
│       ├── code_meta.go        # GenMetaCodeV0
│       ├── code_content_*.go   # GenText/Image/Audio/Video/MixedCodeV0
│       ├── code_data.go        # GenDataCodeV0 + DataHasher
│       ├── code_instance.go    # GenInstanceCodeV0 + InstanceHasher
│       ├── code_iscc.go        # GenIsccCodeV0
│       ├── conformance.go      # ConformanceSelftest
│       └── *_test.go           # Conformance tests
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
    └── release.yml             # Publish to crates.io, PyPI, npm, Maven Central
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
| `packages/go` | Go module                        | go             | pkg.go.dev    |

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

| Task                   | Description                                                            |
| ---------------------- | ---------------------------------------------------------------------- |
| `mise run cid:run [n]` | Run CID loop up to n iterations (default: 20)                          |
| `mise run cid:step`    | Run one full iteration (update-state → define-next → advance → review) |
| `mise run cid:status`  | Show current project state from state.md                               |
| `mise run cid:stats`   | Show iteration log summary statistics                                  |
| `mise run cid:improve` | Run the meta-improve self-improvement role once                        |
| `mise run cid:state`   | Run update-state agent only                                            |
| `mise run cid:next`    | Run define-next agent only                                             |
| `mise run cid:advance` | Run advance agent only                                                 |
| `mise run cid:review`  | Run review agent only                                                  |

### Version and Release Tasks

| Task                     | Description                                             |
| ------------------------ | ------------------------------------------------------- |
| `mise run version:sync`  | Sync all manifest versions with workspace `Cargo.toml`  |
| `mise run version:check` | Validate version consistency (used in CI)               |
| `mise run test:install`  | Test published packages are installable from registries |
| `mise run pr:main`       | Create a PR from `develop` to `main`                    |

## Releasing

All packages share a single version defined in the root `Cargo.toml` workspace. The
`version_sync.py` script propagates it to all language-specific manifests and documentation. CI
enforces consistency on every push.

### Version Bump

```bash
# 1. Edit the canonical version in Cargo.toml (line 13)
#    version = "X.Y.Z"

# 2. Propagate to all manifests and docs
mise run version:sync

# 3. Update Cargo.lock
cargo update -w

# 4. Format, stage, and commit
mise run format
git add -A
git commit -m "Release X.Y.Z"
```

`version:sync` updates these files automatically:

| File                            | What it updates                    |
| ------------------------------- | ---------------------------------- |
| `pyproject.toml`                | Root project version               |
| `crates/iscc-napi/package.json` | npm package version                |
| `crates/iscc-jni/java/pom.xml`  | Maven artifact version             |
| `mise.toml`                     | Default `--version` flag           |
| `scripts/test_install.py`       | Registry check fallback version    |
| `README.md`                     | Maven dependency snippet           |
| `crates/iscc-jni/README.md`     | Maven dependency snippet           |
| `docs/howto/java.md`            | Maven dependency snippet           |
| `docs/java-api.md`              | Maven + Gradle dependency snippets |

Cargo workspace members (`Cargo.lock`) inherit the version automatically via
`version.workspace = true`.

### Publish

```bash
# 1. Merge develop into main
mise run pr:main          # Create PR
# Merge PR after CI passes

# 2. Tag the release on main
git checkout main && git pull
git tag vX.Y.Z
git push origin vX.Y.Z
```

The tag push triggers `.github/workflows/release.yml`, which publishes to all registries in
parallel:

| Registry      | Package            | Auth Method                        |
| ------------- | ------------------ | ---------------------------------- |
| crates.io     | `iscc-lib`         | OIDC trusted publishing            |
| PyPI          | `iscc-lib`         | OIDC trusted publishing            |
| npm           | `@iscc/lib`        | `NPM_TOKEN` secret                 |
| npm           | `@iscc/wasm`       | `NPM_TOKEN` secret                 |
| Maven Central | `io.iscc:iscc-lib` | GPG signing + Sonatype credentials |

Each publish job checks if the version already exists on the registry and skips gracefully.

You can also publish to individual registries via `workflow_dispatch` on the
[Release workflow](https://github.com/iscc/iscc-lib/actions/workflows/release.yml) page (select the
checkboxes for crates.io, PyPI, npm, or Maven).

### Post-Release Verification

```bash
mise run test:install --version X.Y.Z
```

This installs each package from its registry in a clean environment and runs a smoke test.
