// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

class Solution {
 public:
  static int divide(int dividend, int divisor) {
  }
};


void checkSolution() {
  assert(Solution::divide(10, 3) == 3);
  assert(Solution::divide(7, -3) == -2);
}

int main() {
  checkSolution();
  return 0;
}
