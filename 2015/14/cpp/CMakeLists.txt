cmake_minimum_required(VERSION 3.16)

project(aoc_2015_14
        VERSION 1.0
        LANGUAGES CXX)

add_executable(aoc_2015_14 src/main.cpp src/reindeer.cpp include/reindeer.h)

target_compile_features(aoc_2015_14 PUBLIC cxx_std_20)
target_compile_options(aoc_2015_14 PUBLIC -Wall -Wextra -Wpedantic -Werror)
set_target_properties(aoc_2015_14
  PROPERTIES
    CXX_EXTENSIONS OFF)
target_include_directories(aoc_2015_14 PUBLIC include)
