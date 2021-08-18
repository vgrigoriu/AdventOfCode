cmake_minimum_required(VERSION 3.16)

project(aoc_2015_15
        VERSION 1.0
        LANGUAGES CXX)

add_executable(app src/main.cpp)

target_compile_features(app PUBLIC cxx_std_20)
target_compile_options(app PUBLIC -Wall -Wextra -Wpedantic -Werror)
set_target_properties(app
  PROPERTIES
    CXX_EXTENSIONS OFF)
target_include_directories(app PUBLIC include)
