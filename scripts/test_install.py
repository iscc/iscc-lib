"""Test that published packages are installable from their respective registries.

Verifies each published package can be installed in a clean environment and
runs a minimal smoke test (import + gen_meta_code_v0 call). Requires network
access to package registries.

Usage:
    uv run scripts/test_install.py              # Test all registries
    uv run scripts/test_install.py --pypi       # Test PyPI only
    uv run scripts/test_install.py --npm        # Test npm only
    uv run scripts/test_install.py --crates     # Test crates.io only
    uv run scripts/test_install.py --go         # Test Go module only
    uv run scripts/test_install.py --maven      # Test Maven Central only
    uv run scripts/test_install.py --version 0.0.3  # Test specific version
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

# Expected ISCC for gen_meta_code_v0("Hello World") with default bits=64
EXPECTED_ISCC = "ISCC:AAAWN77F727NXSUS"


@dataclass
class TestResult:
    """Result of a single install test."""

    registry: str
    package: str
    passed: bool
    message: str
    version_tested: str = ""


def run(
    cmd: list[str],
    *,
    cwd: str | Path | None = None,
    timeout: int = 120,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess:
    """Run a command and return the result."""
    run_env = {**os.environ, **(env or {})}
    return subprocess.run(  # noqa: S603
        cmd, capture_output=True, text=True, cwd=cwd, timeout=timeout, env=run_env
    )


def check_command(name: str) -> bool:
    """Check if a command is available on PATH."""
    return shutil.which(name) is not None


def venv_python(venv_dir: Path) -> str:
    """Return the path to the Python executable in a venv (cross-platform)."""
    if sys.platform == "win32":
        return str(venv_dir / "Scripts" / "python.exe")
    return str(venv_dir / "bin" / "python")


def test_pypi(version: str) -> TestResult:
    """Test installing iscc-lib from PyPI."""
    registry = "PyPI"
    package = "iscc-lib"

    if not check_command("uv"):
        return TestResult(registry, package, False, "uv not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_pypi_") as tmpdir:
        venv_dir = Path(tmpdir) / "venv"
        version_spec = f"iscc-lib=={version}" if version else "iscc-lib"

        result = run(["uv", "venv", str(venv_dir)], cwd=tmpdir)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"venv creation failed: {result.stderr}"
            )

        python = venv_python(venv_dir)
        result = run(
            ["uv", "pip", "install", version_spec, "--python", python], cwd=tmpdir
        )
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"pip install failed: {result.stderr}"
            )

        # Smoke test: import and call gen_meta_code_v0
        smoke_test = f"""\
import iscc_lib
result = iscc_lib.gen_meta_code_v0("Hello World")
assert result["iscc"] == "{EXPECTED_ISCC}", f"ISCC mismatch: {{result['iscc']}}"
print(f"OK: iscc_lib {{iscc_lib.__version__}} — {{result['iscc']}}")
"""
        result = run([python, "-c", smoke_test], cwd=tmpdir)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"smoke test failed: {result.stderr}"
            )

        tested_version = ""
        for line in result.stdout.splitlines():
            if line.startswith("OK:"):
                tested_version = line.split()[2]
                break

        return TestResult(
            registry, package, True, result.stdout.strip(), tested_version
        )


def test_crates_io(version: str) -> TestResult:
    """Test installing iscc-lib from crates.io and running a smoke test."""
    registry = "crates.io"
    package = "iscc-lib"

    if not check_command("cargo"):
        return TestResult(registry, package, False, "cargo not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_crates_") as tmpdir:
        proj_dir = Path(tmpdir) / "test_proj"
        result = run(["cargo", "init", "--name", "iscc_install_test", str(proj_dir)])
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"cargo init failed: {result.stderr}"
            )

        add_cmd = (
            ["cargo", "add", f"iscc-lib@{version}"]
            if version
            else ["cargo", "add", "iscc-lib"]
        )
        result = run(add_cmd, cwd=proj_dir, timeout=60)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"cargo add failed: {result.stderr}"
            )

        # Correct API: gen_meta_code_v0(name, description, meta, bits) -> Result<MetaCodeResult>
        main_rs = f"""\
fn main() {{
    let result = iscc_lib::gen_meta_code_v0("Hello World", None, None, 64).unwrap();
    assert_eq!(result.iscc, "{EXPECTED_ISCC}", "ISCC mismatch");
    println!("OK: iscc-lib — {{}}", result.iscc);
}}
"""
        (proj_dir / "src" / "main.rs").write_text(main_rs)

        result = run(["cargo", "run", "--release"], cwd=proj_dir, timeout=300)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"cargo run failed: {result.stderr}"
            )

        return TestResult(registry, package, True, result.stdout.strip())


def test_npm_lib(version: str) -> TestResult:
    """Test installing @iscc/lib from npm."""
    registry = "npm"
    package = "@iscc/lib"

    if not check_command("node"):
        return TestResult(registry, package, False, "node not found on PATH")
    if not check_command("npm"):
        return TestResult(registry, package, False, "npm not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_npm_lib_") as tmpdir:
        pkg = {
            "name": "iscc-install-test",
            "version": "0.0.0",
            "private": True,
            "type": "module",
        }
        Path(tmpdir, "package.json").write_text(json.dumps(pkg))

        version_spec = f"@iscc/lib@{version}" if version else "@iscc/lib"
        result = run(["npm", "install", version_spec], cwd=tmpdir, timeout=120)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"npm install failed: {result.stderr}"
            )

        # napi bindings return the ISCC string directly from gen_meta_code_v0
        smoke_test = f"""\
import {{ gen_meta_code_v0 }} from '@iscc/lib';
const result = gen_meta_code_v0('Hello World');
const expected = '{EXPECTED_ISCC}';
if (result !== expected) {{
    console.error(`ISCC mismatch: ${{result}} !== ${{expected}}`);
    process.exit(1);
}}
console.log(`OK: @iscc/lib — ${{result}}`);
"""
        Path(tmpdir, "test.mjs").write_text(smoke_test)
        result = run(["node", "test.mjs"], cwd=tmpdir)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"smoke test failed: {result.stderr}"
            )

        return TestResult(registry, package, True, result.stdout.strip())


def test_npm_wasm(version: str) -> TestResult:
    """Test installing @iscc/wasm from npm."""
    registry = "npm"
    package = "@iscc/wasm"

    if not check_command("node"):
        return TestResult(registry, package, False, "node not found on PATH")
    if not check_command("npm"):
        return TestResult(registry, package, False, "npm not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_npm_wasm_") as tmpdir:
        pkg = {
            "name": "iscc-install-test-wasm",
            "version": "0.0.0",
            "private": True,
            "type": "module",
        }
        Path(tmpdir, "package.json").write_text(json.dumps(pkg))

        version_spec = f"@iscc/wasm@{version}" if version else "@iscc/wasm"
        result = run(["npm", "install", version_spec], cwd=tmpdir, timeout=120)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"npm install failed: {result.stderr}"
            )

        # WASM web target needs manual init with wasm bytes in Node.js
        # wasm-bindgen returns strings directly from gen_meta_code_v0
        smoke_test = f"""\
import {{ readFile }} from 'fs/promises';
import {{ createRequire }} from 'module';
const require = createRequire(import.meta.url);
const wasmPkgPath = require.resolve('@iscc/wasm');
const wasmPath = wasmPkgPath.replace(/[^\\/\\\\]*$/, 'iscc_wasm_bg.wasm');
const wasmBytes = await readFile(wasmPath);
const {{ default: init, gen_meta_code_v0 }} = await import('@iscc/wasm');
await init(wasmBytes);
const result = gen_meta_code_v0('Hello World');
const expected = '{EXPECTED_ISCC}';
if (result !== expected) {{
    console.error(`ISCC mismatch: ${{result}} !== ${{expected}}`);
    process.exit(1);
}}
console.log(`OK: @iscc/wasm — ${{result}}`);
"""
        Path(tmpdir, "test.mjs").write_text(smoke_test)
        result = run(["node", "test.mjs"], cwd=tmpdir)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"smoke test failed: {result.stderr}"
            )

        return TestResult(registry, package, True, result.stdout.strip())


def test_go_module(version: str) -> TestResult:
    """Test installing the Go module via go get."""
    registry = "Go proxy"
    package = "github.com/iscc/iscc-lib/packages/go"

    if not check_command("go"):
        return TestResult(registry, package, False, "go not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_go_") as tmpdir:
        result = run(["go", "mod", "init", "iscc_install_test"], cwd=tmpdir)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"go mod init failed: {result.stderr}"
            )

        pkg_path = "github.com/iscc/iscc-lib/packages/go"
        get_spec = f"{pkg_path}@v{version}" if version else pkg_path
        result = run(["go", "get", get_spec], cwd=tmpdir, timeout=120)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"go get failed: {result.stderr}"
            )

        # Pure Go API: iscc.GenMetaCodeV0(name, desc, meta, bits)
        main_go = f"""\
package main

import (
\t"fmt"
\t"log"

\tiscc "{pkg_path}"
)

func main() {{
\tresult, err := iscc.GenMetaCodeV0("Hello World", nil, nil, 64)
\tif err != nil {{
\t\tlog.Fatal(err)
\t}}
\texpected := "{EXPECTED_ISCC}"
\tif result.Iscc != expected {{
\t\tlog.Fatalf("ISCC mismatch: %s != %s", result.Iscc, expected)
\t}}
\tfmt.Printf("OK: go iscc — %s\\n", result.Iscc)
}}
"""
        Path(tmpdir, "main.go").write_text(main_go)

        result = run(["go", "mod", "tidy"], cwd=tmpdir, timeout=120)
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"go mod tidy failed: {result.stderr}"
            )

        result = run(
            ["go", "run", "."],
            cwd=tmpdir,
            timeout=300,
            env={"CGO_ENABLED": "0"},
        )
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"go run failed: {result.stderr}"
            )

        return TestResult(registry, package, True, result.stdout.strip())


def test_maven(version: str) -> TestResult:
    """Test installing io.iscc:iscc-lib from Maven Central."""
    registry = "Maven Central"
    package = "io.iscc:iscc-lib"

    if not check_command("mvn"):
        return TestResult(registry, package, False, "mvn not found on PATH")

    with tempfile.TemporaryDirectory(prefix="iscc_test_maven_") as tmpdir:
        dep_version = version if version else "RELEASE"

        pom_xml = f"""\
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>test</groupId>
    <artifactId>iscc-install-test</artifactId>
    <version>0.0.0</version>
    <properties>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
    </properties>
    <dependencies>
        <dependency>
            <groupId>io.iscc</groupId>
            <artifactId>iscc-lib</artifactId>
            <version>{dep_version}</version>
        </dependency>
    </dependencies>
</project>
"""
        Path(tmpdir, "pom.xml").write_text(pom_xml)

        src_dir = Path(tmpdir, "src", "main", "java")
        src_dir.mkdir(parents=True)

        main_java = f"""\
import io.iscc.iscc_lib.IsccLib;

public class Main {{
    public static void main(String[] args) {{
        String result = IsccLib.genMetaCodeV0("Hello World", null, null, 64);
        String expected = "{EXPECTED_ISCC}";
        if (!result.equals(expected)) {{
            System.err.println("ISCC mismatch: " + result + " != " + expected);
            System.exit(1);
        }}
        System.out.println("OK: io.iscc:iscc-lib — " + result);
    }}
}}
"""
        Path(src_dir, "Main.java").write_text(main_java)

        result = run(
            ["mvn", "-q", "compile", "exec:java", "-Dexec.mainClass=Main"],
            cwd=tmpdir,
            timeout=300,
        )
        if result.returncode != 0:
            return TestResult(
                registry, package, False, f"maven test failed: {result.stderr}"
            )

        return TestResult(registry, package, True, result.stdout.strip())


def check_registry_availability(version: str) -> dict[str, bool]:
    """Check which packages are available on their registries without installing."""
    available = {}

    # PyPI
    result = run(["curl", "-sf", f"https://pypi.org/pypi/iscc-lib/{version}/json"])
    available["pypi"] = result.returncode == 0

    # crates.io
    result = run(["curl", "-sf", f"https://crates.io/api/v1/crates/iscc-lib/{version}"])
    available["crates"] = result.returncode == 0

    # npm @iscc/lib
    if check_command("npm"):
        result = run(["npm", "view", f"@iscc/lib@{version}", "version"])
        available["npm_lib"] = result.returncode == 0
    else:
        available["npm_lib"] = False

    # npm @iscc/wasm
    if check_command("npm"):
        result = run(["npm", "view", f"@iscc/wasm@{version}", "version"])
        available["npm_wasm"] = result.returncode == 0
    else:
        available["npm_wasm"] = False

    # Maven Central
    result = run(
        [
            "curl",
            "-sf",
            f"https://search.maven.org/solrsearch/select?q=g:io.iscc+AND+a:iscc-lib+AND+v:{version}&rows=1&wt=json",
        ]
    )
    available["maven"] = result.returncode == 0 and '"numFound":1' in result.stdout

    return available


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Test published package installability"
    )
    parser.add_argument(
        "--version", default="", help="Specific version to test (default: latest)"
    )
    parser.add_argument("--pypi", action="store_true", help="Test PyPI only")
    parser.add_argument("--npm", action="store_true", help="Test npm packages only")
    parser.add_argument("--crates", action="store_true", help="Test crates.io only")
    parser.add_argument("--go", action="store_true", help="Test Go module only")
    parser.add_argument("--maven", action="store_true", help="Test Maven Central only")
    parser.add_argument(
        "--check-only", action="store_true", help="Only check registry availability"
    )
    args = parser.parse_args()

    test_all = not (args.pypi or args.npm or args.crates or args.go or args.maven)
    version = args.version

    print("=== iscc-lib install test protocol ===")
    if version:
        print(f"Testing version: {version}")
    else:
        print("Testing latest available version")
    print()

    # Check registry availability first
    if args.check_only or test_all:
        v = version or "0.0.3"
        print("--- Registry availability ---")
        available = check_registry_availability(v)
        for reg, avail in available.items():
            status = "AVAILABLE" if avail else "NOT FOUND"
            print(f"  {reg:12s}: {status}")
        print()
        if args.check_only:
            return 0

    results: list[TestResult] = []

    if test_all or args.pypi:
        print("--- Testing PyPI: iscc-lib ---")
        r = test_pypi(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

    if test_all or args.crates:
        print("--- Testing crates.io: iscc-lib ---")
        r = test_crates_io(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

    if test_all or args.npm:
        print("--- Testing npm: @iscc/lib ---")
        r = test_npm_lib(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

        print("--- Testing npm: @iscc/wasm ---")
        r = test_npm_wasm(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

    if test_all or args.go:
        print("--- Testing Go: github.com/iscc/iscc-lib/packages/go ---")
        r = test_go_module(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

    if test_all or args.maven:
        print("--- Testing Maven Central: io.iscc:iscc-lib ---")
        r = test_maven(version)
        results.append(r)
        print(f"  {'PASS' if r.passed else 'FAIL'}: {r.message}")
        print()

    # Summary
    passed = sum(1 for r in results if r.passed)
    failed = sum(1 for r in results if not r.passed)
    print("=== Summary ===")
    for r in results:
        icon = "PASS" if r.passed else "FAIL"
        ver = f" ({r.version_tested})" if r.version_tested else ""
        print(f"  [{icon}] {r.registry:20s} {r.package}{ver}")
    print(f"\n  {passed} passed, {failed} failed, {len(results)} total")

    return 1 if failed > 0 else 0


if __name__ == "__main__":
    sys.exit(main())
