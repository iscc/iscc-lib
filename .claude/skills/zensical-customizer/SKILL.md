---
name: zensical-customizer
description: >-
  Customize and extend Zensical documentation sites with interactive pages, custom templates,
  JavaScript widgets, and CSS styling. Use when working on documentation site customization
  including: (1) creating custom or interactive doc pages, (2) adding JavaScript widgets or
  third-party libraries to docs, (3) overriding templates or partials, (4) adding/modifying
  CSS styles, (5) creating custom page layouts (landing pages, demos, dashboards),
  (6) configuring zensical.toml settings, (7) integrating external JS frameworks into doc pages.
  Triggers on: docs site, documentation page, interactive demo, custom template, zensical,
  extra_css, extra_javascript, overrides, template block, page layout, widget integration.
  Do NOT use for: writing or editing markdown documentation content, general MkDocs questions
  (this is Zensical-specific), or reading/viewing existing docs pages.
---

# Zensical Customizer

Zensical is the successor to Material for MkDocs, built by the same team. Config file:
`zensical.toml` (TOML format, all settings under `[project]`). Template engine: MiniJinja (Rust).
Dev server: `uv run poe docs-serve`. Build: `uv run poe docs-build`.

Use DeepWiki (`mcp__deepwiki__ask_question` with repo `zensical/zensical`) for questions about
Zensical internals not covered here.

## Decision Tree

1. **Add styles to existing pages** -> Edit `docs/stylesheets/extra.css` or create a new stylesheet
    and register in `extra_css`. See [project-setup.md](references/project-setup.md).

2. **Add JavaScript to all pages** -> Place script in `docs/javascripts/`, register in
    `extra_javascript` in `zensical.toml`. See [javascript.md](references/javascript.md).

3. **Add interactive widget to specific page** -> Add `<div id="mount">` in markdown, create JS that
    uses `document$` to mount on that element. See "Pattern 1" in
    [javascript.md](references/javascript.md).

4. **Create a fully custom page layout** -> Set up `overrides/` directory, create custom template
    extending `base.html`, reference via page frontmatter `template:` field. See
    [templates.md](references/templates.md).

5. **Override header/footer/nav** -> Override partials in `overrides/partials/`. See "Overridable
    Partials" in [templates.md](references/templates.md).

6. **Embed a JS framework app (React, Vue, Svelte)** -> Build externally, output bundle to
    `docs/javascripts/`, mount via `document$` or custom template. See "Pattern 2" and "Pattern 3"
    in [javascript.md](references/javascript.md).

## Quick Start: Add an Interactive Page

### Step 1: Enable template overrides (one-time)

Add to `zensical.toml` under `[project.theme]`:

```toml
custom_dir = "overrides"
```

Create the directory: `overrides/`

### Step 2: Create a custom template

Create `overrides/interactive.html`:

```jinja2
{% extends "base.html" %}

{% block content %}
  {{ page.content }}
  <div id="app-mount"></div>
{% endblock %}

{% block scripts %}
  {{ super() }}
  <script type="module" src="{{ base_url }}/javascripts/my-app.js"></script>
{% endblock %}
```

### Step 3: Create the page

Create a markdown file (e.g., `docs/demos/my-demo.md`):

```markdown
---
template: interactive.html
icon: lucide/play
---

# Interactive Demo

Description of the demo.
```

### Step 4: Add JavaScript

Place `docs/javascripts/my-app.js` with initialization logic.

### Step 5: Register in navigation

Add to `nav` in `zensical.toml`:

```text
{ "Demos" = ["demos/my-demo.md"] },
```

## Key Constraints

- **No full plugin/module system** - extensibility is limited to templates, JS, CSS
- **MiniJinja, not Jinja2** - mostly compatible, but complex Jinja2 patterns may differ
- **125% root font-size** - Zensical sets `html { font-size: 125% }` (20px); third-party libraries
    with rem-based sizing render 25% larger. Fix with explicit pixel values.
- **Markdown extensions are all-or-nothing** - defining any custom extensions requires configuring
    ALL extensions (defaults don't apply partially)
- **`navigation.instant` not enabled** in this project - `window.load` works for site-wide scripts,
    but page-specific widgets should still use `document$` for forward compatibility

## References

- **[templates.md](references/templates.md)** - Template blocks, custom page templates, partials,
    context variables, MiniJinja syntax
- **[javascript.md](references/javascript.md)** - JS/CSS injection, document$ observable, instant
    navigation, interactive component patterns
- **[project-setup.md](references/project-setup.md)** - This project's directory structure, existing
    customizations, ISCC brand colors, Zensical quirks
