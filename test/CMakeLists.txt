cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

project(test)
add_catalog_recursive(${PROJECT_SOURCE_DIR} / test_SOURCE)
add_executable(${PROJECT_NAME} ${test_SOURCE})

target_link_libraries(soil2)
