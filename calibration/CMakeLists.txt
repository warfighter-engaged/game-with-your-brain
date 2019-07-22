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


# We need the SDL2 DLL copied over to the executable
# Windows only
IF (WIN32)
    if(CMAKE_SIZEOF_VOID_P EQUAL 8)
        set(VC_LIB_PATH_SUFFIX lib/x64)
    else()
        set(VC_LIB_PATH_SUFFIX lib/x86)
    endif()
    add_custom_command(TARGET SDL2Test POST_BUILD        # Adds a post-build event to MyTest
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2DIR}/${VC_LIB_PATH_SUFFIX}/SDL2.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:SDL2Test>)                 # <--this is out-file path
ENDIF()