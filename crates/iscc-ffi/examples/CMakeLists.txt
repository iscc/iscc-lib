# CMakeLists.txt — build the iscc_sum example against the iscc-ffi library.
#
# Build against a pre-built iscc-ffi library:
#   cmake -B build -DCMAKE_PREFIX_PATH=/path/to/iscc-ffi
#   cmake --build build
#
# Build against a Cargo-built library (from repo root):
#   cargo build -p iscc-ffi --release
#   cmake -B crates/iscc-ffi/examples/build \
#         -S crates/iscc-ffi/examples \
#         -DCMAKE_LIBRARY_PATH=target/release
#   cmake --build crates/iscc-ffi/examples/build

cmake_minimum_required(VERSION 3.14)
project(iscc_sum_example C)

add_executable(iscc_sum iscc_sum.c)
target_include_directories(iscc_sum PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)
target_link_libraries(iscc_sum PRIVATE iscc_ffi)
