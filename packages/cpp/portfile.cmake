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
    set(ISCC_SHA512 "ebed9b508cdc93330dbd026b9ea01deaa9c161613c31f35eaa6a7c0798d7cf8937331218b16556847a9831cf5d38507a24e36f1bf393753f5037594b5e09126a")
elseif(VCPKG_TARGET_IS_LINUX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-unknown-linux-gnu")
    set(ISCC_LIB_NAME "libiscc_ffi.so")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "93344da5962ffbab6ec82103bd631125d80f1ae221c2bcac5e15588c95ecbe1653ae9f714f12914c6d272e3158e2a4804f08248c740f0fb8d1da2c8815ffc2a2")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "arm64")
    set(ISCC_TARGET "aarch64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "53b15a771aef1cb2ede2a3eed0ae29eb9b273cbf98c1fa38fd358f5f6922e8d2668179bf5aa4a3c44e48768e848d86efa35f579895434a4ad28dafebd46e0b3b")
elseif(VCPKG_TARGET_IS_OSX AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-apple-darwin")
    set(ISCC_LIB_NAME "libiscc_ffi.dylib")
    set(ISCC_STATIC_LIB_NAME "libiscc_ffi.a")
    set(ISCC_ARCHIVE_EXT ".tar.gz")
    set(ISCC_SHA512 "c09e7615940400eb1fe261e162ddd2ded69888091cc45952f6aaec2ca350a9d7ae478f263c71f839ef17bcd6a23fc72fbb867de2984aaf687fa9b776b2d35ad4")
elseif(VCPKG_TARGET_IS_WINDOWS AND VCPKG_TARGET_ARCHITECTURE STREQUAL "x64")
    set(ISCC_TARGET "x86_64-pc-windows-msvc")
    set(ISCC_LIB_NAME "iscc_ffi.dll")
    set(ISCC_STATIC_LIB_NAME "iscc_ffi.lib")
    set(ISCC_ARCHIVE_EXT ".zip")
    set(ISCC_SHA512 "b46222c31783d43c9f88c4ce255fd54677f24f22325658ee18eb90e9cfd04ca9fa6fa8a10a36e3f80fa22f5350a41cb64f637fa7b9bbf1697f710afd8003b775")
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
