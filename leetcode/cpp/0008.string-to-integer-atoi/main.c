// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

int myAtoi(char* s) {
  char* ptr = s;
  long val = 0;
  int is_negative = 0;
  int has_sign = 0;
  int digit_count = 0;
  const long kIntMax = -(long)INT_MIN;

  while (*ptr != '\0') {
    char c = *ptr;
    ptr += 1;
    
    if (c == ' ' && has_sign == 0 && digit_count == 0) {
      continue;
    }

    if (c == '-' || c == '+') {
      if (has_sign || digit_count) {
        // Invalid sign.
        break;
      }

      has_sign = 1;
      if (c == '-') {
        is_negative = 1;
      }
    } else if (c >= '0' && c <= '9') {
      int digit = c - '0';
      val = val * 10 + digit;
      digit_count += 1;
      // FIXME(shaohua): overflow
      if (val > kIntMax) {
        val = kIntMax;
      }
    } else {
      // End of valid digits.
      break;
    }
  }

  if (val >= kIntMax) {
    if (is_negative) {
      val = INT_MIN;
    } else {
      val = INT_MAX;
    }
  } else if (is_negative) {
    val = -val;
  }

  return val;
}

void checkSolution(char* s, int exp) {
  int val = myAtoi(s);
  printf("val: %d, exp: %d\n", val, exp);
  assert(val == exp);
}

int main() {
  checkSolution("42", 42);
  checkSolution("  -42", -42);
  checkSolution("4193 with words", 4193);
  checkSolution("-91283472332", -2147483648);
  checkSolution(" +0 123", 0);
  checkSolution("20000000000000000000", 2147483647);
  checkSolution("-2147483647", -2147483647);
  checkSolution("-5-", -5);
  return 0;
}
