# Developer Tooling

Task runners, tool version management, linting, quality gates, documentation, and pre-commit hooks.

## Task Runners

| Tool           | Used By          | Pros                                 | Cons                            |
| -------------- | ---------------- | ------------------------------------ | ------------------------------- |
| **Make**       | minijinja        | Universal, zero install              | No Windows support, limited DAG |
| **go-task**    | kreuzberg        | YAML-based, includes, cross-platform | Requires Go or binary install   |
| **mise tasks** | (recommended)    | Integrated with tool manager, TOML   | Newer ecosystem                 |
| **just**       | (common in Rust) | Make-like but cross-platform, simple | No dependency DAG               |

**minijinja uses Make** with straightforward targets:

```makefile
build:
	cargo build

test:
	cargo test

format:
	cargo fmt --all

lint:
	cargo clippy --all -- -F clippy::dbg-macro -D warnings
```

**kreuzberg uses go-task** with modular includes:

```yaml
# Taskfile.yml
version: '3'
includes:
  rust: .task/languages/rust.yml
  python: .task/languages/python.yml
  node: .task/languages/node.yml
  wasm: .task/languages/wasm.yml

tasks:
  setup:
    desc: Install all development dependencies
    cmds:
      - task: rust:install
      - task: python:install
      - task: node:install

  test:
    desc: Run all tests
    deps: [rust:test, python:test, node:test, wasm:test]

  build:
    desc: Build all targets
    deps: [rust:build, python:build, node:build, wasm:build]
```

## mise — Recommended Tool Version Manager

Neither minijinja nor kreuzberg uses [mise](https://mise.jdx.dev/) (minijinja uses manual rustup,
kreuzberg uses sdkman). However, mise is the strongest option for a new polyglot project because it
unifies tool version management, environment variables, and task running in a single tool —
replacing sdkman, nvm, pyenv, and the task runner in one go.

**Why mise over alternatives:**

| Tool            | Scope                | Cross-platform    | Speed        | Config         |
| --------------- | -------------------- | ----------------- | ------------ | -------------- |
| **mise**        | Tools + env + tasks  | Yes (Rust binary) | Fast         | TOML           |
| sdkman          | JVM-focused tools    | Unix only         | Medium       | .sdkmanrc      |
| asdf            | Tools                | Unix only         | Slow (shell) | .tool-versions |
| nvm/pyenv/rbenv | Single language each | Varies            | Medium       | Various        |

**Example `mise.toml` for a polyglot Rust project:**

```toml
[tools]
rust = "stable"
python = "3.12"
node = "22"
prek = "latest"
"cargo:cargo-dist" = "latest"
"cargo:cargo-release" = "latest"
"npm:@napi-rs/cli" = "latest"
"pipx:maturin" = "latest"

[env]
RUST_BACKTRACE = "1"

[tasks.setup]
description = "Install all development dependencies"
run = """
uv sync
pnpm install
"""

[tasks.build]
description = "Build all targets"
depends = ["build:rust", "build:python", "build:node", "build:wasm"]

[tasks."build:rust"]
description = "Build Rust workspace"
run = "cargo build --workspace"

[tasks."build:python"]
description = "Build Python wheel (dev)"
run = "uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml"

[tasks."build:node"]
description = "Build Node.js native addon"
run = "pnpm --filter @iscc/node build"

[tasks."build:wasm"]
description = "Build WASM package"
run = "wasm-pack build crates/iscc-wasm --target bundler"

[tasks.test]
description = "Run all tests"
depends = ["test:rust", "test:python", "test:node"]

[tasks."test:rust"]
run = "cargo test --workspace"

[tasks."test:python"]
run = "uv run pytest crates/iscc-py/tests"

[tasks."test:node"]
run = "pnpm --filter @iscc/node test"

[tasks.lint]
description = "Run all linters"
run = """
cargo fmt --all --check
cargo clippy --all -- -D warnings
uv run ruff check
uv run ty check
"""

[tasks.format]
description = "Format all code"
run = """
cargo fmt --all
uv run ruff format
uv run mdformat docs/
pnpm --filter @iscc/node exec biome format --write .
"""

[tasks."docs:serve"]
description = "Serve documentation locally"
run = "uv run zensical serve"

[tasks."docs:build"]
description = "Build documentation"
run = "uv run zensical build"
```

**CI integration** — mise provides a GitHub Action:

```yaml
  - uses: jdx/mise-action@v2
    with:
      install: true
```

This installs all tools defined in `mise.toml` with a single step, replacing multiple
`actions/setup-*` steps.

## Linting and Formatting

**minijinja** — minimal, Rust-focused:

- `cargo fmt` (rustfmt)
- `cargo clippy`
- `pyright` for Python type checking

**kreuzberg** — comprehensive, per-language via pre-commit:

| Language       | Formatter     | Linter               |
| -------------- | ------------- | -------------------- |
| Rust           | cargo fmt     | clippy + cargo-deny  |
| Python         | ruff format   | ruff check + mypy    |
| TypeScript     | biome         | biome + tsc --noEmit |
| Go             | go fmt        | golangci-lint        |
| Shell          | shfmt         | shellcheck           |
| TOML           | taplo         | taplo                |
| GitHub Actions | —             | actionlint           |
| Ruby           | rubocop       | rubocop              |
| Java           | spotless      | checkstyle + pmd     |
| C#             | dotnet format | dotnet format        |

**Recommended for iscc-lib**: Start with cargo fmt + clippy + ruff (Python) + ty (Python type
checking) + biome (TypeScript). Use prek to run them consistently.

> **Note**: iscc-sum uses mypy for type checking and bandit for security scanning — both should
> carry forward to iscc-lib. For iscc-lib, prefer **ty** (astral-sh/ty) over mypy — it's 10-100x
> faster, written in Rust, and integrates naturally with the ruff/uv toolchain from Astral.

## Rust Quality Gates (high leverage)

Consider adding a small set of "always on" Rust checks as the project grows:

- `cargo test --workspace` (and optionally `cargo nextest` for speed)
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --all --check`
- `cargo deny check` (licenses/advisories/sources) and/or `cargo audit` (RustSec advisories)
- `cargo semver-checks` (lint public API for unintentional breaking changes — especially valuable
    when the core crate's API stability affects all downstream bindings)

## Complexity Gates (enforce simplicity)

Maximum cognitive complexity is **15** (Radon grade C) across all languages. Functions above this
threshold must be refactored before merging.

**Rust:**

- `cargo clippy` uses the `cognitive_complexity` lint. Threshold is set in `clippy.toml` at the
    workspace root:
    ```toml
    # clippy.toml
    cognitive-complexity-threshold = 15
    ```

**Python:**

- Ruff enforces complexity via the `C901` rule. Threshold is set in `pyproject.toml`:
    ```toml
    [tool.ruff.lint.mccabe]
    max-complexity = 15
    ```

**TypeScript/JavaScript:**

- biome includes a `noExcessiveCognitiveComplexity` rule:
    ```json
    {
      "linter": {
        "rules": {
          "complexity": {
            "noExcessiveCognitiveComplexity": {
              "level": "error",
              "options": {
                "maxAllowedComplexity": 15
              }
            }
          }
        }
      }
    }
    ```

**CI enforcement**: All complexity checks run in CI as blocking gates. No exceptions without
explicit review approval and a tracking issue for refactoring.

## Documentation (zensical)

Use **zensical** for project documentation, published via GitHub Pages at `lib.iscc.codes`.

Zensical is a Rust-powered static site generator from the Material for MkDocs team — a drop-in
replacement that's significantly faster for builds and live reload. It reads `zensical.toml` (or
`mkdocs.yml` for migration) and supports the full Material for MkDocs extension ecosystem.

**Project structure:**

```
iscc-lib/
├── zensical.toml                # Documentation config
├── overrides/
│   └── main.html                # Template overrides (OG/Twitter meta tags)
└── docs/
    ├── index.md
    ├── stylesheets/
    │   └── extra.css            # ISCC brand colors
    ├── assets/
    │   ├── favicon.png
    │   ├── logo_light.png
    │   └── logo_dark.png
    ├── tutorials/
    ├── how-to/
    ├── explanation/
    ├── reference/
    │   └── api.md               # Auto-generated from docstrings (mkdocstrings)
    └── development/
        └── contributing.md
```

**Key configuration (from iscc-usearch template):**

- Dual-palette theme (light/dark) with ISCC brand colors
- mkdocstrings with Python handler for API reference
- Custom CSS for header/footer branding
- OG/Twitter meta tag overrides for social sharing

**CI workflow** (`.github/workflows/docs.yml`):

```yaml
on:
  push:
    branches: [main]

jobs:
  docs:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pages: write
      id-token: write
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v5
      - uses: actions/setup-python@v5
        with: {python-version: '3.12'}
      - run: uv sync --group dev
      - run: uv run zensical build
      - uses: actions/upload-pages-artifact@v3
        with: {path: site}
      - uses: actions/deploy-pages@v4
```

**Markdown formatting**: Use `mdformat` with the `mdformat-mkdocs` plugin in prek hooks for
consistent Markdown style across documentation and README files.

## Pre-Commit Hooks (prek)

Use **prek** (j178/prek) as the pre-commit framework. It's a Rust-based drop-in replacement for
`pre-commit` that's ~10x faster on cold install, uses 50% less disk, and ships as a single binary
with no Python dependency.

**Example `.pre-commit-config.yaml`:**

```yaml
repos:
  - repo: builtin
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.11.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  - repo: https://github.com/KyleKing/mdformat-mkdocs
    rev: v4.1.0
    hooks:
      - id: mdformat
        additional_dependencies:
          - mdformat-mkdocs
          - mdformat-footnote
          - mdformat-tables
          - mdformat-gfm-alerts

  - repo: https://github.com/astral-sh/ty
    rev: v0.0.4
    hooks:
      - id: ty
```

**CI integration** (GitHub Actions):

```yaml
  - uses: j178/prek-action@v1
```

**Install** (via mise): `prek = "latest"` in `mise.toml`, or standalone via `cargo binstall prek` /
`uv tool install prek`.

## Python-Specific Tooling

Both projects use:

- **uv** — Fast Python package manager (replaces pip/virtualenv)
- **maturin** — Build backend for PyO3 extensions
- **ruff** — Linter and formatter (replaces black + flake8 + isort)
- **ty** — Fast Rust-based type checker from Astral (replaces mypy/pyright, 10-100x faster)

Development workflow:

```bash
# Create venv and install deps
uv sync

# Build and install the extension in development mode
uv run maturin develop

# Run tests
uv run pytest

# Type check
uv run ty check
```

## Node.js-Specific Tooling

kreuzberg uses:

- **pnpm** — Fast, disk-efficient package manager
- **napi-rs** + `@napi-rs/cli` — Native addon build system
- **biome** — Formatter and linter (replaces eslint + prettier)
