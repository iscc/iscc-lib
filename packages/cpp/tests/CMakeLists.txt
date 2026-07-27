# Test build configuration for the iscc C++ wrapper.
#
# Accepts FFI_LIB_DIR as a CMake variable pointing to the directory
# containing the compiled FFI shared library (libiscc_ffi.so / .dylib / .dll).
#
# Optional: -DSANITIZE_ADDRESS=ON to enable AddressSanitizer.

# Path to the compiled FFI shared library
set(FFI_LIB_DIR "" CACHE PATH "Directory containing the iscc_ffi shared library")

# AddressSanitizer option
option(SANITIZE_ADDRESS "Enable AddressSanitizer" OFF)

add_executable(test_iscc test_iscc.cpp)
target_link_libraries(test_iscc PRIVATE iscc::iscc)

# Generated Unicode 16.0.0 boundary vectors (unicode_boundary_vectors.h) — test-only
# include directory; never add this to the public iscc INTERFACE target.
target_include_directories(test_iscc PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../../../crates/iscc-ffi/tests)

# Link against the iscc_ffi shared library
if(FFI_LIB_DIR)
    target_link_directories(test_iscc PRIVATE ${FFI_LIB_DIR})
endif()
target_link_libraries(test_iscc PRIVATE iscc_ffi)

# Platform-specific link dependencies
if(UNIX AND NOT APPLE)
    target_link_libraries(test_iscc PRIVATE pthread dl m)
endif()

# AddressSanitizer
if(SANITIZE_ADDRESS)
    target_compile_options(test_iscc PRIVATE -fsanitize=address -fno-omit-frame-pointer)
    target_link_options(test_iscc PRIVATE -fsanitize=address)
endif()
