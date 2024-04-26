// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

static struct TreeNode* treeNew(int val) {
  struct TreeNode* node = malloc(sizeof(struct TreeNode));
  assert(node != NULL);
  node->val = val;
  node->left = NULL;
  node->right = NULL;
  return node;
}

static struct TreeNode* treeFromArrayHelper(int arr[], int len, int index) {
  if (index >= len) {
    return NULL;
  }

  if (arr[index] == INT_MIN) {
    return NULL;
  }

  struct TreeNode* node = treeNew(arr[index]);
  node->left = treeFromArrayHelper(arr, len, index * 2 + 1);
  node->right = treeFromArrayHelper(arr, len, index * 2 + 2);
  return node;
}

static struct TreeNode* treeFromArray(int arr[], int len) {
  assert(len >= 0);
  struct TreeNode* root = treeFromArrayHelper(arr, len, 0);

  return root;
}

static void treeFree(struct TreeNode* root) {
  if (root != NULL) {
    treeFree(root->left);
    treeFree(root->right);
    free(root);
  }
}

static void treeTraversePreOrder(
                    struct TreeNode* node,
                    void (*apply)(struct TreeNode* node, void* user_data),
                    void* user_data) {
  if (node != NULL) {
    apply(node, user_data);
    treeTraversePreOrder(node->left, apply, user_data);
    treeTraversePreOrder(node->right, apply, user_data);
  }
}

static void debugTreeNodeHelper(struct TreeNode* node) {
  if (node != NULL) {
    printf("%d, ", node->val);
    debugTreeNodeHelper(node->left);
    debugTreeNodeHelper(node->right);
  } else {
    printf("null, ");
  }
}

static void debugTreeNode(struct TreeNode* root) {
  assert(root != NULL);
  debugTreeNodeHelper(root);
  printf("\n");
}

static struct TreeNode* treeGetRightLeaf(struct TreeNode* root) {
  if (root == NULL) {
    return NULL;
  }
  while (root != NULL) {
    if (root->right == NULL) {
      return root;
    }
    root = root->right;
  }
  return NULL;
}

static void flatten(struct TreeNode* root) {
  while (root != NULL) {
    if (root->left != NULL) {
      struct TreeNode* right_leaf = treeGetRightLeaf(root->left);
      right_leaf->right = root->right;
      root->right = root->left;
      root->left = NULL;
    }
    root = root->right;
  }
}

static void flatten2(struct TreeNode* root) {
  while (root != NULL) {
    if (root->left != NULL) {
      struct TreeNode* right_leaf = root->left;
      while (right_leaf->right != NULL) {
        right_leaf = right_leaf->right;
      }

      right_leaf->right = root->right;
      root->right = root->left;
      root->left = NULL;
    }
    root = root->right;
  }
}

static void checkSolution(int arr[], int len) {
  struct TreeNode* root = treeFromArray(arr, len);
  flatten(root);
  //flatten2(root);
  debugTreeNode(root);

  treeFree(root);
}

int main() {
  int arr[] = {1, 2, 5, 3, 4, INT_MIN, 6};
  //int arr[] = {1, 2, INT_MIN, 3};
  checkSolution(arr, sizeof(arr) / sizeof(arr[0]));

  return 0;
}
