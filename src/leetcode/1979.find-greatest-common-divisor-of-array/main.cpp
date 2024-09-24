// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <algorithm>
#include <vector>

int gcd(int a, int b) {
  assert(a >= b);
  if (b == 0) {
    return a;
  } else {
    return gcd(b, a % b);
  }
}

class Solution {
 public:
  int findGCD(std::vector<int>& nums) {
    const int min_num = *std::min_element(nums.cbegin(), nums.cend());
    const int max_num = *std::max_element(nums.cbegin(), nums.cend());
    return gcd(max_num, min_num);
  }
};

int main() {
  return 0;
}
