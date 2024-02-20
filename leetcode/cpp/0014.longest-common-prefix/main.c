// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* longestCommonPrefix(char** strs, int strsSize) {
  int index = 0;
  int stop = 0;
  while (!stop) {
    if (strs[0][index] == '\0') {
      break;
    }

    for (int i = 1; i < strsSize; ++i) {
      if (strs[0][index] != strs[i][index]) {
        stop = 1;
        break;
      }
    }
    if (!stop) {
      index += 1;
    }
  }

  char* out = (char*)malloc(index + 1);
  assert(out);
  memcpy(out, strs[0], index);
  out[index] = '\0';
  return out;
}

void checkSolutioin(char** strs, int strsSize, char* expected) {
  char* got = longestCommonPrefix(strs, strsSize);
  printf("got: %s, expected: %s\n", got, expected);
  assert(strcmp(got, expected));
}

int main() {
  char* strs[] = {"flower","flow","flight"};
  checkSolutioin(strs, 3, "fl");

  char* strs2[] = {"dog","racecar","car"};
  checkSolutioin(strs2, 3, "");

  return 0;
}
