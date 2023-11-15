
add_library(array-list STATIC
    array_list.c
    list.h
    )

add_library(linked-list STATIC
    linked_list.c
    list.h
    )

add_library(array-stack STATIC
    array_stack.c
    stack.h
    )

add_library(linked-stack STATIC
    linked_stack.c
    stack.h
    )

add_executable(calculator-stack
    calculator_stack.c
    )
target_link_libraries(calculator-stack
    array-stack
    )

add_library(array-queue STATIC
    array_queue.c
    queue.h
    )
add_library(linked-queue STATIC
    linked_queue.c
    queue.h
    )

# Unittest
enable_testing()
find_package(GTest REQUIRED)
add_executable(array-list-test
    list_test.cpp
    list.h
    )
target_link_libraries(array-list-test
    GTest::gtest_main
    array-list
    )

add_executable(linked-list-test
    list_test.cpp
    list.h
    )
target_link_libraries(linked-list-test
    GTest::gtest_main
    linked-list
    )

add_executable(array-stack-test
    stack_test.cpp
    stack.h
    )
target_link_libraries(array-stack-test
    GTest::gtest_main
    array-stack
    )

add_executable(linked-stack-test
    stack_test.cpp
    stack.h
    )
target_link_libraries(linked-stack-test
    GTest::gtest_main
    linked-stack
    )

add_executable(array-queue-test
    queue_test.cpp
    queue.h
    )
target_link_libraries(array-queue-test
    GTest::gtest_main
    array-queue
    )

add_executable(linked-queue-test
    queue_test.cpp
    queue.h
    )
target_link_libraries(linked-queue-test
    GTest::gtest_main
    linked-queue
    )