cmake_minimum_required(VERSION 3.20)
project(cii C)

set(CMAKE_C_STANDARD 11)

set(SOURCE_FILES
    src/assert.c
    src/atom.c
    src/except.c
    src/hash.c
    src/mem.c
    )
set(HEADER_FILES
    include/cii/assert.h
    include/cii/atom.h
    include/cii/except.h
    include/cii/hash.h
    include/cii/mem.h
    )

add_library(cii_static STATIC
    ${SOURCE_FILES}
    ${HEADER_FILES}
    )
target_include_directories(cii_static
    PUBLIC include
    )

add_executable(mem_test
    src/mem_test.c
    )
target_link_libraries(mem_test
    cii_static
    )