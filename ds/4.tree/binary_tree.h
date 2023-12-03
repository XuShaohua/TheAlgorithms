// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef BINARY_TREE_H_
#define BINARY_TREE_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct binary_tree_s binary_tree_t;

/**
 * Create an empty binary tree.
 *
 * @return
 */
extern binary_tree_t* binary_tree_new();

/**
 * Deallocate tree nodes in binary tree and free the binary tree descriptor.
 *
 * @param tree
 */
extern void binary_tree_free(binary_tree_t* tree);

/**
 * Get number of elements in binary tree.
 *
 * @param tree
 * @return
 */
extern size_t binary_tree_size(binary_tree_t* tree);

/**
 * Returns true if binary tree has no node.
 *
 * @param tree
 * @return
 */
extern bool binary_tree_is_empty(binary_tree_t* tree);

enum tree_traverse_mode {
  kTraverseInOrder = 0,
  kTraversePreOrder,
  kTraversePostOrder,
};

/**
 * Calls |apply| function for each node in binary tree.
 *
 * @param tree
 * @param apply
 * @param user_data
 */
extern void binary_tree_traverse(binary_tree_t* tree,
                                 tree_traverse_mode mode,
                                 void (*apply)(void* value, void* user_data),
                                 void* user_data);

#ifdef __cplusplus
}
#endif

#endif  // BINARY_TREE_H_
