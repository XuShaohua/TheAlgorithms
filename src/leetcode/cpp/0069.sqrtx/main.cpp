// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>
#include <climits>
#include <cstdint>

class Solution {
 public:
  static int mySqrt1(int x) {
    long x1 = x;
    long high = x1;
    long low = 0;
    long middle = 0;
    long ans = 0;

    while (low <= high) {
      middle = low + (high - low) / 2.0;
      long mid_sq = middle * middle;

      if (mid_sq > x1) {
        high = middle - 1;
      } else if (mid_sq < x1) {
        ans = middle;
        low = middle + 1;
      } else {
        return middle;
      }
    }

    return ans;
  }

  static int mySqrt(int x) {
    int high = x;
    int low = 0;
    int ans = 0;

    while (low <= high) {
      int64_t middle = low + (high - low) / 2.0;
      int64_t mid_sq = middle * middle;

      if (mid_sq > x) {
        high = middle - 1;
      } else {
        ans = middle;
        low = middle + 1;
      }
    }

    return ans;
  }

  static int mySqrt3(int x) {
    long x1 = x;
    int high = x1;
    long low = 0;
    long middle = 0;
    long ans = 0;

    while (low <= high) {
      middle = low + (high - low) / 2.0;
      long mid_sq = middle * middle;

      if (mid_sq > x1) {
        high = middle - 1;
      } else if (mid_sq < x1) {
        ans = middle;
        low = middle + 1;
      } else {
        return middle;
      }
    }

    return ans;
  }
};


void checkSolution() {
  assert(Solution::mySqrt(36) == 6);
  assert(Solution::mySqrt(4) == 2);
  assert(Solution::mySqrt(10) == 3);
  assert(Solution::mySqrt(8) == 2);
  assert(Solution::mySqrt(9) == 3);
  assert(Solution::mySqrt(0) == 0);
  assert(Solution::mySqrt(INT_MAX) == 46340);
  assert(Solution::mySqrt(1) == 1);
  assert(Solution::mySqrt(2) == 1);
  assert(Solution::mySqrt(3) == 1);
}

int main() {
  checkSolution();
  return 0;
}
