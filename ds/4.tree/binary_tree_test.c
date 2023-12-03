// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <stdio.h>

#include "binary_tree.h"

int main(void) {
  binary_tree_t* tree = binary_tree_new();

  binary_tree_free(tree);
  tree = NULL;

  return 0;
}
