// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

bool isSymmetricHelper(struct TreeNode* left, struct TreeNode* right) {
  if (left == NULL && right == NULL) {
    return true;
  }
  if (left == NULL || right == NULL) {
    return false;
  }
  if (left->val != right->val) {
    return false;
  }
  return isSymmetricHelper(left->left, right->right) &&
    isSymmetricHelper(left->right, right->left);
}

bool isSymmetric(struct TreeNode* root) {
  return isSymmetricHelper(root->left, root->right);
}

int main() {
  return 0;
}
