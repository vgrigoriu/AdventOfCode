cmake_minimum_required(VERSION 3.16)

project(aoc_2015_13 CXX)

#set_target_properties(aoc_2015_13 PROPERTIES
set(CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)
set(CXX_EXTENSIONS OFF)
set(COMPILE_FLAGS "-Wall -Wextra -Wpedantic -Werror")
#)

add_executable(Application src/main.cpp)
