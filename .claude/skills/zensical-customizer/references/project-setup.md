# Project Documentation Setup

## Configuration File

Config: `zensical.toml` at project root. All settings under `[project]`.

Key paths:

- Docs source: `docs/`
- Build output: `site/`
- Dev server: `uv run poe docs-serve` (localhost:8000)
- Production build: `uv run poe docs-build`

## Directory Structure

```
docs/
├── index.md
├── assets/                    # Images and logos
│   ├── logo_light.png
│   ├── logo_dark.png
│   └── favicon.png
├── stylesheets/
│   ├── extra.css              # ISCC brand color overrides
│   └── copilot.css            # Copilot widget theme (Shadow DOM)
├── javascripts/
│   └── copilot.js             # Copilot widget initialization
├── includes/
│   └── abbreviations.md       # Auto-appended abbreviations
├── tutorials/                 # Getting started guides
├── howto/                     # Task-oriented guides
├── explanation/               # Conceptual docs
├── reference/                 # API reference (mkdocstrings)
└── development/               # Contributing guide
```

If no `overrides/` directory exists, create it when template overrides are needed and add
`custom_dir = "overrides"` to `[project.theme]` in `zensical.toml`.

## Existing Customizations

### CSS: `docs/stylesheets/extra.css`

Overrides Zensical's default indigo palette with ISCC brand colors:

- Light/dark mode support via `[data-md-color-*]` attribute selectors
- Header: ISCC Blue (#0054b2) in light mode, Deep Navy (#123663) in dark mode
- Footer: Deep Navy (#123663) with white text
- Links: ISCC Blue (#0054b2)
- Logo: CSS `filter: invert(1)` for white-on-blue header and dark mode

### CSS: `docs/stylesheets/copilot.css`

Styles the Chainlit copilot widget inside its Shadow DOM:

- Fixes Zensical's 125% root font-size inflation (html { font-size: 20px })
- Pins Tailwind text utilities to intended pixel values
- Forces ISCC color scheme via CSS custom properties
- Loaded via `customCssUrl` parameter, NOT via `extra_css`

### JS: `docs/javascripts/copilot.js`

Mounts Chainlit copilot widget with anonymous JWT authentication:

- Uses `window.load` event (site-wide widget, not page-specific)
- Fetches token from `https://iscc.ai/api/copilot-token`
- Passes `customCssUrl` pointing to copilot.css with cache-busting param

### External scripts

```toml
extra_javascript = [
  { path = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js", async = true },
  "https://iscc.ai/copilot/index.js",
  "javascripts/copilot.js",
]
```

## ISCC Brand Colors

| Name       | Hex       | Usage                               |
| ---------- | --------- | ----------------------------------- |
| ISCC Blue  | `#0054b2` | Primary, links, header (light mode) |
| Sky Blue   | `#4596f5` | Primary light variant, dark mode    |
| Deep Navy  | `#123663` | Header (dark), footer, accents      |
| ISCC Coral | `#f56169` | Floating buttons, CTAs              |
| White      | `#ffffff` | Backgrounds, text on dark surfaces  |

## Zensical Quirks

See also Key Constraints in SKILL.md for platform-level constraints (font-size, MiniJinja, plugin
system, markdown extensions).

1. **CSS hot-reload works**: During `zensical serve`, CSS changes trigger browser reload without
    full page refresh.

2. **`navigation.instant` is not enabled**: This project does not list `navigation.instant` in
    features, so `window.load` works for site-wide scripts. If instant navigation is enabled later,
    page-specific scripts must use `document$`.
