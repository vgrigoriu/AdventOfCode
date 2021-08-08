cmake_minimum_required(VERSION 3.16)

project(aoc_2015_13 CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_COMPILE_FLAGS "-Wall -Wextra -Wpedantic -Werror")

add_executable(aoc_2015_13 src/main.cpp)
