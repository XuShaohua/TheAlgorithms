// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <vector>

class Solution {
 public:
  static
  int maximumProduct(std::vector<int>& nums) {
    assert(nums.size() >= 3);

    std::sort(nums.begin(), nums.end());
    const auto len = nums.size();
    std::vector<int> max_vals = {
      // -3, -2, -1
      nums.at(len - 3) * nums.at(len - 2) * nums.at(len - 1),
      // 0, -2, -1
      nums.at(0) * nums.at(len - 2) * nums.at(len - 1),
      // 0, 1, -1
      nums.at(0) * nums.at(1) * nums.at(len - 1),
    };
    std::sort(max_vals.begin(), max_vals.end());
    return max_vals.at(max_vals.size() - 1);
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {-100,-98,-1,2,3,4};
  }
}

int main() {
  checkSolution();
  return 0;
}
