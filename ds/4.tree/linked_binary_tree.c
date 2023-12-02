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