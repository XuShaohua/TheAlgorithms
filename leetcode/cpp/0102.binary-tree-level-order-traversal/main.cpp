// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>

#include <queue>
#include <vector>

struct TreeNode {
  int val;
  struct TreeNode* left;
  struct TreeNode* right;
};

class Solution {
 public:
  void levelOrderHelper(std::queue<TreeNode*> queue, std::vector<std::vector<int>>& out) {
    std::queue<TreeNode*> next_queue;
    std::vector<int> values;

    while (!queue.empty()) {
      TreeNode* node = queue.front();
      queue.pop();
      values.push_back(node->val);
      if (node->left != nullptr) {
        next_queue.push(node->left);
      }
      if (node->right != nullptr) {
        next_queue.push(node->right);
      }
    }
    out.push_back(values);

    if (!next_queue.empty()) {
      levelOrderHelper(next_queue, out);
    }
  }

  std::vector<std::vector<int>> levelOrder(TreeNode* root) {
    std::vector<std::vector<int>> out;
    if (root == nullptr) {
      return out;
    }

    std::queue<TreeNode*> queue;
    queue.push(root);
    levelOrderHelper(queue, out);

    return out;
  }

  std::vector<std::vector<int>> levelOrderIterative(TreeNode* root) {
    std::vector<std::vector<int>> out;
    if (root == nullptr) {
      return out;
    }

    std::queue<TreeNode*> queue;
    queue.push(root);

    while (!queue.empty()) {
      size_t size = queue.size();
      std::vector<int> values;
      // current level.
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

    return out;
  }
};

int main() {
  return 0;
}
