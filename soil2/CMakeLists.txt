cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

include(../utils.cmake)

project(soil2)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

if (UNIX)
    if(APPLE)
        find_library(FOUND Foundation)
        target_link_libraries(${PROJECT_NAME} ${FOUND})
        if(${CMAKE_SYSTEM_NAME} STREQUAL "iOS")
            find_library(OPEN_GLES OpenGLES)
            target_link_libraries(${PROJECT_NAME} ${OPEN_GLES})
        else()
            find_library(GL OpenGL)
            target_link_libraries(${PROJECT_NAME} ${GL})
        endif()
    else()
        find_library(GL OpenGL)
        target_link_libraries(${PROJECT_NAME} ${GL})
        find_library(GLX GLX)
        target_link_libraries(${PROJECT_NAME} ${GLX})
    endif()
else()
    find_package(OpenGL REQUIRED)
    target_link_libraries(${PROJECT_NAME} ${OPENGL_gl_LIBRARY})
endif()

install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Debug
        ARCHIVE DESTINATION .
        #LIBRARY DESTINATION Debug/bin
        #RUNTIME DESTINATION Debug/bin
        )

install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Release
        ARCHIVE DESTINATION .
        #LIBRARY DESTINATION Release/bin
        #RUNTIME DESTINATION Release/bin
        )
