# vcpkg portfile for iscc — downloads pre-built FFI tarballs from GitHub Releases.
#
# The ISCC C++ wrapper is header-only but requires the pre-built FFI shared library.
# Tarballs contain a flat layout: iscc.hpp, iscc.h, libiscc_ffi.so/.dylib/.dll, LICENSE.

vcpkg_check_linkage(ONLY_DYNAMIC_LIBRARY)

# Map vcpkg triplets to GitHub Release target triples
if(VCPKG_TARGET_IS_LINUX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-unknown-linux-gnu")
    set(ISCC_LIB_NAME "libiscc_ffi.so")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
elseif(VCPKG_TARGET_IS_LINUX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-unknown-linux-gnu")
    set(ISCC_LIB_NAME "libiscc_ffi.so")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
elseif(VCPKG_TARGET_IS_WINDOWS AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-pc-windows-msvc")
    set(ISCC_LIB_NAME "iscc_ffi.dll")
    set(ISCC_STATIC_LIB_NAME "iscc_ffi.lib")
    set(ISCC_ARCHIVE_EXT ".zip")
else()
    message(FATAL_ERROR "Unsupported platform: ${VCPKG_TARGET_ARCHITECTURE}-${VCPKG_CMAKE_SYSTEM_NAME}")
endif()

set(ISCC_VERSION "${VERSION}")
set(ISCC_ARTIFACT "iscc-ffi-v${ISCC_VERSION}-${ISCC_TARGET}")
set(ISCC_URL "https://github.com/iscc/iscc-lib/releases/download/v${ISCC_VERSION}/${ISCC_ARTIFACT}${ISCC_ARCHIVE_EXT}")

vcpkg_download_distfile(ARCHIVE
    URLS "${ISCC_URL}"
    FILENAME "${ISCC_ARTIFACT}${ISCC_ARCHIVE_EXT}"
    SKIP_SHA512
)

# Extract the archive
vcpkg_extract_source_archive(SOURCE_PATH
    ARCHIVE "${ARCHIVE}"
    NO_REMOVE_ONE_LEVEL
)

# Install headers — create iscc/ subdirectory for #include <iscc/iscc.hpp>
file(INSTALL "${SOURCE_PATH}/${ISCC_ARTIFACT}/iscc.hpp"
    DESTINATION "${CURRENT_PACKAGES_DIR}/include/iscc"
)
file(INSTALL "${SOURCE_PATH}/${ISCC_ARTIFACT}/iscc.h"
    DESTINATION "${CURRENT_PACKAGES_DIR}/include/iscc"
)

# Install shared library
file(INSTALL "${SOURCE_PATH}/${ISCC_ARTIFACT}/${ISCC_LIB_NAME}"
    DESTINATION "${CURRENT_PACKAGES_DIR}/lib"
)

# Install import library (Windows) or static library (Unix)
if(EXISTS "${SOURCE_PATH}/${ISCC_ARTIFACT}/${ISCC_STATIC_LIB_NAME}")
    file(INSTALL "${SOURCE_PATH}/${ISCC_ARTIFACT}/${ISCC_STATIC_LIB_NAME}"
        DESTINATION "${CURRENT_PACKAGES_DIR}/lib"
    )
endif()

# On Windows, DLLs go in bin/
if(VCPKG_TARGET_IS_WINDOWS)
    file(MAKE_DIRECTORY "${CURRENT_PACKAGES_DIR}/bin")
    file(RENAME "${CURRENT_PACKAGES_DIR}/lib/${ISCC_LIB_NAME}" "${CURRENT_PACKAGES_DIR}/bin/${ISCC_LIB_NAME}")
    # Install DLL import lib (iscc_ffi.dll.lib) if present
    if(EXISTS "${SOURCE_PATH}/${ISCC_ARTIFACT}/iscc_ffi.dll.lib")
        file(INSTALL "${SOURCE_PATH}/${ISCC_ARTIFACT}/iscc_ffi.dll.lib"
            DESTINATION "${CURRENT_PACKAGES_DIR}/lib"
        )
    endif()
endif()

# Install license
vcpkg_install_copyright(FILE_LIST "${SOURCE_PATH}/${ISCC_ARTIFACT}/LICENSE")
