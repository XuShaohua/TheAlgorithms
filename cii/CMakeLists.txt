cmake_minimum_required(VERSION 3.20)
project(cii C)

set(CMAKE_C_STANDARD 11)

set(SOURCE_FILES
    src/atom.c
    )
set(HEADER_FILES
    include/cii/atom.h
    )

add_library(cii STATIC
    ${SOURCE_FILES}
    ${HEADER_FILES}
    )
target_include_directories(cii
    PUBLIC include
    )