# iscc-lib

High-performance polyglot implementation of
[ISO 24138:2024](https://www.iso.org/standard/77899.html) — International Standard Content Code
(ISCC).

Built in Rust with bindings for Python, Node.js, WASM, and C. Compatible with the official
[iscc-core](https://github.com/iscc/iscc-core) Python reference implementation.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with Dev Containers support
- [VS Code](https://code.visualstudio.com/) with the
  [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers),
  or any Dev Containers-compatible IDE
- A GitHub personal access token (`GH_TOKEN`) for repo operations
- Claude Code credentials (`~/.claude/.credentials.json`)

## Bootstrap

### 1. Set environment variables on your host

```bash
export GH_TOKEN="ghp_your_token_here"
```

### 2. Open in Dev Container

Open the repository in VS Code and select **Reopen in Container** (or use the Dev Containers CLI).
The container provides all development tools pre-installed:

| Tool | Purpose |
|------|---------|
| Rust (stable) | Core library |
| Python 3.12 + uv | Python bindings, dev tools |
| Node.js 20 | Node.js bindings, Claude Code |
| mise | Tool versions + task runner |
| Claude Code | AI agent loop |
| gh | GitHub CLI |

### 3. Install pre-commit hooks

```bash
uv run prek install --hook-type pre-commit --hook-type pre-push
```

### 4. Configure git identity (if not inherited from host)

```bash
git config user.name "Your Name"
git config user.email "your@email.com"
```

## Running the CID Loop

This project uses **Continuous Iterative Development (CID)** — an autonomous agent loop that
advances the codebase in small, verified increments. Four specialized agents cycle through:

```
update-state → define-next → advance → review (+ push on PASS)
```

| Agent | Role |
|-------|------|
| `update-state` | Assess where the project stands |
| `define-next` | Scope one small, verifiable work package |
| `advance` | Implement the work package with tests |
| `review` | Verify quality, update learnings, push on PASS |

### Run one full iteration

```bash
mise run cid:loop
```

### Run individual agents

```bash
mise run cid:state     # Assess current project state
mise run cid:next      # Define next work package
mise run cid:advance   # Implement work package
mise run cid:review    # Review and push if passing
```

### Run multiple iterations

```bash
# Run 5 consecutive CID iterations
for i in $(seq 1 5); do echo "=== Iteration $i ===" && mise run cid:loop; done
```

## Quality Gates

Two-stage git hook setup via [prek](https://github.com/feltroidprime/prek):

**Pre-commit** (fast, auto-fix on every commit):
file hygiene, `cargo fmt`, `ruff check --fix`, `ruff format`, `taplo fmt`, `yamlfix`, `mdformat`

**Pre-push** (thorough, run before code leaves the machine):
`cargo clippy`, `cargo test`, `ty check`, Ruff security scan, Ruff complexity check,
`pytest` with 100% coverage enforcement

### Manual quality checks

```bash
mise run check          # Run all pre-commit hooks
mise run lint           # Format checks + clippy + ruff
mise run format         # Apply all formatting
mise run test           # Run all tests (Rust + Python)
```

## Project Structure

```
.claude/
  agents/               # CID agent definitions
  context/              # CID state files (state.md, target.md, next.md, handoff.md, learnings.md)
.devcontainer/          # Dev container config
notes/                  # Architecture documents (00-09)
```

## License

TBD
