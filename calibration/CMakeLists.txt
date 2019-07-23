# Minimum CMake version required to generate
# our build system
cmake_minimum_required(VERSION 3.5)

# Name of our Project
project(calibration)

set (CMAKE_CXX_STANDARD 14)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_CURRENT_SOURCE_DIR}/cmake/")

find_package(SDL2 REQUIRED)
find_package(SDL2_IMAGE REQUIRED)
find_package(SDL2TTF REQUIRED)

include_directories(${SDL2_INCLUDE_DIRS})
include_directories(${SDL2IMAGE_INCLUDE_DIR})
include_directories(${SDL2TTF_INCLUDE_DIR})

include_directories(include)

add_executable(calibration
    src/main.cpp
    src/game_scene.cpp
    src/game_state.cpp
    src/main_menu_scene.cpp
    src/renderer.cpp
    src/sprite.cpp
    src/vec2.cpp
)


target_link_libraries(calibration ${SDL2_LIBRARIES} ${SDL2TTF_LIBRARY} ${SDL2_IMAGE_LIBRARIES})
target_compile_options(calibration PRIVATE
    $<$<OR:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_ID:GNU>>:
        -Wall -Werror -Wextra>
    $<$<CXX_COMPILER_ID:MSVC>:
        /W4 /WX /analyze /permissive-
        >)


# We need the SDL2 DLL copied over to the executable
# Windows only
IF (WIN32)
    if(CMAKE_SIZEOF_VOID_P EQUAL 8)
        set(VC_LIB_PATH_SUFFIX lib/x64)
    else()
        set(VC_LIB_PATH_SUFFIX lib/x86)
    endif()
    # Copy over SDL2.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2DIR}/${VC_LIB_PATH_SUFFIX}/SDL2.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
    # Copy over SDL2_image.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2IMAGEDIR}/${VC_LIB_PATH_SUFFIX}/SDL2_image.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
    # Copy over SDL2_ttf.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2TTFDIR}/${VC_LIB_PATH_SUFFIX}/SDL2_ttf.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
    # Copy over libfreetype.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2TTFDIR}/${VC_LIB_PATH_SUFFIX}/libfreetype-6.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
    # Copy over zlib1.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2TTFDIR}/${VC_LIB_PATH_SUFFIX}/zlib1.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
    # Copy over libpng16-16.dll
    add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
            "$ENV{SDL2IMAGEDIR}/${VC_LIB_PATH_SUFFIX}/libpng16-16.dll"      # <--this is in-file
            $<TARGET_FILE_DIR:calibration>)                 # <--this is out-file path
ENDIF()

# Copy data files

add_custom_command(TARGET calibration POST_BUILD        # Adds a post-build event to calibration
        COMMAND ${CMAKE_COMMAND} -E copy_directory  # which executes "cmake - E copy_if_different..."
        "${CMAKE_CURRENT_SOURCE_DIR}/data"      # <--this is in-file
            "$<TARGET_FILE_DIR:calibration>/data")
