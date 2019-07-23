# Minimum CMake version required to generate
# our build system
cmake_minimum_required(VERSION 3.5)

# Name of our Project
project(calibration)

set (CMAKE_CXX_STANDARD 14)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_CURRENT_SOURCE_DIR}/cmake/")

find_package(SDL2 REQUIRED)

include_directories(${SDL2_INCLUDE_DIRS})

include_directories(include)

add_executable(calibration
    src/main.cpp
    src/game_state.cpp
)


target_link_libraries(calibration ${SDL2_LIBRARIES})


# We need the SDL2 DLL copied over to the executable
# Windows only
IF (WIN32)
    if(CMAKE_SIZEOF_VOID_P EQUAL 8)
        set(VC_LIB_PATH_SUFFIX lib/x64)
    else()
        set(VC_LIB_PATH_SUFFIX lib/x86)
    endif()
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2DIR}/${VC_LIB_PATH_SUFFIX}/SDL2.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
ENDIF()

# Copy data files

add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_directory  # which executes "cmake - E copy_if_different..."
        "${CMAKE_CURRENT_SOURCE_DIR}/data"      # <--this is in-file
            "$<TARGET_FILE_DIR:calibration>/data")
