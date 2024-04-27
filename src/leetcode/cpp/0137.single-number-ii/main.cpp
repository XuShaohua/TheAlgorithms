// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <unordered_map>
#include <vector>

class Solution {
 public:
  static int singleNumber(std::vector<int>& nums) {
    std::unordered_map<int, int> cache;
    cache.reserve(nums.size() / 3 + 1);
    for (int num: nums) {
      cache[num] += 1;
    }
    for (auto pair: cache) {
      if (pair.second == 1) {
        return pair.first;
      }
    }
    return 0;
  }
};


void checkSolution() {
  {
    std::vector<int> nums = {2, 2, 3, 2};
    assert(Solution::singleNumber(nums) == 3);
  }

  {
    std::vector<int> nums = {0,1,0,1,0,1,99};
    assert(Solution::singleNumber(nums) == 99);
  }
}

int main() {
  checkSolution();
  return 0;
}
