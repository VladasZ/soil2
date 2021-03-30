cmake_minimum_required(VERSION 3.9.2 FATAL_ERROR)

setup_exe(soil2_test)
link_project(soil2)

set_target_properties(${PROJECT_NAME} PROPERTIES FOLDER "tests")
