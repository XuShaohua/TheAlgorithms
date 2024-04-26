// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <vector>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

class Solution {
public:
  static
  void helper(TreeNode* root, std::vector<int>& values) {
    if (root == nullptr) {
      return;
    }
    values.push_back(root->val);
    helper(root->left, values);
    helper(root->right, values);
  }

  std::vector<int> preorderTraversal(TreeNode* root) {
    std::vector<int> values;
    helper(root, values);
    return values;
  }
};


int main() {

  return 0;
}
