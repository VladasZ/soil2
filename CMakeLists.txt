cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

project(soil2)

add_subdirectory(soil2)

if(NOT ${CMAKE_SYSTEM_NAME} STREQUAL "iOS")
    add_subdirectory(test)
endif()
