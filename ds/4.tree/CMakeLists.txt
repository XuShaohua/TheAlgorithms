
add_library(linked-binary-tree-static STATIC
    linked_binary_tree.c
    binary_tree.h
    )

add_executable(binary-tree-test
    binary_tree_test.c
)
target_link_libraries(binary-tree-test
    linked-binary-tree-static
)
