// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <climits>
#include <cstdint>
#include <cstdio>

class Solution {
 public:
  // Binary search
  static bool isPerfectSquare(int num) {
    int left = 1;
    int right = num;
    while (left <= right) {
      int middle = left + (right - left) / 2;
      int64_t square = (int64_t)middle * (int64_t)middle;
      if (square > num) {
        right = middle - 1;
      } else if (square < num) {
        left = middle + 1;
      } else {
        return true;
      }
    }
    return false;
  }
};


void checkSolution() {
  assert(Solution::isPerfectSquare(1));
  assert(!Solution::isPerfectSquare(8));
  assert(Solution::isPerfectSquare(9));
  assert(!Solution::isPerfectSquare(10));
  assert(!Solution::isPerfectSquare(14));
  assert(Solution::isPerfectSquare(16));
  assert(!Solution::isPerfectSquare(INT_MAX));
}

int main() {
  checkSolution();
  return 0;
}
