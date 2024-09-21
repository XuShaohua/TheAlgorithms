// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>
#include <climits>

#include <iostream>
#include <stack>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<int> nextGreaterElement(std::vector<int>& nums1, std::vector<int>& nums2) {
  std::stack<int> monotonic_stack;
  std::unordered_map<int, int> map;
  int max_num = INT_MAX;

  // 构造递增式单调栈
  for (int num2 : nums2) {
    if (num2 < max_num) {
      // 将较小的元素入栈
      max_num = num2;
      monotonic_stack.push(num2);
    } else {
      // 将较小的元素出栈
      while (!monotonic_stack.empty() && monotonic_stack.top() < num2) {
        const int top = monotonic_stack.top();
        monotonic_stack.pop();
        map.emplace(top, num2);
      }
      // 将当前元素入栈
      monotonic_stack.push(num2);
    }
  }

  std::vector<int> out;
  for (int num1 : nums1) {
    auto iter = map.find(num1);
    if (iter == map.cend()) {
      out.push_back(-1);
    } else {
      out.push_back(iter->second);
    }
  }

  return out;
}

void checkSolution() {
  {
    std::vector<int> nums1 = {4, 1, 2};
    std::vector<int> nums2 = {1, 3, 4, 2};
    std::vector<int> expected = {-1, 3, -1};
    assert(nextGreaterElement(nums1, nums2) == expected);
  }

  {
    std::vector<int> nums1 = {2, 4};
    std::vector<int> nums2 = {1, 2, 3, 4};
    std::vector<int> expected = {3, -1};
    assert(nextGreaterElement(nums1, nums2) == expected);
  }

  {
    std::vector<int> nums1 = {1, 3, 5, 2, 4};
    std::vector<int> nums2 = {6, 5, 4, 3, 2, 1, 7};
    std::vector<int> expected = {7, 7, 7, 7, 7};
    assert(nextGreaterElement(nums1, nums2) == expected);
  }
}

int main() {
  checkSolution();

  return 0;
}
