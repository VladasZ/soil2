cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

include(../utils.cmake)

project(soil2)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

if (APPLE)
    find_library(FOUND Foundation)
	find_library(GL OpenGL)
    target_link_libraries(${PROJECT_NAME} ${FOUND})
	target_link_libraries(${PROJECT_NAME} ${GL})
else()
	find_package(OpenGL REQUIRED)
	target_link_libraries(${PROJECT_NAME} ${OPENGL_gl_LIBRARY})
endif()


install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Debug
        RUNTIME DESTINATION Debug/bin)
install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Release
        RUNTIME DESTINATION Release/bin)
