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
    set(ISCC_SHA512 "499fe408910c113ce68968c11ff24be30f3b098a016c74813c077e430390b588f3c247be7d45317a896d9efc24156cda5315d0b94f6c3c8fa382821cb62e91d3")
elseif(VCPKG_TARGET_IS_LINUX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-unknown-linux-gnu")
    set(ISCC_LIB_NAME "libiscc_ffi.so")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "58b7398cbccd672c0001fb0a8dc96a867aa7d41d05de4846a1ce247451ba9b0d9bfeb7a5a3ea7d828f6722e699f1b2f75451cf6d1de6d818f841594c0eca109c")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "b69a172d16ae10df4a6dcf21bf582046f623d9ae92746a119a39d5f71ac1600519b06e35d4d3e0f5d923753d15a848a82c961b2eab86a4afa22ffec5b94e68cd")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "ce20e8c5dfa165b0d767c593751f5d11df8f2b5e2cb04fc0886cde7cdda7ecccba21a5467bf2ab1f969292979251fb33d383cffe2f56d91e904f4996b643f700")
elseif(VCPKG_TARGET_IS_WINDOWS AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-pc-windows-msvc")
    set(ISCC_LIB_NAME "iscc_ffi.dll")
    set(ISCC_STATIC_LIB_NAME "iscc_ffi.lib")
    set(ISCC_ARCHIVE_EXT ".zip")
    set(ISCC_SHA512 "ae08e508dbf09a0dd0a067270788f05f74c11bf0e37df9d3f84ece3a252510249db8ce2367f7c39e0fef9415492bd491aed69e633b89d46cafce306938a6c7ce")
else()
    message(FATAL_ERROR "Unsupported platform: ${VCPKG_TARGET_ARCHITECTURE}-${VCPKG_CMAKE_SYSTEM_NAME}")
endif()

set(ISCC_VERSION "${VERSION}")
set(ISCC_ARTIFACT "iscc-ffi-v${ISCC_VERSION}-${ISCC_TARGET}")
set(ISCC_URL "https://github.com/iscc/iscc-lib/releases/download/v${ISCC_VERSION}/${ISCC_ARTIFACT}${ISCC_ARCHIVE_EXT}")

vcpkg_download_distfile(ARCHIVE
    URLS "${ISCC_URL}"
    FILENAME "${ISCC_ARTIFACT}${ISCC_ARCHIVE_EXT}"
    SHA512 "${ISCC_SHA512}"
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
