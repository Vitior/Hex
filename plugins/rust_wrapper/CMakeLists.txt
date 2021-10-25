cmake_minimum_required(VERSION 3.16)
project(rust_wrapper)

set(CMAKE_CXX_STANDARD 20)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC")

add_library(rust_wrapper STATIC
    source/rust_utils.cpp
)

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../../external/cxx.rs ${CMAKE_CURRENT_BINARY_DIR}/external/cxx.rs EXCLUDE_FROM_ALL)

target_include_directories(rust_wrapper PUBLIC include)
target_link_libraries(rust_wrapper PUBLIC libimhex cxx_rs)
