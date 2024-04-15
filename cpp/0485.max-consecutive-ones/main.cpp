// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <vector>

class Solution {
 public:
  static int findMaxConsecutiveOnes(std::vector<int>& nums) {
    int max_consecutive = 0;

    int count_of_1 = 0;
    for (int num : nums) {
      if (num == 1) {
        count_of_1 += 1;
      } else {
        max_consecutive = std::max(max_consecutive, count_of_1);
        count_of_1 = 0;
      }
    }

    return std::max(max_consecutive, count_of_1);
  }
};


void checkSolution() {
  std::vector<int> nums = {1,1,0,1,1,1};
  assert(Solution::findMaxConsecutiveOnes(nums) == 3);

  std::vector<int> nums2 = {1,0,1,1,0,1};
  assert(Solution::findMaxConsecutiveOnes(nums2) == 2);
}

int main() {
  checkSolution();
  return 0;
}
