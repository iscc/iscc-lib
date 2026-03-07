# CMake configuration for the iscc C++17 header-only wrapper.
#
# Usage:
#   cmake -B build -DFFI_LIB_DIR=../../target/debug
#   cmake --build build
#   LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc

cmake_minimum_required(VERSION 3.14)
project(iscc-cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Header-only interface library
add_library(iscc INTERFACE)
add_library(iscc::iscc ALIAS iscc)
target_include_directories(iscc INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/../../crates/iscc-ffi/include
)

add_subdirectory(tests)
