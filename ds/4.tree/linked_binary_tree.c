// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "binary_tree.h"

#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

typedef struct tree_node_s tree_node_t;

struct tree_node_s {
  tree_node_t* left;
  tree_node_t* right;
  void* value;
};

struct binary_tree_s {
  size_t size;
  tree_node_t* root;
};

binary_tree_t* binary_tree_new() {
  binary_tree_t* tree = malloc(sizeof(binary_tree_t));
  assert(tree != NULL);
  tree->size = 0;
  tree->root = NULL;

  return tree;
}

void binary_tree_free(binary_tree_t* tree) {
  assert(tree != NULL);
  // TODO(Shaohua): deallocate tree nodes.
  free(tree);
}

size_t binary_tree_size(binary_tree_t* tree) {
  assert(tree != NULL);
  return tree->size;
}

bool binary_tree_is_empty(binary_tree_t* tree) {
  assert(tree != NULL);
  return tree->size > 0;
}

static void tree_node_traverse_in_order(tree_node_t* node,
                                       void (*apply)(void* value, void* user_data),
                                       void* user_data) {
  if (node != NULL) {
    tree_node_traverse_in_order(node->left, apply, user_data);
    apply(node->value, user_data);
    tree_node_traverse_in_order(node->right, apply, user_data);
  }
}

static void tree_node_traverse_pre_order(tree_node_t* node,
                                       void (*apply)(void* value, void* user_data),
                                       void* user_data) {
  if (node != NULL) {
    apply(node->value, user_data);
    tree_node_traverse_pre_order(node->left, apply, user_data);
    tree_node_traverse_pre_order(node->right, apply, user_data);
  }
}

static void tree_node_traverse_post_order(tree_node_t* node,
                                       void (*apply)(void* value, void* user_data),
                                       void* user_data) {
  if (node != NULL) {
    tree_node_traverse_post_order(node->left, apply, user_data);
    tree_node_traverse_post_order(node->right, apply, user_data);
    apply(node->value, user_data);
  }
}

void binary_tree_traverse(binary_tree_t* tree,
                          tree_traverse_mode mode,
                          void (*apply)(void* value, void* user_data),
                          void* user_data) {
  assert(tree != NULL);
  switch (mode) {
    case kTraverseInOrder: {
      tree_node_traverse_in_order(tree->root, apply, user_data);
      break;
    }
    case kTraversePreOrder: {
      tree_node_traverse_pre_order(tree->root, apply, user_data);
      break;
    }
    case kTraversePostOrder: {
      tree_node_traverse_post_order(tree->root, apply, user_data);
      break;
    }
    default: {
      fprintf(stderr, "Invalid traverse mode\n");
      assert(0);
    }
  }
}
