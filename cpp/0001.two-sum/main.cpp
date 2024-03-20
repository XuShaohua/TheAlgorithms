// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <map>
#include <vector>

class Solution {
 public:
  static
  std::vector<int> twoSum(std::vector<int>& nums, int target) {
    if (nums.size() < 2) {
      return {};
    }

    for (int i = 0; i < nums.size() - 1; ++i) {
      for (int j = i + 1; j < nums.size(); ++j) {
        if (nums[i] + nums[j] == target) {
          return {i, j};
        }
      }
    }

    return {};
  }

  static
  std::vector<int> twoSum2(std::vector<int>& nums, int target) {
    if (nums.size() < 2) {
      return {};
    }

    std::map<int, int> cache;
    for (int i = 0; i < nums.size(); ++i) {
      const int num = nums[i];
      const auto iter = cache.find(target - num);
      if (iter != cache.cend()) {
        return {iter->second, i};
      } else {
        cache.emplace(num, i);
      }
    }

    return {};
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {2, 7, 11, 15};
    const std::vector<int> result = Solution::twoSum2(nums, 9);
    assert(result[0] == 0 && result[1] == 1);
  }

  {
    std::vector<int> nums = {3, 2, 4};
    const std::vector<int> result = Solution::twoSum2(nums, 6);
    assert(result[0] == 1 && result[1] == 2);
  }

  {
    std::vector<int> nums = {3, 3};
    const std::vector<int> result = Solution::twoSum2(nums, 6);
    assert(result[0] == 0 && result[1] == 1);
  }
}

int main() {
  checkSolution();
  return 0;
}
