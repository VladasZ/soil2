cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

include(../utils.cmake)

project(soil2)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

if (APPLE)
    find_library(Foundation Foundation)
    target_link_libraries(${PROJECT_NAME} ${Foundation})
endif()

find_library(OpenGL OpenGL)
target_link_libraries(${PROJECT_NAME} ${OpenGL})
