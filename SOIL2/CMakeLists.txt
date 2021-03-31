cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)


project(soil2)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / soil2_SOURCE)
add_library(${PROJECT_NAME} ${soil2_SOURCE})

#install(TARGETS ${PROJECT_NAME}
 #       CONFIGURATIONS Debug
 #       RUNTIME DESTINATION Debug/bin)
#install(TARGETS ${PROJECT_NAME}
#        CONFIGURATIONS Release
#        RUNTIME DESTINATION Release/bin)