// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <string>

class Solution {
 public:
  static int maxPower(std::string s) {
    int max_chars = 0;
    char c;
    int char_count = 0;
    for (int i = 0; i < s.size(); ++i) {
      if (s[i] == c) {
        char_count += 1;
      } else {
        c = s[i];
        max_chars = std::max(max_chars, char_count);
        char_count = 1;
      }
    }

    max_chars = std::max(max_chars, char_count);
    return max_chars;
  }
};


void checkSolution() {
  std::string s = "leetcode";
  assert(Solution::maxPower(s) == 2);

  std::string s2 = "abbcccddddeeeeedcba";
  assert(Solution::maxPower(s2) == 5);
}

int main() {
  checkSolution();
  return 0;
}
