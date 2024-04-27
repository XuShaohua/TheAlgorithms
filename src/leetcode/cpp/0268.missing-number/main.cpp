// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <bitset>
#include <vector>

class Solution {
 public:
  static int missingNumber(std::vector<int>& nums) {
    constexpr size_t MAX_LEN = 10000;
    std::bitset<MAX_LEN> bits;
    for (int num: nums) {
      bits.set(num);
    }
    for (size_t i = 0; i < nums.size(); ++i) {
      if (!bits[i]) {
        return i;
      }
    }
    return nums.size();
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {3,0,1};
    assert(Solution::missingNumber(nums) == 2);
  }
 
  {
    std::vector<int> nums = {9,6,4,2,3,5,7,0,1};
    assert(Solution::missingNumber(nums) == 8);
  }
}

int main() {
  checkSolution();
  return 0;
}
