# Minimum CMake version required to generate
# our build system
cmake_minimum_required(VERSION 3.5)

# Name of our Project
project(hello)

# add_executable creates an executable with
# The given name. In our case it is "Hello"
# Source files are given as parameters. In our
# Case we only have one source file Hello.cpp
add_executable(Hello src/Hello.cpp)

project(SDL2Test)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

find_package(SDL2 REQUIRED)

include_directories(${SDL2_INCLUDE_DIRS})

add_executable(SDL2Test src/SDL2Test.cpp)
target_link_libraries(SDL2Test ${SDL2_LIBRARIES})
