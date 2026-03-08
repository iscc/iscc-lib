"""Conan 2.x recipe for the ISCC C++ wrapper (header-only + pre-built FFI shared library).

Downloads platform-specific pre-built FFI tarballs from GitHub Releases.
Mirrors the platform mapping logic in portfile.cmake (vcpkg).
"""

import os

from conan import ConanFile
from conan.errors import ConanInvalidConfiguration
from conan.tools.files import copy, download, unzip


class IsccConan(ConanFile):
    """Conan recipe for the ISCC C++ header-only wrapper over the FFI shared library."""

    name = "iscc"
    version = "0.2.0"
    license = "Apache-2.0"
    url = "https://github.com/iscc/iscc-lib"
    homepage = "https://github.com/iscc/iscc-lib"
    description = "ISCC - International Standard Content Code (ISO 24138:2024)"
    topics = ("iscc", "content-identification", "iso-24138", "fingerprinting")

    package_type = "shared-library"
    settings = "os", "arch"

    def _target_triple(self):
        """Map Conan os/arch settings to GitHub Release target triple, lib name, and archive ext."""
        mapping = {
            ("Linux", "x86_64"): (
                "x86_64-unknown-linux-gnu",
                "libiscc_ffi.so",
                "libiscc_ffi.a",
                ".tar.gz",
            ),
            ("Linux", "armv8"): (
                "aarch64-unknown-linux-gnu",
                "libiscc_ffi.so",
                "libiscc_ffi.a",
                ".tar.gz",
            ),
            ("Macos", "armv8"): (
                "aarch64-apple-darwin",
                "libiscc_ffi.dylib",
                "libiscc_ffi.a",
                ".tar.gz",
            ),
            ("Macos", "x86_64"): (
                "x86_64-apple-darwin",
                "libiscc_ffi.dylib",
                "libiscc_ffi.a",
                ".tar.gz",
            ),
            ("Windows", "x86_64"): (
                "x86_64-pc-windows-msvc",
                "iscc_ffi.dll",
                "iscc_ffi.lib",
                ".zip",
            ),
        }
        key = (str(self.settings.os), str(self.settings.arch))
        # Also accept "aarch64" as an alias for "armv8"
        if key not in mapping and key[1] == "aarch64":
            key = (key[0], "armv8")
        return mapping.get(key)

    def validate(self):
        """Validate that the target platform is supported."""
        if self._target_triple() is None:
            raise ConanInvalidConfiguration(
                f"Unsupported platform: {self.settings.os}-{self.settings.arch}"
            )

    def build(self):
        """Download and extract the pre-built FFI tarball for the target platform."""
        target, _, _, ext = self._target_triple()
        artifact = f"iscc-ffi-v{self.version}-{target}"
        url = (
            f"https://github.com/iscc/iscc-lib/releases/download/"
            f"v{self.version}/{artifact}{ext}"
        )
        archive_path = os.path.join(self.build_folder, f"{artifact}{ext}")
        download(self, url, archive_path)
        unzip(self, archive_path, destination=self.build_folder)

    def package(self):
        """Install headers, shared library, and license into the Conan package."""
        target, lib_name, static_lib_name, _ = self._target_triple()
        artifact_dir = os.path.join(
            self.build_folder, f"iscc-ffi-v{self.version}-{target}"
        )

        # Headers into include/iscc/ for #include <iscc/iscc.hpp>
        copy(
            self,
            "*.hpp",
            src=artifact_dir,
            dst=os.path.join(self.package_folder, "include", "iscc"),
        )
        copy(
            self,
            "*.h",
            src=artifact_dir,
            dst=os.path.join(self.package_folder, "include", "iscc"),
        )

        # Shared library into lib/ (or bin/ for Windows DLLs)
        if str(self.settings.os) == "Windows":
            copy(
                self,
                lib_name,
                src=artifact_dir,
                dst=os.path.join(self.package_folder, "bin"),
            )
            # Import library (.lib) into lib/
            copy(
                self,
                static_lib_name,
                src=artifact_dir,
                dst=os.path.join(self.package_folder, "lib"),
            )
            # DLL import lib (iscc_ffi.dll.lib) if present
            copy(
                self,
                "iscc_ffi.dll.lib",
                src=artifact_dir,
                dst=os.path.join(self.package_folder, "lib"),
            )
        else:
            copy(
                self,
                lib_name,
                src=artifact_dir,
                dst=os.path.join(self.package_folder, "lib"),
            )
            # Static library if present
            copy(
                self,
                static_lib_name,
                src=artifact_dir,
                dst=os.path.join(self.package_folder, "lib"),
            )

        # License
        copy(
            self,
            "LICENSE",
            src=artifact_dir,
            dst=os.path.join(self.package_folder, "licenses"),
        )

    def package_info(self):
        """Set compiler and linker flags for consumers."""
        self.cpp_info.libs = ["iscc_ffi"]
        self.cpp_info.includedirs = ["include"]
        if str(self.settings.os) == "Windows":
            self.cpp_info.bindirs = ["bin"]
        self.cpp_info.set_property("cmake_file_name", "iscc")
        self.cpp_info.set_property("cmake_target_name", "iscc::iscc")
        self.cpp_info.set_property("cmake_find_mode", "config")
        self.cpp_info.set_property("pkg_config_name", "iscc")
        # Require C++17
        self.cpp_info.cxxflags = ["-std=c++17"]
