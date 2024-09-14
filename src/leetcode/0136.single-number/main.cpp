// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <vector>

class Solution {
 public:
  static int singleNumber(std::vector<int>& nums) {
    int current = 0;
    for (int num: nums) {
      current = current ^ num;
    }
    return current;
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {2,2,1};
    assert(Solution::singleNumber(nums) == 1);
  }
  {
    std::vector<int> nums = {4,1,2,1,2};
    assert(Solution::singleNumber(nums) == 4);
  }
  {
    std::vector<int> nums = {1};
    assert(Solution::singleNumber(nums) == 1);
  }
}

int main() {
  checkSolution();
  return 0;
}
