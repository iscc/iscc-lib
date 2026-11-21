"""Docs-claim test: the ``@iscc/wasm`` pages describe the package ``release.yml`` publishes.

``release.yml`` builds ``@iscc/wasm`` with ``wasm-pack build --target web``: one ES module whose
``init()`` fetches the ``.wasm`` file by URL, which Node's ``fetch`` cannot do for ``file:`` URLs.
So ``docs/howto/wasm.md`` and ``crates/iscc-wasm/README.md`` must not tell readers to build with
``--target nodejs`` or claim separate browser and Node.js targets, and their Node.js setup must
pass the WASM bytes through the wasm-bindgen options object (``initSync({ module })`` or
``init({ module_or_path })``). Both pages name ``WasmSumCodeResult``, the object
``gen_sum_code_v0`` returns, and the ``test-wasm`` smoke script uses the options object rather
than the deprecated positional ``init(bytes)``. No network, no mocks.
"""

import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
RELEASE_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
HOWTO = ROOT / "docs" / "howto" / "wasm.md"
README = ROOT / "crates" / "iscc-wasm" / "README.md"
WASM_DOCS = [HOWTO, README]
BANNED_PHRASES = ["--target nodejs", "Both browser and Node.js targets"]
OPTIONS_INIT_RE = re.compile(r"\binitSync\(\s*\{|\binit\(\s*\{")
POSITIONAL_INIT_RE = re.compile(r"\binit(?:Sync)?\(\s*[A-Za-z_$][\w$]*\s*\)")


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


def test_positional_init_pattern_matches_deprecated_forms():
    """The positional-init pattern flags the deprecated forms and spares the options object."""
    assert POSITIONAL_INIT_RE.search("await init(wasmBytes);")
    assert POSITIONAL_INIT_RE.search("initSync(bytes)")
    assert not POSITIONAL_INIT_RE.search("await init();")
    assert not POSITIONAL_INIT_RE.search("await init({ module_or_path: wasmBytes });")
    assert not POSITIONAL_INIT_RE.search("initSync({ module: bytes })")
