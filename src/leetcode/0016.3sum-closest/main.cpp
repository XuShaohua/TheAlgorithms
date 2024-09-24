// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <climits>

#include <algorithm>
#include <vector>

class Solution {
 public:
  static
  int threeSumClosest(std::vector<int>& nums, int target) {
    // 先排序
    std::sort(nums.begin(), nums.end());
    assert(nums.size() >= 3);

    int closest = nums[0] + nums[1] + nums[2];
    int min_diff = INT_MAX;
    if (nums.size() == 3 || closest == target) {
      return closest;
    }

    // 遍历数组
    for (size_t i = 0; i < nums.size() - 2; ++i) {
      // 初始化双指针
      int left = i + 1;
      int right = nums.size() - 1;
      int first = nums[i];

      while (left < right) {
        int sum = first + nums[left] + nums[right];
        // 如果与 target 相等, 就直接返回.
        if (sum == target) {
          return sum;
        } else if (sum < target) {
          left += 1;
        } else if (sum > target) {
          right -= 1;
        }

        const int diff = std::abs(sum - target);
        if (diff < min_diff) {
          // 更新最小差值
          min_diff = diff;
          closest = sum;
        }
      }
    }

    return closest;
  }
};

void checkSolution(int* nums, int numsSize, int target, int expectedSum) {
  std::vector<int> vector(nums, nums + numsSize);
  int sum = Solution::threeSumClosest(vector, target);
  assert(sum == expectedSum);
}

int main() {
  int arr1[] = {-1, 2, 1, -4};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 1, 2);

  return 0;
}
