// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int climbStairs(int n) {
  if (n == 0 || n == 1) {
    return 1;
  }
  return climbStairs(n - 1) + climbStairs(n - 2);
}

int climbStairs2(int n) {
  if (n == 0 || n == 1) {
    return 1;
  }
  int f1 = 1;
  int f2 = 1;
  int f3 = f1 + f2;
  for (int i = 2; i <= n; ++i) {
    f3 = f1 + f2;
    f1 = f2;
    f2 = f3;
  }
  return f3;
}

void checkSolution(int n, int expected) {
  int out = climbStairs2(n);
  printf("out: %d, exp: %d\n", out, expected);
  assert(out == expected);
}

int main() {
  checkSolution(2, 2);
  checkSolution(3, 3);
  return 0;
}
