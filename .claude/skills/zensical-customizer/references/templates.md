# Zensical Template System Reference

## Template Override Setup

Set `custom_dir` in `zensical.toml`:

```toml
[project.theme]
custom_dir = "overrides"
```

Place override files in `overrides/` mirroring theme structure. Files here take precedence over
built-in templates of the same name.

## Available Template Blocks

Override blocks in `overrides/main.html`:

```jinja2
{% extends "base.html" %}

{% block <block_name> %}
  <!-- Replace block content -->
{% endblock %}
```

Use `{{ super() }}` to preserve original content and append/prepend:

```jinja2
{% block scripts %}
  {{ super() }}
  <script src="my-widget.js"></script>
{% endblock %}
```

| Block       | Location               | Purpose                             |
| ----------- | ---------------------- | ----------------------------------- |
| `analytics` | Closing `</body>`      | Analytics integration               |
| `announce`  | Header region          | Announcement bar                    |
| `config`    | `<head>`               | JavaScript app config               |
| `container` | Below header           | Main content container              |
| `content`   | Inside container       | Primary page content area           |
| `extrahead` | `<head>`               | Custom meta tags, link tags         |
| `fonts`     | `<head>`               | Font definitions                    |
| `footer`    | Bottom                 | Footer navigation and copyright     |
| `header`    | Top                    | Fixed header bar                    |
| `hero`      | Between header/content | Hero teaser section (if used)       |
| `htmltitle` | `<head>`               | `<title>` tag content               |
| `libs`      | `<head>`               | JavaScript libraries (loaded early) |
| `outdated`  | Header region          | Version warning banner              |
| `scripts`   | Closing `</body>`      | JavaScript app code (loaded late)   |
| `site_meta` | `<head>`               | Document meta tags                  |
| `site_nav`  | Side regions           | Navigation sidebar and TOC          |
| `styles`    | `<head>`               | Stylesheets                         |
| `tabs`      | Below header           | Tabs navigation (if enabled)        |

## Custom Page Templates

Assign a unique template to a specific page via frontmatter:

```yaml
---
template: custom_landing.html
---
```

Place `custom_landing.html` in `overrides/`. It can extend `base.html` or start from scratch.

### Extending base.html for a custom layout

```jinja2
{% extends "base.html" %}

{% block container %}
  <div class="custom-hero">
    <h1>{{ page.title }}</h1>
  </div>
  <div class="interactive-demo" id="app-mount"></div>
  {{ page.content }}
{% endblock %}

{% block scripts %}
  {{ super() }}
  <script type="module" src="{{ base_url }}/javascripts/my-app.js"></script>
{% endblock %}
```

### Standalone template (no theme chrome)

```jinja2
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>{{ page.title }} - {{ config.site_name }}</title>
  {% for css in extra_css %}
  <link rel="stylesheet" href="{{ css }}">
  {% endfor %}
</head>
<body>
  {{ page.content }}
  {% for js in extra_javascript %}
  {{ js | script_tag }}
  {% endfor %}
</body>
</html>
```

## Overridable Partials

Override individual partials by placing files at `overrides/partials/<name>.html`:

| Partial          | Purpose                         |
| ---------------- | ------------------------------- |
| `actions.html`   | Edit/view source action buttons |
| `alternate.html` | Language selector               |
| `comments.html`  | Comments section                |
| `consent.html`   | Cookie consent                  |
| `content.html`   | Content wrapper                 |
| `copyright.html` | Copyright notice                |
| `feedback.html`  | Page feedback widget            |
| `footer.html`    | Full footer                     |
| `header.html`    | Full header                     |
| `logo.html`      | Logo element                    |
| `nav.html`       | Navigation sidebar              |
| `nav-item.html`  | Single navigation item          |
| `palette.html`   | Color scheme toggle             |
| `search.html`    | Search interface                |
| `social.html`    | Social links                    |
| `source.html`    | Repository link                 |
| `tabs.html`      | Tabs container                  |
| `tabs-item.html` | Single tab item                 |
| `tags.html`      | Tags display                    |
| `toc.html`       | Table of contents sidebar       |
| `toc-item.html`  | Single TOC entry                |
| `top.html`       | Back-to-top button              |

## Template Context Variables

Variables available in all page templates:

| Variable           | Type      | Description                                |
| ------------------ | --------- | ------------------------------------------ |
| `generator`        | string    | Zensical version identifier                |
| `nav`              | object    | Navigation structure with active page      |
| `base_url`         | string    | Base URL for current page                  |
| `extra_css`        | list[str] | Extra CSS file paths                       |
| `extra_javascript` | list[obj] | Extra JS entries (use `script_tag` filter) |
| `config`           | object    | Full project configuration                 |
| `tags`             | list      | Page tags                                  |
| `page`             | object    | Current page object (see below)            |

### `page` object properties

| Property             | Description                    |
| -------------------- | ------------------------------ |
| `page.url`           | Page URL relative to site root |
| `page.title`         | Page title                     |
| `page.meta`          | Frontmatter metadata dict      |
| `page.content`       | Rendered HTML content          |
| `page.toc`           | Table of contents entries      |
| `page.path`          | Source file path               |
| `page.edit_url`      | Edit URL (if repo configured)  |
| `page.canonical_url` | Canonical URL                  |
| `page.ancestors`     | Parent navigation items        |
| `page.previous_page` | Previous page in navigation    |
| `page.next_page`     | Next page in navigation        |

## MiniJinja Syntax

Zensical uses MiniJinja (Rust), not Jinja2 (Python). Syntax is largely compatible:

```jinja2
{# Comments #}
{{ variable }}
{{ variable | filter }}
{% if condition %}...{% endif %}
{% for item in list %}...{% endfor %}
{% block name %}...{% endblock %}
{% extends "parent.html" %}
{{ super() }}
{% include "partial.html" %}
```

Custom filters available:

- `script_tag` - Render an `ExtraScript` object as `<script>` tag with correct attributes
- `url` - Resolve relative URLs against current page or base URL
- `striptags` - Remove HTML tags from string
