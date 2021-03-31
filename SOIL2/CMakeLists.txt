cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

project(soil2 CXX)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

if (APPLE)
    find_library(Foundation Foundation)
    target_link_libraries(${PROJECT_NAME} ${Foundation})
endif()

find_library(OpenGL OpenGL)
target_link_libraries(${PROJECT_NAME} ${OpenGL})


install(TARGETS ${PROJECT_NAME}
         CONFIGURATIONS Debug
         RUNTIME DESTINATION Debug/bin)
install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Release
        RUNTIME DESTINATION Release/bin)
