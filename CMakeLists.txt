cmake_minimum_required(VERSION 3.21.0)
project(cryptography-labs VERSION 0.1)

include(CTest)
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
if (NOT DEFINED IS_SUBDIR)
    include(CPack)
    set(IS_SUBDIR ON)
endif ()
enable_testing()

enable_language(CXX C)

set(CMAKE_SOURCE_DIR ${PROJECT_SOURCE_DIR})
set(CHISEL_X 1)

message(STATUS "${PROJECT_NAME} started building on ${CMAKE_SYSTEM_NAME} using ${CMAKE_GENERATOR}!")

# Include
set(INCLUDE_DIRS
        "${CMAKE_SOURCE_DIR}/src"
        "${CMAKE_SOURCE_DIR}/src/include")
message(STATUS "Include dirs: ${INCLUDE_DIRS}")
include_directories(${INCLUDE_DIRS})

# Links
link_libraries(m)

file(GLOB LABS_PATH "${CMAKE_SOURCE_DIR}/src/*")

foreach(LAB_PATH ${LABS_PATH})
    string(REGEX REPLACE ".+/(.+)" "\\1" LAB ${LAB_PATH})
    if("include" STREQUAL ${LAB})
        continue()
    endif()
    include_directories("${CMAKE_SOURCE_DIR}/src/${LAB}")
    file(GLOB_RECURSE SOURCE_FILES "${CMAKE_SOURCE_DIR}/src/${LAB}/*.c" "${CMAKE_SOURCE_DIR}/src/${LAB}/*.cpp")
    add_executable(${LAB} ${SOURCE_FILES})
    file(GLOB TESTS_PATH "${CMAKE_SOURCE_DIR}/src/${LAB}/tests/*.c" "${CMAKE_SOURCE_DIR}/src/${LAB}/tests/*.cpp")
    foreach(TEST_PATH ${TESTS_PATH})
        string(REGEX REPLACE ".*/(.+)\\..*" "\\1" TEST ${TEST_PATH})
        add_executable(${LAB}-${TEST} ${TEST_PATH})
        add_test(NAME ${LAB}-${TEST} COMMAND ${LAB}-${TEST})
    endforeach()
    add_test(NAME ${LAB} COMMAND ${LAB})
endforeach()
