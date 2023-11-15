// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "binary_tree.h"

#include <assert.h>
#include <stddef.h>
#include <string.h>

typedef struct tree_node_s tree_node_t;

struct tree_node_s {
  tree_node_t* left;
  tree_node_t* right;
  element_type value;
};

struct binary_tree_s {
  tree_node_t* root;
  size_t size;
};

binary_tree_t* binary_tree_new() {

}

void binary_tree_free(binary_tree_t* tree) {
  
}