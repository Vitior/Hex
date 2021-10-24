cmake_minimum_required(VERSION 3.16)
project(cxx_rs)

set(CMAKE_CXX_STANDARD 17)

add_library(cxx_rs INTERFACE)

target_include_directories(cxx_rs INTERFACE include)
