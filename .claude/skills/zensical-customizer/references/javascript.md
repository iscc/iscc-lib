# Zensical JavaScript & CSS Injection Reference

## Extra CSS Configuration

Place stylesheets in `docs/stylesheets/` and register in `zensical.toml`:

```toml
[project]
extra_css = [
  "stylesheets/extra.css",
  "stylesheets/my-widget.css",
]
```

CSS is injected into the `<head>` of every page. CSS hot-swapping is supported during
`zensical serve` (no full page reload needed).

## Extra JavaScript Configuration

### Simple string paths

```toml
[project]
extra_javascript = [
  "javascripts/extra.js",
  "https://cdn.example.com/lib.js",
]
```

### Structured entries with attributes

```toml
[[project.extra_javascript]]
path = "javascripts/app.js"
type = "module"

[[project.extra_javascript]]
path = "javascripts/analytics.js"
async = true

[[project.extra_javascript]]
path = "javascripts/deferred.js"
defer = true
```

### Mixed format

```toml
extra_javascript = [
  "javascripts/simple.js",
  { path = "javascripts/app.mjs", type = "module" },
  { path = "https://cdn.example.com/lib.js", async = true },
]
```

Files with `.mjs` extension automatically get `type="module"`.

## The document$ Observable

Zensical exports a `document$` RxJS observable. Subscribe to run code on every page load, including
instant navigation transitions (no full browser reload).

```javascript
document$.subscribe(function() {
    // Initialize or re-initialize components here.
    // This fires on initial load AND on every instant navigation.
    console.log("Page loaded:", location.pathname);
});
```

### Why this matters

With `navigation.instant` enabled, clicking internal links dispatches XHR requests instead of full
page reloads. Standard `DOMContentLoaded` or `window.load` events do NOT fire on navigation. The
`document$` observable fills this gap.

### Pattern: mount/unmount components on navigation

```javascript
document$.subscribe(function() {
    var container = document.getElementById("my-widget");
    if (container) {
        // Page has the mount point - initialize widget
        initMyWidget(container);
    }
});
```

This pattern ensures widgets only initialize on pages that contain their mount point, and
re-initialize correctly when navigating away and back.

## Instant Navigation Compatibility

When `navigation.instant` is enabled in the project:

1. **DO** use `document$` for initialization, not `DOMContentLoaded` or `window.load`
2. **DO** clean up resources (event listeners, intervals) when re-initializing
3. **DO** check for mount point existence before initializing (the element may not exist on every
    page)
4. **DON'T** rely on `<script>` execution order across navigations
5. **DON'T** use global state that persists across page transitions without cleanup

When `navigation.instant` is NOT enabled, `window.load` works fine. The copilot widget in this
project uses `window.load` because it's a site-wide floating widget, not page-specific.

## Interactive Component Patterns

### Pattern 1: Page-specific widget via mount point

In the markdown page:

```markdown
<div id="interactive-demo"></div>
```

In `docs/javascripts/demo.js`:

```javascript
document$.subscribe(function() {
    var el = document.getElementById("interactive-demo");
    if (!el) return;

    // Mount your component (vanilla JS, React, Vue, etc.)
    el.innerHTML = ""; // Clean up previous instance
    renderDemo(el);
});
```

Register in `zensical.toml`:

```toml
extra_javascript = ["javascripts/demo.js"]
```

### Pattern 2: Framework app mounted into a page

Build your JS app externally (Vite, webpack, esbuild, etc.), output to `docs/javascripts/`, then
mount it:

```javascript
// docs/javascripts/my-app.js (ES module)
import {
    createApp
} from "./vendor/petite-vue.js";

document$.subscribe(function() {
    var mount = document.querySelector("[data-app]");
    if (!mount) return;
    createApp({
        /* state */
    }).mount(mount);
});
```

In markdown:

```markdown
<div data-app>
  {{ counter }}
  <button @click="counter++">+1</button>
</div>
```

### Pattern 3: Custom page template with embedded app

Create `overrides/interactive_page.html`:

```jinja2
{% extends "base.html" %}

{% block content %}
  <div id="app-root"></div>
{% endblock %}

{% block scripts %}
  {{ super() }}
  <script type="module" src="{{ base_url }}/javascripts/my-app.js"></script>
{% endblock %}
```

Page frontmatter:

```yaml
---
template: interactive_page.html
---
```

### Pattern 4: Site-wide floating widget

Like the copilot chat widget in this project - always visible regardless of page:

```javascript
window.addEventListener("load", async function() {
    // Load external resources, initialize widget
    // No need for document$ since this persists across navigations
});
```

### Pattern 5: Third-party library integration (MathJax, chart libs, etc.)

```toml
extra_javascript = [
  { path = "https://cdn.example.com/chart-lib.min.js", async = true },
  "javascripts/charts-init.js",
]
```

```javascript
// docs/javascripts/charts-init.js
document$.subscribe(function() {
    document.querySelectorAll("[data-chart]").forEach(function(el) {
        var config = JSON.parse(el.dataset.chart);
        new ChartLib(el, config);
    });
});
```

In markdown:

```markdown
<canvas data-chart='{"type":"bar","data":{"labels":["A","B"],"values":[10,20]}}'></canvas>
```
