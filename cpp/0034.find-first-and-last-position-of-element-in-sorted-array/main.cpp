// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <vector>

class Solution {
 public:
  static
  std::vector<int> searchRange(std::vector<int>& nums, int target) {
    std::vector<int> range;
    range.push_back(-1);
    range.push_back(-1);
    if (nums.empty()) {
      return range;
    }

    int left = 0;
    int right = nums.size() - 1;

    while (left <= right) {
      int middle = left + (right - left) / 2;
      if (nums[middle] == target) {
        for (int i = middle; i >= 0 && nums[i] == target; --i) {
          range[0] = i;
        }
        for (int i = middle; i < nums.size() && nums[i] == target; ++i) {
          range[1] = i;
        }
        break;
      } else if (nums[middle] < target) {
        left = middle + 1;
      } else {
        right = middle - 1;
      }
    }

    return range;
  }
};

void checkSolution(int* nums, int numsSize, int target, int left, int right) {
  std::vector<int> vec{nums, nums + numsSize};
  std::vector<int> range = Solution::searchRange(vec, target);
  assert(range[0] == left);
  assert(range[1] == right);
}

int main() {
  int arr1[] = {5,7,7,8,8,10};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 8, 3, 4);

  int arr2[] = {1};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 1, 0, 0);
  return 0;
}
