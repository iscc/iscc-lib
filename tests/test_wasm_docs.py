"""Docs-claim test: the ``@iscc/wasm`` pages describe the package ``release.yml`` publishes.

``release.yml`` builds ``@iscc/wasm`` with ``wasm-pack build --target web``: one ES module whose
``init()`` fetches the ``.wasm`` file by URL, which Node's ``fetch`` cannot do for ``file:`` URLs.
So ``docs/howto/wasm.md`` and ``crates/iscc-wasm/README.md`` must not tell readers to build with
``--target nodejs`` or claim separate browser and Node.js targets, and their Node.js setup must
pass the WASM bytes through the wasm-bindgen options object (``initSync({ module })`` or
``init({ module_or_path })``). Both pages name ``WasmSumCodeResult``, the object
``gen_sum_code_v0`` returns, and the ``test-wasm`` smoke script uses the options object rather
than the deprecated positional ``init(bytes)``, and so does the registry install check in
``scripts/test_install.py``.

Every ``@iscc/wasm`` snippet on the README, the docs home page and the getting-started tutorial
initializes the module before its first call, and a bare ``init()`` (browsers and bundlers only)
carries a comment that sends Node.js readers elsewhere. The WASM agent guidance names no build
target other than the published ``--target web``. No network, no mocks.
"""

import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
RELEASE_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
HOWTO = ROOT / "docs" / "howto" / "wasm.md"
README = ROOT / "crates" / "iscc-wasm" / "README.md"
INSTALL_SCRIPT = ROOT / "scripts" / "test_install.py"
WASM_DOCS = [HOWTO, README]
BANNED_PHRASES = ["--target nodejs", "Both browser and Node.js targets"]
OPTIONS_INIT_RE = re.compile(r"\binitSync\(\s*\{|\binit\(\s*\{")
POSITIONAL_INIT_RE = re.compile(r"\binit(?:Sync)?\(\s*[A-Za-z_$][\w$]*\s*\)")
SNIPPET_PAGES = [
    ROOT / "README.md",
    ROOT / "docs" / "index.md",
    ROOT / "docs" / "tutorials" / "getting-started.md",
]
AGENT_GUIDANCE = [
    ROOT / "crates" / "iscc-wasm" / "CLAUDE.md",
    ROOT / ".claude" / "context" / "specs" / "wasm-bindings.md",
]
UNPUBLISHED_TARGETS = ["--target nodejs", "--target bundler"]
WASM_IMPORT_RE = re.compile(r"""from\s+["']@iscc/wasm["']""")
INIT_CALL_RE = re.compile(r"\binit(?:Sync)?\(")
BARE_INIT_RE = re.compile(r"\binit\(\s*\)")
FENCED_JS_RE = re.compile(r"^( *)```javascript\n(.*?)^\1```", re.MULTILINE | re.DOTALL)


def normalise_whitespace(text):
    """Collapse every whitespace run to one space (mdformat reflows prose at 100 columns)."""
    return re.sub(r"\s+", " ", text)


def markdown_section(text, heading):
    """Return the body of the Markdown section that starts at ``heading`` (to the next heading).

    The section ends at the next heading of the same or a higher level. Raises ``ValueError``
    when the heading is missing.
    """
    level = len(heading) - len(heading.lstrip("#"))
    lines = text.splitlines()
    try:
        start = lines.index(heading)
    except ValueError:
        raise ValueError(f"heading not found: {heading!r}") from None
    body = []
    for line in lines[start + 1 :]:
        match = re.match(r"^(#+) ", line)
        if match and len(match.group(1)) <= level:
            break
        body.append(line)
    return "\n".join(body)


def javascript_blocks(text):
    """Return the contents of every fenced ``javascript`` code block in ``text``."""
    return re.findall(r"^```javascript\n(.*?)^```", text, re.MULTILINE | re.DOTALL)


def indented_javascript_blocks(text):
    """Return every fenced ``javascript`` block in ``text``, dedented, at any fence indent.

    Content tabs (``=== "WASM"``) indent their fences by four spaces, which
    ``javascript_blocks`` does not see.
    """
    blocks = []
    for indent, body in FENCED_JS_RE.findall(text):
        lines = body.splitlines()
        blocks.append("\n".join(line.removeprefix(indent) for line in lines) + "\n")
    return blocks


def snippet_problems(block):
    """Return the init rules an ``@iscc/wasm`` snippet breaks (empty for any other block)."""
    if not WASM_IMPORT_RE.search(block):
        return []
    problems = []
    if not INIT_CALL_RE.search(block):
        problems.append("no init() or initSync() call")
    if BARE_INIT_RE.search(block) and not OPTIONS_INIT_RE.search(block):
        comments = [
            line for line in block.splitlines() if line.lstrip().startswith("//")
        ]
        if not any("Node" in line for line in comments):
            problems.append("bare init() without a comment naming Node")
    if POSITIONAL_INIT_RE.search(block):
        problems.append("positional init")
    return problems


def unpublished_targets(text):
    """Return the unpublished ``wasm-pack`` targets that ``text`` names."""
    flat = normalise_whitespace(text)
    return [target for target in UNPUBLISHED_TARGETS if target in flat]


def test_wasm_docs_drop_node_target_claims():
    """Neither page tells readers to build for Node.js or claims a separate Node.js target."""
    for path in WASM_DOCS:
        text = normalise_whitespace(path.read_text(encoding="utf-8"))
        for phrase in BANNED_PHRASES:
            assert phrase not in text, f"{path.relative_to(ROOT)} contains {phrase!r}"


def test_howto_node_section_initializes_with_options_object():
    """The how-to's Node.js setup passes the WASM bytes through the options object."""
    section = markdown_section(HOWTO.read_text(encoding="utf-8"), "### Node.js")
    blocks = javascript_blocks(section)
    assert blocks, "docs/howto/wasm.md Node.js section has no javascript block"
    assert any(OPTIONS_INIT_RE.search(block) for block in blocks)
    assert all("readFileSync" in block for block in blocks)


def test_readme_shows_node_initialization_with_options_object():
    """The npm README carries a Node.js snippet that initializes with the options object."""
    blocks = javascript_blocks(README.read_text(encoding="utf-8"))
    node_blocks = [block for block in blocks if "node:fs" in block]
    assert node_blocks, "crates/iscc-wasm/README.md has no Node.js javascript block"
    assert all(OPTIONS_INIT_RE.search(block) for block in node_blocks)


def test_wasm_docs_name_sum_code_result():
    """Both pages name ``WasmSumCodeResult``, the return type of ``gen_sum_code_v0``."""
    for path in WASM_DOCS:
        assert "WasmSumCodeResult" in path.read_text(encoding="utf-8"), path


def test_wasm_docs_show_no_positional_init():
    """No javascript example on either page uses the deprecated positional initializer."""
    for path in WASM_DOCS:
        for block in javascript_blocks(path.read_text(encoding="utf-8")):
            assert not POSITIONAL_INIT_RE.search(block), block


def test_wasm_smoke_script_uses_options_object():
    """The ``test-wasm`` smoke script initializes with an options object, not ``init(bytes)``."""
    workflow = yaml.safe_load(RELEASE_WORKFLOW.read_text(encoding="utf-8"))
    steps = workflow["jobs"]["test-wasm"]["steps"]
    script = "\n".join(str(step.get("run", "")) for step in steps)
    assert "init(" in script, "test-wasm runs no init call"
    assert OPTIONS_INIT_RE.search(script)
    assert not POSITIONAL_INIT_RE.search(script), script


def test_install_script_uses_options_object():
    """The registry install check initializes ``@iscc/wasm`` with an options object."""
    source = INSTALL_SCRIPT.read_text(encoding="utf-8")
    assert OPTIONS_INIT_RE.search(source), (
        "scripts/test_install.py runs no options-object init"
    )
    assert not POSITIONAL_INIT_RE.search(source)


def test_positional_init_pattern_matches_deprecated_forms():
    """The positional-init pattern flags the deprecated forms and spares the options object."""
    assert POSITIONAL_INIT_RE.search("await init(wasmBytes);")
    assert POSITIONAL_INIT_RE.search("initSync(bytes)")
    assert not POSITIONAL_INIT_RE.search("await init();")
    assert not POSITIONAL_INIT_RE.search("await init({ module_or_path: wasmBytes });")
    assert not POSITIONAL_INIT_RE.search("initSync({ module: bytes })")


def test_snippet_pages_initialize_every_wasm_snippet():
    """Every ``@iscc/wasm`` snippet on the snippet pages initializes in a labelled form."""
    for path in SNIPPET_PAGES:
        text = path.read_text(encoding="utf-8")
        blocks = indented_javascript_blocks(text)
        wasm_blocks = [block for block in blocks if WASM_IMPORT_RE.search(block)]
        assert wasm_blocks, f"{path.relative_to(ROOT)} has no @iscc/wasm snippet"
        checked_imports = sum(len(WASM_IMPORT_RE.findall(block)) for block in blocks)
        assert checked_imports == len(WASM_IMPORT_RE.findall(text)), (
            f"{path.relative_to(ROOT)} imports @iscc/wasm outside a javascript block"
        )
        for block in wasm_blocks:
            assert not snippet_problems(block), (path.relative_to(ROOT), block)


def test_getting_started_checks_every_wasm_tab():
    """The tutorial's WASM tabs (indented fences) all reach the snippet rule."""
    text = SNIPPET_PAGES[2].read_text(encoding="utf-8")
    blocks = indented_javascript_blocks(text)
    assert sum(bool(WASM_IMPORT_RE.search(block)) for block in blocks) >= 6
    assert not any(WASM_IMPORT_RE.search(block) for block in javascript_blocks(text))


def test_agent_guidance_names_only_web_target():
    """The WASM agent guidance names no unpublished ``wasm-pack`` target."""
    for path in AGENT_GUIDANCE:
        found = unpublished_targets(path.read_text(encoding="utf-8"))
        assert not found, f"{path.relative_to(ROOT)} names {found}"


def test_snippet_rule_flags_missing_init():
    """A snippet that imports ``@iscc/wasm`` and never initializes breaks the rule."""
    block = 'import { gen_text_code_v0 } from "@iscc/wasm";\ngen_text_code_v0("Hi");\n'
    assert snippet_problems(block) == ["no init() or initSync() call"]


def test_snippet_rule_flags_unlabelled_bare_init():
    """A bare ``init()`` without a comment naming Node breaks the rule; with one it passes."""
    block = 'import init, { f } from "@iscc/wasm";\n// Initialize first\nawait init();\nf();\n'
    assert snippet_problems(block) == ["bare init() without a comment naming Node"]
    labelled = block.replace(
        "// Initialize first", "// Browsers; for Node.js use initSync"
    )
    assert snippet_problems(labelled) == []


def test_snippet_rule_flags_positional_init():
    """A snippet with the deprecated positional initializer breaks the rule."""
    block = 'import { initSync } from "@iscc/wasm";\ninitSync(bytes);\n'
    assert snippet_problems(block) == ["positional init"]


def test_snippet_rule_ignores_other_packages():
    """A block that does not import ``@iscc/wasm`` is outside the rule."""
    assert snippet_problems('import { f } from "@iscc/lib";\nf();\n') == []


def test_indented_blocks_reach_tab_content():
    """An indented content-tab fence is found and dedented; the column-0 helper misses it."""
    text = '=== "WASM"\n\n    ```javascript\n    import { f } from "@iscc/wasm";\n    ```\n'
    assert javascript_blocks(text) == []
    blocks = indented_javascript_blocks(text)
    assert blocks == ['import { f } from "@iscc/wasm";\n']
    assert snippet_problems(blocks[0]) == ["no init() or initSync() call"]


def test_unpublished_target_check_fires():
    """The guidance check flags both unpublished targets, also across a line break."""
    text = "wasm-pack build --target\n    nodejs and --target bundler; --target web is fine"
    assert unpublished_targets(text) == ["--target nodejs", "--target bundler"]
    assert unpublished_targets("wasm-pack build --target web") == []
