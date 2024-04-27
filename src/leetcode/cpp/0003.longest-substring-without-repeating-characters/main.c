// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <stdio.h>
#include <string.h>
#include <assert.h>
#include <stdlib.h>

int lengthOfLongestSubstring(char* s) {
  int n = strlen(s);
  if (n == 0) {
    return 0;
  }
  if (n == 1) {
    return 1;
  }

  int max_val = 1;
  int len = 0;

  for (int i = 0; i < n; ++i) {
    for (int j = i + 1; j < n; ++j) {
      if (s[i] == s[j]) {
        break;
      }

      len = j - i - 1;
      if (max_val < len) {
        max_val = len;
      }
    }
  }
  return max_val;
}

void checkSolutioin(char* s, int exp) {
  int out = lengthOfLongestSubstring(s);
  printf("out: %d, exp: %d\n", out, exp);
  assert(out == exp);
}

int main() {
  char* s1 = "abcabcbb";
  checkSolutioin(s1, 3);
  char* s2 = "bbbbbb";
  checkSolutioin(s2, 1);
  char* s3 = "pwwkew";
  checkSolutioin(s3, 3);

  return 0;
}
