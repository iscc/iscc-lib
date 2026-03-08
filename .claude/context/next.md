# Next Work Package

## Step: Add language logos to README

## Goal

Add programming language logo icons to the README's Installation and Quick Start section headers,
resolving the "Add programming language logos to README and docs" normal-priority issue. Visual
language indicators help developers quickly identify and navigate to their language's section.

## Scope

- **Create**: (none — use CDN-hosted logos, no vendored files)
- **Modify**: `README.md`
- **Reference**: `docs/index.md` (for context on docs site — NOT modified this step)

## Not In Scope

- Updating the docs site (`docs/index.md`, howto guides) with logos — that's a follow-up step if
    desired after reviewing how the README logos look
- Fixing the stale "Available Bindings" table in `docs/index.md` (missing Ruby, C#/.NET, C++)
- Adding a visual language grid/banner at the top of the README — keep changes minimal
- Vendoring logo SVG/PNG files into the repo — use a CDN instead for zero maintenance

## Implementation Notes

**Approach:** Add small inline `<img>` tags to each `### Language` header in both the Installation
and Quick Start sections. Use [Simple Icons](https://simpleicons.org/) via their CDN
(`https://cdn.simpleicons.org/{slug}/{color}`) for clean, consistent SVG logos.

**Language → Simple Icons mapping (9 languages):**

| Section Header | Simple Icons slug | Suggested color |
| -------------- | ----------------- | --------------- |
| Rust           | `rust`            | `000000`        |
| Python         | `python`          | `3776AB`        |
| Node.js        | `nodedotjs`       | `5FA04E`        |
| Java           | (see below)       | `ED8B00`        |
| Go             | `go`              | `00ADD8`        |
| Ruby           | `ruby`            | `CC342D`        |
| C# / .NET      | `dotnet`          | `512BD4`        |
| C / C++        | `cplusplus`       | `00599C`        |
| WASM           | `webassembly`     | `654FF0`        |

**Java logo:** Simple Icons has `openjdk` (slug: `openjdk`). Use that or the generic coffee cup
icon. Check `https://cdn.simpleicons.org/openjdk/ED8B00`.

**Header format pattern:**

```markdown
### <img src="https://cdn.simpleicons.org/rust/000000" width="20" height="20" alt="Rust"> Rust
```

Use `width="20" height="20"` for consistent sizing. Include `alt` text for accessibility.

**Dark mode consideration:** Simple Icons SVGs are single-color. The colors chosen should be
readable on both white and dark backgrounds. GitHub renders README in both light and dark mode.
Consider using the official brand colors as listed above — they generally have sufficient contrast.
If a color is too light for dark mode, the advance agent may use the `<picture>` element approach
with `prefers-color-scheme` media queries, but this adds complexity. Start with simple `<img>` tags
and only add dark mode variants if colors genuinely don't work.

**Sections to update (18 headers total):**

1. Installation section: 9 headers (Rust, Python, Node.js, Java, Go, Ruby, C# / .NET, C / C++, WASM)
2. Quick Start section: 9 headers (same languages)

## Verification

- `grep -c '<img src=.*simpleicons.*width=' README.md` returns `18` (9 Installation + 9 Quick Start
    headers)
- `grep -c 'alt="' README.md` returns at least `18` (accessibility alt text on each logo)
- All 9 language slugs appear:
    `grep -cP '(rust|python|nodedotjs|openjdk|go/|ruby|dotnet|cplusplus|webassembly)' README.md`
    returns at least `18`
- `mise run format` exits 0 (README passes formatting checks)
- `mise run check` exits 0 (all pre-commit hooks pass)

## Done When

All verification criteria pass — every Installation and Quick Start language header has an inline
logo image from Simple Icons CDN with consistent sizing and alt text.
