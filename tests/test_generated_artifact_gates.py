"""Anchor test: the gates that refresh or diff generated and built artifacts stay wired.

Three artifacts are produced by a tool rather than written by hand, and a test that reads a
stale copy passes against the wrong code:

- the gitignored Python extension ``_lowlevel.abi3.so``: the pre-push ``pytest`` hook and
  ``mise run test`` rebuild it (``maturin develop``) before pytest imports it;
- ``packages/dotnet/Iscc.Lib/NativeMethods.g.cs``: csbindgen rewrites it on every
  ``cargo build -p iscc-ffi``, so the ``c-ffi`` CI job diffs it after that build;
- the committed UniFFI outputs: the ``kotlin`` CI job regenerates the Kotlin and Swift
  bindings and compares them with the committed copies.

The workflow and hook files are parsed with PyYAML; ``mise.toml`` with a regex because CI's
python-test matrix includes 3.10 (no ``tomllib``). No network, no mocks.
"""

import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
CI_WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"
PRE_COMMIT_CONFIG = ROOT / ".pre-commit-config.yaml"
MISE_TOML = ROOT / "mise.toml"
NATIVE_METHODS = "packages/dotnet/Iscc.Lib/NativeMethods.g.cs"
UNIFFI_OUTPUTS = [
    "packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt",
    "packages/swift/Sources/IsccLib/iscc_uniffi.swift",
    "packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h",
]
MISE_RUN_RE = re.compile(r"mise run ([\w:.-]+)")


def ci_steps(job):
    """Return the step list of a job in ``ci.yml``."""
    workflow = yaml.safe_load(CI_WORKFLOW.read_text(encoding="utf-8"))
    return workflow["jobs"][job]["steps"]


def step_index(steps, needle):
    """Return the index of the first step whose ``run`` body contains ``needle``."""
    for index, step in enumerate(steps):
        if needle in step.get("run", ""):
            return index
    raise AssertionError(f"no step runs {needle!r}")


def hook_entry(hook_id):
    """Return the ``entry`` of a hook in ``.pre-commit-config.yaml``."""
    config = yaml.safe_load(PRE_COMMIT_CONFIG.read_text(encoding="utf-8"))
    for repo in config["repos"]:
        for hook in repo.get("hooks", []):
            if hook["id"] == hook_id:
                return hook["entry"]
    raise AssertionError(f"no hook {hook_id!r}")


def mise_task(name):
    """Return the body of a ``[tasks.<name>]`` table in ``mise.toml``."""
    key = re.escape(name) if re.fullmatch(r"\w+", name) else re.escape(f'"{name}"')
    pattern = rf"^\[tasks\.{key}\][ \t]*$(.*?)(?=^\[|\Z)"
    match = re.search(
        pattern, MISE_TOML.read_text(encoding="utf-8"), re.MULTILINE | re.DOTALL
    )
    assert match, f"no mise task {name!r}"
    return match.group(1)


def expand_mise_runs(command):
    """Return ``command`` with each ``mise run <task>`` replaced by that task's body."""
    return MISE_RUN_RE.sub(lambda m: mise_task(m.group(1)), command)


def test_native_methods_diffed_after_ffi_build():
    """The ``c-ffi`` job diffs ``NativeMethods.g.cs`` after the build that rewrites it."""
    steps = ci_steps("c-ffi")
    build = step_index(steps, "cargo build -p iscc-ffi")
    diff = step_index(steps, f"git diff --exit-code {NATIVE_METHODS}")
    assert build < diff


def test_kotlin_job_regenerates_and_diffs_uniffi_outputs():
    """The ``kotlin`` job runs uniffi-bindgen for both languages and names every output."""
    steps = ci_steps("kotlin")
    build = step_index(steps, "cargo build -p iscc-uniffi")
    check = step_index(steps, "uniffi-bindgen")
    body = steps[check]["run"]
    assert build < check
    assert "--language kotlin" in body
    assert "--language swift" in body
    assert "diff" in body
    for path in UNIFFI_OUTPUTS:
        assert path in body, path


def test_uniffi_check_skips_hand_edited_modulemap():
    """The hand-edited ``module.modulemap`` is not compared with the generator output."""
    body = ci_steps("kotlin")[step_index(ci_steps("kotlin"), "uniffi-bindgen")]["run"]
    assert "module.modulemap" not in body


def test_pytest_hook_rebuilds_extension_first():
    """The pre-push ``pytest`` hook runs ``maturin develop`` before ``pytest``."""
    command = expand_mise_runs(hook_entry("pytest"))
    assert "maturin develop" in command
    assert command.index("maturin develop") < command.index("pytest")


def test_mise_test_task_rebuilds_extension_first():
    """``mise run test`` depends on the task that runs ``maturin develop``."""
    test_task = mise_task("test")
    depends = re.search(r"^depends\s*=\s*\[(.*?)\]", test_task, re.MULTILINE)
    assert depends, "mise task 'test' has no depends"
    names = re.findall(r'"([^"]+)"', depends.group(1))
    assert any("maturin develop" in mise_task(name) for name in names)
    assert "uv run pytest" in test_task
