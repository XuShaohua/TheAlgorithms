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
    std::sort(nums.begin(), nums.end());
    int min_diff = INT_MAX;
    int result = 0;

    for (size_t i = 0; i < nums.size() - 2; ++i) {
      int left = i + 1;
      int right = nums.size() - 1;
      int first = nums[i];

      while (left < right) {
        int sum = first + nums[left] + nums[right];
        if (sum == target) {
          return sum;
        }

        int d = std::abs(sum - target);
        if (d < min_diff) {
          min_diff = d;
          result = sum;
        }
        if (sum < target) {
          left += 1;
        } else if (sum > target) {
          right -= 1;
        }
      }
    }

    return result;
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
