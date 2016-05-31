cmake_minimum_required(VERSION 3.0)
project(changeme)

set(METAMOD_PATH "" CACHE PATH "Path to Metamod-p source root")

if(NOT METAMOD_PATH)
    message(FATAL_ERROR "METAMOD_PATH must be defined.")
endif()

#HLSDK includes
include_directories("${METAMOD_PATH}/hlsdk")
include_directories("${METAMOD_PATH}/hlsdk/engine")
include_directories("${METAMOD_PATH}/hlsdk/common")
include_directories("${METAMOD_PATH}/hlsdk/pm_shared")
include_directories("${METAMOD_PATH}/hlsdk/dlls")

#metamod includes
include_directories("${METAMOD_PATH}/metamod")

#our includes
include_directories("${PROJECT_SOURCE_DIR}/include")

#set CMAKE_BUILD_TYPE
if(CMAKE_BUILD_TYPE STREQUAL "")
    set(CMAKE_BUILD_TYPE Debug)
endif()

#debug/release flags
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_definitions(-DOPT_TYPE="debugging")
else()
    add_definitions(-DOPT_TYPE="optimized+meta_debug-disabled" -D__BUILD_FAST_METAMOD__)
endif()

#compiler flags
set(gcc_flags "-m32 -Wall -Werror -Wno-unused -Wno-deprecated-writable-strings -Wno-unknown-pragmas")

if(CMAKE_C_COMPILER MATCHES "gcc" OR CMAKE_C_COMPILER MATCHES "clang")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 ${gcc_flags}")
    set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG}")
endif()

if(CMAKE_CXX_COMPILER MATCHES "g\\+\\+" OR CMAKE_CXX_COMPILER MATCHES "clang")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 ${gcc_flags}")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG}")
endif()

unset(gcc_flags)

#compiler defines
if(${UNIX})
    add_definitions(-Dlinux)
endif() #_WIN32 should be defined automatically

#place all binaries at top of bin folder
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG "${CMAKE_BINARY_DIR}")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE "${CMAKE_BINARY_DIR}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG "${CMAKE_BINARY_DIR}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE "${CMAKE_BINARY_DIR}")

add_subdirectory(src)
