// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int reverse(int x) {
  int rev = 0;
  while (x != 0) {
    if (rev > INT_MAX / 10 || rev < INT_MIN / 10) {
      return 0;
    }
    rev = rev * 10 + x % 10;
    x /= 10;
  }

  return rev;
}

void checkSolution(int x, int expected_rev_x) {
  int rev_x = reverse(x);
  printf("x: %d, rev_x: %d, expected: %d\n", x, rev_x, expected_rev_x);
  assert(rev_x == expected_rev_x);
}

int main() {
  checkSolution(123, 321);
  checkSolution(-123, -321);
  checkSolution(120, 21);
  checkSolution(1534236469, 0);
  checkSolution(-2147483648, 0);
  checkSolution(-2147483412, -2143847412);

  return 0;
}
