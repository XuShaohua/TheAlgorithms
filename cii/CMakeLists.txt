cmake_minimum_required(VERSION 3.20)
project(cii C CXX)

set(CMAKE_C_STANDARD 11)

option(BUILD_TESTING "" ON)

set(SOURCE_FILES
    src/assert.c
    src/atom.c
    src/except.c
    src/hash.c
    src/list.c
    src/mem.c
    src/table.c
    )
set(HEADER_FILES
    include/cii/assert.h
    include/cii/atom.h
    include/cii/except.h
    include/cii/hash.h
    include/cii/list.h
    include/cii/mem.h
    include/cii/table.h
    )

add_library(cii_static STATIC
    ${SOURCE_FILES}
    ${HEADER_FILES}
    )
target_include_directories(cii_static
    PUBLIC include
    )

if (BUILD_TESTING)
#  add_executable(cii_mem_test
#      tests/mem_test.c
#      tests/mem_test.h
#      )
#  target_link_libraries(cii_mem_test
#      cii_static
#      )

  add_executable(cii_list_test
      tests/list_test.c
      )
  target_link_libraries(cii_list_test
      cii_static
      )

  add_executable(cii_table_test
      tests/table_test.c
      tests/get_word.c
      tests/get_word.h
      tests/wf.c
      tests/wf.h
      )
  target_link_libraries(cii_table_test
      cii_static
      )

  add_executable(cii_table_test_cpp
      tests/word_count.cpp
      )
endif (BUILD_TESTING)