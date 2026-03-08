"""Conan 2.x recipe for the ISCC C++ wrapper (header-only + pre-built FFI shared library)."""

import os

from conan import ConanFile
from conan.tools.cmake import CMake, cmake_layout
from conan.tools.files import copy


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
    settings = "os", "compiler", "build_type", "arch"
    generators = "CMakeToolchain", "CMakeDeps"

    exports_sources = "include/*", "CMakeLists.txt", "LICENSE"

    def layout(self):
        """Set standard CMake layout."""
        cmake_layout(self)

    def validate(self):
        """Validate that the compiler supports C++17."""
        from conan.tools.build import check_min_cppstd

        check_min_cppstd(self, "17")

    def build(self):
        """Build the CMake project (header-only, builds tests if enabled)."""
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        """Install headers and license into the Conan package."""
        copy(
            self,
            "LICENSE",
            src=self.source_folder,
            dst=os.path.join(self.package_folder, "licenses"),
        )
        copy(
            self,
            "*.hpp",
            src=os.path.join(self.source_folder, "include"),
            dst=os.path.join(self.package_folder, "include"),
        )
        copy(
            self,
            "*.h",
            src=os.path.join(self.source_folder, "include"),
            dst=os.path.join(self.package_folder, "include"),
        )

    def package_info(self):
        """Set compiler and linker flags for consumers."""
        self.cpp_info.libs = ["iscc_ffi"]
        self.cpp_info.includedirs = ["include"]
        self.cpp_info.set_property("cmake_file_name", "iscc")
        self.cpp_info.set_property("cmake_target_name", "iscc::iscc")
        self.cpp_info.set_property("cmake_find_mode", "config")
        self.cpp_info.set_property("pkg_config_name", "iscc")
        # Require C++17
        if self.settings.compiler == "msvc":
            self.cpp_info.cxxflags = ["/std:c++17"]
        else:
            self.cpp_info.cxxflags = ["-std=c++17"]
