// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <set>
#include <vector>

class Solution {
public:
  static
  std::vector<std::vector<int>> fourSum(std::vector<int>& nums, int target) {
    int len = nums.size();
    std::vector<std::vector<int>> result;
    std::sort(nums.begin(), nums.end());

    for (int i = 0; i < len-3; ++i) {
      if (i > 0 && nums[i] == nums[i-1]) {
        continue;
      }

      for (int j = i+1; j < len-2; ++j) {
        if (j > i + 1 && nums[j] == nums[j-1]) {
          continue;
        }

        int left = j + 1;
        int right = len - 1;
        long remains = (long)target - (long)nums[i] - (long)nums[j];
        while (left < right) {
          long sum = (long)nums[left] + (long)nums[right];
          if (remains < sum) {
            right -= 1;
          } else if (remains > sum) {
            left += 1;
          } else {
            result.push_back({nums[i], nums[j], nums[left], nums[right]});
            while (left < right && nums[left] == nums[left+1]) {
              left += 1;
            }
            while (left < right && nums[right] == nums[right-1]) {
              right -= 1;
            }
            left += 1;
            right -= 1;
          }
        }
      }
    }

    return result;
  }
};

void checkSolution(int* arr, int arr_len, int target, int out_len) {
  std::vector<int> nums{arr, arr + arr_len};
  std::vector<std::vector<int>> result = Solution::fourSum(nums, target);
  for (const auto& entry : result) {
    printf("[%d, %d, %d, %d]\n", entry[0], entry[1], entry[2], entry[3]);
  }
  printf("result size: %d, expected; %d\n", result.size(), out_len);
  assert(result.size() == out_len);
}

int main() {
  int arr1[] = {1, 0, -1, 0, -2, 2}; 
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 0, 3);

  int arr2[] = {2, 2, 2, 2, 2};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 8, 1);

  int arr3[] = {2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2};
  checkSolution(arr3, sizeof(arr3) / sizeof(arr3[0]), 8, 1);

  int arr4[] = {1, -2, -5, -4, -3, 3, 3, 5};
  checkSolution(arr4, sizeof(arr4) / sizeof(arr4[0]), -11, 1);

  return 0;
}
