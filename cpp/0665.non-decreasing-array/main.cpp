// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <vector>

class Solution {
 public:
  static
  bool checkPossibility(std::vector<int>& nums) {
    if (nums.size() <= 2) {
      return true;
    }

    int swap_count = 0;
    for (int i = 1; i < nums.size() && swap_count < 2; ++i) {
      if (nums[i - 1] >= nums[i]) {
        // swap value
        swap_count += 1;
        if (i > 2 && nums[i - 2] > nums[i]) {
          nums[i] = nums[i - 1];
        } else {
          nums[i - 1] = nums[i];
        }
      }
    }

    return swap_count <= 1;
  }
};

void checkSolution() {
  {
    std::vector<int> nums = {4, 2, 3};
    assert(Solution::checkPossibility(nums));
  }

  {
    std::vector<int> nums = {4, 2, 1};
    assert(!Solution::checkPossibility(nums));
  }

  {
    std::vector<int> nums = {3,4,2,3};
    assert(!Solution::checkPossibility(nums));
  }

  {
    std::vector<int> nums = {5,7,1,8};
    assert(Solution::checkPossibility(nums));
  }
}

int main() {
  checkSolution();
  return 0;
}
