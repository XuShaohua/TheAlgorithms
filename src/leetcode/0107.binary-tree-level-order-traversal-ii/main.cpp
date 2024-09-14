// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <algorithm>
#include <queue>
#include <vector>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

class Solution {
 public:
  std::vector<std::vector<int>> levelOrderBottom(TreeNode* root) {
    std::vector<std::vector<int>> out;
    if (root == nullptr) {
      return out;
    }

    std::queue<TreeNode*> queue;
    queue.push(root);
    while (!queue.empty()) {

      // current level
      size_t size = queue.size();
      std::vector<int> values;
      for (size_t i = 0; i < size; ++i) {
        TreeNode* node = queue.front();
        queue.pop();
        values.push_back(node->val);
        if (node->left != nullptr) {
          queue.push(node->left);
        }
        if (node->right != nullptr) {
          queue.push(node->right);
        }
      }
      out.push_back(values);
    }

    std::reverse(out.begin(), out.end());
    return out;
  }
};

int main() {
  return 0;
}
