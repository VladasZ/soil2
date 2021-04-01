cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

include(../utils.cmake)

project(soil2)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

if (UNIX)
    if(APPLE)
        find_library(GL OpenGL)
        target_link_libraries(${PROJECT_NAME} ${GL})
        find_library(FOUND Foundation)
        target_link_libraries(${PROJECT_NAME} ${FOUND})
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



# execute_process (
#     COMMAND dir
#     OUTPUT outVar
# )
# message(${outVar})

execute_process(COMMAND ${CMAKE_COMMAND} -E echo bar OUTPUT_VARIABLE FOO)

install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Debug
        ARCHIVE DESTINATION Debug/bin
        #LIBRARY DESTINATION Debug/bin
        #RUNTIME DESTINATION Debug/bin
        )

install(TARGETS ${PROJECT_NAME}
        CONFIGURATIONS Release
        ARCHIVE DESTINATION Release/bin
        #LIBRARY DESTINATION Release/bin
        #RUNTIME DESTINATION Release/bin
        )
