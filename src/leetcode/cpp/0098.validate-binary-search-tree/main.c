// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <limits.h>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

struct TreeNode* findMin(struct TreeNode* root) {
  if (root == NULL) {
    return NULL;
  }
  // traverse
  struct TreeNode* left = findMin(root->left);
  struct TreeNode* right = findMin(root->left);
  struct TreeNode* min_node = root;
  if (left != NULL && left->val < min_node->val) {
    min_node = left;
  }
  if (right != NULL && right->val < min_node->val) {
    min_node = right;
  }
  return min_node;
}

struct TreeNode* findMax(struct TreeNode* root) {
  if (root == NULL) {
    return NULL;
  }
  struct TreeNode* left = findMax(root->left);
  struct TreeNode* right = findMax(root->left);
  struct TreeNode* max_node = root;
  if (left != NULL && left->val > max_node->val) {
    max_node = left;
  }
  if (right != NULL && right->val > max_node->val) {
    max_node = right;
  }
  return max_node;
}

bool isValidBST(struct TreeNode* root) {
  if (root == NULL) {
    return true;
  }
  struct TreeNode* left_max = findMax(root->left);
  if (left_max != NULL && left_max->val >= root->val) {
    return false;
  }
  struct TreeNode* right_min = findMin(root->right);
  if (right_min != NULL && right_min->val <= root->val) {
    return false;
  }
  return true;
}

bool isValidBST2Helper(struct TreeNode* root, long min, long max) {
  if (root == NULL) {
    return true;
  }
  if (root->val < max && root->val > min) {
    return isValidBST2Helper(root->left, min, root->val) &&
      isValidBST2Helper(root->right, root->val, max);
  }
  return false;
}

bool isValidBST2(struct TreeNode* root) {
  return isValidBST2Helper(root, LONG_MIN, LONG_MAX);
}

int main() {
  return 0;
}
