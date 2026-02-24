---
name: docs-for-agents
description: >-
  Create and maintain a "For Coding Agents" reference page in project documentation.
  Generates dense, prescriptive documentation optimized for AI coding agents — architecture
  maps, decision dispatch tables, constraints catalogs, side effects matrices, task recipes,
  change playbooks, and common mistakes. Use when asked to "create agent docs", "update agent
  reference", "add docs for agents", "write coding agent documentation", or "update the
  for-coding-agents page". Also triggers on "agent-oriented docs", "AI-friendly reference".
  Do NOT use for general documentation, API reference generation, or user-facing tutorials.
user-invocable: true
argument-hint: create | update
---

# Docs for Agents

Generate or update a "For Coding Agents" reference page — a compressed, prescriptive reference
optimized for AI coding agents working on or integrating with the project.

## Principles

- **Dense and declarative.** Tables and code blocks over prose.
- **Action-oriented.** Constraint catalogs, decision dispatch, task recipes, change playbooks.
- **Consistent terminology.** Match the codebase exactly — no synonyms.
- **Self-contained.** An agent should not need to read other docs to act correctly.
- **Cross-referenced.** Link to existing docs where agents need deeper context.

## Workflow

### 1. Analyze the codebase

Read all source files to extract:

- File layout and what each file contains
- Class/module hierarchy
- Import dependency graph
- Public API surface (`__all__`, exports, entry points)
- Constructor constraints and validation rules
- Method signatures, side effects, and state mutations
- Concurrency model and thread-safety rules
- Persistence behavior (what writes to disk, what resets state)
- Key invariants that are not obvious from type signatures

### 2. Check existing documentation infrastructure

Identify:

- Documentation framework and config (e.g., `mkdocs.yml`, `zensical.toml`, `docs/` layout)
- Navigation structure — where to add the new page
- LLM doc generation scripts (e.g., `gen_llms_full.py`, `llms.txt` generators)
- Existing pages to cross-reference

### 3. Write the reference page

Create the page with these sections (omit any that don't apply to the project):

#### Architecture map

- **File layout** — table: file path → what it contains
- **Class/module hierarchy** — text tree showing inheritance/composition
- **Import dependency flow** — A → B → C diagram
- **Public API exports** — what's in `__all__` or equivalent

#### Decision dispatch

- Tables mapping use-case → class/function/method
- One table per decision axis (e.g., "which class?", "which method?", "which config?")

#### Constraints and invariants

- Concurrency rules
- Input constraints (types, ranges, alignment, dtypes)
- Key/ID constraints
- State machine semantics (counters, flags, lifecycle)
- Resource management rules

#### Side effects catalog

- Table: method → disk writes, state mutations, cache updates, counter changes
- Cover all public methods that mutate state

#### Task recipes

- Step-by-step code examples for common integration tasks
- Patterns for typical workflows

#### Change playbook

- "If modifying X → also update Y, Z" checklists
- Cover every module/subsystem where a change has non-obvious ripple effects

#### Common mistakes

- NEVER / ALWAYS format
- Concrete negative examples with corrections
- Each mistake separated by a horizontal rule for scannability

### 4. Register the page

- Add to documentation navigation config
- Add to LLM doc generation scripts if they exist
- Verify build succeeds

### 5. Verify

- Build the docs site — confirm the page renders and nav link works
- Run any LLM doc generators — confirm the page is included in output
- Review for accuracy against the source code

## Updating an existing page

When updating rather than creating:

1. Read the current for-coding-agents page
2. Read all source files to identify what changed since the page was written
3. Update affected sections — do not rewrite sections that are still accurate
4. Verify the build still passes

## File naming

Use `for-coding-agents.md` as the filename to avoid conflicts with the `AGENTS.md` convention (which
serves a different purpose at the repo root).
