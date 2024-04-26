// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <vector>
#include <algorithm>

class Solution {
 public:
  static
  int thirdMax(std::vector<int>& nums) {
    assert(!nums.empty());

    std::sort(nums.rbegin(), nums.rend());
    auto iter = std::unique(nums.begin(), nums.end());
    nums.erase(iter, nums.end());

    if (nums.size() < 3) {
      return nums.at(0);
    }
    return nums.at(2);
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {3, 2, 1};
    int target = Solution::thirdMax(nums);
    assert(target == 1);
  }

  {
    std::vector<int> nums = {1, 2};
    int target = Solution::thirdMax(nums);
    assert(target == 2);
  }

  {
    std::vector<int> nums = {2, 2, 3, 1};
    int target = Solution::thirdMax(nums);
    assert(target == 1);
  }

  {
    std::vector<int> nums = {1, 1, 2};
    int target = Solution::thirdMax(nums);
    assert(target == 2);
  }
}

int main() {
  checkSolution();
  return 0;
}
